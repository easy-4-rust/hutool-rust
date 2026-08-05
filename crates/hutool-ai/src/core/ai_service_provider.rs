//! `AIServiceProvider` SPI 接口。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.AIServiceProvider`
//!
//! Rust 侧通过 `registry()` 注册中心完成 Java 端 SPI 等价物。

use super::ai_service::AIService;
use super::base_config::BaseConfig;
use crate::{ModelName, ProviderError};
use std::fmt;
use std::sync::Arc;

/// Provider SPI 注册入口契约。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.AIServiceProvider`
pub trait AIServiceProvider: fmt::Debug + Send + Sync {
    /// 厂商标识。
    fn service_name(&self) -> ModelName;

    /// 根据配置创建服务实例。
    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError>;
}

/// Java 端 `AIConfigRegistry` 的 Rust 等价物。
///
/// Java 端通过 `ServiceLoaderUtil.load(AIConfig.class)` 反射注册；
/// Rust 端直接复用 `AIServiceFactory::registry()`，避免重复状态。
/// 该别名保留 Java 命名，作为镜像占位，暂无调用方。
#[allow(dead_code)]
pub type AIConfigRegistry = crate::ai_service_factory::ProviderRegistry;
