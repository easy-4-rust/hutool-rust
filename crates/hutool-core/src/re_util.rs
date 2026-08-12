//! 对齐: `cn.hutool.core.util.ReUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReUtil.java
//!
//! Rust 版本使用 `regex` crate 提供正则表达式工具。

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::{Captures, Regex};

/// 对齐 Java: `PatternPool.GROUP_VAR` → `\$(\d+)`
static GROUP_VAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$(\d+)").unwrap());

/// 对齐 Java: `ReUtil.RE_KEYS`
const RE_KEYS: &str = r"$()*+.[?\\^{}|";

/// 对齐 Java: `cn.hutool.core.util.ReUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ReUtil;

impl ReUtil {
    // ── 匹配判断 ──

    /// 对齐 Java: `ReUtil.isMatch(Pattern, CharSequence)`
    pub fn is_match(pattern: &str, text: &str) -> bool {
        Regex::new(pattern).map_or(false, |re| re.is_match(text))
    }

    /// 对齐 Java: `ReUtil.isMatch(String, CharSequence)`
    pub fn is_match_str(pattern: &str, text: &str) -> bool {
        Self::is_match(pattern, text)
    }

    // ── 提取操作 ──

    /// 对齐 Java: `ReUtil.get(Pattern, CharSequence, int)`
    pub fn find(pattern: &str, text: &str) -> Option<String> {
        Regex::new(pattern)
            .ok()?
            .find(text)
            .map(|m| m.as_str().to_string())
    }

    /// 对齐 Java: `ReUtil.get(Pattern, CharSequence, int groupIndex)`
    pub fn group(pattern: &str, text: &str, group: usize) -> Option<String> {
        Regex::new(pattern)
            .ok()?
            .captures(text)?
            .get(group)
            .map(|m| m.as_str().to_string())
    }

    /// 对齐 Java: `ReUtil.get(String regex, CharSequence content, String groupName)`
    pub fn group_by_name(pattern: &str, text: &str, group_name: &str) -> Option<String> {
        Regex::new(pattern)
            .ok()?
            .captures(text)?
            .name(group_name)
            .map(|m| m.as_str().to_string())
    }

    /// 对齐 Java: `ReUtil.getGroup0(Pattern, CharSequence)`
    pub fn group0(pattern: &str, text: &str) -> Option<String> {
        Self::group(pattern, text, 0)
    }

    /// 对齐 Java: `ReUtil.getGroup1(Pattern, CharSequence)`
    pub fn group1(pattern: &str, text: &str) -> Option<String> {
        Self::group(pattern, text, 1)
    }

    /// 对齐 Java: `ReUtil.getAllGroupNames(Pattern, CharSequence)`
    pub fn get_all_group_names(pattern: &str, text: &str) -> HashMap<String, String> {
        let Some(re) = Regex::new(pattern).ok() else {
            return HashMap::new();
        };
        let Some(caps) = re.captures(text) else {
            return HashMap::new();
        };
        let mut result = HashMap::new();
        for name in re.capture_names().flatten() {
            if let Some(m) = caps.name(name) {
                result.insert(name.to_string(), m.as_str().to_string());
            }
        }
        result
    }

    /// 对齐 Java: `ReUtil.extractMulti(String regex, CharSequence content, String template)`
    pub fn extract_multi(pattern: &str, content: &str, template: &str) -> Option<String> {
        let re = Regex::new(pattern).ok()?;
        let caps = re.captures(content)?;
        Some(apply_group_template(template, &caps, true))
    }

    // ── 提取所有匹配 ──

