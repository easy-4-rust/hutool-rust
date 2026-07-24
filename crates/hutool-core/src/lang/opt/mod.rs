//! 对齐: `cn.hutool.core.lang.Opt`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Opt.java
//!
//! Hutool `Opt` 的 idiomatic Rust 实现：在 [`Option`] 之上附加可选异常上下文。

use crate::string::is_blank;
use std::fmt;

mod opt;
mod opt_empty_error;

pub use opt::Opt;
pub use opt_empty_error::OptEmptyError;
