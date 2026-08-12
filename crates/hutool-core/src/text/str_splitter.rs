//! 对齐: `cn.hutool.core.text.StrSplitter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrSplitter.java

use crate::Result;
use crate::text::finder::char_finder::CharFinder;
use crate::text::finder::str_finder::StrFinder;
use crate::text::split::split_iter::SplitIter;
use regex::Regex;

/// 对齐 Java: `StrSplitter#`
#[derive(Debug, Clone, Copy, Default)]
pub struct StrSplitter;

impl StrSplitter {
    /// 对齐 Java: `split(CharSequence, char, int, boolean, boolean)`
    pub fn split_char_limit(
        str: &str,
        sep: char,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_char_limit_case(str, sep, limit, trim, ignore_empty, false)
    }

    /// 对齐 Java: `split(CharSequence, char, int, boolean, boolean, boolean ignoreCase)`
    pub fn split_char_limit_case(
        str: &str,
        sep: char,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
        ignore_case: bool,
    ) -> Result<Vec<String>> {
        let finder = CharFinder::with_case(sep, ignore_case);
        let mut iter = SplitIter::by_char(str, finder, limit, ignore_empty)?;
        iter.to_list(trim)
    }

    /// 对齐 Java: `split(CharSequence, char, boolean, boolean)`
    pub fn split_char(str: &str, sep: char, trim: bool, ignore_empty: bool) -> Result<Vec<String>> {
        Self::split_char_limit(str, sep, 0, trim, ignore_empty)
    }

    /// 对齐 Java: `splitTrim(CharSequence, char, boolean)`
    pub fn split_trim_char(str: &str, sep: char, ignore_empty: bool) -> Result<Vec<String>> {
        Self::split_char(str, sep, true, ignore_empty)
    }

    /// 对齐 Java: `splitTrim(CharSequence, char, int, boolean)`
    pub fn split_trim_char_limit(
        str: &str,
        sep: char,
        limit: i32,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_char_limit(str, sep, limit, true, ignore_empty)
    }

    /// 对齐 Java: `splitIgnoreCase(CharSequence, char, int, boolean, boolean)`
    pub fn split_ignore_case_char(
        str: &str,
        sep: char,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_char_limit_case(str, sep, limit, trim, ignore_empty, true)
    }

    /// 对齐 Java: `split(CharSequence, CharSequence, int, boolean, boolean)`
    pub fn split_str_limit(
        str: &str,
        sep: &str,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_str_limit_case(str, sep, limit, trim, ignore_empty, false)
    }

    /// 对齐 Java: `split(CharSequence, String, int, boolean, boolean, boolean)`
    pub fn split_str_limit_case(
        str: &str,
        sep: &str,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
        ignore_case: bool,
    ) -> Result<Vec<String>> {
        let finder = StrFinder::new(sep, ignore_case)?;
        let mut iter = SplitIter::by_str(str, finder, limit, ignore_empty)?;
        iter.to_list(trim)
    }

    /// 对齐 Java: `split(CharSequence, String, boolean, boolean)`
    pub fn split_str(str: &str, sep: &str, trim: bool, ignore_empty: bool) -> Result<Vec<String>> {
        Self::split_str_limit(str, sep, 0, trim, ignore_empty)
    }

    /// 对齐 Java: `splitTrim(CharSequence, String, boolean)`
    pub fn split_trim_str(str: &str, sep: &str, ignore_empty: bool) -> Result<Vec<String>> {
        Self::split_str(str, sep, true, ignore_empty)
    }

    /// 对齐 Java: `splitTrim(CharSequence, String, int, boolean)`
    pub fn split_trim_str_limit(
        str: &str,
        sep: &str,
        limit: i32,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_str_limit(str, sep, limit, true, ignore_empty)
    }

    /// 对齐 Java: `splitIgnoreCase(CharSequence, String, int, boolean, boolean)`
    pub fn split_ignore_case_str(
        str: &str,
        sep: &str,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_str_limit_case(str, sep, limit, trim, ignore_empty, true)
    }

    /// 对齐 Java: `splitTrimIgnoreCase(CharSequence, String, int, boolean)`
    pub fn split_trim_ignore_case(
        str: &str,
        sep: &str,
        limit: i32,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_str_limit_case(str, sep, limit, true, ignore_empty, true)
    }

    /// 对齐 Java: `split(CharSequence, int)` — by blank
    pub fn split_by_blank(str: &str, limit: i32) -> Result<Vec<String>> {
        let re = Regex::new(r"\s+").unwrap();
        let mut parts: Vec<String> = re
            .split(str)
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        if limit > 0 && parts.len() as i32 > limit {
            parts.truncate(limit as usize);
        }
        Ok(parts)
    }

    /// 对齐 Java: `splitToArray(String, int)` — by blank
    pub fn split_by_blank_to_array(str: &str, limit: i32) -> Result<Vec<String>> {
        Self::split_by_blank(str, limit)
    }

    /// 对齐 Java: `splitPath(CharSequence, int)`
    pub fn split_path_limit(str: &str, limit: i32) -> Result<Vec<String>> {
        let trimmed = str.trim_matches(|c| c == '/' || c == '\\');
        // 统一为正斜杠再切
        let normalized = trimmed.replace('\\', "/");
        Self::split_char_limit(&normalized, '/', limit, true, true)
    }

