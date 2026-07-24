//! 对齐: `cn.hutool.core.lang.Assert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Assert.java
//!
//! Rust 版本以 [`Result`] + [`AssertError`] 表达 Java 的断言失败抛出；
//! 成功时返回被检查值，便于链式调用。

use std::collections::HashMap;
use std::fmt::{Display, Write};

use crate::string::{format_template, is_blank};

mod assert_error;
mod assert_result;
mod assert;

pub use assert_error::AssertError;
pub use assert_result::AssertResult;
pub use assert::Assert;
