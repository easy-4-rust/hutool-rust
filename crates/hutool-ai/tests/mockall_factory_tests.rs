//! mockall 驱动的 `AIServiceFactory`/`ProviderRegistry` 分发测试。
//!
//! 用 `mock!` 生成 `AiProvider`（`AIServiceProvider`）与 `AiService`
//! （`AIService`），验证注册/查找/创建分发与错误路径，无需真实 provider 与 HTTP。
//! 对齐 Java 语义：`ServiceLoader` SPI 注册 → `AIServiceFactory.getAIService`
//! 按厂商名路由 → provider 创建服务实例；注册同名 provider 等价于 SPI 覆盖。

use std::fmt;
use std::sync::Arc;

use hutool_ai::{AIConfig, AIService, AIServiceProvider, BaseConfig};
use hutool_ai::{AIException, ModelName, ProviderError};
use hutool_ai::{AIResponse, Operation, StreamCallback};
use hutool_ai::{AIServiceFactory, ProviderRegistry, provider_registry};
use mockall::mock;
use secrecy::ExposeSecret;

mock! {
    /// `AIServiceProvider` 的 mock：`service_name` 与 `create` 均可设置期望。
    pub AiProvider {}
    impl fmt::Debug for AiProvider {
        fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
            write!(f, "AiProvider")
        }
    }
    impl AIServiceProvider for AiProvider {
        fn service_name(&self) -> ModelName;
        fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError>;
    }
}

mock! {
    /// `AIService` 的 mock：`execute` 与 `execute_stream` 可设置返回。
    pub AiService {}
    impl fmt::Debug for AiService {
        fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
            write!(f, "AiService")
        }
    }
    #[async_trait::async_trait]
    impl AIService for AiService {
        async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError>;
        async fn execute_stream(
            &self,
            operation: Operation,
            callback: StreamCallback,
        ) -> Result<(), ProviderError>;
    }
}

/// 独立注册表：注册/查找大小写归一（Java SPI 按类名小写匹配）、未命中返回 None。
#[test]
fn registry_register_lookup_case_insensitive() {
    let registry = ProviderRegistry::new();
    let mut hutool = MockAiProvider::new();
    hutool.expect_service_name().returning(|| ModelName::Hutool);
    let mut ollama = MockAiProvider::new();
    ollama.expect_service_name().returning(|| ModelName::Ollama);

    registry.register(Arc::new(hutool));
    registry.register(Arc::new(ollama));

    // 查询按 service 名小写归一（对齐 Java SPI 的厂商名解析）
    assert!(registry.lookup("hutool").is_some());
    assert!(registry.lookup("HUTOOL").is_some());
    assert!(registry.lookup("ollama").is_some());
    assert!(registry.lookup("gemini").is_none());
}

/// 注册同名 provider 覆盖旧实现（等价 Java SPI 后加载覆盖先加载）。
#[test]
fn registry_register_overwrites_same_name() {
    let registry = ProviderRegistry::new();
    let mut first = MockAiProvider::new();
    first.expect_service_name().returning(|| ModelName::OpenAi);
    first.expect_create().never();
    registry.register(Arc::new(first));

    let mut second = MockAiProvider::new();
    second.expect_service_name().returning(|| ModelName::OpenAi);
    second
        .expect_create()
        .once()
        .returning(|_| Ok(Arc::new(MockAiService::new()) as Arc<dyn AIService>));
    registry.register(Arc::new(second));

    // lookup 后 create 只落在覆盖者身上（first 的 never() 期望保证）
    let provider = registry.lookup("openai").expect("registered");
    provider
        .create(BaseConfig::with_api_key(ModelName::OpenAi, "key").unwrap())
        .expect("create");
}

/// 全局 registry 注册 mock provider 覆盖内建实现后，
/// `AIServiceFactory::get_ai_service` 按厂商名路由到 mock（SPI 等价）。
#[tokio::test]
async fn factory_routes_to_mock_provider() {
    let mut provider = MockAiProvider::new();
    provider
        .expect_service_name()
        .returning(|| ModelName::OpenAi);
    // 验证 create 收到正确的厂商配置（对齐 Java getAIService 传递 BaseConfig）
    provider
        .expect_create()
        .withf(|config| {
            config.model_name() == ModelName::OpenAi && config.api_key().expose_secret() == "k1"
        })
        .returning(|_| {
            let mut service = MockAiService::new();
            service.expect_execute().returning(|op| {
                assert!(matches!(op, Operation::Chat { .. }), "expected chat op");
                Ok(AIResponse::Bytes(b"mock".to_vec()))
            });
            Ok(Arc::new(service) as Arc<dyn AIService>)
        });
    provider_registry().register(Arc::new(provider));

    let service = AIServiceFactory::get_ai_service(
        BaseConfig::with_api_key(ModelName::OpenAi, "k1").unwrap(),
    )
    .expect("factory routes to mock provider");
    let response = service
        .execute(Operation::Chat {
            messages: Vec::new(),
        })
        .await
        .expect("mock execute");
    assert_eq!(response.into_bytes(), b"mock");
}

/// provider 创建失败（Java `ProviderException`）→ `AIException` 传播。
#[test]
fn factory_propagates_provider_error() {
    let mut provider = MockAiProvider::new();
    provider
        .expect_service_name()
        .returning(|| ModelName::Ollama);
    provider
        .expect_create()
        .once()
        .returning(|_| Err(ProviderError::EmptyChoices));
    provider_registry().register(Arc::new(provider));

    let error = AIServiceFactory::get_ai_service(
        BaseConfig::with_api_key(ModelName::Ollama, "k2").unwrap(),
    )
    .expect_err("provider error propagates");
    assert!(matches!(error, AIException::Message(_)));
    assert!(error.to_string().contains("no chat choices"));
}

/// `AiService` 独立验证：execute 返回与 `execute_stream` 回调路径。
#[tokio::test]
async fn mock_service_execute_and_stream() {
    let mut service = MockAiService::new();
    service
        .expect_execute()
        .withf(|op| matches!(op, Operation::Beta { prompt } if prompt == "probe"))
        .returning(|_| Ok(AIResponse::Json(serde_json::json!({"ok": true}))));
    service
        .expect_execute_stream()
        .once()
        .returning(|_, _| Ok(()));

    let response = service
        .execute(Operation::Beta {
            prompt: "probe".to_string(),
        })
        .await
        .expect("execute");
    assert_eq!(response.into_text(), "{\"ok\":true}");
    service
        .execute_stream(
            Operation::Beta {
                prompt: "probe".to_string(),
            },
            Arc::new(|_: String| {}),
        )
        .await
        .expect("execute_stream");
}
