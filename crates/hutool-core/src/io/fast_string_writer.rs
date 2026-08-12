//! 对齐: `cn.hutool.core.io.FastStringWriter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FastStringWriter.java
//!
//! 借助{@link StrBuilder} 提供快读的字符串写出，相比jdk的StringWriter非线程安全，速度更快。
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 借助{@link StrBuilder} 提供快读的字符串写出，相比jdk的StringWriter非线程安全，速度更快。
#[derive(Debug, Clone, Copy, Default)]
pub struct FastStringWriter;

impl FastStringWriter {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine(
            "FastStringWriter::pending_io_alignment",
        ))
    }
}
