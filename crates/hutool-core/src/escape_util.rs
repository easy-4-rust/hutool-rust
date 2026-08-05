//! 对齐: `cn.hutool.core.util.EscapeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/EscapeUtil.java
//!
//! Rust 版本提供字符转义的 idiomatic 实现。

/// 对齐 Java: `cn.hutool.core.util.EscapeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct EscapeUtil;

/// JS `escape()` 默认不转义的 ASCII 符号。
const NOT_ESCAPE_CHARS: &str = "*@-_+./";

impl EscapeUtil {
    // ── HTML4 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeHtml4(CharSequence)`
    ///
    /// HTML4 转义不转义单引号。
    pub fn escape_html(input: &str) -> String {
        Self::escape_html4(input)
    }

    /// 对齐 Java: `EscapeUtil.escapeHtml4(CharSequence)`
    pub fn escape_html4(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `EscapeUtil.unescapeHtml4(CharSequence)`
    pub fn unescape_html(input: &str) -> String {
        Self::unescape_html4(input)
    }

    /// 对齐 Java: `EscapeUtil.unescapeHtml4(CharSequence)`
    pub fn unescape_html4(input: &str) -> String {
        input
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&#39;", "'")
    }

    // ── XML 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeXml(CharSequence)`
    pub fn escape_xml(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&apos;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `EscapeUtil.unescapeXml(CharSequence)`
    pub fn unescape_xml(input: &str) -> String {
        input
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    // ── Java/JavaScript 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeJava(CharSequence)`
    pub fn escape_java(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        for c in input.chars() {
            match c {
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                '\\' => result.push_str("\\\\"),
                '"' => result.push_str("\\\""),
                '\'' => result.push_str("\\'"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `EscapeUtil.unescapeJava(CharSequence)`
    pub fn unescape_java(input: &str) -> String {
        let mut result = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some('\'') => result.push('\''),
                    Some(other) => {
                        result.push('\\');
                        result.push(other);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    // ── SQL 转义 ──

    /// 对齐 Java: `EscapeUtil.escapeSql(CharSequence)`
    pub fn escape_sql(input: &str) -> String {
        input.replace('\'', "''")
    }

    // ── JS Unicode 转义 ──

    /// 对齐 Java: `EscapeUtil.escape(CharSequence)` — JS `escape()` 语义
    pub fn escape(input: &str) -> String {
        Self::escape_with_filter(input, Self::js_escape_filter)
    }

    /// 对齐 Java: `EscapeUtil.escapeAll(CharSequence)`
    pub fn escape_all(input: &str) -> String {
        Self::escape_with_filter(input, |_| true)
    }

    /// 对齐 Java: `EscapeUtil.unescape(String)`
    pub fn unescape(input: &str) -> String {
        if input.is_empty() || input.chars().all(char::is_whitespace) {
            return input.to_string();
        }

        let len = input.len();
        let mut tmp = String::with_capacity(len);
        let mut last_pos = 0usize;
        let mut pending_high: Option<u16> = None;
        let bytes = input.as_bytes();

        while last_pos < len {
            if let Some(rel_pos) = input[last_pos..].find('%') {
                let pos = last_pos + rel_pos;
                if pos == last_pos {
                    if pos + 1 < len && bytes[pos + 1] == b'u' {
                        if pos + 6 <= len {
                            if let Ok(unit) = u16::from_str_radix(&input[pos + 2..pos + 6], 16) {
                                Self::push_java_unit(&mut tmp, unit, &mut pending_high);
                                last_pos = pos + 6;
                                continue;
                            }
                            Self::flush_pending_high(&mut tmp, &mut pending_high);
                            tmp.push_str(&input[pos..]);
                            break;
                        } else {
                            Self::flush_pending_high(&mut tmp, &mut pending_high);
                            tmp.push_str(&input[pos..]);
                            break;
                        }
                    } else if pos + 3 <= len {
                        if let Ok(unit) = u16::from_str_radix(&input[pos + 1..pos + 3], 16) {
                            Self::push_java_unit(&mut tmp, unit, &mut pending_high);
                            last_pos = pos + 3;
                            continue;
                        }
                        Self::flush_pending_high(&mut tmp, &mut pending_high);
                        tmp.push_str(&input[pos..]);
                        break;
                    } else {
                        Self::flush_pending_high(&mut tmp, &mut pending_high);
                        tmp.push_str(&input[pos..]);
                        break;
                    }
                } else {
                    Self::flush_pending_high(&mut tmp, &mut pending_high);
                    tmp.push_str(&input[last_pos..pos]);
                    last_pos = pos;
                }
            } else {
                Self::flush_pending_high(&mut tmp, &mut pending_high);
                tmp.push_str(&input[last_pos..]);
                break;
            }
        }
        Self::flush_pending_high(&mut tmp, &mut pending_high);
        tmp
    }

    /// 追加 Java `char` 码元，并在代理对齐全时输出完整 Unicode 字符。
    fn push_java_unit(out: &mut String, unit: u16, pending_high: &mut Option<u16>) {
        if let Some(high) = pending_high.take() {
            if (0xDC00..=0xDFFF).contains(&unit) {
                let combined = 0x1_0000 + (((high as u32 - 0xD800) << 10) | (unit as u32 - 0xDC00));
                if let Some(ch) = char::from_u32(combined) {
                    out.push(ch);
                    return;
                }
            } else {
                Self::push_bmp_unit(out, high);
            }
        }

        if (0xD800..=0xDBFF).contains(&unit) {
            *pending_high = Some(unit);
        } else {
            Self::push_bmp_unit(out, unit);
        }
    }

    /// 输出单个 BMP 字符。
    fn push_bmp_unit(out: &mut String, unit: u16) {
        if let Some(ch) = char::from_u32(u32::from(unit)) {
            out.push(ch);
        }
    }

    /// 输出尚未配对的 high surrogate。
    fn flush_pending_high(out: &mut String, pending_high: &mut Option<u16>) {
        if let Some(high) = pending_high.take() {
            Self::push_bmp_unit(out, high);
        }
    }

    /// 对齐 Java: `EscapeUtil.unescape(null)` — Rust 使用 `Option`
    pub fn unescape_option(input: Option<&str>) -> Option<String> {
        input.map(Self::unescape)
    }

    /// 对齐 Java: `EscapeUtil.safeUnescape(String)`
    pub fn safe_unescape(input: &str) -> String {
        Self::unescape(input)
    }

    /// JS escape 默认过滤器：字母数字与 `*@-_+./` 不转义。
    fn js_escape_filter(c: char) -> bool {
        !(c.is_ascii_alphanumeric() || NOT_ESCAPE_CHARS.contains(c))
    }

    /// 对齐 Java: `EscapeUtil.escape(CharSequence, Filter<Character>)`
    fn escape_with_filter(input: &str, filter: impl Fn(char) -> bool) -> String {
        if input.is_empty() {
            return input.to_string();
        }

        let mut tmp = String::with_capacity(input.len() * 6);
        for c in input.chars() {
            if !filter(c) {
                tmp.push(c);
            } else {
                let code = c as u32;
                if code < 256 {
                    tmp.push('%');
                    if code < 16 {
                        tmp.push('0');
                    }
                    tmp.push_str(&format!("{code:x}"));
                } else {
                    tmp.push_str("%u");
                    if code <= 0xfff {
                        tmp.push('0');
                    }
                    tmp.push_str(&format!("{code:x}"));
                }
            }
        }
        tmp
    }
}
