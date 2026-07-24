//! 对齐: `cn.hutool.core.text.StrJoiner`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrJoiner.java
//!
//! 字符串拼接器(类似 Java `StringJoiner`,支持前缀/后缀/`null` 模式)。

use crate::Result;

mod null_mode;
mod str_joiner;

pub use null_mode::NullMode;
pub use str_joiner::StrJoiner;
