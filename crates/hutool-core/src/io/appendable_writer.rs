//! 对齐: `cn.hutool.core.io.AppendableWriter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/AppendableWriter.java
//!
//! 同时继承{@link Writer}和实现{@link Appendable}的聚合类，用于适配两种接口操作
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 同时继承{@link Writer}和实现{@link Appendable}的聚合类，用于适配两种接口操作
#[derive(Debug, Clone, Copy, Default)]
pub struct AppendableWriter;

impl AppendableWriter {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine(
            "AppendableWriter::pending_io_alignment",
        ))
    }
}
