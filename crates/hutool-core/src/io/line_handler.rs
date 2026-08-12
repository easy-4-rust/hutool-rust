//! 对齐: `cn.hutool.core.io.LineHandler`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/LineHandler.java
//!
//! 行处理器
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 行处理器
pub trait LineHandler {
    /// 对齐 Java 接口，等待具体实现。
    fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine(
            "LineHandler::pending_io_alignment",
        ))
    }
}
