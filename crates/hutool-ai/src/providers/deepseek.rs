//! `DeepSeekProvider` 与 Java `DeepSeekService`/`DeepSeekServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.deepseek.*`（5 个类）
//!
//! - Java `DeepSeekProvider`（SPI 工厂）→ 本文件的 `DeepSeekProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `DeepSeekServiceImpl` 的 `chat` / `beta` / `models` / `balance` 端点请求由
//!   通用 `ProviderService` + `Operation` 枚举承载（`/chat/completions`、`/beta/completions`、
//!   `/models`、`/user/balance`，含 SSE 流式），与 Java `sendPost` / `sendGet` 行为对齐；
//! - `DeepSeekConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载，
//!   `DeepSeekCommon` / `DeepSeekConfig` 保留为镜像别名。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `DeepSeekProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct DeepSeekProvider;

impl AIServiceProvider for DeepSeekProvider {
    fn service_name(&self) -> ModelName {
        ModelName::DeepSeek
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new DeepSeekServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `DeepSeekProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn deepseek_service_name() -> ModelName {
    ModelName::DeepSeek
}

/// Java `DeepSeekConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type DeepSeekConfig = BaseConfig;

/// Java `DeepSeekCommon` 的 Rust 等价（当前无额外枚举，保留镜像位置）。
#[allow(dead_code)]
pub struct DeepSeekCommon;

/// Java `DeepSeekService` 接口：`chat`/`beta`/`models`/`balance` 由
/// `AIService::execute(Operation)` 承载（`Beta`/`ListModels`/`Balance` 变体）。
#[allow(dead_code)]
pub struct DeepSeekService;

/// `DeepSeek` 推理模型枚举镜像（`DeepSeekModel` 定义于 `models.rs`）。
/// 镜像保留 Java 命名，暂无调用方。
#[allow(unused_imports)]
pub use crate::models::DeepSeekModel as DeepSeekReasoning;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deepseek_provider_name_and_create() {
        assert_eq!(DeepSeekProvider.service_name(), ModelName::DeepSeek);
        assert_eq!(deepseek_service_name(), ModelName::DeepSeek);
        let config = BaseConfig::with_api_key(ModelName::DeepSeek, "key").unwrap();
        let service = DeepSeekProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn deepseek_defaults_match_java_config() {
        // Java DeepSeekConfig: API_URL=https://api.deepseek.com, DEFAULT_MODEL=deepseek-chat
        let (url, model) = ModelName::DeepSeek.defaults();
        assert_eq!(url, "https://api.deepseek.com");
        assert_eq!(model, "deepseek-chat");
    }
}
