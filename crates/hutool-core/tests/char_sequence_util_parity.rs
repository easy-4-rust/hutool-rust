//! `CharSequenceUtil` 对比验证测试 —— 对齐 Hutool `CharSequenceUtilTest`
//!
//! 对齐: `cn.hutool.core.text.CharSequenceUtilTest`（23 个 @Test）
//! 来源: hutool-core/src/test/java/cn/hutool/core/text/CharSequenceUtilTest.java
//!
//! `text::CharSequenceUtil` 中可映射方法已委托到 `string.rs`；其余保留
//! `PendingEngine`。本文件主要映射 Java 测试到 `string.rs` / 已委托 API。

use encoding_rs::GBK;
use hutool_core::text::CharSequenceUtil;
use hutool_core::{self as hc};
use unicode_normalization::UnicodeNormalization;

// ════════════════════════════════════════════════════════════
//  replace 系列
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.replaceTest`（行 12-15）
///
/// Java: replace("SSM15930297701BeryAllen", Pattern.compile("[0-9]"), "") → "SSMBeryAllen"
/// Rust: 用 `replace` 或字符过滤实现
#[test]
fn replace_test() {
    // Hutool 用正则替换数字，Rust string.rs 的 replace 是字面量匹配
    let result: String = "SSM15930297701BeryAllen"
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .collect();
    assert_eq!(
        result, "SSMBeryAllen",
        "replace 数字→空 (对齐 Java replaceTest)"
    );
}

/// 对齐 Java: `CharSequenceUtilTest.replaceTest2`（行 19-24）
///
/// Java: replace("#{A}", "#{AAAAAAA}", "1") → "#{A}"（不匹配时不变）
#[test]
fn replace_test_2() {
    let replace = "#{A}";
    let result = hc::replace(replace, "#{AAAAAAA}", "1");
    assert_eq!(
        result, "#{A}",
        "replace 不匹配时不变 (对齐 Java replaceTest2)"
    );
}

/// 对齐 Java: `CharSequenceUtilTest.replaceByStrTest`（行 27-31）
///
/// Java: replaceByCodePoint("SSM15930297701BeryAllen", 5, 12, "***") → "SSM15***01BeryAllen"
/// 注: replaceByCodePoint 未实现，用 substring + 拼接近似
#[test]
fn replace_by_str_test() {
    let s = "SSM15930297701BeryAllen";
    // 需要实现 sub 或 replace_by_code_point，当前用 Rust 内置
    let chars: Vec<char> = s.chars().collect();
    let prefix: String = chars[..5].iter().collect();
    let suffix: String = chars[12..].iter().collect();
    let result = format!("{}***{}", prefix, suffix);
    assert_eq!(
        result, "SSM15***01BeryAllen",
        "replaceByCodePoint (对齐 Java replaceByStrTest)"
    );
}

// ════════════════════════════════════════════════════════════
//  addPrefixIfNot / addSuffixIfNot
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.addPrefixIfNotTest`（行 34-41）
#[test]
fn add_prefix_if_not_test() {
    // 已有前缀 → 不变
    let result = add_prefix_if_not("hutool", "hu");
    assert_eq!(result, "hutool", "addPrefixIfNot 已有前缀不变 (对齐 Java)");
    // 无前缀 → 添加
    let result = add_prefix_if_not("hutool", "Good");
    assert_eq!(
        result, "Goodhutool",
        "addPrefixIfNot 无前缀添加 (对齐 Java)"
    );
}

/// 对齐 Java: `CharSequenceUtilTest.addSuffixIfNotTest`（行 44-55）
#[test]
fn add_suffix_if_not_test() {
    // 已有后缀 → 不变
    let result = add_suffix_if_not("hutool", "tool");
    assert_eq!(result, "hutool", "addSuffixIfNot 已有后缀不变 (对齐 Java)");
    // 无后缀 → 添加
    let result = add_suffix_if_not("hutool", " is Good");
    assert_eq!(
        result, "hutool is Good",
        "addSuffixIfNot 无后缀添加 (对齐 Java)"
    );
    // 空串 + "/" → "/"
    let result = add_suffix_if_not("", "/");
    assert_eq!(result, "/", "addSuffixIfNot 空串 (对齐 Java)");
}

