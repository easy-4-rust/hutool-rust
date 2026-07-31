//! `AIServiceFactory` 对象。
//!
//! 对齐 Java 来源: `cn.hutool.ai.AIServiceFactory`
//!
//! Java 端通过 SPI（`ServiceLoader`）扫描 `AIServiceProvider` 实现；Rust 端没有 JVM SPI，
//! 改为静态注册表 + 缺省委托 `ProviderService`。

use crate::core::ai_config::AIConfig;
use crate::core::{AIService, AIServiceProvider, BaseConfig, ProviderService};
use crate::AIException;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 提供商 SPI 注册中心。
///
/// Rust 侧 `AIServiceFactory` 的核心数据结构；与 Java 侧 SPI 行为等价。
#[derive(Debug, Default)]
pub struct ProviderRegistry {
    providers: RwLock<HashMap<String, Arc<dyn AIServiceProvider>>>,
}

impl ProviderRegistry {
    /// 创建空注册表。
    #[must_use]
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
        }
    }

    /// 注册一个 `AIServiceProvider`，service 名已统一小写。
    pub fn register(&self, provider: Arc<dyn AIServiceProvider>) {
        let mut guard = self.providers.write().expect("provider registry poisoned");
        guard.insert(provider.service_name().value().to_ascii_lowercase(), provider);
    }

    /// 查询一个 service 名对应的 provider。
    #[must_use]
    pub fn lookup(&self, name: &str) -> Option<Arc<dyn AIServiceProvider>> {
        let guard = self.providers.read().expect("provider registry poisoned");
        guard.get(&name.to_ascii_lowercase()).cloned()
    }
}

/// AIServiceFactory 静态注册表。
///
/// Rust 侧全局单例；首次调用时会自动注册内置 OpenAI-compatible provider。
pub fn registry() -> &'static ProviderRegistry {
    static REGISTRY: std::sync::OnceLock<ProviderRegistry> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| {
        let registry = ProviderRegistry::new();
        registry.register(builtin_provider());
        registry
    })
}

/// 内置 OpenAI-compatible provider（懒加载）。
fn builtin_provider() -> Arc<dyn AIServiceProvider> {
    #[derive(Debug)]
    struct BuiltinProvider;
    impl crate::core::AIServiceProvider for BuiltinProvider {
        fn service_name(&self) -> crate::ModelName {
            crate::ModelName::OpenAi
        }
        fn create(
            &self,
            config: BaseConfig,
        ) -> Result<Arc<dyn crate::core::AIService>, crate::ProviderError> {
            ProviderService::new(config)
                .map(|s| Arc::new(s) as Arc<dyn crate::core::AIService>)
        }
    }
    Arc::new(BuiltinProvider)
}

/// `AIServiceFactory` 静态门面。
///
/// 对齐 Java 来源: `cn.hutool.ai.AIServiceFactory`
pub struct AIServiceFactory;

impl AIServiceFactory {
    /// 根据 `BaseConfig` 解析 provider 并创建 `AIService`。
    ///
    /// 对齐 Java 行为：未注册对应厂商时返回 `IllegalArgumentException`。
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, AIException> {
        let provider_name = config.model_name().value();
        // Try registry first; fall back to builtin ProviderService so the original
        // Hutool-agnostic flow continues to work even without explicit registration.
        if let Some(provider) = registry().lookup(provider_name) {
            return provider.create(config).map_err(AIException::from);
        }
        match ProviderService::new(config) {
            Ok(service) => Ok(Arc::new(service) as Arc<dyn AIService>),
            Err(error) => Err(AIException::Message(error.to_string())),
        }
    }
}

/// 注册内置 `ProviderService`（OpenAI 兼容）作为兜底实现。
/// 保留为 Java 镜像 API，仅在测试中使用。
#[allow(dead_code)]
pub fn register_builtin_provider() {
    #[derive(Debug)]
    struct BuiltinProvider;
    impl crate::core::AIServiceProvider for BuiltinProvider {
        fn service_name(&self) -> crate::ModelName {
            crate::ModelName::OpenAi
        }
        fn create(
            &self,
            config: BaseConfig,
        ) -> Result<Arc<dyn crate::core::AIService>, crate::ProviderError> {
            ProviderService::new(config).map(|s| Arc::new(s) as Arc<dyn crate::core::AIService>)
        }
    }
    registry().register(Arc::new(BuiltinProvider));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ProviderError;

    #[test]
    fn factory_returns_ai_service_for_builtin_provider() {
        register_builtin_provider();
        let config = BaseConfig::with_api_key(crate::ModelName::OpenAi, "key").unwrap();
        let service = AIServiceFactory::get_ai_service(config).unwrap();
        let _: Arc<dyn AIService> = service;
    }

    #[test]
    fn factory_rejects_unsupported_provider() {
        // Hutool provider has no builtin; factory must error.
        let config = BaseConfig::with_api_key(crate::ModelName::Hutool, "key").unwrap();
        let err = AIServiceFactory::get_ai_service(config).unwrap_err();
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn provider_error_converts_to_ai_exception() {
        let err: AIException = ProviderError::EmptyChoices.into();
        assert!(err.to_string().contains("no chat choices"));
    }
}