    /// 对齐 Java: `splitPath(CharSequence)`
    pub fn split_path(str: &str) -> Result<Vec<String>> {
        Self::split_path_limit(str, 0)
    }

    /// 对齐 Java: `splitPathToArray(CharSequence)`
    pub fn split_path_to_array(str: &str) -> Result<Vec<String>> {
        Self::split_path(str)
    }

    /// 对齐 Java: `splitPathToArray(CharSequence, int)`
    pub fn split_path_to_array_limit(str: &str, limit: i32) -> Result<Vec<String>> {
        Self::split_path_limit(str, limit)
    }

    /// 对齐 Java: `StrSplitter.splitToArray(char separator, ...)`
    ///
    /// 与 `CharSequenceUtil.splitToArray` 不同:此处 `None` 返回空数组(对齐 `StrSplitterTest.splitNullTest`)。
    pub fn split_to_array_char(
        str: Option<&str>,
        sep: char,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        let Some(value) = str else {
            return Ok(Vec::new());
        };
        if value.is_empty() {
            return Ok(vec![String::new()]);
        }
        Self::split_char_limit(value, sep, limit, trim, ignore_empty)
    }

    /// 对齐 Java: `splitToArray`
    pub fn split_to_array(
        str: Option<&str>,
        sep: &str,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        let Some(s) = str else {
            return Ok(Vec::new());
        };
        if s.is_empty() {
            // Java: "".split(",") → [""]
            return Ok(vec![String::new()]);
        }
        Self::split_str_limit(s, sep, limit, trim, ignore_empty)
    }

    /// 对齐 Java: `splitByRegex`
    pub fn split_by_regex(
        text: &str,
        regex: &str,
        limit: i32,
        is_trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        if regex.is_empty() {
            return if ignore_empty && text.is_empty() {
                Ok(Vec::new())
            } else {
                Ok(vec![text.to_string()])
            };
        }
        let re = Regex::new(regex).map_err(|e| crate::CoreError::Codec(format!("regex: {e}")))?;
        let mut parts: Vec<String> = Vec::new();
        let mut last = 0usize;
        for m in re.find_iter(text) {
            parts.push(text[last..m.start()].to_string());
            last = m.end();
        }
        parts.push(text[last..].to_string());
        if is_trim {
            for p in &mut parts {
                *p = p.trim().to_string();
            }
        }
        if ignore_empty {
            parts.retain(|p| !p.is_empty());
        }
        if limit > 0 && parts.len() as i32 > limit {
            parts.truncate(limit as usize);
        }
        Ok(parts)
    }

    /// 对齐 Java: `split(String, Pattern, ...)` —— Pattern 以正则字符串传入。
    pub fn split_by_pattern(
        text: &str,
        pattern: &str,
        limit: i32,
        is_trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_by_regex(text, pattern, limit, is_trim, ignore_empty)
    }

    /// 对齐 Java: `splitToArray(String, Pattern, ...)`
    pub fn split_to_array_by_pattern(
        text: &str,
        pattern: &str,
        limit: i32,
        is_trim: bool,
        ignore_empty: bool,
    ) -> Result<Vec<String>> {
        Self::split_by_regex(text, pattern, limit, is_trim, ignore_empty)
    }

    /// 对齐 Java: `splitByLength(CharSequence, int)`
    pub fn split_by_length(text: &str, len: i32) -> Result<Vec<String>> {
        if len <= 0 {
            return Ok(vec![text.to_string()]);
        }
        let step = len as usize;
        let chars: Vec<char> = text.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < chars.len() {
            let end = (i + step).min(chars.len());
            out.push(chars[i..end].iter().collect());
            i = end;
        }
        if out.is_empty() {
            out.push(String::new());
        }
        Ok(out)
    }

    /// 对齐 Java: `split(..., Function)` mapping
    pub fn split_map<T, F>(
        str: &str,
        sep: char,
        limit: i32,
        trim: bool,
        ignore_empty: bool,
        map: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(&str) -> std::result::Result<T, String>,
    {
        let parts = Self::split_char_limit(str, sep, limit, trim, ignore_empty)?;
        let mut out = Vec::with_capacity(parts.len());
        for p in parts {
            out.push(map(&p).map_err(crate::CoreError::Codec)?);
        }
        Ok(out)
    }

    /// 对齐 Java: `split(..., ignoreCase, Function)` mapping
    pub fn split_map_case<T, F>(
        str: &str,
        sep: char,
        limit: i32,
        ignore_empty: bool,
        ignore_case: bool,
        map: F,
    ) -> Result<Vec<T>>
    where
        F: Fn(&str) -> std::result::Result<T, String>,
    {
        let parts = Self::split_char_limit_case(str, sep, limit, false, ignore_empty, ignore_case)?;
        let mut out = Vec::with_capacity(parts.len());
        for p in parts {
            out.push(map(&p).map_err(crate::CoreError::Codec)?);
        }
        Ok(out)
    }
}