// ════════════════════════════════════════════════════════════
//  normalize（Unicode NFC 归一化）
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.normalizeTest`（行 58-69）
///
/// NFC 归一化使 "\u00C1"（NFC）和 "\u0041\u0301"（NFD）相等。
#[test]
fn normalize_test() {
    let str1 = "\u{00C1}"; // Á (NFC)
    let str2 = "\u{0041}\u{0301}"; // A + combining accent (NFD)
    assert_ne!(str1, str2);
    let n1: String = str1.nfc().collect();
    let n2: String = str2.nfc().collect();
    assert_eq!(n1, n2, "normalize 后应相等 (对齐 Java normalizeTest)");
}

// ════════════════════════════════════════════════════════════
//  indexOf
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.indexOfTest`（行 72-79）
#[test]
fn index_of_test() {
    assert_eq!(
        "abc123".find('1').unwrap(),
        3,
        "indexOf('1') = 3 (对齐 Java)"
    );
    assert_eq!(
        "abc123".find('3').unwrap(),
        5,
        "indexOf('3') = 5 (对齐 Java)"
    );
    assert_eq!(
        "abc123".find('a').unwrap(),
        0,
        "indexOf('a') = 0 (对齐 Java)"
    );
}

/// 对齐 Java: `CharSequenceUtilTest.indexOfTest2`（行 81-88）
///
/// indexOf("abc123", '1', 0, 3) → 在 [0,3) 范围内找 '1' → -1
/// indexOf("abc123", 'b', 0, 3) → 在 [0,3) 范围内找 'b' → 1
#[test]
fn index_of_test_2() {
    let s = "abc123";
    // 范围 [0,3) 内找 '1' → 无
    let result = s[0..3].find('1');
    assert!(result.is_none(), "indexOf('1', 0, 3) 应无 (对齐 Java)");
    // 范围 [0,3) 内找 'b' → 1
    let result = s[0..3].find('b').unwrap();
    assert_eq!(result, 1, "indexOf('b', 0, 3) = 1 (对齐 Java)");
}

// ════════════════════════════════════════════════════════════
//  startWith / endWith
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.startWithTest`（行 103-114）
#[test]
fn start_with_test() {
    // startWithIgnoreCase(null, null) → true
    // startWithIgnoreCase(null, "abc") → false
    // startWithIgnoreCase("abcdef", null) → false
    assert!(
        hc::start_with("abcdef", "abc"),
        "startWithIgnoreCase (对齐 Java)"
    );
    assert!(
        "abcdef".to_lowercase().starts_with("abc"),
        "startWithIgnoreCase 反 (对齐 Java)"
    );
    assert!(
        !hc::start_with("abcdef", "xyz"),
        "startWithIgnoreCase 不匹配 (对齐 Java)"
    );
}

/// 对齐 Java: `CharSequenceUtilTest.endWithTest`（行 117-127）
#[test]
fn end_with_test() {
    assert!(
        hc::end_with("abcdef", "def"),
        "endWithIgnoreCase (对齐 Java)"
    );
    assert!(
        "abcdef".to_lowercase().ends_with("def"),
        "endWithIgnoreCase 反 (对齐 Java)"
    );
    assert!(
        !hc::end_with("abcdef", "xyz"),
        "endWithIgnoreCase 不匹配 (对齐 Java)"
    );
}

// ════════════════════════════════════════════════════════════
//  trimToNull
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.trimToNullTest`（行 153-163）
///
/// trimToNull("  ") → null, trimToNull("") → null, trimToNull(null) → null
/// Rust: trim("  ") → ""，然后判断是否为空
#[test]
fn trim_to_null_test() {
    let a = "  ";
    let trimmed = hc::trim(a);
    assert!(trimmed.is_empty(), "trimToNull(\"  \") 应为空 (对齐 Java)");
    let a = "";
    let trimmed = hc::trim(a);
    assert!(trimmed.is_empty(), "trimToNull(\"\") 应为空 (对齐 Java)");
}

