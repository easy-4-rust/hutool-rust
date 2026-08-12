//! `GrokProvider` 与 Java `GrokService`/`GrokServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.grok.*`（6 个类）
//!
//! - Java `GrokProvider`（SPI 工厂）→ 本文件的 `GrokProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `GrokServiceImpl` 的 `chat` / `chatVision` / `message` / `models` /
//!   `languageModels` / `imagesGenerations` / `tokenizeText` / `deferredCompletion`
//!   等端点请求由通用 `ProviderService` + `Operation` 枚举承载
//!   （`Message`/`ListModels`/`GetModel`/`ListLanguageModels`/`GetLanguageModel`/
//!   `GenerateImage`/`Tokenize`/`DeferredCompletion`，含 SSE 流式）；
//! - `GrokConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载，
//!   `GrokCommon` 的 `GrokVision` 枚举已迁入 `models.rs`。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `GrokProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct GrokProvider;

impl AIServiceProvider for GrokProvider {
    fn service_name(&self) -> ModelName {
        ModelName::Grok
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new GrokServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `GrokProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn grok_service_name() -> ModelName {
    ModelName::Grok
}

/// Java `GrokConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type GrokConfig = BaseConfig;

/// Java `GrokCommon` 的 Rust 等价（枚举已迁入 `models.rs`）。
#[allow(dead_code)]
pub struct GrokCommon;

/// Grok 视觉细节枚举镜像（`VisionDetail` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GrokVision;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grok_provider_name_and_create() {
        assert_eq!(GrokProvider.service_name(), ModelName::Grok);
        assert_eq!(grok_service_name(), ModelName::Grok);
        let config = BaseConfig::with_api_key(ModelName::Grok, "key").unwrap();
        let service = GrokProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn grok_defaults_match_java_config() {
        // Java GrokConfig 使用 api.x.ai 地址
        let (url, model) = ModelName::Grok.defaults();
        assert_eq!(url, "https://api.x.ai/v1");
        assert!(model.starts_with("grok-"));
    }
}