    /// 对齐 Java: `ReUtil.findAll(Pattern, CharSequence, int)`
    pub fn find_all(pattern: &str, text: &str) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| re.find_iter(text).map(|m| m.as_str().to_string()).collect())
            .unwrap_or_default()
    }

    /// 对齐 Java: `ReUtil.findAllGroup(Pattern, CharSequence, int)`
    pub fn find_all_groups(pattern: &str, text: &str, group: usize) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| {
                re.captures_iter(text)
                    .filter_map(|caps| caps.get(group).map(|m| m.as_str().to_string()))
                    .collect()
            })
            .unwrap_or_default()
    }

    // ── 删除操作 ──

    /// 对齐 Java: `ReUtil.delFirst(String regex, CharSequence content)`
    pub fn del_first(pattern: &str, text: &str) -> String {
        Self::replace_first(pattern, text, "")
    }

    /// 对齐 Java: `ReUtil.delLast(String regex, CharSequence str)`
    pub fn del_last(pattern: &str, text: &str) -> String {
        let Ok(re) = Regex::new(pattern) else {
            return text.to_string();
        };
        Self::del_last_with_regex(&re, text)
    }

    /// 对齐 Java: `ReUtil.delAll(String regex, CharSequence content)`
    pub fn del_all(pattern: &str, text: &str) -> String {
        if pattern.is_empty() || text.is_empty() {
            return text.to_string();
        }
        Self::replace_all(pattern, text, "")
    }

    /// 对齐 Java: `ReUtil.delPre(String regex, CharSequence content)`
    pub fn del_pre(pattern: &str, text: &str) -> String {
        let Ok(re) = Regex::new(pattern) else {
            return text.to_string();
        };
        if let Some(m) = re.find(text) {
            text[m.end()..].to_string()
        } else {
            text.to_string()
        }
    }

    // ── 替换操作 ──

    /// 对齐 Java: `ReUtil.replaceAll(CharSequence, Pattern, String replacementTemplate)`
    pub fn replace_all(pattern: &str, text: &str, replacement: &str) -> String {
        let Ok(re) = Regex::new(pattern) else {
            return text.to_string();
        };
        Self::replace_all_with_regex(&re, text, replacement)
    }

    /// 对齐 Java: `ReUtil.replaceFirst(Pattern, CharSequence, String replacement)`
    pub fn replace_first(pattern: &str, text: &str, replacement: &str) -> String {
        let Ok(re) = Regex::new(pattern) else {
            return text.to_string();
        };
        if text.is_empty() {
            return text.to_string();
        }
        if let Some(caps) = re.captures(text) {
            let m = caps.get(0).unwrap();
            let mut result = String::with_capacity(text.len());
            result.push_str(&text[..m.start()]);
            result.push_str(&Self::build_replacement(replacement, &caps));
            result.push_str(&text[m.end()..]);
            result
        } else {
            text.to_string()
        }
    }

    // ── 分割操作 ──

    /// 对齐 Java: `ReUtil.split(CharSequence, Pattern)`
    pub fn split(pattern: &str, text: &str) -> Vec<String> {
        Regex::new(pattern)
            .map(|re| re.split(text).map(String::from).collect())
            .unwrap_or_else(|_| vec![text.to_string()])
    }

    // ── 常用正则 ──

    /// 对齐 Java: `ReUtil.isEmail(CharSequence)`
    pub fn is_email(text: &str) -> bool {
        Self::is_match(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$", text)
    }

    /// 对齐 Java: `ReUtil.isIpv4(CharSequence)`
    pub fn is_ipv4(text: &str) -> bool {
        Self::is_match(r"^(\d{1,3}\.){3}\d{1,3}$", text)
    }

    /// 对齐 Java: `ReUtil.isIpv6(CharSequence)`
    pub fn is_ipv6(text: &str) -> bool {
        Self::is_match(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$", text)
    }

    /// 对齐 Java: `ReUtil.isUrl(CharSequence)`
    pub fn is_url(text: &str) -> bool {
        Self::is_match(r"^https?://[^\s/$.?#].[^\s]*$", text)
    }

    /// 对齐 Java: `ReUtil.isChinese(CharSequence)`
    pub fn is_chinese(text: &str) -> bool {
        Self::is_match(r"^[\u4e00-\u9fa5]+$", text)
    }

    /// 对齐 Java: `ReUtil.isMobile(CharSequence)`
    pub fn is_mobile(text: &str) -> bool {
        Self::is_match(r"^1[3-9]\d{9}$", text)
    }

    /// 对齐 Java: `ReUtil.isIdCard(CharSequence)`
    pub fn is_id_card(text: &str) -> bool {
        Self::is_match(r"^\d{17}[\dX]$", text)
    }

    // ── 提取数字 ──

    /// 提取字符串中的第一个数字
    pub fn extract_number(text: &str) -> Option<i64> {
        Self::find(r"\d+", text)?.parse().ok()
    }

    /// 提取字符串中的所有数字
    pub fn extract_numbers(text: &str) -> Vec<i64> {
        Self::find_all(r"\d+", text)
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    // ── 转义 ──

    /// 对齐 Java: `ReUtil.escape(CharSequence)`
    pub fn escape_special(text: &str) -> String {
        let mut result = String::with_capacity(text.len() * 2);
        for c in text.chars() {
            if RE_KEYS.contains(c) {
                result.push('\\');
            }
            result.push(c);
        }
        result
    }

    /// 对齐 Java: `ReUtil.escape` 别名。
    pub fn escape(text: &str) -> String {
        Self::escape_special(text)
    }

    /// 对齐 Java: `ReUtil.getGroup0`
    pub fn get_group0(pattern: &str, text: &str) -> Option<String> {
        Self::group0(pattern, text)
    }

    /// 对齐 Java: `ReUtil.getGroup1`
    pub fn get_group1(pattern: &str, text: &str) -> Option<String> {
        Self::group1(pattern, text)
    }

    /// 对齐 Java: `ReUtil.get` / `getAllGroups` — 首个匹配的全部捕获组。
    pub fn get_all_groups(pattern: &str, text: &str, with_group0: bool) -> Vec<String> {
        let Ok(re) = Regex::new(pattern) else {
            return Vec::new();
        };
        let Some(caps) = re.captures(text) else {
            return Vec::new();
        };
        let start = if with_group0 { 0 } else { 1 };
        (start..caps.len())
            .filter_map(|i| caps.get(i).map(|m| m.as_str().to_string()))
            .collect()
    }

    /// 对齐 Java: `ReUtil.contains`
    pub fn contains(pattern: &str, text: &str) -> bool {
        Self::is_match(pattern, text)
    }

    /// 对齐 Java: `ReUtil.count`
    pub fn count(pattern: &str, text: &str) -> usize {
        Regex::new(pattern)
            .map(|re| re.find_iter(text).count())
            .unwrap_or(0)
    }

    /// 对齐 Java: `ReUtil.indexOf`
    pub fn index_of(pattern: &str, text: &str) -> Option<usize> {
        Regex::new(pattern).ok()?.find(text).map(|m| m.start())
    }

    /// 对齐 Java: `ReUtil.lastIndexOf`
    pub fn last_index_of(pattern: &str, text: &str) -> Option<usize> {
        let Ok(re) = Regex::new(pattern) else {
            return None;
        };
        re.find_iter(text).last().map(|m| m.start())
    }

    /// 对齐 Java: `ReUtil.getFirstNumber`
    pub fn get_first_number(text: &str) -> Option<i64> {
        Self::extract_number(text)
    }

    /// 对齐 Java: `ReUtil.findAllGroup0`
    pub fn find_all_group0(pattern: &str, text: &str) -> Vec<String> {
        Self::find_all_groups(pattern, text, 0)
    }

    /// 对齐 Java: `ReUtil.findAllGroup1`
    pub fn find_all_group1(pattern: &str, text: &str) -> Vec<String> {
        Self::find_all_groups(pattern, text, 1)
    }

    /// 对齐 Java: `ReUtil.delLast(Pattern, CharSequence)` 内部逻辑
    fn del_last_with_regex(re: &Regex, text: &str) -> String {
        if text.is_empty() {
            return text.to_string();
        }
        let mut last_match: Option<(usize, usize)> = None;
        for m in re.find_iter(text) {
            last_match = Some((m.start(), m.end()));
        }
        match last_match {
            Some((start, end)) => {
                let mut result = String::with_capacity(text.len());
                result.push_str(&text[..start]);
                result.push_str(&text[end..]);
                result
            }
            None => text.to_string(),
        }
    }

    /// 对齐 Java: `ReUtil.replaceAll(CharSequence, Pattern, String)` 核心实现
    fn replace_all_with_regex(re: &Regex, text: &str, replacement_template: &str) -> String {
        if text.is_empty() {
            return text.to_string();
        }
        let mut result = String::with_capacity(text.len());
        let mut last_end = 0;
        let mut matched = false;
        for caps in re.captures_iter(text) {
            matched = true;
            let m = caps.get(0).unwrap();
            result.push_str(&text[last_end..m.start()]);
            result.push_str(&Self::build_replacement(replacement_template, &caps));
            last_end = m.end();
        }
        if !matched {
            return text.to_string();
        }
        result.push_str(&text[last_end..]);
        result
    }

    /// 根据模板与捕获组构造替换文本（字面插入，对齐 Hutool Matcher 模板替换）。
    fn build_replacement(template: &str, caps: &Captures<'_>) -> String {
        apply_group_template(template, caps, false)
    }
}

/// 解析模板中的 `$1` / `$10` 等分组变量并按 Hutool 规则替换
fn apply_group_template(template: &str, caps: &Captures<'_>, numeric_desc: bool) -> String {
    let mut vars = parse_group_vars(template);
    if numeric_desc {
        vars.sort_by(|a, b| {
            let ai: u32 = a.parse().unwrap_or(0);
            let bi: u32 = b.parse().unwrap_or(0);
            bi.cmp(&ai)
        });
    }
    let mut result = template.to_string();
    for var in vars {
        let group: usize = var.parse().unwrap_or(0);
        let replacement = caps.get(group).map(|m| m.as_str()).unwrap_or("");
        result = result.replace(&format!("${var}"), replacement);
    }
    result
}

/// 从模板中提取 `\$(\d+)` 分组编号，按长度降序（优先匹配 `$10` 而非 `$1`）
fn parse_group_vars(template: &str) -> Vec<String> {
    let mut vars: Vec<String> = GROUP_VAR
        .captures_iter(template)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect();
    vars.sort_by(|a, b| b.len().cmp(&a.len()).then(b.cmp(a)));
    vars.dedup();
    vars
}