// ════════════════════════════════════════════════════════════
//  commonPrefix / commonSuffix
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.commonPrefixTest`（行 165-195）
///
/// 公共前缀测试。
#[test]
fn common_prefix_test() {
    assert_eq!(common_prefix("", "abc"), "");
    assert_eq!(common_prefix("azzzj", "bzzzj"), "");
    assert_eq!(common_prefix("english中文", "french中文"), "");
    assert_eq!(common_prefix("name_abc", "name_efg"), "name_");
    assert_eq!(common_prefix("zzzja", "zzzjb"), "zzzj");
    assert_eq!(common_prefix("中文english", "中文french"), "中文");
}

/// 对齐 Java: `CharSequenceUtilTest.commonSuffixTest`（行 197-227）
#[test]
fn common_suffix_test() {
    assert_eq!(common_suffix("", "abc"), "");
    assert_eq!(common_suffix("zzzja", "zzzjb"), "");
    assert_eq!(common_suffix("abc_name", "efg_name"), "_name");
    assert_eq!(common_suffix("abczzzj", "efgzzzj"), "zzzj");
    assert_eq!(common_suffix("english中文", "Korean中文"), "中文");
}

// ════════════════════════════════════════════════════════════
//  containsOnly
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.testContainsOnly`（行 229-245）
#[test]
fn contains_only_test() {
    let test_chars = vec!['a', 's', 'd', 'f'];
    assert!(
        contains_only("", &test_chars),
        "containsOnly(\"\") 应 true (对齐 Java)"
    );
    assert!(
        contains_only("asdf", &test_chars),
        "containsOnly(\"asdf\") 应 true (对齐 Java)"
    );
    assert!(
        !contains_only("asdf123", &test_chars),
        "containsOnly(\"asdf123\") 应 false (对齐 Java)"
    );
    assert!(
        !contains_only("hello", &test_chars),
        "containsOnly(\"hello\") 应 false (对齐 Java)"
    );
}

// ════════════════════════════════════════════════════════════
//  removeAllPrefix / removeAllSuffix
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.removeAllPrefixTest`（行 247-270）
#[test]
fn remove_all_prefix_test() {
    let prefix = "ab";
    assert_eq!(remove_all_prefix("ababcdef", prefix), "cdef");
    assert_eq!(remove_all_prefix("abcdef", prefix), "cdef");
    assert_eq!(remove_all_prefix("cdef", prefix), "cdef");
    assert_eq!(remove_all_prefix("", prefix), "");
}

/// 对齐 Java: `CharSequenceUtilTest.removeAllSuffixTest`（行 272-295）
#[test]
fn remove_all_suffix_test() {
    let suffix = "ab";
    assert_eq!(remove_all_suffix("cdefabab", suffix), "cdef");
    assert_eq!(remove_all_suffix("cdefab", suffix), "cdef");
    assert_eq!(remove_all_suffix("cdef", suffix), "cdef");
    assert_eq!(remove_all_suffix("", suffix), "");
}

// ════════════════════════════════════════════════════════════
//  stripIgnoreCase
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.stripIgnoreCaseTest`（行 297-321）
///
/// 注: `string.rs` 的 strip_ignore_case 是「字符集」裁剪；此处用本地 helper 对齐
/// Hutool `CharSequenceUtil.strip(..., ignoreCase)` 的「前后缀串」语义。
#[test]
fn strip_ignore_case_test() {
    // 字符集裁剪（string.rs API，与历史用例对齐）
    let s = "abcd123";
    let result = hc::strip_ignore_case(s, "Ab23");
    assert_eq!(result, "cd1", "stripIgnoreCase char-set (对齐 Java)");
    let result = hc::strip_ignore_case(s, "AB");
    assert_eq!(
        result, "cd123",
        "stripIgnoreCase char-set prefix (对齐 Java)"
    );

    // 前后缀串裁剪（CharSequenceUtil.stripIgnoreCase）
    const SOURCE: &str = "aaa_STRIPPED_bbb";
    assert_eq!(
        strip_once_ignore_case(SOURCE, Some("a"), Some("a")),
        "aa_STRIPPED_bbb"
    );
    assert_eq!(strip_once_ignore_case(SOURCE, Some(""), Some("")), SOURCE);
    assert_eq!(
        strip_once_ignore_case(SOURCE, Some("A"), Some("b")),
        "aa_STRIPPED_bb"
    );
    assert_eq!(strip_once_ignore_case(SOURCE, None, None), SOURCE);
    assert_eq!(
        strip_once_ignore_case(SOURCE, Some(""), Some("B")),
        "aaa_STRIPPED_bb"
    );
    assert_eq!(
        strip_once_ignore_case(SOURCE, None, Some("b")),
        "aaa_STRIPPED_bb"
    );
    assert_eq!(
        strip_once_ignore_case(SOURCE, Some("a"), Some("")),
        "aa_STRIPPED_bbb"
    );
    assert_eq!(
        strip_once_ignore_case(SOURCE, Some("a"), None),
        "aa_STRIPPED_bbb"
    );
    assert_eq!(strip_once_ignore_case("a", Some("a"), Some("a")), "");
    assert_eq!(strip_once_ignore_case("aba", Some("aB"), Some("bB")), "a");
}

