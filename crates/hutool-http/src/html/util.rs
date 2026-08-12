//! 对齐: `cn.hutool.http.HtmlUtil`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HtmlUtil.java
//! 中文说明: HTML工具类，提供标签移除、属性清理、HTML转义等辅助方法

use super::filter::HtmlFilter;
use fancy_regex::Regex;
use std::sync::OnceLock;

/// Escapes a literal for use inside a regex pattern.
fn re_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$'
            | '#' => {
                out.push('\\');
                out.push(ch);
            }
            c => out.push(c),
        }
    }
    out
}

/// Compiles a case-insensitive (and optionally dot-all) fancy-regex pattern.
fn re_build(pattern: &str, case_insensitive: bool, dotall: bool) -> Regex {
    let mut flags = String::new();
    if case_insensitive {
        flags.push('i');
    }
    if dotall {
        flags.push('s');
    }
    let wrapped = if flags.is_empty() {
        pattern.to_string()
    } else {
        format!("(?{flags}){pattern}")
    };
    Regex::new(&wrapped).unwrap_or_else(|err| panic!("invalid HtmlUtil regex: {err}"))
}

/// HTML utility functions matching `cn.hutool.http.HtmlUtil`.
pub struct HtmlUtil;

impl HtmlUtil {
    /// Escapes HTML special characters (`' " & < >` and NBSP).
    ///
    /// Java: `HtmlUtil.escape(String text)`
    pub fn escape(text: &str) -> String {
        let mut out = String::with_capacity(text.len() + text.len() / 4);
        for ch in text.chars() {
            match ch {
                '\'' => out.push_str("&#039;"),
                '"' => out.push_str("&quot;"),
                '&' => out.push_str("&amp;"),
                '<' => out.push_str("&lt;"),
                '>' => out.push_str("&gt;"),
                '\u{00A0}' => out.push_str("&nbsp;"),
                c if (c as u32) < 256 => out.push(c),
                c => out.push(c),
            }
        }
        out
    }

