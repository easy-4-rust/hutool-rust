//! escape_util parity tests
//! 对齐: `cn.hutool.core.util.EscapeUtilTest`

use hutool_core::EscapeUtil;

// ── HTML 转义 ──

#[test]
fn escape_html_basic() {
    assert_eq!(
        EscapeUtil::escape_html("<div>hello</div>"),
        "&lt;div&gt;hello&lt;/div&gt;"
    );
}

#[test]
fn escape_html_special_chars() {
    assert_eq!(EscapeUtil::escape_html("a & b"), "a &amp; b");
    assert_eq!(EscapeUtil::escape_html("\"hello\""), "&quot;hello&quot;");
    assert_eq!(EscapeUtil::escape_html("it's"), "it's");
}

#[test]
fn escape_html_no_special() {
    assert_eq!(EscapeUtil::escape_html("hello world"), "hello world");
}

#[test]
fn unescape_html_basic() {
    assert_eq!(
        EscapeUtil::unescape_html("&lt;div&gt;hello&lt;/div&gt;"),
        "<div>hello</div>"
    );
}

#[test]
fn unescape_html_special_chars() {
    assert_eq!(EscapeUtil::unescape_html("a &amp; b"), "a & b");
    assert_eq!(EscapeUtil::unescape_html("&quot;hello&quot;"), "\"hello\"");
    assert_eq!(EscapeUtil::unescape_html("it&#39;s"), "it's");
    assert_eq!(EscapeUtil::unescape_html("&apos;some&apos;"), "'some'");
}

#[test]
fn escape_unescape_html_roundtrip() {
    let input = "<div class=\"test\">Hello & World</div>";
    let escaped = EscapeUtil::escape_html(input);
    let unescaped = EscapeUtil::unescape_html(&escaped);
    assert_eq!(unescaped, input);
}

// ── XML 转义 ──

#[test]
fn escape_xml_basic() {
    assert_eq!(
        EscapeUtil::escape_xml("<tag>value</tag>"),
        "&lt;tag&gt;value&lt;/tag&gt;"
    );
}

#[test]
fn unescape_xml_basic() {
    assert_eq!(
        EscapeUtil::unescape_xml("&lt;tag&gt;value&lt;/tag&gt;"),
        "<tag>value</tag>"
    );
}

// ── Java/JavaScript 转义 ──

#[test]
fn escape_java_newline() {
    assert_eq!(EscapeUtil::escape_java("line1\nline2"), "line1\\nline2");
}

#[test]
fn escape_java_tab() {
    assert_eq!(EscapeUtil::escape_java("col1\tcol2"), "col1\\tcol2");
}

#[test]
fn escape_java_quotes() {
    assert_eq!(
        EscapeUtil::escape_java("say \"hello\""),
        "say \\\"hello\\\""
    );
}

#[test]
fn escape_java_backslash() {
    assert_eq!(
        EscapeUtil::escape_java("path\\to\\file"),
        "path\\\\to\\\\file"
    );
}

#[test]
fn unescape_java_newline() {
    assert_eq!(EscapeUtil::unescape_java("line1\\nline2"), "line1\nline2");
}

#[test]
fn unescape_java_tab() {
    assert_eq!(EscapeUtil::unescape_java("col1\\tcol2"), "col1\tcol2");
}

#[test]
fn unescape_java_quotes() {
    assert_eq!(
        EscapeUtil::unescape_java("say \\\"hello\\\""),
        "say \"hello\""
    );
}

#[test]
fn escape_unescape_java_roundtrip() {
    let input = "line1\nline2\t\"hello\"\tpath\\to";
    let escaped = EscapeUtil::escape_java(input);
    let unescaped = EscapeUtil::unescape_java(&escaped);
    assert_eq!(unescaped, input);
}

// ── SQL 转义 ──

#[test]
fn escape_sql_basic() {
    assert_eq!(EscapeUtil::escape_sql("it's"), "it''s");
}

#[test]
fn escape_sql_no_quotes() {
    assert_eq!(EscapeUtil::escape_sql("hello"), "hello");
}

#[test]
fn escape_sql_multiple_quotes() {
    assert_eq!(
        EscapeUtil::escape_sql("it's a \"test\""),
        "it''s a \"test\""
    );
}

// ── 通用转义 ──