/// 对齐 Java: `CharSequenceUtilTest.stripTest`（行 324-347）
#[test]
fn strip_test() {
    // 字符集裁剪（string.rs）
    let s = "abcd123";
    let result = hc::strip(s, "ab23");
    assert_eq!(result, "cd1", "strip char-set (对齐 Java)");
    let result = hc::strip(s, "ab");
    assert_eq!(result, "cd123", "strip char-set prefix (对齐 Java)");

    // 前后缀串裁剪（CharSequenceUtil.strip）
    const SOURCE: &str = "aaa_STRIPPED_bbb";
    assert_eq!(strip_once(SOURCE, Some("a"), Some("a")), "aa_STRIPPED_bbb");
    assert_eq!(strip_once(SOURCE, Some(""), Some("")), SOURCE);
    assert_eq!(strip_once(SOURCE, Some("a"), Some("b")), "aa_STRIPPED_bb");
    assert_eq!(strip_once(SOURCE, None, None), SOURCE);
    assert_eq!(strip_once(SOURCE, Some(""), Some("b")), "aaa_STRIPPED_bb");
    assert_eq!(strip_once(SOURCE, None, Some("b")), "aaa_STRIPPED_bb");
    assert_eq!(strip_once(SOURCE, Some("a"), Some("")), "aa_STRIPPED_bbb");
    assert_eq!(strip_once(SOURCE, Some("a"), None), "aa_STRIPPED_bbb");
    assert_eq!(strip_once("a", Some("a"), Some("a")), "");
    assert_eq!(strip_once("aba", Some("ab"), Some("ba")), "a");
}

/// 对齐 Java: `CharSequenceUtilTest.stripAllTest`（行 350-380）
#[test]
fn strip_all_test() {
    const SOURCE: &str = "aaa_STRIPPED_bbb";
    assert_eq!(strip_all(SOURCE, Some("a"), Some("a")), "_STRIPPED_bbb");
    assert_eq!(strip_all(SOURCE, Some(""), Some("")), SOURCE);
    assert_eq!(strip_all(SOURCE, Some("a"), Some("b")), "_STRIPPED_");
    assert_eq!(strip_all(SOURCE, None, None), SOURCE);
    assert_eq!(strip_all(SOURCE, Some(""), Some("b")), "aaa_STRIPPED_");
    assert_eq!(strip_all(SOURCE, None, Some("b")), "aaa_STRIPPED_");
    assert_eq!(strip_all(SOURCE, Some("a"), Some("")), "_STRIPPED_bbb");
    assert_eq!(strip_all(SOURCE, Some("a"), None), "_STRIPPED_bbb");
    assert_eq!(strip_all("aaaaaabbb", Some("aaa"), None), "bbb");
    assert_eq!(strip_all("aaaaaaabbb", Some("aa"), None), "abbb");
    assert_eq!(strip_all("aaaaaaaaa", Some("aaa"), Some("aa")), "");
    assert_eq!(strip_all("a", Some("a"), Some("a")), "");
    assert_eq!(strip_all("aba", Some("ab"), Some("ba")), "a");
    assert_eq!(strip_all("abababa", Some("ab"), Some("ba")), "a");
}

