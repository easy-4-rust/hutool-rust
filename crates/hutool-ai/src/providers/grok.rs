//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.grok.*`（6 个类）
//!
//! Rust 侧尚未迁移 `GrokService`；本文件仅保留路径以保证 1:1 结构对齐。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `GrokProvider` / `GrokServiceImpl`。
pub fn grok_service_name() -> ModelName {
    ModelName::Grok
}

/// 🚫 Rust 侧未实现。Java `GrokProvider.create`。
pub fn grok_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("Grok provider is not implemented in the Rust port yet")
}