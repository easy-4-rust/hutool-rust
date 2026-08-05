//! 对齐: `cn.hutool.core.text.CharSequenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/CharSequenceUtil.java
//!
//! 字符序列工具类(Java 中为 `StrUtil` 的实现基类),含判空、切片、替换、格式化、Unicode 等。
//! 注意: hutool-rs 惯用实现在 `string.rs`。本类型按 Java 类粒度保留 Hutool 命名表面,
//! 可映射方法委托到 `string.rs` / `StrSplitter`;语义可用本地安全实现补齐的方法一并落地。
//! 仍依赖 Java `Object`/引擎能力、无法无损映射的 API 见 `util/str_util.rs` 与 compat。

use std::fmt::Display;

use crate::string::{
    clean_blank as string_clean_blank, contains as string_contains,
    contains_ignore_case as string_contains_ignore_case, cut as string_cut,
    end_with as string_end_with, equals as string_equals,
    equals_ignore_case as string_equals_ignore_case, format_template as string_format_template,
    index_of_ignore_case as string_index_of_ignore_case, indexed_format as string_indexed_format,
    is_blank as string_is_blank, last_index_of as string_last_index_of, length as string_length,
    remove_all as string_remove_all, repeat as string_repeat, replace as string_replace,
    replace_by_code_point as string_replace_by_code_point, split as string_split,
    split_to_array as string_split_to_array, split_to_array_limit as string_split_to_array_limit,
    start_with as string_start_with, strip as string_strip,
    sub_by_code_point as string_sub_by_code_point, trim as string_trim,
};
use crate::text::str_splitter::StrSplitter;
use crate::{CoreError, Result};

/// 对齐 Java: `CharSequenceUtil#`
#[derive(Debug, Clone, Copy, Default)]
pub struct CharSequenceUtil;

impl CharSequenceUtil {
    // ---- 常量对齐 ----

    /// 对齐 Java: `CharSequenceUtil::INDEX_NOT_FOUND`
    pub const INDEX_NOT_FOUND: i32 = -1;
    /// 对齐 Java: `CharSequenceUtil::NULL`
    pub const NULL: &'static str = "null";
    /// 对齐 Java: `CharSequenceUtil::EMPTY`
    pub const EMPTY: &'static str = "";
    /// 对齐 Java: `CharSequenceUtil::SPACE`
    pub const SPACE: &'static str = " ";

    // ---- 判空 ----

    /// 对齐 Java: `CharSequenceUtil::isBlank#boolean (CharSequence str)`
    pub fn is_blank(str: &str) -> Result<bool> {
        // 委托: crate::string::is_blank
        Ok(string_is_blank(str))
    }

    /// 对齐 Java: `CharSequenceUtil::isNotBlank#boolean (CharSequence str)`
    pub fn is_not_blank(str: &str) -> Result<bool> {
        Ok(!string_is_blank(str))
    }

