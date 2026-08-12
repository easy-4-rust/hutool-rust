//! `HutoolProvider` 与 Java `HutoolService`/`HutoolServiceImpl` 的 Rust 承载。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.hutool.*`（5 个类）
//!
//! - Java `HutoolProvider`（SPI 工厂）→ 本文件的 `HutoolProvider`，经
//!   `AIServiceFactory::registry()` 注册后按名称路由；
//! - Java `HutoolServiceImpl` 的 `chat` / `chatVision` / `imagesGenerations` /
//!   `textToSpeech` / `videoTasks` 等端点请求由通用 `ProviderService` + `Operation`
//!   枚举承载（对齐 Hutool 网关 `api.hutool.cn/ai/api` 的端点行为，含 SSE 流式）；
//! - `HutoolConfig` 的默认 API 地址与模型由 `ModelName::defaults()` 承载，
//!   `HutoolCommon` 的 `HutoolVision`/`HutoolSpeech`/`HutoolVideo` 已迁入 `models.rs`
//!   （视频参数以 `VideoParameter` 表达 `--rt`/`--dur`/`--fps`/`--rs`/`--wm` 键值对）。

use crate::core::{AIServiceProvider, BaseConfig, ProviderService};
use crate::{AIService, ModelName, ProviderError};
use std::sync::Arc;

/// Java `HutoolProvider` 的 Rust 等价物。
#[derive(Debug, Default)]
pub struct HutoolProvider;

impl AIServiceProvider for HutoolProvider {
    fn service_name(&self) -> ModelName {
        ModelName::Hutool
    }

    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        // Java `new HutoolServiceImpl(config)`；Rust 侧统一由 ProviderService 承载
        // 厂商端点路由（Operation 枚举），对齐 BaseAIService 的 sendGet/sendPost 行为。
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}

/// Java `HutoolProvider.getServiceName()` 的镜像入口。
#[allow(dead_code)]
#[must_use]
pub fn hutool_service_name() -> ModelName {
    ModelName::Hutool
}

/// Java `HutoolConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
#[allow(dead_code)]
pub type HutoolConfig = BaseConfig;

/// Java `HutoolCommon` 的 Rust 等价（枚举已迁入 `models.rs`）。
#[allow(dead_code)]
pub struct HutoolCommon;

/// Hutool 视觉细节枚举镜像（`VisionDetail` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::HutoolVision;

/// Hutool 音色枚举镜像（`SpeechVoice` 定义于 `models.rs`）。
#[allow(unused_imports)]
pub use crate::models::HutoolSpeech;

/// Hutool 视频参数键常量（对齐 Java `HutoolVideo` 的 `--rt`/`--dur`/`--fps`/`--rs`/`--wm`）。
#[allow(dead_code)]
pub struct HutoolVideo;

#[allow(dead_code)]
impl HutoolVideo {
    /// 宽高比参数键 `--rt`。
    pub const RATIO: &'static str = "--rt";
    /// 视频时长参数键 `--dur`。
    pub const DURATION: &'static str = "--dur";
    /// 帧率参数键 `--fps`。
    pub const FPS: &'static str = "--fps";
    /// 分辨率参数键 `--rs`。
    pub const RESOLUTION: &'static str = "--rs";
    /// 水印参数键 `--wm`。
    pub const WATERMARK: &'static str = "--wm";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hutool_provider_name_and_create() {
        assert_eq!(HutoolProvider.service_name(), ModelName::Hutool);
        assert_eq!(hutool_service_name(), ModelName::Hutool);
        let config = BaseConfig::with_api_key(ModelName::Hutool, "key").unwrap();
        let service = HutoolProvider.create(config).unwrap();
        assert!(format!("{service:?}").contains("ProviderService"));
    }

    #[test]
    fn hutool_defaults_and_video_keys() {
        // Java HutoolConfig 使用 api.hutool.cn 网关
        let (url, model) = ModelName::Hutool.defaults();
        assert_eq!(url, "https://api.hutool.cn/ai/api");
        assert_eq!(model, "hutool");
        // Java HutoolVideo 参数键
        assert_eq!(HutoolVideo::RATIO, "--rt");
        assert_eq!(HutoolVideo::DURATION, "--dur");
        assert_eq!(HutoolVideo::FPS, "--fps");
        assert_eq!(HutoolVideo::RESOLUTION, "--rs");
        assert_eq!(HutoolVideo::WATERMARK, "--wm");
    }
}
