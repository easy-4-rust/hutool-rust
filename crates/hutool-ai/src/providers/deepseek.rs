//! 🚧 占位模块。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.deepseek.*`（5 个类）
//!
//! Rust 侧尚未迁移 `DeepSeekService`；本文件仅保留路径以保证 1:1 结构对齐。

#![allow(dead_code, clippy::missing_docs_in_inline_items)]

use crate::core::BaseConfig;
use crate::{ModelName, ProviderError};
use std::sync::Arc;

/// 🚫 Rust 侧未实现。Java `DeepSeekProvider` / `DeepSeekServiceImpl`。
pub fn deepseek_service_name() -> ModelName {
    ModelName::DeepSeek
}

/// 🚫 Rust 侧未实现。Java `DeepSeekProvider.create`。
pub fn deepseek_create(_config: BaseConfig) -> Result<Arc<()>, ProviderError> {
    unimplemented!("DeepSeek provider is not implemented in the Rust port yet")
}

/// 🚫 占位：DeepSeek 推理模型枚举已统一放入 `models.rs` 的 `DeepSeekModel`。
/// 保留 Java 命名作为镜像别名，暂无调用方。
#[allow(unused_imports)]
pub use crate::models::DeepSeekModel as DeepSeekReasoning;