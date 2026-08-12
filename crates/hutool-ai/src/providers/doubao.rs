//! `DoubaoProvider` 与 Java `DoubaoService`/`DoubaoServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.doubao.*`（6 个类）
//!
//! - Java `DoubaoProvider`（SPI 工厂）→ 本文件的 `DoubaoProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `DoubaoServiceImpl` 的 `chat` / `botsChat` / `batchChat` / `createContext` /
//!   `contextChat` / `embeddingText` / `embeddingVision` / `tokenization` / `imagesGenerations`
//!   / `videoTasks` 等端点请求由通用 `ProviderService` + `Operation` 枚举承载
//!   （`BotChat`/`BatchChat`/`CreateContext`/`ContextChat`/`EmbedText`/`EmbedVision`/
//!   `Tokenize`/`GenerateImage`/`CreateVideo`/`GetVideo`，含 SSE 流式）；
//! - `DoubaoConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载，
//!   `DoubaoCommon` 的枚举（`DoubaoContext`/`DoubaoVision`/`DoubaoVideo`）已迁入 `models.rs`。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `DoubaoProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct DoubaoProvider;

impl AIServiceProvider for DoubaoProvider {
    fn service_name(&self) -> ModelName {
        ModelName::Doubao
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new DoubaoServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `DoubaoProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn doubao_service_name() -> ModelName {
    ModelName::Doubao
}

/// Java `DoubaoConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type DoubaoConfig = BaseConfig;

/// Java `DoubaoCommon` 的 Rust 等价（枚举已迁入 `models.rs`）。
#[allow(dead_code)]
pub struct DoubaoCommon;

/// Doubao 上下文模式枚举镜像（`DoubaoContext` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::DoubaoContext;

/// Doubao 视觉细节枚举镜像（`VisionDetail` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::DoubaoVision;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubao_provider_name_and_create() {
        assert_eq!(DoubaoProvider.service_name(), ModelName::Doubao);
        assert_eq!(doubao_service_name(), ModelName::Doubao);
        let config = BaseConfig::with_api_key(ModelName::Doubao, "key").unwrap();
        let service = DoubaoProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn doubao_defaults_match_java_config() {
        // Java DoubaoConfig 使用火山引擎 ark 地址与 doubao 默认模型
        let (url, model) = ModelName::Doubao.defaults();
        assert!(url.contains("volces.com"));
        assert!(model.starts_with("doubao-"));
    }
}
