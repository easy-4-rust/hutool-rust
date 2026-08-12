//! 对齐: `cn.hutool.core.text.UnicodeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/UnicodeUtil.java
//!
//! Unicode 字符串(`\\uXXXX`)与普通字符串互转。

use crate::Result;
use crate::hex_util::HexUtil;

/// 对齐 Java: `UnicodeUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct UnicodeUtil;

impl UnicodeUtil {
    /// 对齐 Java: `UnicodeUtil::toString#String (String unicode)`
    pub fn to_string(unicode: &str) -> Result<String> {
        if unicode.trim().is_empty() {
            return Ok(unicode.to_string());
        }
        let lower = unicode.to_ascii_lowercase();
        let bytes = unicode.as_bytes();
        let lower_bytes = lower.as_bytes();
        let len = unicode.len();
        let mut sb = String::with_capacity(len);
        let mut pos = 0usize;
        while let Some(rel) = find_ci_u(lower_bytes, pos) {
            sb.push_str(&unicode[pos..rel]);
            pos = rel;
            if rel + 5 < len {
                let hex = &unicode[rel + 2..rel + 6];
                match u32::from_str_radix(hex, 16) {
                    Ok(code) if code <= 0xFFFF => {
                        if let Some(ch) = char::from_u32(code) {
                            sb.push(ch);
                            pos = rel + 6;
                            continue;
                        }
                    }
                    _ => {}
                }
                sb.push_str(&unicode[pos..rel + 2]);
                pos = rel + 2;
            } else {
                break;
            }
        }
        if pos < len {
            sb.push_str(&unicode[pos..]);
        }
        let _ = bytes;
        Ok(sb)
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (char c)`
    pub fn to_unicode_char(c: char) -> Result<String> {
        Ok(HexUtil::to_unicode_hex(c))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (int c)`
    pub fn to_unicode_int(c: i32) -> Result<String> {
        Ok(HexUtil::to_unicode_hex_i32(c))
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (String str)`
    pub fn to_unicode(str: &str) -> Result<String> {
        Self::to_unicode_skip_ascii(str, true)
    }

    /// 对齐 Java: `UnicodeUtil::toUnicode#String (String str, boolean isSkipAscii)`
    pub fn to_unicode_skip_ascii(str: &str, skip_ascii: bool) -> Result<String> {
        if str.is_empty() {
            return Ok(str.to_string());
        }
        let mut unicode = String::with_capacity(str.len() * 6);
        for c in str.chars() {
            if skip_ascii && is_ascii_printable(c) {
                unicode.push(c);
            } else {
                unicode.push_str(&HexUtil::to_unicode_hex(c));
            }
        }
        Ok(unicode)
    }
}

/// 对齐 Java: `CharUtil.isAsciiPrintable`
fn is_ascii_printable(c: char) -> bool {
    let u = c as u32;
    (0x20..=0x7E).contains(&u)
}

/// 查找 `\\u`（忽略大小写），返回字节/字符索引（ASCII 安全）。
fn find_ci_u(lower_bytes: &[u8], from: usize) -> Option<usize> {
    let needle = b"\\u";
    lower_bytes[from..]
        .windows(2)
        .position(|w| w == needle)
        .map(|p| from + p)
}
