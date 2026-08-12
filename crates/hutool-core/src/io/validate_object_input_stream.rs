//! 对齐: `cn.hutool.core.io.ValidateObjectInputStream`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/ValidateObjectInputStream.java
//!
//! 带有类验证的对象流，用于避免反序列化漏洞<br>
//! 状态: 对齐桩,等待实现
//!
//! Rust 化要点:
//! - 静态方法类 → ZST + 关联函数
//! - Java interface → Rust trait
//! - 异常类 → thiserror Error 枚举
//! - 工具类的常量 → 关联常量

use crate::{CoreError, Result};

/// 带有类验证的对象流，用于避免反序列化漏洞<br>
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidateObjectInputStream;

impl ValidateObjectInputStream {
    /// Sentinel used until the corresponding IO engine is implemented.
    pub fn pending_io_alignment() -> Result<()> {
        Err(CoreError::PendingEngine(
            "ValidateObjectInputStream::pending_io_alignment",
        ))
    }
}