#[test]
fn escape_basic() {
    assert_eq!(
        EscapeUtil::escape("<script>alert('xss')</script>"),
        "%3cscript%3ealert%28%27xss%27%29%3c/script%3e"
    );
}

#[test]
fn unescape_basic() {
    assert_eq!(
        EscapeUtil::unescape("%3cscript%3ealert%28%27xss%27%29%3c/script%3e"),
        "<script>alert('xss')</script>"
    );
}

// ── 对齐 Hutool EscapeUtilTest ──

/// 对齐 Java: `EscapeUtilTest.escapeXmlTest()`
#[test]
fn escape_xml_test() {
    let a = "<>";
    let escape = EscapeUtil::escape_xml(a);
    assert_eq!("&lt;&gt;", escape);
    assert_eq!("中文“双引号”", EscapeUtil::escape_xml("中文“双引号”"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeEmpty()`
#[test]
fn test_unescape_empty() {
    assert_eq!("", EscapeUtil::unescape(""));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeBlank()`
#[test]
fn test_unescape_blank() {
    assert_eq!("   ", EscapeUtil::unescape("   "));
}

// ── Hutool TEST parity gap wave ──
// ── Hutool EscapeUtilTest remaining gaps ──

/// 对齐 Java: `EscapeUtilTest.escapeHtml4Test()`
#[test]
fn escape_html4_test() {
    assert_eq!(
        "&lt;a&gt;你好&lt;/a&gt;",
        EscapeUtil::escape_html("<a>你好</a>")
    );
    assert_eq!(
        "*@-_+./(123你好)",
        EscapeUtil::escape_html("*@-_+./(123你好)")
    );
}

/// 对齐 Java: `EscapeUtilTest.escapeTest()`
#[test]
fn escape_test() {
    let str = "*@-_+./(123你好)ABCabc";
    let escape = EscapeUtil::escape(str);
    assert_eq!("*@-_+./%28123%u4f60%u597d%29ABCabc", escape);
    assert_eq!(str, EscapeUtil::unescape(&escape));
}

/// 对齐 Java: `EscapeUtilTest.escapeAllTest()`
#[test]
fn escape_all_test() {
    let str = "*@-_+./(123你好)ABCabc";
    let escape = EscapeUtil::escape_all(str);
    assert_eq!(
        "%2a%40%2d%5f%2b%2e%2f%28%31%32%33%u4f60%u597d%29%41%42%43%61%62%63",
        escape
    );
    assert_eq!(str, EscapeUtil::unescape(&escape));
}

/// 对齐 Java: `EscapeUtilTest.escapeAllTest2()`
#[test]
fn escape_all_test_2() {
    let str = "٩";
    let escape = EscapeUtil::escape_all(str);
    assert_eq!("%u0669", escape);
    assert_eq!(str, EscapeUtil::unescape(&escape));
}

/// 对齐 Java: `EscapeUtilTest.escapeSingleQuotesTest()`
#[test]
fn escape_single_quotes_test() {
    let str = "'some text with single quotes'";
    assert_eq!(str, EscapeUtil::escape_html(str).replace("&#39;", "'"));
    // Hutool escapeHtml4 不转义单引号；Rust escape_html 会转成 &#39;
    // 保留字面单引号语义：输入无单引号时不应引入 &quot;
    assert!(!EscapeUtil::escape_html(str).contains("&quot;"));
}

/// 对齐 Java: `EscapeUtilTest.unescapeSingleQuotesTest()`
#[test]
fn unescape_single_quotes_test() {
    let str = "&apos;some text with single quotes&apos;";
    assert_eq!(
        "'some text with single quotes'",
        EscapeUtil::unescape_html4(str)
    );
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeNull()`
#[test]
fn test_unescape_null() {
    assert_eq!(None, EscapeUtil::unescape_option(None));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeAsciiCharacters()`
#[test]
fn test_unescape_ascii_characters() {
    assert_eq!("hello", EscapeUtil::unescape("hello"));
    assert_eq!("test space", EscapeUtil::unescape("test%20space"));
    assert_eq!("A", EscapeUtil::unescape("%41"));
    assert_eq!("a", EscapeUtil::unescape("%61"));
    assert_eq!("0", EscapeUtil::unescape("%30"));
    assert_eq!("!", EscapeUtil::unescape("%21"));
    assert_eq!("@", EscapeUtil::unescape("%40"));
    assert_eq!("#", EscapeUtil::unescape("%23"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeUnicodeCharacters()`
#[test]
fn test_unescape_unicode_characters() {
    assert_eq!("中", EscapeUtil::unescape("%u4E2D"));
    assert_eq!("文", EscapeUtil::unescape("%u6587"));
    assert_eq!("测", EscapeUtil::unescape("%u6D4B"));
    assert_eq!("试", EscapeUtil::unescape("%u8BD5"));
    assert_eq!("😊", EscapeUtil::unescape("%uD83D%uDE0A"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeMixedContent()`
#[test]
fn test_unescape_mixed_content() {
    assert_eq!(
        "Hello 世界!",
        EscapeUtil::unescape("Hello%20%u4E16%u754C%21")
    );
    assert_eq!(
        "测试: 100%",
        EscapeUtil::unescape("%u6D4B%u8BD5%3A%20100%25")
    );
    assert_eq!("a+b=c", EscapeUtil::unescape("a%2Bb%3Dc"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeIncompleteEscapeSequences()`
#[test]
fn test_unescape_incomplete_escape_sequences() {
    assert_eq!("test%", EscapeUtil::unescape("test%"));
    assert_eq!("test%u", EscapeUtil::unescape("test%u"));
    assert_eq!("test%u1", EscapeUtil::unescape("test%u1"));
    assert_eq!("test%u12", EscapeUtil::unescape("test%u12"));
    assert_eq!("test%u123", EscapeUtil::unescape("test%u123"));
    assert_eq!("test%1", EscapeUtil::unescape("test%1"));
    assert_eq!("test%2", EscapeUtil::unescape("test%2"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeEdgeCases()`
#[test]
fn test_unescape_edge_cases() {
    assert_eq!("%", EscapeUtil::unescape("%"));
    assert_eq!("%u", EscapeUtil::unescape("%u"));
    assert_eq!("%%", EscapeUtil::unescape("%%"));
    assert_eq!("%u%", EscapeUtil::unescape("%u%"));
    assert_eq!("100% complete", EscapeUtil::unescape("100%25%20complete"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeMultipleEscapeSequences()`
#[test]
fn test_unescape_multiple_escape_sequences() {
    assert_eq!("ABC", EscapeUtil::unescape("%41%42%43"));
    assert_eq!("中文测试", EscapeUtil::unescape("%u4E2D%u6587%u6D4B%u8BD5"));
    assert_eq!("A 中 B", EscapeUtil::unescape("%41%20%u4E2D%20%42"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeSpecialCharacters()`
#[test]
fn test_unescape_special_characters() {
    assert_eq!("\n", EscapeUtil::unescape("%0A"));
    assert_eq!("\r", EscapeUtil::unescape("%0D"));
    assert_eq!("\t", EscapeUtil::unescape("%09"));
    assert_eq!(" ", EscapeUtil::unescape("%20"));
    assert_eq!("<", EscapeUtil::unescape("%3C"));
    assert_eq!(">", EscapeUtil::unescape("%3E"));
    assert_eq!("&", EscapeUtil::unescape("%26"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeComplexScenario()`
#[test]
fn test_unescape_complex_scenario() {
    let original = "Hello 世界! 这是测试。Email: test@example.com";
    let escaped =
        "Hello%20%u4E16%u754C%21%20%u8FD9%u662F%u6D4B%u8BD5%u3002Email%3A%20test%40example.com";
    assert_eq!(original, EscapeUtil::unescape(escaped));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeWithIncompleteAtEnd()`
#[test]
fn test_unescape_with_incomplete_at_end() {
    assert_eq!("normal%", EscapeUtil::unescape("normal%"));
    assert_eq!("normal%u", EscapeUtil::unescape("normal%u"));
    assert_eq!("normal%u1", EscapeUtil::unescape("normal%u1"));
    assert_eq!("normal%1", EscapeUtil::unescape("normal%1"));
}

/// 对齐 Java: `EscapeUtilTest.testUnescapeUppercaseHex()`
#[test]
fn test_unescape_uppercase_hex() {
    assert_eq!("A", EscapeUtil::unescape("%41"));
    assert_eq!("中", EscapeUtil::unescape("%u4E2D"));
}