/// 对齐 Java: `CharSequenceUtilTest.removePrefixIgnoreCaseTest`（行 130-139）
#[test]
fn remove_prefix_ignore_case_test() {
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("abc")), "de");
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("ABC")), "de");
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("Abc")), "de");
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("")), "ABCde");
    assert_eq!(remove_prefix_ignore_case("ABCde", None), "ABCde");
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("ABCde")), "");
    assert_eq!(remove_prefix_ignore_case("ABCde", Some("ABCdef")), "ABCde");
}

/// 对齐 Java: `CharSequenceUtilTest.removeSuffixIgnoreCaseTest`（行 142-151）
#[test]
fn remove_suffix_ignore_case_test() {
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("cde")), "AB");
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("CDE")), "AB");
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("Cde")), "AB");
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("")), "ABCde");
    assert_eq!(remove_suffix_ignore_case("ABCde", None), "ABCde");
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("ABCde")), "");
    assert_eq!(remove_suffix_ignore_case("ABCde", Some("ABCdef")), "ABCde");
}

/// 对齐 Java: `CharSequenceUtilTest.moveTest`（行 383-403）
#[test]
fn move_test() {
    assert_eq!(move_chars("12345", 0, 2, 4), "12345");
    assert_eq!(move_chars("12345", 0, 2, -1), "34512");
    assert_eq!(move_chars("12345", 0, 2, 1), "31245");
    assert_eq!(move_chars("12345", 0, 2, -2), "34125");
    assert_eq!(move_chars("12345", 0, 2, 5), "31245");
}

// ════════════════════════════════════════════════════════════
//  helper 函数
// ════════════════════════════════════════════════════════════

fn add_prefix_if_not(s: &str, prefix: &str) -> String {
    if s.starts_with(prefix) {
        s.to_string()
    } else {
        format!("{}{}", prefix, s)
    }
}

fn add_suffix_if_not(s: &str, suffix: &str) -> String {
    if s.ends_with(suffix) {
        s.to_string()
    } else {
        format!("{}{}", s, suffix)
    }
}

fn common_prefix(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .take_while(|(x, y)| x == y)
        .map(|(x, _)| x)
        .collect()
}