    /// Restores HTML4 entities (basic set used by Hutool tests).
    ///
    /// Java: `HtmlUtil.unescape(String htmlStr)`
    pub fn unescape(html: &str) -> String {
        if html.is_empty() {
            return String::new();
        }
        let mut out = String::with_capacity(html.len());
        let bytes = html.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'&' {
                if let Some((entity, consumed)) = parse_entity(&html[i..]) {
                    out.push_str(&entity);
                    i += consumed;
                    continue;
                }
            }
            out.push(html[i..].chars().next().unwrap());
            i += html[i..].chars().next().unwrap().len_utf8();
        }
        out
    }

    /// Removes all HTML tags, keeping inner text.
    ///
    /// Java: `HtmlUtil.cleanHtmlTag(String content)`
    pub fn clean_html_tag(content: &str) -> String {
        // Hutool: `(<[^<]*?>)|(<[\s]*?/[^<]*?>)|(<[^<]*?/[\s]*?>)`
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| {
            Regex::new(r"(<[^<]*?>)|(</?\s*[^<]*?>)|(<[^<]*?/\s*>)").expect("html mark")
        });
        re.replace_all(content, "").into_owned()
    }

    /// Removes empty paired tags such as `<p></p>`.
    ///
    /// Java: `HtmlUtil.cleanEmptyTag(String content)`
    pub fn clean_empty_tag(content: &str) -> String {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"<(\w+)([^>]*)>\s*</\1>").expect("empty tag"));
        re.replace_all(content, "").into_owned()
    }

    /// Removes tags and their content.
    ///
    /// Java: `HtmlUtil.removeHtmlTag(String content, String... tagNames)`
    pub fn remove_html_tag(content: &str, tag_names: &[&str]) -> String {
        Self::remove_html_tag_with(content, true, tag_names)
    }

    /// Removes tags but keeps inner content.
    ///
    /// Java: `HtmlUtil.unwrapHtmlTag(String content, String... tagNames)`
    pub fn unwrap_html_tag(content: &str, tag_names: &[&str]) -> String {
        Self::remove_html_tag_with(content, false, tag_names)
    }

    /// Clears specified tags; `with_tag_content` controls whether inner text is removed.
    ///
    /// Java: `HtmlUtil.removeHtmlTag(String content, boolean withTagContent, String... tagNames)`
    pub fn remove_html_tag_with(
        content: &str,
        with_tag_content: bool,
        tag_names: &[&str],
    ) -> String {
        let mut result = content.to_string();
        for tag in tag_names {
            let tag = tag.trim();
            if tag.is_empty() {
                continue;
            }
            let escaped = re_escape(tag);
            let re = if with_tag_content {
                re_build(
                    &format!(r"<{escaped}(\s+[^>]*?)?/?>(.*?</{escaped}>)?"),
                    true,
                    true,
                )
            } else {
                re_build(
                    &format!(r"<{escaped}(\s+[^>]*?)?/?>|</?{escaped}>"),
                    true,
                    false,
                )
            };
            result = re.replace_all(&result, "").into_owned();
        }
        result
    }

    /// Removes named attributes from all tags.
    ///
    /// Java: `HtmlUtil.removeHtmlAttr(String content, String... attrs)`
    pub fn remove_html_attr(content: &str, attrs: &[&str]) -> String {
        let mut result = content.to_string();
        for attr in attrs {
            let re = re_build(
                &format!(
                    r#"(\s*{attr}\s*=\s*)((["][^"]+?["])|([^>]+?\s*(?=\s|>)))"#,
                    attr = re_escape(attr)
                ),
                true,
                false,
            );
            result = re.replace_all(&result, "").into_owned();
        }
        // issue#I8YV0K: trim trailing space before `>` / `/>`
        static TRAIL: OnceLock<Regex> = OnceLock::new();
        let trail = TRAIL.get_or_init(|| Regex::new(r"\s+(>|/>)").expect("trail"));
        trail.replace_all(&result, "$1").into_owned()
    }

    /// Removes all attributes from the named tags.
    ///
    /// Java: `HtmlUtil.removeAllHtmlAttr(String content, String... tagNames)`
    pub fn remove_all_html_attr(content: &str, tag_names: &[&str]) -> String {
        let mut result = content.to_string();
        for tag in tag_names {
            let escaped = re_escape(tag);
            let re = re_build(&format!(r"<{escaped}[^>]*?>"), true, false);
            let replacement = format!("<{tag}>");
            result = re.replace_all(&result, replacement.as_str()).into_owned();
        }
        result
    }

    /// Filters HTML to mitigate XSS using the default allow-list.
    ///
    /// Java: `HtmlUtil.filter(String htmlContent)`
    pub fn filter(html: &str) -> String {
        HtmlFilter::new().filter(html)
    }
}

/// Parses a single HTML entity starting at `&`.
fn parse_entity(s: &str) -> Option<(String, usize)> {
    if !s.starts_with('&') {
        return None;
    }
    if let Some(rest) = s.strip_prefix("&nbsp;") {
        let _ = rest;
        return Some(("\u{00A0}".to_string(), 6));
    }
    if let Some(rest) = s.strip_prefix("&lt;") {
        let _ = rest;
        return Some(("<".to_string(), 4));
    }
    if let Some(rest) = s.strip_prefix("&gt;") {
        let _ = rest;
        return Some((">".to_string(), 4));
    }
    if let Some(rest) = s.strip_prefix("&amp;") {
        let _ = rest;
        return Some(("&".to_string(), 5));
    }
    if let Some(rest) = s.strip_prefix("&quot;") {
        let _ = rest;
        return Some(("\"".to_string(), 6));
    }
    if let Some(rest) = s.strip_prefix("&apos;") {
        let _ = rest;
        return Some(("'".to_string(), 6));
    }
    if let Some(rest) = s.strip_prefix("&#039;") {
        let _ = rest;
        return Some(("'".to_string(), 6));
    }
    if let Some(rest) = s.strip_prefix("&#39;") {
        let _ = rest;
        return Some(("'".to_string(), 5));
    }
    // numeric &#NNN; or &#xHH;
    if let Some(body) = s.strip_prefix("&#x").or_else(|| s.strip_prefix("&#X")) {
        let end = body.find(';')?;
        let n = u32::from_str_radix(&body[..end], 16).ok()?;
        let ch = char::from_u32(n)?;
        return Some((ch.to_string(), 3 + end + 1));
    }
    if let Some(body) = s.strip_prefix("&#") {
        let end = body.find(';')?;
        let n: u32 = body[..end].parse().ok()?;
        let ch = char::from_u32(n)?;
        return Some((ch.to_string(), 2 + end + 1));
    }
    None
}
