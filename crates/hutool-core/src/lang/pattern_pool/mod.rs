//! 对齐: `cn.hutool.core.lang.PatternPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/PatternPool.java
//!
//! 编译正则缓存；flags 对齐 Java `Pattern` 位掩码的常用子集（CASE_INSENSITIVE=2）。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::Mutex;
use regex::{Regex, RegexBuilder};

mod pattern_pool;
mod regex_with_flag;

pub use pattern_pool::PatternPool;
pub use regex_with_flag::RegexWithFlag;

pub const FLAG_CASE_INSENSITIVE: i32 = 2;

fn pool() -> &'static Mutex<HashMap<RegexWithFlag, Arc<Regex>>> {
    static POOL: std::sync::OnceLock<Mutex<HashMap<RegexWithFlag, Arc<Regex>>>> =
        std::sync::OnceLock::new();
    POOL.get_or_init(|| Mutex::new(HashMap::new()))
}

fn compile(regex: &str, flags: i32) -> Result<Regex, regex::Error> {
    let mut builder = RegexBuilder::new(regex);
    if flags & FLAG_CASE_INSENSITIVE != 0 {
        builder.case_insensitive(true);
    }
    builder.build()
}
