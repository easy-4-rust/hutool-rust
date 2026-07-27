//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.ollama.*`（6 个类）
//!
//! Rust 侧尚未迁移 `OllamaService`；本文件仅保留路径以保证 1:1 结构对齐。
//! Ollama endpoint 适配已经通过 `Operation::endpoint` 间接覆盖。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `OllamaProvider` / `OllamaServiceImpl`。
pub fn ollama_service_name() -> ModelName {
    ModelName::Ollama
}

/// 🚫 Rust 侧未实现。Java `OllamaProvider.create`。
pub fn ollama_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("Ollama provider is not implemented in the Rust port yet")
}