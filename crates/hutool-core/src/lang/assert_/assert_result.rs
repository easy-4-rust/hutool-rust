//! 对齐: `cn.hutool.core.lang.Assert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Assert.java
//!
//! Rust 版本以 [`Result`] + [`AssertError`] 表达 Java 的断言失败抛出；
//! 成功时返回被检查值，便于链式调用。

use super::assert_error::AssertError;

/// 断言结果别名。
#[allow(dead_code)] // 对齐 Java Assert 结果类型，暂未接线，预留
pub type AssertResult<T> = Result<T, AssertError>;
