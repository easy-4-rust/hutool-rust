//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.hutool.*`（5 个类）
//!
//! Rust 侧尚未迁移 Hutool 自家网关服务；本文件仅保留路径以保证 1:1 结构对齐。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `HutoolProvider` / `HutoolServiceImpl`。
pub fn hutool_service_name() -> ModelName {
    ModelName::Hutool
}

/// 🚫 Rust 侧未实现。Java `HutoolProvider.create`。
pub fn hutool_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("Hutool provider is not implemented in the Rust port yet")
}