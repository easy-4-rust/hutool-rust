//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.doubao.*`（6 个类）
//!
//! Rust 侧尚未迁移 `DoubaoService`；本文件仅保留路径以保证 1:1 结构对齐。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `DoubaoProvider` / `DoubaoServiceImpl`。
pub fn doubao_service_name() -> ModelName {
    ModelName::Doubao
}

/// 🚫 Rust 侧未实现。Java `DoubaoProvider.create`。
pub fn doubao_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("Doubao provider is not implemented in the Rust port yet")
}