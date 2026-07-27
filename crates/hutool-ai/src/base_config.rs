//! `BaseConfig` 顶级 re-export。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.BaseConfig`
//!
//! Rust 侧 `BaseConfig` 实际位于 `core/base_config.rs` 模块，以避免与 Java 同名类重复；
//! 本文件仅作为兼容 re-export，方便 Java 包路径 `cn.hutool.ai.BaseConfig` 的访问者。

pub use crate::core::base_config::BaseConfig;