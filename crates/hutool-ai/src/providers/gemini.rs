//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.gemini.*`（6 个类）
//!
//! Rust 侧尚未迁移 `GeminiService`；本文件仅保留路径以保证 1:1 结构对齐。
//! Gemini endpoint 适配已经通过 `Operation::endpoint` 间接覆盖。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `GeminiProvider` / `GeminiServiceImpl`。
pub fn gemini_service_name() -> ModelName {
    ModelName::Gemini
}

/// 🚫 Rust 侧未实现。Java `GeminiProvider.create`。
pub fn gemini_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("Gemini provider is not implemented in the Rust port yet")
}