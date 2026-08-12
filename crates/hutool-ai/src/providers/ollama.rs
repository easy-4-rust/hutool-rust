//! `OllamaProvider` 与 Java `OllamaService`/`OllamaServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.ollama.*`（6 个类）
//!
//! - Java `OllamaProvider`（SPI 工厂）→ 本文件的 `OllamaProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `OllamaServiceImpl` 的 `chat` / `generate` / `embeddings` / `showModel` /
//!   `pullModel` / `deleteModel` / `copyModel` 等端点请求由通用 `ProviderService` +
//!   `Operation` 枚举承载（`/api/chat`、`/api/generate`、`/api/embeddings`、
//!   `/api/show`、`/api/pull`、`/api/delete`、`/api/copy`，含 SSE 流式）；
//! - `OllamaConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载
//!   （`http://localhost:11434`），`OllamaCommon` 的 `OllamaFormat`/`Options`
//!   已迁入 `models.rs`。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `OllamaProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct OllamaProvider;

impl AIServiceProvider for OllamaProvider {
    fn service_name(&self) -> ModelName {
        ModelName::Ollama
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new OllamaServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `OllamaProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn ollama_service_name() -> ModelName {
    ModelName::Ollama
}

/// Java `OllamaConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type OllamaConfig = BaseConfig;

/// Java `OllamaCommon` 的 Rust 等价（枚举已迁入 `models.rs`）。
#[allow(dead_code)]
pub struct OllamaCommon;

/// Ollama 响应格式枚举镜像（`OllamaFormat` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::OllamaFormat;

/// Ollama 生成选项键镜像（`OllamaOptions` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::OllamaOptions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_provider_name_and_create() {
        assert_eq!(OllamaProvider.service_name(), ModelName::Ollama);
        assert_eq!(ollama_service_name(), ModelName::Ollama);
        let config = BaseConfig::with_api_key(ModelName::Ollama, "key").unwrap();
        let service = OllamaProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn ollama_defaults_match_java_config() {
        // Java OllamaConfig 使用本地 localhost:11434
        let (url, model) = ModelName::Ollama.defaults();
        assert_eq!(url, "http://localhost:11434");
        assert_eq!(model, "qwen3:32b");
    }
}
