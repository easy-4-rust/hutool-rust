//! 对齐: `cn.hutool.core.text.ASCIIStrCache`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/ASCIIStrCache.java
//!
//! ASCII 字符对应的字符串缓存。Java 的静态类对应 Rust 的零大小标记类型 + 关联函数。

use std::sync::OnceLock;

use crate::Result;

/// ASCII 可缓存字符数（对齐 Java `ASCII_LENGTH`）。
const ASCII_LENGTH: usize = 128;

/// 对齐 Java: `ASCIIStrCache#`
#[derive(Debug, Clone, Copy, Default)]
pub struct AsciiStrCache;

impl AsciiStrCache {
    /// 返回预热后的 ASCII 字符串缓存表。
    fn cache() -> &'static [String; ASCII_LENGTH] {
        static CACHE: OnceLock<[String; ASCII_LENGTH]> = OnceLock::new();
        CACHE.get_or_init(|| {
            let mut arr: [String; ASCII_LENGTH] = std::array::from_fn(|_| String::new());
            for (i, slot) in arr.iter_mut().enumerate() {
                *slot = (i as u8 as char).to_string();
            }
            arr
        })
    }

    /// 对齐 Java: `ASCIIStrCache::toString#String (char c)`
    ///
    /// ASCII 范围内返回缓存副本；超出范围则直接 `to_string`。
    pub fn to_string(c: char) -> Result<String> {
        let code = c as u32;
        if code < ASCII_LENGTH as u32 {
            Ok(Self::cache()[code as usize].clone())
        } else {
            Ok(c.to_string())
        }
    }
}

/// Java 类名别名（`ASCIIStrCache`）。
pub type ASCIIStrCache = AsciiStrCache;
