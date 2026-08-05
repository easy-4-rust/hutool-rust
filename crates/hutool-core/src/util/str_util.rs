//! 对齐: `cn.hutool.core.util.StrUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/StrUtil.java
//!
//! Hutool 的 `StrUtil` 继承 `CharSequenceUtil`。本文件提供 **StrUtil 特有**
//! 的高阶便捷方法,内部委托给 `crate::string` 模块中的惯用 Rust 函数。
//!
//! 本模块位于 `util/` 包镜像中，**默认未接入** `lib.rs` 编译树。
//!
//! **请优先使用：**
//! - 惯用 API：`crate::string`（`is_blank` / `trim` / `format_template` 等）
//! - Hutool 命名表面：`crate::text::CharSequenceUtil`（已委托到 `string`）
//! - 迁移门面：`hutool-compat-hutool::StrUtil`
//! - UUID：`crate::IdUtil::uuid`（对齐 `StrUtil.uuid`）
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    non_snake_case
)]

use std::collections::HashMap;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.util.StrUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct StrUtil;

impl StrUtil {
    // ═══════════════════════════════════════════════════════
    //  空 / 空白 判断
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::isBlankIfStr#boolean (Object obj)`
    ///
    /// 若 `obj` 为 `None` 或仅含空白字符则返回 `true`。
    #[must_use]
    pub fn is_blank_if_str(obj: Option<&str>) -> bool {
        obj.map_or(true, crate::string::is_blank)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::isEmptyIfStr#boolean (Object obj)`
    ///
    /// 若 `obj` 为 `None` 或空字符串则返回 `true`。
    #[must_use]
    pub fn is_empty_if_str(obj: Option<&str>) -> bool {
        obj.map_or(true, str::is_empty)
    }

    // ═══════════════════════════════════════════════════════
    //  批量 trim
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::trim#void (String[] strs)`
    ///
    /// 就地去除每个字符串的首尾空白。
    pub fn trim(strs: &mut [String]) {
        for s in strs.iter_mut() {
            let trimmed = s.trim().to_owned();
            *s = trimmed;
        }
    }

    // ═══════════════════════════════════════════════════════
    //  对象 → 字符串转换
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::utf8Str#String (Object obj)`
    ///
    /// 将任意实现了 `Display` 的对象转为 UTF-8 字符串。
    /// `None` 返回空字符串。
    #[must_use]
    pub fn utf8_str(obj: Option<&dyn Display>) -> String {
        match obj {
            Some(v) => format!("{v}"),
            None => String::new(),
        }
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (Object obj, String charsetName)`
    ///
    /// 将对象转为字符串。`charset` 参数在 Rust 中忽略（始终 UTF-8）。
    #[must_use]
    pub fn str(obj: &dyn Display, _charset: &str) -> String {
        format!("{obj}")
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (Object obj, Charset charset)`
    ///
    /// 将对象转为字符串。`charset` 参数在 Rust 中忽略（始终 UTF-8）。
    #[must_use]
    pub fn str_2(obj: &dyn Display, _charset: &str) -> String {
        format!("{obj}")
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (byte[] bytes, String charset)`
    ///
    /// 将字节数组解码为字符串。`charset` 参数在 Rust 中忽略（始终 UTF-8）。
    /// 非 UTF-8 字节使用 U+FFFD 替换字符。
    #[must_use]
    pub fn str_3(bytes: &[u8], _charset: &str) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (byte[] data, Charset charset)`
    ///
    /// 将字节数组解码为字符串。`charset` 参数在 Rust 中忽略（始终 UTF-8）。
    #[must_use]
    pub fn str_4(data: &[u8], _charset: &str) -> String {
        String::from_utf8_lossy(data).into_owned()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (Byte[] bytes, String charset)`
    ///
    /// 将 `Option<u8>` 切片转为字符串。`None` 元素被跳过。
    #[must_use]
    pub fn str_5(bytes: &[Option<u8>], _charset: &str) -> String {
        let buf: Vec<u8> = bytes.iter().copied().flatten().collect();
        String::from_utf8_lossy(&buf).into_owned()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (Byte[] data, Charset charset)`
    ///
    /// 将 `Option<u8>` 切片转为字符串。`None` 元素被跳过。
    #[must_use]
    pub fn str_6(data: &[Option<u8>], _charset: &str) -> String {
        Self::str_5(data, _charset)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (ByteBuffer data, String charset)`
    ///
    /// 将字节切片解码为字符串。
    #[must_use]
    pub fn str_7(data: &[u8], _charset: &str) -> String {
        String::from_utf8_lossy(data).into_owned()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::str#String (ByteBuffer data, Charset charset)`
    ///
    /// 将字节切片解码为字符串。
    #[must_use]
    pub fn str_8(data: &[u8], _charset: &str) -> String {
        String::from_utf8_lossy(data).into_owned()
    }

    // ═══════════════════════════════════════════════════════
    //  toString 系列
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::toString#String (Object obj)`
    ///
    /// 调用 `Display` 将对象转为字符串；`None` 返回 `"null"`。
    #[must_use]
    pub fn to_string(obj: Option<&dyn Display>) -> String {
        match obj {
            Some(v) => format!("{v}"),
            None => "null".to_owned(),
        }
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::toStringOrNull#String (Object obj)`
    ///
    /// 调用 `Display` 将对象转为字符串；`None` 返回 `None`。
    #[must_use]
    pub fn to_string_or_null(obj: Option<&dyn Display>) -> Option<String> {
        obj.map(|v| format!("{v}"))
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::toStringOrEmpty#String (Object obj)`
    ///
    /// 调用 `Display` 将对象转为字符串；`None` 返回空字符串。
    #[must_use]
    pub fn to_string_or_empty(obj: Option<&dyn Display>) -> String {
        match obj {
            Some(v) => format!("{v}"),
            None => String::new(),
        }
    }

    // ═══════════════════════════════════════════════════════
    //  StringBuilder / StrBuilder（在 Rust 中等价于 String）
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::builder#StringBuilder ()`
    ///
    /// 返回一个新的空 `String`（Rust 中无独立 StringBuilder 类型）。
    #[must_use]
    pub fn builder() -> String {
        String::new()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::strBuilder#StrBuilder ()`
    ///
    /// 返回一个新的空 `String`。
    #[must_use]
    pub fn str_builder() -> String {
        String::new()
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::builder#StringBuilder (int capacity)`
    ///
    /// 返回一个具有指定初始容量的空 `String`。
    #[must_use]
    pub fn builder_with_capacity(capacity: usize) -> String {
        String::with_capacity(capacity)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::strBuilder#StrBuilder (int capacity)`
    ///
    /// 返回一个具有指定初始容量的空 `String`。
    #[must_use]
    pub fn str_builder_with_capacity(capacity: usize) -> String {
        String::with_capacity(capacity)
    }

    // ═══════════════════════════════════════════════════════
    //  Reader / Writer
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::getReader#StringReader (CharSequence str)`
    ///
    /// 在 Rust 中无需包装 `StringReader`；直接使用 `&str` 读取即可。
    /// 本方法原样返回输入引用。
    #[must_use]
    pub fn get_reader<'a>(s: &'a str) -> &'a str {
        s
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::getWriter#StringWriter ()`
    ///
    /// 在 Rust 中 `String` 即可充当 `StringWriter`。返回空 `String`。
    #[must_use]
    pub fn get_writer() -> String {
        String::new()
    }

    // ═══════════════════════════════════════════════════════
    //  反转
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::reverse#String (final String str)`
    ///
    /// 反转字符串（委托给 `crate::string::reverse`）。
    #[must_use]
    pub fn reverse(s: &str) -> String {
        crate::string::reverse(s)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::reverseByCodePoint#String (String str)`
    ///
    /// 按 Unicode 码点反转字符串。在 Rust 中等价于 `reverse`，
    /// 因为 `str::chars()` 已经按码点迭代。
    #[must_use]
    pub fn reverse_by_code_point(s: &str) -> String {
        crate::string::reverse_by_code_point(s)
    }

    // ═══════════════════════════════════════════════════════
    //  填充
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::fillBefore#String (String str, char filledChar, int len)`
    ///
    /// 在字符串左侧填充 `filled_char`，使总字符数达到 `len`。
    #[must_use]
    pub fn fill_before(s: &str, filled_char: char, len: usize) -> String {
        crate::string::fill_before(s, filled_char, len)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::fillAfter#String (String str, char filledChar, int len)`
    ///
    /// 在字符串右侧填充 `filled_char`，使总字符数达到 `len`。
    #[must_use]
    pub fn fill_after(s: &str, filled_char: char, len: usize) -> String {
        crate::string::fill_after(s, filled_char, len)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::fill#String (String str, char filledChar, int len, boolean isPre)`
    ///
    /// 根据 `is_pre` 分派到 `fill_before` 或 `fill_after`。
    #[must_use]
    pub fn fill(s: &str, filled_char: char, len: usize, is_pre: bool) -> String {
        crate::string::fill(s, filled_char, len, is_pre)
    }

    // ═══════════════════════════════════════════════════════
    //  相似度
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::similar#double (String str1, String str2)`
    ///
    /// 计算两个字符串的相似度比率 (0.0 ~ 1.0)，
    /// 基于 Levenshtein 编辑距离。
    #[must_use]
    pub fn similar(s1: &str, s2: &str) -> f64 {
        crate::string::similarity(s1, s2)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::similar#String (String str1, String str2, int scale)`
    ///
    /// 返回相似度字符串，保留 `scale` 位小数。
    #[must_use]
    pub fn similar_str(s1: &str, s2: &str, scale: usize) -> String {
        crate::string::similarity_str(s1, s2, scale)
    }

    // ═══════════════════════════════════════════════════════
    //  UUID
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::uuid#String ()`
    ///
    /// 生成一个随机 UUID v4（带连字符）。委托给 `crate::IdUtil::uuid`。
    #[must_use]
    pub fn uuid() -> String {
        crate::IdUtil::uuid()
    }

    // ═══════════════════════════════════════════════════════
    //  模板格式化（Map 替换）
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::format#String (CharSequence template, Map<?, ?> map)`
    ///
    /// 使用 `map` 中的键值替换模板中的 `{key}` 占位符。
    /// `{{` 产生字面量 `{`，`}}` 产生字面量 `}`。
    /// 未匹配的占位符保持原样。
    #[must_use]
    pub fn format(template: &str, map: &HashMap<&str, &str>) -> String {
        crate::string::format_map(template, map)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::format#String (CharSequence template, Map<?, ?> map, boolean ignoreNull)`
    ///
    /// 同 `format`，但当 `ignore_null = true` 且 map 中无对应键时，
    /// 占位符会被移除而非保留。
    #[must_use]
    pub fn format_ignore_null(
        template: &str,
        map: &HashMap<&str, Option<&str>>,
        ignore_null: bool,
    ) -> String {
        crate::string::format_map_optional(template, map, ignore_null)
    }

    // ═══════════════════════════════════════════════════════
    //  截断
    // ═══════════════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::truncateUtf8#String (String str, int maxBytes)`
    ///
    /// 截断字符串使其 UTF-8 编码不超过 `max_bytes` 字节。
    /// 截断点保证在 UTF-8 字符边界。
    #[must_use]
    pub fn truncate_utf8(s: &str, max_bytes: usize) -> &str {
        crate::string::truncate_utf8(s, max_bytes)
    }

    /// 对齐 Java: `cn.hutool.core.util::StrUtil::truncateByByteLength#String (String str, Charset charset, int maxBytesLength, int factor, boolean appendDots)`
    ///
    /// 按字节长度截断字符串。`factor` 控制截断粒度(字节步长)；
    /// `append_dots` 为 `true` 时在末尾附加 `"..."`。
    ///
    /// charset 参数在 Rust 中忽略(始终 UTF-8)。
    #[must_use]
    pub fn truncate_by_byte_length(
        s: &str,
        _charset: &str,
        max_bytes: usize,
        factor: usize,
        append_dots: bool,
    ) -> String {
        crate::string::truncate_by_byte_length(s, max_bytes, factor, append_dots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── isBlankIfStr / isEmptyIfStr ──

    #[test]
    fn is_blank_if_str_none_is_blank() {
        assert!(StrUtil::is_blank_if_str(None));
    }

    #[test]
    fn is_blank_if_str_whitespace_is_blank() {
        assert!(StrUtil::is_blank_if_str(Some("  \t\n  ")));
    }

    #[test]
    fn is_blank_if_str_non_blank() {
        assert!(!StrUtil::is_blank_if_str(Some("hello")));
    }

    #[test]
    fn is_empty_if_str_none_is_empty() {
        assert!(StrUtil::is_empty_if_str(None));
    }

    #[test]
    fn is_empty_if_str_empty_is_empty() {
        assert!(StrUtil::is_empty_if_str(Some("")));
    }

    #[test]
    fn is_empty_if_str_whitespace_is_not_empty() {
        assert!(!StrUtil::is_empty_if_str(Some("  ")));
    }

    // ── trim ──

    #[test]
    fn trim_mutates_in_place() {
        let mut v = vec!["  hello  ".to_owned(), " world\t".to_owned()];
        StrUtil::trim(&mut v);
        assert_eq!(v, vec!["hello", "world"]);
    }

    // ── utf8Str ──

    #[test]
    fn utf8_str_some() {
        assert_eq!(StrUtil::utf8_str(Some(&42)), "42");
    }

    #[test]
    fn utf8_str_none() {
        assert_eq!(StrUtil::utf8_str(None), "");
    }

    // ── str (byte conversions) ──

    #[test]
    fn str_from_bytes() {
        assert_eq!(StrUtil::str_3(b"hello", "UTF-8"), "hello");
    }

    #[test]
    fn str_from_bytes_lossy() {
        let bad: &[u8] = &[0xFF, 0xFE];
        let result = StrUtil::str_3(bad, "UTF-8");
        assert!(result.contains('\u{FFFD}'));
    }

    #[test]
    fn str_from_option_bytes() {
        let data = vec![Some(b'a'), None, Some(b'b')];
        assert_eq!(StrUtil::str_5(&data, "UTF-8"), "ab");
    }

    // ── toString 系列 ──

    #[test]
    fn to_string_none_returns_null() {
        assert_eq!(StrUtil::to_string(None), "null");
    }

    #[test]
    fn to_string_some() {
        assert_eq!(StrUtil::to_string(Some(&42i32)), "42");
    }

    #[test]
    fn to_string_or_null_none() {
        assert_eq!(StrUtil::to_string_or_null(None), None);
    }

    #[test]
    fn to_string_or_empty_none() {
        assert_eq!(StrUtil::to_string_or_empty(None), "");
    }

    // ── builder ──

    #[test]
    fn builder_returns_empty_string() {
        assert_eq!(StrUtil::builder(), "");
        assert_eq!(StrUtil::str_builder(), "");
    }

    #[test]
    fn builder_with_capacity() {
        let s = StrUtil::builder_with_capacity(128);
        assert!(s.capacity() >= 128);
        assert!(s.is_empty());
    }

    // ── reverse ──

    #[test]
    fn reverse_delegates() {
        assert_eq!(StrUtil::reverse("abc"), "cba");
        assert_eq!(StrUtil::reverse_by_code_point("abc"), "cba");
    }

    // ── fill ──

    #[test]
    fn fill_before_works() {
        assert_eq!(StrUtil::fill_before("123", '0', 6), "000123");
    }

    #[test]
    fn fill_after_works() {
        assert_eq!(StrUtil::fill_after("123", '0', 6), "123000");
    }

    #[test]
    fn fill_dispatches() {
        assert_eq!(StrUtil::fill("ab", '*', 5, true), "***ab");
        assert_eq!(StrUtil::fill("ab", '*', 5, false), "ab***");
    }

    // ── similar ──

    #[test]
    fn similar_identical() {
        assert!((StrUtil::similar("abc", "abc") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn similar_str_format() {
        assert_eq!(StrUtil::similar_str("abc", "abc", 2), "1.00");
    }

    // ── uuid ──

    #[test]
    fn uuid_is_nonempty_and_unique() {
        let u1 = StrUtil::uuid();
        let u2 = StrUtil::uuid();
        assert!(!u1.is_empty());
        assert_ne!(u1, u2);
        assert_eq!(u1.len(), 36); // UUID with hyphens
    }

    // ── format (map) ──

    #[test]
    fn format_map_replaces_keys() {
        let mut map = HashMap::new();
        map.insert("name", "Alice");
        map.insert("age", "30");
        assert_eq!(
            StrUtil::format("Hello {name}, you are {age}.", &map),
            "Hello Alice, you are 30."
        );
    }

    #[test]
    fn format_ignore_null_removes_missing_keys() {
        let mut map = HashMap::new();
        map.insert("name", Some("Alice"));
        let result = StrUtil::format_ignore_null("Hello {name}, age: {age}", &map, true);
        assert_eq!(result, "Hello Alice, age: ");
    }

    // ── truncate ──

    #[test]
    fn truncate_utf8_respects_boundary() {
        assert_eq!(StrUtil::truncate_utf8("hello", 3), "hel");
        assert_eq!(StrUtil::truncate_utf8("hello", 10), "hello");
    }

    #[test]
    fn truncate_by_byte_length_with_dots() {
        let result = StrUtil::truncate_by_byte_length("hello world", "UTF-8", 8, 1, true);
        assert!(result.len() <= 8);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn truncate_by_byte_length_no_dots() {
        let result = StrUtil::truncate_by_byte_length("hello world", "UTF-8", 5, 1, false);
        assert!(result.len() <= 5);
    }
}
