//! 对齐: `cn.hutool.core.io.watch.WatchException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/watch/WatchException.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java 异常类: `cn.hutool.core.io.watch.WatchException`
///
/// 在 Rust 中异常类映射为 [`thiserror::Error`] 枚举变体或独立 Error 类型。
/// 该桩保留类型命名,等待完整实现。
#[derive(Debug, thiserror::Error)]
#[error("WatchException: 对齐桩,等待完整实现")]
pub struct WatchException {
    /// 错误消息。
    pub message: String,
}

impl WatchException {
    /// 创建新的错误实例。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