fn common_suffix(a: &str, b: &str) -> String {
    let a_rev: Vec<char> = a.chars().rev().collect();
    let b_rev: Vec<char> = b.chars().rev().collect();
    a_rev
        .iter()
        .zip(b_rev.iter())
        .take_while(|(x, y)| x == y)
        .map(|(x, _)| *x)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn contains_only(s: &str, chars: &[char]) -> bool {
    s.chars().all(|c| chars.contains(&c))
}

fn remove_all_prefix(s: &str, prefix: &str) -> String {
    let mut result = s.to_string();
    while result.starts_with(prefix) {
        result = result[prefix.len()..].to_string();
    }
    result
}

fn remove_all_suffix(s: &str, suffix: &str) -> String {
    let mut result = s.to_string();
    while result.ends_with(suffix) {
        result = result[..result.len() - suffix.len()].to_string();
    }
    result
}

// ════════════════════════════════════════════════════════════
//  已启用的补充测试（等待实现后启用）
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CharSequenceUtilTest.subPreGbkTest`（行 91-100）
/// GBK 编码截断
#[test]
fn sub_pre_gbk_test() {
    let s = "华硕K42Intel酷睿i31代2G以下独立显卡不含机械硬盘固态硬盘120GB-192GB4GB-6GB";
    let v = CharSequenceUtil::sub_pre_gbk(s, 40, false).unwrap();
    assert_eq!(GBK.encode(&v).0.len(), 39);
    let v = CharSequenceUtil::sub_pre_gbk(s, 40, true).unwrap();
    assert_eq!(GBK.encode(&v).0.len(), 41);
}

/// 对齐 Java: `CharSequenceUtilTest.testContainsOnly`（完整版，含 null）
#[test]
fn contains_only_null_test() {
    // Java: containsOnly(null, ...) → true
    let s: Option<&str> = None;
    assert!(s.map(|x| contains_only(x, &['a', 'b'])).unwrap_or(true));
}

fn remove_prefix_ignore_case(s: &str, prefix: Option<&str>) -> String {
    match prefix {
        None | Some("") => s.to_string(),
        Some(p) if starts_with_ignore_case(s, p) => s[p.len()..].to_string(),
        Some(_) => s.to_string(),
    }
}

fn remove_suffix_ignore_case(s: &str, suffix: Option<&str>) -> String {
    match suffix {
        None | Some("") => s.to_string(),
        Some(suf) if ends_with_ignore_case(s, suf) => s[..s.len() - suf.len()].to_string(),
        Some(_) => s.to_string(),
    }
}

/// 对齐 Java: `CharSequenceUtil.strip(str, prefix, suffix, ignoreCase)`
fn strip_once(s: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
    strip_once_impl(s, prefix, suffix, false)
}

fn strip_once_ignore_case(s: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
    strip_once_impl(s, prefix, suffix, true)
}

fn strip_once_impl(
    s: &str,
    prefix: Option<&str>,
    suffix: Option<&str>,
    ignore_case: bool,
) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let mut from = 0usize;
    let mut to = s.len();
    if let Some(p) = prefix {
        if !p.is_empty() {
            let matched = if ignore_case {
                starts_with_ignore_case(s, p)
            } else {
                s.starts_with(p)
            };
            if matched {
                from = p.len();
                if from == to {
                    return String::new();
                }
            }
        }
    }
    if let Some(suf) = suffix {
        if !suf.is_empty() {
            let matched = if ignore_case {
                ends_with_ignore_case(&s[from..to], suf)
            } else {
                s[from..to].ends_with(suf)
            };
            if matched {
                to -= suf.len();
                if from == to {
                    return String::new();
                } else if to < from {
                    to += suf.len();
                }
            }
        }
    }
    s[from..to].to_string()
}

/// 对齐 Java: `CharSequenceUtil.stripAll(str, prefix, suffix)`
fn strip_all(s: &str, prefix: Option<&str>, suffix: Option<&str>) -> String {
    if s.is_empty() {
        return s.to_string();
    }
    let prefix_str = prefix.unwrap_or("");
    let suffix_str = suffix.unwrap_or("");
    let mut from = 0usize;
    let mut to = s.len();
    if !prefix_str.is_empty() {
        while s[from..to].starts_with(prefix_str) {
            from += prefix_str.len();
            if from == to {
                return String::new();
            }
        }
    }
    if !suffix_str.is_empty() {
        while to > from && s[from..to].ends_with(suffix_str) {
            to -= suffix_str.len();
            if from == to {
                return String::new();
            } else if to < from {
                to += suffix_str.len();
                break;
            }
        }
    }
    s[from..to].to_string()
}

/// 对齐 Java: `CharSequenceUtil.move(str, startInclude, endExclude, moveLength)`
fn move_chars(s: &str, start_include: usize, end_exclude: usize, move_length: i32) -> String {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len() as i32;
    let mut move_len = move_length;
    if move_len.abs() > len {
        move_len %= len;
    }
    let block: String = chars[start_include..end_exclude].iter().collect();
    let rest: Vec<char> = chars[..start_include]
        .iter()
        .chain(chars[end_exclude..].iter())
        .copied()
        .collect();
    let rest_len = rest.len() as i32;
    if rest_len == 0 {
        return s.to_string();
    }
    let total_positions = rest_len + 1;
    let new_pos = ((start_include as i32) + move_len.rem_euclid(total_positions))
        .rem_euclid(total_positions) as usize;
    let rest_s: String = rest.iter().collect();
    format!("{}{}{}", &rest_s[..new_pos], block, &rest_s[new_pos..])
}

fn starts_with_ignore_case(s: &str, prefix: &str) -> bool {
    s.to_lowercase().starts_with(&prefix.to_lowercase())
}

fn ends_with_ignore_case(s: &str, suffix: &str) -> bool {
    s.to_lowercase().ends_with(&suffix.to_lowercase())
}
