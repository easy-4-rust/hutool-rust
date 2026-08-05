//! `GeminiProvider` 与 Java `GeminiService`/`GeminiServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.gemini.*`（6 个类）
//!
//! - Java `GeminiProvider`（SPI 工厂）→ 本文件的 `GeminiProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `GeminiServiceImpl` 的 `chat` / `chatMultimodal` / `chatJson` /
//!   `imagesGenerations` / `videoTasks` / `upload` 等端点请求由通用 `ProviderService` +
//!   `Operation` 枚举承载（`/models/{model}:generateContent`、`:predict` 等，含 SSE 流式）；
//! - `GeminiConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载，
//!   `GeminiCommon` 的枚举（`GeminiImageCount`/`GeminiImageSize`/`GeminiAspectRatio`/
//!   `GeminiPersonGeneration`/`GeminiDurationSeconds`/`GeminiVoice`）已迁入 `models.rs`。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `GeminiProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct GeminiProvider;

impl AIServiceProvider for GeminiProvider {
    fn service_name(&self) -> ModelName {
        ModelName::Gemini
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new GeminiServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `GeminiProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn gemini_service_name() -> ModelName {
    ModelName::Gemini
}

/// Java `GeminiConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type GeminiConfig = BaseConfig;

/// Java `GeminiCommon` 的 Rust 等价（枚举已迁入 `models.rs`）。
#[allow(dead_code)]
pub struct GeminiCommon;

/// Gemini 图片数量枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiImageCount;

/// Gemini 图片尺寸枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiImageSize;

/// Gemini 宽高比枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiAspectRatio;

/// Gemini 人物生成枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiPersonGeneration;

/// Gemini 视频时长枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiDurationSeconds;

/// Gemini 音色枚举镜像（定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::GeminiVoice;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gemini_provider_name_and_create() {
        assert_eq!(GeminiProvider.service_name(), ModelName::Gemini);
        assert_eq!(gemini_service_name(), ModelName::Gemini);
        let config = BaseConfig::with_api_key(ModelName::Gemini, "key").unwrap();
        let service = GeminiProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn gemini_defaults_match_java_config() {
        // Java GeminiConfig 使用 generativelanguage.googleapis.com 地址
        let (url, model) = ModelName::Gemini.defaults();
        assert!(url.contains("generativelanguage.googleapis.com"));
        assert!(model.starts_with("gemini-"));
    }
}
