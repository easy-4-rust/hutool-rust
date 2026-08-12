//! 对齐: `cn.hutool.core.io.StreamProgress`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/StreamProgress.java
//!
//! Stream进度条<br>
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// Stream进度条<br>
pub trait StreamProgress {
    /// 对齐 Java 接口，等待具体实现。
    fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine(
            "StreamProgress::pending_io_alignment",
        ))
    }
}