    /// 对齐 Java: `CharSequenceUtil::hasBlank#boolean (CharSequence... strs)`
    pub fn has_blank(strs: &[&str]) -> Result<bool> {
        Ok(strs.is_empty() || strs.iter().any(|s| string_is_blank(s)))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllBlank#boolean (CharSequence... strs)`
    pub fn is_all_blank(strs: &[&str]) -> Result<bool> {
        Ok(strs.iter().all(|s| string_is_blank(s)))
    }

    /// 对齐 Java: `CharSequenceUtil::isEmpty#boolean (CharSequence str)`
    pub fn is_empty(str: &str) -> Result<bool> {
        Ok(str.is_empty())
    }

    /// 对齐 Java: `CharSequenceUtil::isNotEmpty#boolean (CharSequence str)`
    pub fn is_not_empty(str: &str) -> Result<bool> {
        Ok(!str.is_empty())
    }

    /// 对齐 Java: `CharSequenceUtil::emptyIfNull#String (CharSequence str)`
    pub fn empty_if_null(str: Option<&str>) -> Result<String> {
        Ok(str.unwrap_or("").to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::nullToEmpty#String (CharSequence str)`
    pub fn null_to_empty(str: Option<&str>) -> Result<String> {
        Ok(str.unwrap_or("").to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::nullToDefault#String (CharSequence str, String defaultStr)`
    pub fn null_to_default(str: Option<&str>, default: &str) -> Result<String> {
        Ok(str.unwrap_or(default).to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::emptyToDefault#String (CharSequence str, String defaultStr)`
    pub fn empty_to_default(str: &str, default: &str) -> Result<String> {
        Ok(if str.is_empty() {
            default.to_owned()
        } else {
            str.to_owned()
        })
    }

    /// 对齐 Java: `CharSequenceUtil::blankToDefault#String (CharSequence str, String defaultStr)`
    pub fn blank_to_default(str: &str, default: &str) -> Result<String> {
        Ok(if string_is_blank(str) {
            default.to_owned()
        } else {
            str.to_owned()
        })
    }

    /// 对齐 Java: `CharSequenceUtil::emptyToNull#String (CharSequence str)`
    pub fn empty_to_null(str: &str) -> Result<Option<String>> {
        Ok(if str.is_empty() {
            None
        } else {
            Some(str.to_owned())
        })
    }

    /// 对齐 Java: `CharSequenceUtil::hasEmpty#boolean (CharSequence... strs)`
    pub fn has_empty(strs: &[&str]) -> Result<bool> {
        Ok(strs.is_empty() || strs.iter().any(|s| s.is_empty()))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllEmpty#boolean (CharSequence... strs)`
    pub fn is_all_empty(strs: &[&str]) -> Result<bool> {
        Ok(strs.iter().all(|s| s.is_empty()))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllNotEmpty#boolean (CharSequence... args)`
    pub fn is_all_not_empty(args: &[&str]) -> Result<bool> {
        Ok(!args.is_empty() && args.iter().all(|s| !s.is_empty()))
    }

    /// 对齐 Java: `CharSequenceUtil::isAllNotBlank#boolean (CharSequence... args)`
    pub fn is_all_not_blank(args: &[&str]) -> Result<bool> {
        Ok(!args.is_empty() && args.iter().all(|s| !string_is_blank(s)))
    }

    /// 对齐 Java: `CharSequenceUtil::isNullOrUndefined#boolean (CharSequence str)`
    pub fn is_null_or_undefined(str: Option<&str>) -> Result<bool> {
        Ok(match str {
            None => true,
            Some(s) => s.eq_ignore_ascii_case("undefined") || s.eq_ignore_ascii_case("null"),
        })
    }

    /// 对齐 Java: `CharSequenceUtil::isEmptyOrUndefined#boolean (CharSequence str)`
    pub fn is_empty_or_undefined(str: Option<&str>) -> Result<bool> {
        Ok(match str {
            None => true,
            Some(s) => {
                s.is_empty()
                    || s.eq_ignore_ascii_case("undefined")
                    || s.eq_ignore_ascii_case("null")
            }
        })
    }

    /// 对齐 Java: `CharSequenceUtil::isBlankOrUndefined#boolean (CharSequence str)`
    pub fn is_blank_or_undefined(str: Option<&str>) -> Result<bool> {
        Ok(match str {
            None => true,
            Some(s) => {
                string_is_blank(s)
                    || s.eq_ignore_ascii_case("undefined")
                    || s.eq_ignore_ascii_case("null")
            }
        })
    }

    // ---- 修剪 ----

    /// 对齐 Java: `CharSequenceUtil::trim#String (CharSequence str)`
    pub fn trim(str: &str) -> Result<String> {
        Ok(string_trim(str).to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::trimToEmpty#String (CharSequence str)`
    pub fn trim_to_empty(str: Option<&str>) -> Result<String> {
        Ok(string_trim(str.unwrap_or("")).to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::trimToNull#String (CharSequence str)`
    pub fn trim_to_null(str: Option<&str>) -> Result<Option<String>> {
        let trimmed = string_trim(str.unwrap_or(""));
        Ok(if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_owned())
        })
    }

    /// 对齐 Java: `CharSequenceUtil::trimStart#String (CharSequence str)`
    pub fn trim_start(str: &str) -> Result<String> {
        Ok(str.trim_start().to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::trimEnd#String (CharSequence str)`
    pub fn trim_end(str: &str) -> Result<String> {
        Ok(str.trim_end().to_owned())
    }

    // ---- 前缀/后缀判断 ----

    /// 对齐 Java: `CharSequenceUtil::startWith#boolean (CharSequence str, char c)`
    pub fn start_with_char(str: &str, c: char) -> Result<bool> {
        Ok(str.starts_with(c))
    }

    /// 对齐 Java: `CharSequenceUtil::startWith#boolean (CharSequence str, CharSequence prefix, boolean ignoreCase)`
    pub fn start_with(str: &str, prefix: &str, ignore_case: bool) -> Result<bool> {
        Ok(if ignore_case {
            str.to_lowercase().starts_with(&prefix.to_lowercase())
        } else {
            string_start_with(str, prefix)
        })
    }

    /// 对齐 Java: `CharSequenceUtil::startWithIgnoreEquals#boolean (CharSequence str, CharSequence prefix)`
    pub fn start_with_ignore_equals(str: &str, prefix: &str) -> Result<bool> {
        Ok(string_start_with(str, prefix) && str != prefix)
    }

    /// 对齐 Java: `CharSequenceUtil::startWithIgnoreCase#boolean (CharSequence str, CharSequence prefix)`
    pub fn start_with_ignore_case(str: &str, prefix: &str) -> Result<bool> {
        Ok(str.to_lowercase().starts_with(&prefix.to_lowercase()))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithAny#boolean (CharSequence str, CharSequence... prefixes)`
    pub fn start_with_any(str: &str, prefixes: &[&str]) -> Result<bool> {
        Ok(prefixes.iter().any(|p| string_start_with(str, p)))
    }

    /// 对齐 Java: `CharSequenceUtil::startWithAnyIgnoreCase#boolean (CharSequence str, CharSequence... prefixes)`
    pub fn start_with_any_ignore_case(str: &str, prefixes: &[&str]) -> Result<bool> {
        let lower = str.to_lowercase();
        Ok(prefixes
            .iter()
            .any(|p| lower.starts_with(&p.to_lowercase())))
    }

    /// 对齐 Java: `CharSequenceUtil::endWith#boolean (CharSequence str, char c)`
    pub fn end_with_char(str: &str, c: char) -> Result<bool> {
        Ok(str.ends_with(c))
    }

    /// 对齐 Java: `CharSequenceUtil::endWith#boolean (CharSequence str, CharSequence suffix, boolean ignoreCase)`
    pub fn end_with(str: &str, suffix: &str, ignore_case: bool) -> Result<bool> {
        Ok(if ignore_case {
            str.to_lowercase().ends_with(&suffix.to_lowercase())
        } else {
            string_end_with(str, suffix)
        })
    }

    /// 对齐 Java: `CharSequenceUtil::endWithIgnoreCase#boolean (CharSequence str, CharSequence suffix)`
    pub fn end_with_ignore_case(str: &str, suffix: &str) -> Result<bool> {
        Ok(str.to_lowercase().ends_with(&suffix.to_lowercase()))
    }

    /// 对齐 Java: `CharSequenceUtil::endWithAny#boolean (CharSequence str, CharSequence... suffixes)`
    pub fn end_with_any(str: &str, suffixes: &[&str]) -> Result<bool> {
        Ok(suffixes.iter().any(|s| string_end_with(str, s)))
    }

    /// 对齐 Java: `CharSequenceUtil::endWithAnyIgnoreCase#boolean (CharSequence str, CharSequence... suffixes)`
    pub fn end_with_any_ignore_case(str: &str, suffixes: &[&str]) -> Result<bool> {
        let lower = str.to_lowercase();
        Ok(suffixes.iter().any(|s| lower.ends_with(&s.to_lowercase())))
    }

    // ---- 包含/位置 ----

    /// 对齐 Java: `CharSequenceUtil::contains#boolean (CharSequence str, char searchChar)`
    pub fn contains_char(str: &str, c: char) -> Result<bool> {
        Ok(str.contains(c))
    }

    /// 对齐 Java: `CharSequenceUtil::contains#boolean (CharSequence str, CharSequence searchStr)`
    pub fn contains(str: &str, search: &str) -> Result<bool> {
        Ok(string_contains(str, search))
    }

    /// 对齐 Java: `CharSequenceUtil::containsAny#boolean (CharSequence str, CharSequence... testStrs)`
    pub fn contains_any(str: &str, test_strs: &[&str]) -> Result<bool> {
        Ok(test_strs.iter().any(|t| string_contains(str, t)))
    }

    /// 对齐 Java: `CharSequenceUtil::containsOnly#boolean (CharSequence str, char... testChars)`
    pub fn contains_only(str: &str, test_chars: &[char]) -> Result<bool> {
        Ok(str.chars().all(|c| test_chars.contains(&c)))
    }

    /// 对齐 Java: `CharSequenceUtil::containsAll#boolean (CharSequence str, CharSequence... testChars)`
    pub fn contains_all(str: &str, test_chars: &[&str]) -> Result<bool> {
        Ok(test_chars.iter().all(|t| string_contains(str, t)))
    }

    /// 对齐 Java: `CharSequenceUtil::containsBlank#boolean (CharSequence str)`
    pub fn contains_blank(str: &str) -> Result<bool> {
        Ok(str.chars().any(|c| c.is_whitespace() || c == '　'))
    }

    /// 对齐 Java: `CharSequenceUtil::containsIgnoreCase#boolean (CharSequence str, CharSequence testStr)`
    pub fn contains_ignore_case(str: &str, test: &str) -> Result<bool> {
        Ok(string_contains_ignore_case(str, test))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOf#int (CharSequence str, char searchChar)`
    pub fn index_of_char(str: &str, c: char) -> Result<i32> {
        Ok(str
            .find(c)
            .map(|i| i as i32)
            .unwrap_or(Self::INDEX_NOT_FOUND))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOf#int (CharSequence str, char searchChar, int start)`
    pub fn index_of_char_start(str: &str, c: char, start: i32) -> Result<i32> {
        if start < 0 {
            return Ok(Self::INDEX_NOT_FOUND);
        }
        let mut start = start as usize;
        if start > str.len() {
            return Ok(Self::INDEX_NOT_FOUND);
        }
        if !str.is_char_boundary(start) {
            start = str.ceil_char_boundary(start);
        }
        Ok(str[start..]
            .find(c)
            .map(|i| (start + i) as i32)
            .unwrap_or(Self::INDEX_NOT_FOUND))
    }

    /// 对齐 Java: `CharSequenceUtil::indexOfIgnoreCase#int (CharSequence str, CharSequence searchStr)`
    pub fn index_of_ignore_case(str: &str, search: &str) -> Result<i32> {
        Ok(string_index_of_ignore_case(str, search)
            .map(|i| i as i32)
            .unwrap_or(Self::INDEX_NOT_FOUND))
    }

    /// 对齐 Java: `CharSequenceUtil::ordinalIndexOf#int (CharSequence str, CharSequence searchStr, int ordinal)`
    pub fn ordinal_index_of(str: &str, search: &str, ordinal: i32) -> Result<i32> {
        if ordinal <= 0 {
            return Ok(Self::INDEX_NOT_FOUND);
        }
        if search.is_empty() {
            return Ok(0);
        }
        let mut found = 0i32;
        let mut from = 0usize;
        while from <= str.len() {
            let Some(rel) = str[from..].find(search) else {
                return Ok(Self::INDEX_NOT_FOUND);
            };
            let index = from + rel;
            found += 1;
            if found >= ordinal {
                return Ok(index as i32);
            }
            from = index + 1;
            if from > str.len() {
                break;
            }
            if !str.is_char_boundary(from) {
                from = str.ceil_char_boundary(from);
            }
        }
        Ok(Self::INDEX_NOT_FOUND)
    }

    // ---- 删除/剥离 ----

    /// 对齐 Java: `CharSequenceUtil::removeAll#String (CharSequence str, CharSequence strToRemove)`
    pub fn remove_all(str: &str, to_remove: &str) -> Result<String> {
        Ok(string_remove_all(str, to_remove))
    }

    /// 对齐 Java: `CharSequenceUtil::removeAllLineBreaks#String (CharSequence str)`
    pub fn remove_all_line_breaks(str: &str) -> Result<String> {
        Ok(str.chars().filter(|c| *c != '\r' && *c != '\n').collect())
    }

    /// 对齐 Java: `CharSequenceUtil::removePrefix#String (CharSequence str, CharSequence prefix)`
    pub fn remove_prefix(str: &str, prefix: &str) -> Result<String> {
        Ok(str.strip_prefix(prefix).unwrap_or(str).to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::removeSuffix#String (CharSequence str, CharSequence suffix)`
    pub fn remove_suffix(str: &str, suffix: &str) -> Result<String> {
        Ok(str.strip_suffix(suffix).unwrap_or(str).to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::cleanBlank#String (CharSequence str)`
    pub fn clean_blank(str: &str) -> Result<String> {
        Ok(string_clean_blank(str))
    }

    /// 对齐 Java: `CharSequenceUtil::strip#String (CharSequence str, CharSequence prefixOrSuffix)`
    pub fn strip(str: &str, prefix_or_suffix: &str) -> Result<String> {
        Ok(string_strip(str, prefix_or_suffix))
    }

    /// 对齐 Java: `CharSequenceUtil::strip#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn strip_full(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        let mut from = 0usize;
        let mut to = str.len();
        if !prefix.is_empty() && string_start_with(str, prefix) {
            from = prefix.len();
            if from == to {
                return Ok(String::new());
            }
        }
        if !suffix.is_empty() && string_end_with(str, suffix) {
            to = to.saturating_sub(suffix.len());
            if from == to {
                return Ok(String::new());
            }
            if to < from {
                // 前缀剥离后与后缀重叠时回退后缀剥离
                to += suffix.len();
            }
        }
        if from > to || !str.is_char_boundary(from) || !str.is_char_boundary(to) {
            return Ok(str.to_owned());
        }
        Ok(str[from..to].to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::addPrefixIfNot#String (CharSequence str, CharSequence prefix)`
    pub fn add_prefix_if_not(str: &str, prefix: &str) -> Result<String> {
        Ok(if string_start_with(str, prefix) {
            str.to_owned()
        } else {
            format!("{prefix}{str}")
        })
    }

    /// 对齐 Java: `CharSequenceUtil::addSuffixIfNot#String (CharSequence str, CharSequence suffix)`
    pub fn add_suffix_if_not(str: &str, suffix: &str) -> Result<String> {
        Ok(if string_end_with(str, suffix) {
            str.to_owned()
        } else {
            format!("{str}{suffix}")
        })
    }

    // ---- 切割/切片 ----

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, char separator)`
    pub fn split_char(str: &str, sep: char) -> Result<Vec<String>> {
        Ok(string_split(str, sep, false, false)
            .into_iter()
            .map(str::to_owned)
            .collect())
    }

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, char separator, int limit)`
    pub fn split_char_limit(str: &str, sep: char, limit: i32) -> Result<Vec<String>> {
        string_split_to_array_limit(Some(str), sep, limit)
    }

    /// 对齐 Java: `CharSequenceUtil::splitToArray#String[] (CharSequence text, char separator, int limit)`
    pub fn split_to_array_limit(
        text: Option<&str>,
        separator: char,
        limit: i32,
    ) -> Result<Vec<String>> {
        string_split_to_array_limit(text, separator, limit)
    }

    /// 对齐 Java: `CharSequenceUtil::splitToArray#String[] (CharSequence text, char separator)`
    pub fn split_to_array(text: Option<&str>, separator: char) -> Result<Vec<String>> {
        string_split_to_array(text, separator)
    }

    /// 对齐 Java: `CharSequenceUtil::split#List<String> (CharSequence str, CharSequence separator)`
    pub fn split_str(str: &str, sep: &str) -> Result<Vec<String>> {
        StrSplitter::split_str(str, sep, false, false)
    }

    /// 对齐 Java: `CharSequenceUtil::cut#String[] (CharSequence str, int partLength)`
    pub fn cut(str: &str, part_length: i32) -> Result<Vec<String>> {
        if part_length <= 0 {
            return Err(CoreError::InvalidArgument {
                name: "partLength",
                reason: "must be greater than zero",
            });
        }
        string_cut(str, part_length as usize)
    }

    /// 对齐 Java: `CharSequenceUtil::sub#String (CharSequence str, int fromIndexInclude, int toIndexExclude)`
    ///
    /// 下标按 Unicode 码点计数(对齐 Rust 侧 Unicode 友好约定;Java 为 UTF-16 代码单元)。
    pub fn sub(str: &str, from: i32, to: i32) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        let chars: Vec<char> = str.chars().collect();
        let len = chars.len() as i32;
        let mut from_index = from;
        let mut to_index = to;

        if from_index < 0 {
            from_index += len;
            if from_index < 0 {
                from_index = 0;
            }
        } else if from_index > len {
            from_index = len;
        }

        if to_index < 0 {
            to_index += len;
            if to_index < 0 {
                to_index = len;
            }
        } else if to_index > len {
            to_index = len;
        }

        if to_index < from_index {
            std::mem::swap(&mut from_index, &mut to_index);
        }
        if from_index == to_index {
            return Ok(String::new());
        }
        Ok(chars[from_index as usize..to_index as usize]
            .iter()
            .collect())
    }

    /// 对齐 Java: `CharSequenceUtil::subByCodePoint#String (CharSequence str, int fromIndex, int toIndex)`
    pub fn sub_by_code_point(str: &str, from_index: i32, to_index: i32) -> Result<String> {
        string_sub_by_code_point(str, from_index, to_index)
    }

    /// 对齐 Java: `CharSequenceUtil::subPre#String (CharSequence string, int toIndexExclude)`
    pub fn sub_pre(str: &str, to_exclude: i32) -> Result<String> {
        Self::sub(str, 0, to_exclude)
    }

    /// 对齐 Java: `CharSequenceUtil::subSuf#String (CharSequence string, int fromIndex)`
    pub fn sub_suf(str: &str, from: i32) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        let len = str.chars().count() as i32;
        Self::sub(str, from, len)
    }

    /// 对齐 Java: `CharSequenceUtil::subBefore#String (CharSequence string, CharSequence separator, boolean isLastSeparator)`
    pub fn sub_before(str: &str, sep: &str, last: bool) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        if sep.is_empty() {
            return Ok(String::new());
        }
        let pos = if last {
            string_last_index_of(str, sep)
        } else {
            str.find(sep)
        };
        Ok(match pos {
            None => str.to_owned(),
            Some(0) => String::new(),
            Some(p) => str[..p].to_owned(),
        })
    }

    /// 对齐 Java: `CharSequenceUtil::subAfter#String (CharSequence string, CharSequence separator, boolean isLastSeparator)`
    pub fn sub_after(str: &str, sep: &str, last: bool) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        if sep.is_empty() {
            return Ok(str.to_owned());
        }
        let pos = if last {
            string_last_index_of(str, sep)
        } else {
            str.find(sep)
        };
        Ok(match pos {
            None => String::new(),
            Some(p) if p + sep.len() >= str.len() => String::new(),
            Some(p) => str[p + sep.len()..].to_owned(),
        })
    }

    /// 对齐 Java: `CharSequenceUtil::subBetween#String (CharSequence str, CharSequence before, CharSequence after)`
    pub fn sub_between(str: &str, before: &str, after: &str) -> Result<String> {
        let start = match str.find(before) {
            Some(s) => s,
            None => return Ok(String::new()),
        };
        let content_start = start + before.len();
        if content_start > str.len() {
            return Ok(String::new());
        }
        let end = match str[content_start..].find(after) {
            Some(rel) => content_start + rel,
            None => return Ok(String::new()),
        };
        Ok(str[content_start..end].to_owned())
    }

    // ---- 重复/拼接/填充 ----

    /// 对齐 Java: `CharSequenceUtil::repeat#String (char c, int count)`
    pub fn repeat_char(c: char, count: i32) -> Result<String> {
        if count < 0 {
            return Err(CoreError::InvalidArgument {
                name: "count",
                reason: "must be >= 0",
            });
        }
        Ok(c.to_string().repeat(count as usize))
    }

    /// 对齐 Java: `CharSequenceUtil::repeat#String (CharSequence str, int count)`
    pub fn repeat(str: &str, count: i32) -> Result<String> {
        if count < 0 {
            return Err(CoreError::InvalidArgument {
                name: "count",
                reason: "must be >= 0",
            });
        }
        Ok(string_repeat(str, count as usize))
    }

    /// 对齐 Java: `CharSequenceUtil::repeatAndJoin#String (CharSequence str, int count, CharSequence delimiter)`
    pub fn repeat_and_join(str: &str, count: i32, delim: &str) -> Result<String> {
        if count <= 0 {
            return Ok(String::new());
        }
        let mut out = String::with_capacity(str.len().saturating_mul(count as usize));
        out.push_str(str);
        for _ in 1..count {
            if !delim.is_empty() {
                out.push_str(delim);
            }
            out.push_str(str);
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::wrap#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn wrap(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        Ok(format!("{prefix}{str}{suffix}"))
    }

    /// 对齐 Java: `CharSequenceUtil::wrapIfMissing#String (CharSequence str, CharSequence prefix, CharSequence suffix)`
    pub fn wrap_if_missing(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        let mut out = String::new();
        if !prefix.is_empty() && !string_start_with(str, prefix) {
            out.push_str(prefix);
        }
        out.push_str(str);
        if !suffix.is_empty() && !string_end_with(str, suffix) {
            out.push_str(suffix);
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::unWrap#String (CharSequence str, String prefix, String suffix)`
    pub fn unwrap(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        if Self::is_wrap(str, prefix, suffix)? {
            Ok(str[prefix.len()..str.len() - suffix.len()].to_owned())
        } else {
            Ok(str.to_owned())
        }
    }

    /// 对齐 Java: `CharSequenceUtil::isWrap#boolean (CharSequence str, String prefix, String suffix)`
    pub fn is_wrap(str: &str, prefix: &str, suffix: &str) -> Result<bool> {
        Ok(string_start_with(str, prefix) && string_end_with(str, suffix))
    }

    /// 对齐 Java: `CharSequenceUtil::padPre#String (CharSequence str, int length, char padChar)`
    pub fn pad_pre_char(str: &str, length: i32, pad: char) -> Result<String> {
        let str_len = str.chars().count() as i32;
        if str_len == length {
            return Ok(str.to_owned());
        }
        if str_len > length {
            return Self::sub_pre(str, length);
        }
        if length <= 0 {
            return Ok(String::new());
        }
        let pad_count = (length - str_len) as usize;
        Ok(format!("{}{str}", pad.to_string().repeat(pad_count)))
    }

    /// 对齐 Java: `CharSequenceUtil::padAfter#String (CharSequence str, int length, char padChar)`
    pub fn pad_after_char(str: &str, length: i32, pad: char) -> Result<String> {
        let str_len = str.chars().count() as i32;
        if str_len == length {
            return Ok(str.to_owned());
        }
        if str_len > length {
            return Self::sub(str, str_len - length, str_len);
        }
        if length <= 0 {
            return Ok(String::new());
        }
        let pad_count = (length - str_len) as usize;
        Ok(format!("{str}{}", pad.to_string().repeat(pad_count)))
    }

    /// 对齐 Java: `CharSequenceUtil::center#String (CharSequence str, final int size)`
    pub fn center(str: &str, size: i32) -> Result<String> {
        if size <= 0 {
            return Ok(str.to_owned());
        }
        let str_len = str.chars().count() as i32;
        let pads = size - str_len;
        if pads <= 0 {
            return Ok(str.to_owned());
        }
        let pad_left = pads / 2;
        let pad_right = pads - pad_left;
        Ok(format!(
            "{}{str}{}",
            " ".repeat(pad_left as usize),
            " ".repeat(pad_right as usize)
        ))
    }

    // ---- 比较 ----

    /// 对齐 Java: `CharSequenceUtil::equals#boolean (CharSequence str1, CharSequence str2)`
    pub fn equals(a: &str, b: &str) -> Result<bool> {
        Ok(string_equals(a, b))
    }

    /// 对齐 Java: `CharSequenceUtil::equalsIgnoreCase#boolean (CharSequence str1, CharSequence str2)`
    pub fn equals_ignore_case(a: &str, b: &str) -> Result<bool> {
        Ok(string_equals_ignore_case(a, b))
    }

    /// 对齐 Java: `CharSequenceUtil::equalsAny#boolean (CharSequence str1, CharSequence... strs)`
    pub fn equals_any(a: &str, strs: &[&str]) -> Result<bool> {
        Ok(strs.iter().any(|s| string_equals(a, s)))
    }

    /// 对齐 Java: `CharSequenceUtil::compare#int (CharSequence str1, CharSequence str2, boolean nullIsLess)`
    ///
    /// Rust 侧入参均为 `&str`,无 null;`null_is_less` 保留签名对齐,不影响比较结果。
    pub fn compare(a: &str, b: &str, _null_is_less: bool) -> Result<i32> {
        Ok(match a.cmp(b) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        })
    }

    /// 对齐 Java: `CharSequenceUtil::count#int (CharSequence content, CharSequence strForSearch)`
    pub fn count_str(content: &str, search: &str) -> Result<i32> {
        if search.is_empty() {
            return Ok(0);
        }
        Ok(content.matches(search).count() as i32)
    }

    /// 对齐 Java: `CharSequenceUtil::count#int (CharSequence content, char charForSearch)`
    pub fn count_char(content: &str, c: char) -> Result<i32> {
        Ok(content.chars().filter(|ch| *ch == c).count() as i32)
    }

    /// 对齐 Java: `CharSequenceUtil::length#int (CharSequence cs)`
    pub fn length(cs: &str) -> Result<i32> {
        Ok(string_length(Some(cs)) as i32)
    }

    // ---- 替换 ----

    /// 对齐 Java: `CharSequenceUtil::replace#String (CharSequence str, CharSequence searchStr, CharSequence replacement)`
    pub fn replace(str: &str, search: &str, replacement: &str) -> Result<String> {
        Ok(string_replace(str, search, replacement))
    }

    /// 对齐 Java: `CharSequenceUtil::replaceByCodePoint#String (CharSequence str, int startInclude, int endExclude, char replacedChar)`
    pub fn replace_by_code_point(
        str: &str,
        start_include: i32,
        end_exclude: i32,
        replaced_char: char,
    ) -> Result<String> {
        Ok(string_replace_by_code_point(
            str,
            start_include,
            end_exclude,
            replaced_char,
        ))
    }

    /// 对齐 Java: `CharSequenceUtil::replaceIgnoreCase#String (CharSequence str, CharSequence searchStr, CharSequence replacement)`
    pub fn replace_ignore_case(str: &str, search: &str, replacement: &str) -> Result<String> {
        if search.is_empty() {
            return Ok(str.to_owned());
        }
        let search_lower = search.to_lowercase();
        let search_char_len = search.chars().count();
        let mut out = String::with_capacity(str.len());
        let mut remaining = str;
        while let Some(pos) = remaining.to_lowercase().find(&search_lower) {
            if !remaining.is_char_boundary(pos) {
                let Some(ch) = remaining.chars().next() else {
                    break;
                };
                out.push(ch);
                remaining = &remaining[ch.len_utf8()..];
                continue;
            }
            out.push_str(&remaining[..pos]);
            out.push_str(replacement);
            let matched: String = remaining[pos..].chars().take(search_char_len).collect();
            remaining = &remaining[pos + matched.len()..];
        }
        out.push_str(remaining);
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::replaceLast#String (CharSequence str, CharSequence searchStr, CharSequence replacedStr)`
    pub fn replace_last(str: &str, search: &str, replacement: &str) -> Result<String> {
        Ok(if let Some(pos) = string_last_index_of(str, search) {
            let mut out = String::with_capacity(str.len());
            out.push_str(&str[..pos]);
            out.push_str(replacement);
            out.push_str(&str[pos + search.len()..]);
            out
        } else {
            str.to_owned()
        })
    }

    /// 对齐 Java: `CharSequenceUtil::replaceFirst#String (CharSequence str, CharSequence searchStr, CharSequence replacedStr)`
    pub fn replace_first(str: &str, search: &str, replacement: &str) -> Result<String> {
        Ok(if let Some(pos) = str.find(search) {
            let mut out = String::with_capacity(str.len());
            out.push_str(&str[..pos]);
            out.push_str(replacement);
            out.push_str(&str[pos + search.len()..]);
            out
        } else {
            str.to_owned()
        })
    }

    /// 对齐 Java: `CharSequenceUtil::hide#String (CharSequence str, int startInclude, int endExclude)`
    pub fn hide(str: &str, start: i32, end: i32) -> Result<String> {
        Ok(string_replace_by_code_point(str, start, end, '*'))
    }

    // ---- 格式化 ----

    /// 对齐 Java: `CharSequenceUtil::format#String (CharSequence template, Object... params)`
    pub fn format(template: &str, params: &[&str]) -> Result<String> {
        let args: Vec<&dyn Display> = params.iter().map(|p| p as &dyn Display).collect();
        Ok(string_format_template(template, &args))
    }

    /// 对齐 Java: `CharSequenceUtil::indexedFormat#String (CharSequence pattern, Object... arguments)`
    pub fn indexed_format(pattern: &str, args: &[&str]) -> Result<String> {
        let display_args: Vec<&dyn std::fmt::Display> = args
            .iter()
            .map(|arg| arg as &dyn std::fmt::Display)
            .collect();
        string_indexed_format(pattern, &display_args)
    }

    /// 对齐 Java: `CharSequenceUtil::appendIfMissing#String (CharSequence str, CharSequence suffix, CharSequence... suffixes)`
    pub fn append_if_missing(str: &str, suffix: &str, others: &[&str]) -> Result<String> {
        if suffix.is_empty() || string_end_with(str, suffix) {
            return Ok(str.to_owned());
        }
        if others
            .iter()
            .any(|s| !s.is_empty() && string_end_with(str, s))
        {
            return Ok(str.to_owned());
        }
        Ok(format!("{str}{suffix}"))
    }

    /// 对齐 Java: `CharSequenceUtil::prependIfMissing#String (CharSequence str, CharSequence prefix, CharSequence... prefixes)`
    pub fn prepend_if_missing(str: &str, prefix: &str, others: &[&str]) -> Result<String> {
        if prefix.is_empty() || string_start_with(str, prefix) {
            return Ok(str.to_owned());
        }
        if others
            .iter()
            .any(|s| !s.is_empty() && string_start_with(str, s))
        {
            return Ok(str.to_owned());
        }
        Ok(format!("{prefix}{str}"))
    }

    // ---- Round2 补齐：大小写 / 字节 / 命名 / 集合首个 ----

    /// 对齐 Java: `CharSequenceUtil::toLowerCase` / `toLoweCase`（拼写兼容）
    pub fn to_lower_case(str: &str) -> Result<String> {
        Ok(str.to_lowercase())
    }

    /// 对齐 Java: `CharSequenceUtil::toUpperCase`
    pub fn to_upper_case(str: &str) -> Result<String> {
        Ok(str.to_uppercase())
    }

    /// 对齐 Java: `CharSequenceUtil::swapCase`
    pub fn swap_case(str: &str) -> Result<String> {
        Ok(str
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase().collect::<String>()
                } else if c.is_lowercase() {
                    c.to_uppercase().collect::<String>()
                } else {
                    c.to_string()
                }
            })
            .collect())
    }

    /// 对齐 Java: `CharSequenceUtil::upperFirst`
    pub fn upper_first(str: &str) -> Result<String> {
        Ok(crate::string::upper_first(str))
    }

    /// 对齐 Java: `CharSequenceUtil::lowerFirst`
    pub fn lower_first(str: &str) -> Result<String> {
        Ok(crate::string::lower_first(str))
    }

    /// 对齐 Java: `CharSequenceUtil::upperFirstAndAddPre`
    pub fn upper_first_and_add_pre(str: &str, pre: &str) -> Result<String> {
        Ok(format!("{pre}{}", crate::string::upper_first(str)))
    }

    /// 对齐 Java: `CharSequenceUtil::utf8Bytes`
    pub fn utf8_bytes(str: &str) -> Result<Vec<u8>> {
        Ok(str.as_bytes().to_vec())
    }

    /// 对齐 Java: `CharSequenceUtil::bytes(CharSequence)` —— UTF-8
    pub fn bytes(str: &str) -> Result<Vec<u8>> {
        Self::utf8_bytes(str)
    }

    /// 对齐 Java: `CharSequenceUtil::bytes(CharSequence, String charset)`
    ///
    /// 仅识别常见 charset 名；未知时回退 UTF-8。
    pub fn bytes_charset(str: &str, charset: &str) -> Result<Vec<u8>> {
        let name = charset.to_ascii_lowercase().replace('_', "-");
        match name.as_str() {
            "utf-8" | "utf8" => Ok(str.as_bytes().to_vec()),
            "utf-16" | "utf16" => Ok(str.encode_utf16().flat_map(|u| u.to_be_bytes()).collect()),
            "iso-8859-1" | "latin1" => Ok(str.chars().map(|c| c as u8).collect()),
            _ => Ok(str.as_bytes().to_vec()),
        }
    }

    /// 对齐 Java: `CharSequenceUtil::removeAny`
    pub fn remove_any(str: &str, to_remove: &[&str]) -> Result<String> {
        let mut out = str.to_owned();
        for s in to_remove {
            if !s.is_empty() {
                out = out.replace(s, "");
            }
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::firstNonNull`
    pub fn first_non_null<'a>(strs: &[Option<&'a str>]) -> Result<Option<&'a str>> {
        Ok(strs.iter().flatten().copied().next())
    }

    /// 对齐 Java: `CharSequenceUtil::firstNonEmpty`
    pub fn first_non_empty<'a>(strs: &[&'a str]) -> Result<Option<&'a str>> {
        Ok(strs.iter().copied().find(|s| !s.is_empty()))
    }

    /// 对齐 Java: `CharSequenceUtil::firstNonBlank`
    pub fn first_non_blank<'a>(strs: &[&'a str]) -> Result<Option<&'a str>> {
        Ok(strs.iter().copied().find(|s| !string_is_blank(s)))
    }

    /// 对齐 Java: `CharSequenceUtil::commonPrefix`
    pub fn common_prefix(a: &str, b: &str) -> Result<String> {
        let mut out = String::new();
        for (ca, cb) in a.chars().zip(b.chars()) {
            if ca == cb {
                out.push(ca);
            } else {
                break;
            }
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::commonSuffix`
    pub fn common_suffix(a: &str, b: &str) -> Result<String> {
        let ac: Vec<char> = a.chars().collect();
        let bc: Vec<char> = b.chars().collect();
        let mut i = 0usize;
        while i < ac.len() && i < bc.len() && ac[ac.len() - 1 - i] == bc[bc.len() - 1 - i] {
            i += 1;
        }
        Ok(ac[ac.len() - i..].iter().collect())
    }

    /// 对齐 Java: `CharSequenceUtil::concat`
    pub fn concat(null_to_empty: bool, strs: &[Option<&str>]) -> Result<String> {
        let mut out = String::new();
        for s in strs {
            match s {
                Some(v) => out.push_str(v),
                None if null_to_empty => {}
                None => return Ok(out),
            }
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::join`
    pub fn join(conjunction: &str, strs: &[&str]) -> Result<String> {
        Ok(strs.join(conjunction))
    }

    /// 对齐 Java: `CharSequenceUtil::toCamelCase`
    pub fn to_camel_case(name: &str) -> Result<String> {
        crate::text::naming_case::NamingCase::to_camel_case(name)
    }

    /// 对齐 Java: `CharSequenceUtil::toUnderlineCase`
    pub fn to_underline_case(name: &str) -> Result<String> {
        crate::text::naming_case::NamingCase::to_underline_case(name)
    }

    /// 对齐 Java: `CharSequenceUtil::toSymbolCase`
    pub fn to_symbol_case(name: &str, symbol: char) -> Result<String> {
        crate::text::naming_case::NamingCase::to_symbol_case(name, symbol)
    }

    /// 对齐 Java: `CharSequenceUtil::isUpperCase`
    pub fn is_upper_case(str: &str) -> Result<bool> {
        Ok(!str.is_empty() && str.chars().all(|c| !c.is_lowercase()))
    }

    /// 对齐 Java: `CharSequenceUtil::isLowerCase`
    pub fn is_lower_case(str: &str) -> Result<bool> {
        Ok(!str.is_empty() && str.chars().all(|c| !c.is_uppercase()))
    }

    /// 对齐 Java: `CharSequenceUtil::isNumeric`
    pub fn is_numeric(str: &str) -> Result<bool> {
        Ok(!str.is_empty() && str.chars().all(|c| c.is_ascii_digit()))
    }

    /// 对齐 Java: `CharSequenceUtil::hasLetter`
    pub fn has_letter(str: &str) -> Result<bool> {
        Ok(str.chars().any(|c| c.is_alphabetic()))
    }

    /// 对齐 Java: `CharSequenceUtil::isCharEquals`
    pub fn is_char_equals(str: &str) -> Result<bool> {
        let mut chars = str.chars();
        let Some(first) = chars.next() else {
            return Ok(true);
        };
        Ok(chars.all(|c| c == first))
    }

    /// 对齐 Java: `CharSequenceUtil::totalLength`
    pub fn total_length(strs: &[&str]) -> Result<i32> {
        Ok(strs.iter().map(|s| s.chars().count() as i32).sum())
    }

    /// 对齐 Java: `CharSequenceUtil::maxLength`
    pub fn max_length(strs: &[&str]) -> Result<i32> {
        Ok(strs
            .iter()
            .map(|s| s.chars().count() as i32)
            .max()
            .unwrap_or(0))
    }

    /// 对齐 Java: `CharSequenceUtil::byteLength` —— UTF-8 字节长度
    pub fn byte_length(str: &str) -> Result<i32> {
        Ok(str.len() as i32)
    }

    /// 对齐 Java: `CharSequenceUtil::filter`
    pub fn filter(str: &str, pred: impl Fn(char) -> bool) -> Result<String> {
        Ok(str.chars().filter(|c| pred(*c)).collect())
    }

    /// 对齐 Java: `CharSequenceUtil::unWrap`
    pub fn un_wrap(str: &str, prefix: char, suffix: char) -> Result<String> {
        let mut chars = str.chars();
        let Some(first) = chars.next() else {
            return Ok(String::new());
        };
        let rest: String = chars.collect();
        if first == prefix && rest.ends_with(suffix) {
            let mut out: Vec<char> = rest.chars().collect();
            out.pop();
            Ok(out.into_iter().collect())
        } else {
            Ok(str.to_owned())
        }
    }

    /// 对齐 Java: `CharSequenceUtil::splitTrim` —— 分隔后 trim
    pub fn split_trim(str: &str, sep: char, ignore_empty: bool) -> Result<Vec<String>> {
        StrSplitter::split_trim_char(str, sep, ignore_empty)
    }

    /// 对齐 Java: `CharSequenceUtil::splitToInt`
    pub fn split_to_int(str: &str, sep: char) -> Result<Vec<i32>> {
        StrSplitter::split_map(str, sep, 0, true, true, |p| {
            p.parse::<i32>().map_err(|e| e.to_string())
        })
    }

    /// 对齐 Java: `CharSequenceUtil::splitToLong`
    pub fn split_to_long(str: &str, sep: char) -> Result<Vec<i64>> {
        StrSplitter::split_map(str, sep, 0, true, true, |p| {
            p.parse::<i64>().map_err(|e| e.to_string())
        })
    }

    /// 对齐 Java: `CharSequenceUtil::strBuilder` / `builder`
    pub fn str_builder() -> Result<crate::text::str_builder::StrBuilder> {
        Ok(crate::text::str_builder::StrBuilder::create())
    }

    /// 对齐 Java: `CharSequenceUtil::lastIndexOf`
    pub fn last_index_of(str: &str, search: &str) -> Result<i32> {
        Ok(match string_last_index_of(str, search) {
            Some(i) => {
                // string_last_index_of 返回字节下标；转为码点下标
                str[..i].chars().count() as i32
            }
            None => Self::INDEX_NOT_FOUND,
        })
    }

    /// 对齐 Java: `CharSequenceUtil::lastIndexOfIgnoreCase`
    pub fn last_index_of_ignore_case(str: &str, search: &str) -> Result<i32> {
        if search.is_empty() {
            return Ok(str.chars().count() as i32);
        }
        let hay = str.to_lowercase();
        let needle = search.to_lowercase();
        match hay.rfind(&needle) {
            Some(byte_pos) => Ok(hay[..byte_pos].chars().count() as i32),
            None => Ok(Self::INDEX_NOT_FOUND),
        }
    }

    /// 对齐 Java: `CharSequenceUtil::desensitized` —— 委托 `DesensitizedUtil`
    pub fn desensitized(
        str: &str,
        des_type: crate::desensitized_util::DesensitizedType,
    ) -> Result<String> {
        Ok(
            crate::desensitized_util::DesensitizedUtil::desensitized(Some(str), des_type)
                .unwrap_or_default(),
        )
    }

    /// 对齐 Java: `CharSequenceUtil::normalize` — Unicode NFC
    pub fn normalize(str: &str) -> Result<String> {
        use unicode_normalization::UnicodeNormalization;
        Ok(str.nfc().collect())
    }

    /// 对齐 Java: `CharSequenceUtil::brief`
    pub fn brief(str: &str, max_length: i32) -> Result<String> {
        let max = max_length.max(0) as usize;
        let chars: Vec<char> = str.chars().collect();
        if chars.len() <= max {
            return Ok(str.to_owned());
        }
        if max <= 3 {
            return Ok(chars.into_iter().take(max).collect());
        }
        let keep = max - 3;
        let head = keep / 2;
        let tail = keep - head;
        let mut out: String = chars.iter().take(head).collect();
        out.push_str("...");
        out.extend(chars[chars.len() - tail..].iter());
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::getContainsStr`
    pub fn get_contains_str(str: &str, test_strs: &[&str]) -> Result<Option<String>> {
        Ok(test_strs
            .iter()
            .find(|t| str.contains(*t))
            .map(|s| (*s).to_string()))
    }

    /// 对齐 Java: `CharSequenceUtil::getContainsStrIgnoreCase`
    pub fn get_contains_str_ignore_case(str: &str, test_strs: &[&str]) -> Result<Option<String>> {
        let lower = str.to_lowercase();
        Ok(test_strs
            .iter()
            .find(|t| lower.contains(&t.to_lowercase()))
            .map(|s| (*s).to_string()))
    }

    /// 对齐 Java: `CharSequenceUtil::builder` —— 同 `strBuilder`
    pub fn builder() -> Result<crate::text::str_builder::StrBuilder> {
        Self::str_builder()
    }

    /// 对齐 Java: `CharSequenceUtil::byteBuffer` —— 返回 UTF-8/指定编码字节。
    pub fn byte_buffer(str: &str, charset: &str) -> Result<Vec<u8>> {
        Self::bytes_charset(str, charset)
    }

    /// 对齐 Java: `CharSequenceUtil::fixLength`
    pub fn fix_length(str: &str, length: i32, pad: char) -> Result<String> {
        let target = length.max(0) as usize;
        let chars: Vec<char> = str.chars().collect();
        if chars.len() >= target {
            Ok(chars[..target].iter().collect())
        } else {
            let mut out: String = chars.into_iter().collect();
            while out.chars().count() < target {
                out.push(pad);
            }
            Ok(out)
        }
    }

    /// 对齐 Java: `CharSequenceUtil::isAllCharMatch`
    pub fn is_all_char_match(str: &str, pred: impl Fn(char) -> bool) -> Result<bool> {
        Ok(!str.is_empty() && str.chars().all(pred))
    }

    /// 对齐 Java: `CharSequenceUtil::isSubEquals`
    pub fn is_sub_equals(str: &str, start: i32, other: &str, ignore_case: bool) -> Result<bool> {
        let chars: Vec<char> = str.chars().collect();
        let s = start.max(0) as usize;
        let other_chars: Vec<char> = other.chars().collect();
        if s + other_chars.len() > chars.len() {
            return Ok(false);
        }
        let slice: String = chars[s..s + other_chars.len()].iter().collect();
        Ok(if ignore_case {
            slice.eq_ignore_ascii_case(other)
        } else {
            slice == other
        })
    }

    /// 对齐 Java: `CharSequenceUtil::isSurround`
    pub fn is_surround(str: &str, prefix: char, suffix: char) -> Result<bool> {
        let mut chars = str.chars();
        let Some(first) = chars.next() else {
            return Ok(false);
        };
        let rest: Vec<char> = chars.collect();
        Ok(first == prefix && rest.last().copied() == Some(suffix))
    }

    /// 对齐 Java: `CharSequenceUtil::move`
    pub fn move_range(str: &str, start: i32, end: i32, move_to: i32) -> Result<String> {
        let mut chars: Vec<char> = str.chars().collect();
        let len = chars.len() as i32;
        let s = start.clamp(0, len) as usize;
        let e = end.clamp(0, len) as usize;
        if s >= e {
            return Ok(str.to_owned());
        }
        let piece: Vec<char> = chars[s..e].to_vec();
        chars.drain(s..e);
        let mut insert = move_to.clamp(0, chars.len() as i32) as usize;
        if (move_to as usize) > s {
            insert = insert.saturating_sub(e - s);
        }
        for (i, c) in piece.into_iter().enumerate() {
            chars.insert(insert + i, c);
        }
        Ok(chars.into_iter().collect())
    }

    /// 对齐 Java: `CharSequenceUtil::removePreAndLowerFirst`
    pub fn remove_pre_and_lower_first(str: &str, prefix: &str) -> Result<String> {
        let rest = if str.starts_with(prefix) {
            &str[prefix.len()..]
        } else {
            str
        };
        Ok(crate::string::lower_first(rest))
    }

    /// 对齐 Java: `CharSequenceUtil::removeSufAndLowerFirst`
    pub fn remove_suf_and_lower_first(str: &str, suffix: &str) -> Result<String> {
        let rest = if str.ends_with(suffix) {
            &str[..str.len() - suffix.len()]
        } else {
            str
        };
        Ok(crate::string::lower_first(rest))
    }

    /// 对齐 Java: `CharSequenceUtil::genGetter`
    pub fn gen_getter(field: &str) -> Result<String> {
        Ok(format!("get{}", crate::string::upper_first(field)))
    }

    /// 对齐 Java: `CharSequenceUtil::genSetter`
    pub fn gen_setter(field: &str) -> Result<String> {
        Ok(format!("set{}", crate::string::upper_first(field)))
    }

    /// 对齐 Java: `CharSequenceUtil::getGeneralField`
    pub fn get_general_field(get_or_set: &str) -> Result<String> {
        let name = if let Some(rest) = get_or_set.strip_prefix("get") {
            rest
        } else if let Some(rest) = get_or_set.strip_prefix("set") {
            rest
        } else if let Some(rest) = get_or_set.strip_prefix("is") {
            rest
        } else {
            get_or_set
        };
        Ok(crate::string::lower_first(name))
    }

    /// 对齐 Java: `CharSequenceUtil::str` —— `CharSequence` → `String`（空输入返回空串）。
    pub fn str_value(str: Option<&str>) -> Result<String> {
        Ok(str.unwrap_or("").to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::toLoweCase`（Hutool 拼写保留）。
    pub fn to_lowe_case(str: &str) -> Result<String> {
        Self::to_lower_case(str)
    }

    /// 对齐 Java: `CharSequenceUtil::compareIgnoreCase`
    pub fn compare_ignore_case(a: &str, b: &str) -> Result<i32> {
        Ok(match a.to_lowercase().cmp(&b.to_lowercase()) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        })
    }

    /// 对齐 Java: `CharSequenceUtil::compareVersion` —— 委托 `VersionUtil`。
    pub fn compare_version(a: &str, b: &str) -> Result<i32> {
        use std::cmp::Ordering;
        let ord = if crate::VersionUtil::is_greater_than(Some(a), Some(b)) {
            Ordering::Greater
        } else if crate::VersionUtil::is_less_than(Some(a), Some(b)) {
            Ordering::Less
        } else {
            Ordering::Equal
        };
        Ok(match ord {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        })
    }

    /// 对齐 Java: `CharSequenceUtil::equalsCharAt`
    pub fn equals_char_at(str: &str, index: i32, c: char) -> Result<bool> {
        let chars: Vec<char> = str.chars().collect();
        let idx = if index < 0 {
            chars.len() as i32 + index
        } else {
            index
        };
        Ok(idx >= 0 && (idx as usize) < chars.len() && chars[idx as usize] == c)
    }

    /// 对齐 Java: `CharSequenceUtil::equalsAnyIgnoreCase`
    pub fn equals_any_ignore_case(a: &str, strs: &[&str]) -> Result<bool> {
        Ok(strs.iter().any(|s| a.eq_ignore_ascii_case(s)))
    }

    /// 对齐 Java: `CharSequenceUtil::containsAnyIgnoreCase`
    pub fn contains_any_ignore_case(str: &str, test_strs: &[&str]) -> Result<bool> {
        let lower = str.to_lowercase();
        Ok(test_strs.iter().any(|t| lower.contains(&t.to_lowercase())))
    }

    /// 对齐 Java: `CharSequenceUtil::appendIfMissingIgnoreCase`
    pub fn append_if_missing_ignore_case(
        str: &str,
        suffix: &str,
        others: &[&str],
    ) -> Result<String> {
        let lower = str.to_lowercase();
        let mut candidates = vec![suffix];
        candidates.extend_from_slice(others);
        if candidates
            .iter()
            .any(|s| lower.ends_with(&s.to_lowercase()))
        {
            Ok(str.to_owned())
        } else {
            Ok(format!("{str}{suffix}"))
        }
    }

    /// 对齐 Java: `CharSequenceUtil::prependIfMissingIgnoreCase`
    pub fn prepend_if_missing_ignore_case(
        str: &str,
        prefix: &str,
        others: &[&str],
    ) -> Result<String> {
        let lower = str.to_lowercase();
        let mut candidates = vec![prefix];
        candidates.extend_from_slice(others);
        if candidates
            .iter()
            .any(|s| lower.starts_with(&s.to_lowercase()))
        {
            Ok(str.to_owned())
        } else {
            Ok(format!("{prefix}{str}"))
        }
    }

    /// 对齐 Java: `CharSequenceUtil::removePrefixIgnoreCase`
    pub fn remove_prefix_ignore_case(str: &str, prefix: &str) -> Result<String> {
        if str.to_lowercase().starts_with(&prefix.to_lowercase()) {
            let chars: Vec<char> = str.chars().collect();
            let n = prefix.chars().count();
            Ok(chars[n.min(chars.len())..].iter().collect())
        } else {
            Ok(str.to_owned())
        }
    }

    /// 对齐 Java: `CharSequenceUtil::removeSuffixIgnoreCase`
    pub fn remove_suffix_ignore_case(str: &str, suffix: &str) -> Result<String> {
        if str.to_lowercase().ends_with(&suffix.to_lowercase()) {
            let chars: Vec<char> = str.chars().collect();
            let n = suffix.chars().count();
            let end = chars.len().saturating_sub(n);
            Ok(chars[..end].iter().collect())
        } else {
            Ok(str.to_owned())
        }
    }

    /// 对齐 Java: `CharSequenceUtil::removeAllPrefix`
    pub fn remove_all_prefix(str: &str, prefix: &str) -> Result<String> {
        if prefix.is_empty() {
            return Ok(str.to_owned());
        }
        let mut out = str.to_owned();
        while out.starts_with(prefix) {
            out = out[prefix.len()..].to_owned();
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::removeAllSuffix`
    pub fn remove_all_suffix(str: &str, suffix: &str) -> Result<String> {
        if suffix.is_empty() {
            return Ok(str.to_owned());
        }
        let mut out = str.to_owned();
        while out.ends_with(suffix) {
            out.truncate(out.len() - suffix.len());
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::stripIgnoreCase`
    pub fn strip_ignore_case(str: &str, prefix_or_suffix: &str) -> Result<String> {
        Self::strip_full_ignore_case(str, prefix_or_suffix, prefix_or_suffix)
    }

    /// 对齐 Java: `CharSequenceUtil::stripIgnoreCase(prefix, suffix)`
    pub fn strip_full_ignore_case(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        let mut out = Self::remove_prefix_ignore_case(str, prefix)?;
        out = Self::remove_suffix_ignore_case(&out, suffix)?;
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::stripAll(CharSequence, CharSequence)`
    pub fn strip_all(str: &str, prefix_or_suffix: &str) -> Result<String> {
        if str == prefix_or_suffix {
            return Ok(String::new());
        }
        Self::strip_all_pair(str, prefix_or_suffix, prefix_or_suffix)
    }

    /// 对齐 Java: `CharSequenceUtil::stripAll(CharSequence, CharSequence, CharSequence)`
    pub fn strip_all_pair(str: &str, prefix: &str, suffix: &str) -> Result<String> {
        if str.is_empty() {
            return Ok(String::new());
        }
        let mut from = 0usize;
        let mut to = str.len();
        if !prefix.is_empty() {
            while str[from..to].starts_with(prefix) {
                from += prefix.len();
                if from >= to {
                    return Ok(String::new());
                }
            }
        }
        if !suffix.is_empty() {
            while to > from && str[from..to].ends_with(suffix) {
                to -= suffix.len();
                if from == to {
                    return Ok(String::new());
                }
                if to < from {
                    to += suffix.len();
                    break;
                }
            }
        }
        Ok(str[from..to].to_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::replaceChars(CharSequence, char[], CharSequence)`
    pub fn replace_chars(str: &str, chars: &[char], replaced: &str) -> Result<String> {
        Ok(str
            .chars()
            .map(|c| {
                if chars.contains(&c) {
                    replaced.to_owned()
                } else {
                    c.to_string()
                }
            })
            .collect())
    }

    /// 对齐 Java: `CharSequenceUtil::replaceChars(CharSequence, CharSequence, CharSequence)`
    pub fn replace_chars_str(str: &str, chars: &str, replaced: &str) -> Result<String> {
        let set: Vec<char> = chars.chars().collect();
        Self::replace_chars(str, &set, replaced)
    }

    /// 对齐 Java: `CharSequenceUtil::repeatByLength`
    pub fn repeat_by_length(str: &str, pad_len: i32) -> Result<String> {
        if pad_len <= 0 {
            return Ok(String::new());
        }
        let chars: Vec<char> = str.chars().collect();
        if chars.is_empty() {
            return Ok(String::new());
        }
        let target = pad_len as usize;
        if chars.len() == target {
            return Ok(str.to_owned());
        }
        if chars.len() > target {
            return Ok(chars[..target].iter().collect());
        }
        let mut out = String::with_capacity(target);
        for i in 0..target {
            out.push(chars[i % chars.len()]);
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::subWithLength`
    pub fn sub_with_length(str: &str, from: i32, length: i32) -> Result<String> {
        let chars: Vec<char> = str.chars().collect();
        let len = chars.len() as i32;
        let start = if from < 0 { (len + from).max(0) } else { from };
        let end = (start + length.max(0)).min(len);
        let s = start.max(0) as usize;
        let e = end.max(0) as usize;
        if s >= e || s >= chars.len() {
            return Ok(String::new());
        }
        Ok(chars[s..e.min(chars.len())].iter().collect())
    }

    /// 对齐 Java: `CharSequenceUtil::subSufByLength`
    pub fn sub_suf_by_length(str: &str, length: i32) -> Result<String> {
        if length <= 0 {
            return Ok(String::new());
        }
        let chars: Vec<char> = str.chars().collect();
        let n = length as usize;
        if chars.len() <= n {
            Ok(str.to_owned())
        } else {
            Ok(chars[chars.len() - n..].iter().collect())
        }
    }

    /// 对齐 Java: `CharSequenceUtil::subPreGbk(CharSequence, int, boolean)`
    pub fn sub_pre_gbk(str: &str, len: i32, half_up: bool) -> Result<String> {
        use encoding_rs::GBK;
        if str.is_empty() || len <= 0 {
            return Ok(if len <= 0 {
                String::new()
            } else {
                str.to_owned()
            });
        }
        let (b, _, _) = GBK.encode(str);
        let mut cut = len as usize;
        if b.len() <= cut {
            return Ok(str.to_owned());
        }
        let mut counter = 0usize;
        for i in 0..cut {
            if (b[i] as i8) < 0 {
                counter += 1;
            }
        }
        if counter % 2 != 0 {
            if half_up {
                cut += 1;
            } else {
                cut = cut.saturating_sub(1);
            }
        }
        cut = cut.min(b.len());
        let (decoded, _, _) = GBK.decode(&b[..cut]);
        Ok(decoded.into_owned())
    }

    /// 对齐 Java: `CharSequenceUtil::subPreGbk(CharSequence, int, CharSequence)`
    pub fn sub_pre_gbk_suffix(str: &str, len: i32, suffix: &str) -> Result<String> {
        Ok(format!("{}{}", Self::sub_pre_gbk(str, len, true)?, suffix))
    }

    /// 对齐 Java: `CharSequenceUtil::subBetweenAll(CharSequence, CharSequence, CharSequence)`
    pub fn sub_between_all(str: &str, prefix: &str, suffix: &str) -> Result<Vec<String>> {
        if str.is_empty() || prefix.is_empty() || suffix.is_empty() || !str.contains(prefix) {
            return Ok(Vec::new());
        }
        let parts: Vec<&str> = str.split(prefix).collect();
        let mut result = Vec::new();
        if prefix == suffix {
            let mut i = 1usize;
            while i + 1 < parts.len() {
                result.push(parts[i].to_owned());
                i += 2;
            }
        } else {
            for fragment in parts.iter().skip(1) {
                if let Some(idx) = fragment.find(suffix) {
                    if idx > 0 {
                        result.push(fragment[..idx].to_owned());
                    }
                }
            }
        }
        Ok(result)
    }

    /// 对齐 Java: `CharSequenceUtil::subBetweenAll(CharSequence, CharSequence)`
    pub fn sub_between_all_pair(str: &str, prefix_and_suffix: &str) -> Result<Vec<String>> {
        Self::sub_between_all(str, prefix_and_suffix, prefix_and_suffix)
    }

    /// 对齐 Java: `CharSequenceUtil::wrapAll`
    pub fn wrap_all(prefix: &str, suffix: &str, strs: &[&str]) -> Result<Vec<String>> {
        Ok(strs
            .iter()
            .map(|s| format!("{prefix}{s}{suffix}"))
            .collect())
    }

    /// 对齐 Java: `CharSequenceUtil::wrapAllIfMissing`
    pub fn wrap_all_if_missing(prefix: &str, suffix: &str, strs: &[&str]) -> Result<Vec<String>> {
        let mut out = Vec::with_capacity(strs.len());
        for s in strs {
            out.push(Self::wrap_if_missing(s, prefix, suffix)?);
        }
        Ok(out)
    }

    /// 对齐 Java: `CharSequenceUtil::wrapAllWithPair`
    pub fn wrap_all_with_pair(prefix_and_suffix: &str, strs: &[&str]) -> Result<Vec<String>> {
        Self::wrap_all(prefix_and_suffix, prefix_and_suffix, strs)
    }

    /// 对齐 Java: `CharSequenceUtil::wrapAllWithPairIfMissing`
    pub fn wrap_all_with_pair_if_missing(
        prefix_and_suffix: &str,
        strs: &[&str],
    ) -> Result<Vec<String>> {
        Self::wrap_all_if_missing(prefix_and_suffix, prefix_and_suffix, strs)
    }
}
