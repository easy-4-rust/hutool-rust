//! `AIServiceFactory` 对象。
//! 对齐 Java 来源: `cn.hutool.ai.AIServiceFactory`
//! 说明: Java 侧通过 SPI 查找 provider，Rust 侧当前固定委托给 `ProviderService`。

use super::{AIService, BaseConfig, ProviderService};
use crate::ProviderError;
use std::sync::Arc;

/// Hutool 兼容层内置服务工厂。
///
/// 对齐 Java 来源: `cn.hutool.ai.AIServiceFactory`
pub struct AIServiceFactory;

impl AIServiceFactory {
    /// 创建内置 provider service。
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        ProviderService::new(config).map(|service| Arc::new(service) as Arc<dyn AIService>)
    }
}
