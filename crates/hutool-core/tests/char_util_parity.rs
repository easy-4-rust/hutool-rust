//! `CharUtil` 对比验证测试 —— 对齐 Hutool `CharUtilTest`
//!
//! 对齐: `cn.hutool.core.util.CharUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/CharUtilTest.java
//!
//! 严格按 Hutool Java 测试用例 1:1 翻译为 Rust,验证相同输入下
//! hutool-rs 与 hutool-java 产生相同输出。

use hutool_core::CharUtil;

/// 对齐 Java: `CharUtilTest.trimTest()`
///
/// Java 源(行 10-14):验证不可见字符 U+202A 被识别为 blank。
/// (此处避免在文档注释中直接嵌入 U+202A,以免触发 Rust 的方向码点 lint)
#[test]
fn trim_test() {
    // 此字符串中的第一个字符为不可见字符: '\u202a'
    let s = "\u{202a}C:/Users/maple/Desktop/tone.txt";
    let first: Vec<char> = s.chars().collect();
    assert_eq!(
        first[0], '\u{202a}',
        "首字符应为 U+202A (对齐 Java trimTest)"
    );
    assert!(
        CharUtil::is_blank_char(first[0]),
        "is_blank_char(U+202A) 应为 true (对齐 Java trimTest)"
    );
}

/// 对齐 Java: `CharUtilTest.isEmojiTest()`
///
/// Java 源(行 16-21):
/// ```java
/// final String a = "莉🌹";
/// assertFalse(CharUtil.isEmoji(a.charAt(0)));
/// assertTrue(CharUtil.isEmoji(a.charAt(1)));
/// ```
#[test]
fn is_emoji_test() {
    let a: Vec<char> = "莉🌹".chars().collect();
    assert!(
        !CharUtil::is_emoji(a[0]),
        "is_emoji('莉') 应为 false (对齐 Java isEmojiTest)"
    );
    assert!(
        CharUtil::is_emoji(a[1]),
        "is_emoji('🌹') 应为 true (对齐 Java isEmojiTest)"
    );
}

/// 对齐 Java: `CharUtilTest.isCharTest()`
///
/// Java 源(行 23-26):
/// ```java
/// final char a = 'a';
/// assertTrue(CharUtil.isChar(a));
/// ```
///
/// 注:Java 的 `isChar(Object)` 用反射判断,Rust 版本需要 `Any`。
/// 该测试简化为验证字符类型本身的判定。
#[test]
fn is_char_test() {
    let a = 'a';
    assert!(
        CharUtil::is_char(&a),
        "is_char('a') 应为 true (对齐 Java isCharTest)"
    );
}

/// 对齐 Java: `CharUtilTest.isBlankCharTest()`
///
/// Java 源(行 28-49):
/// ```java
/// assertTrue(CharUtil.isBlankChar('\u00A0'));
/// assertTrue(CharUtil.isBlankChar('\u0020'));
/// assertTrue(CharUtil.isBlankChar('\u3000'));
/// assertTrue(CharUtil.isBlankChar('\u0000'));
/// // ...
/// assertTrue(CharUtil.isBlankChar('\u200c'));
/// ```
#[test]
fn is_blank_char_test() {
    for ch in [
        '\u{00A0}', '\u{0020}', '\u{3000}', '\u{0000}', ' ', '\u{200c}',
    ] {
        assert!(
            CharUtil::is_blank_char(ch),
            "is_blank_char({ch:?}) 应为 true (对齐 Java isBlankCharTest)"
        );
    }
}

/// 对齐 Java: `CharUtilTest.toCloseCharTest()`
///
/// Java 源(行 52-57):
/// ```java
/// assertEquals('②', CharUtil.toCloseChar('2'));
/// assertEquals('Ⓜ', CharUtil.toCloseChar('M'));
/// assertEquals('ⓡ', CharUtil.toCloseChar('r'));
/// ```
#[test]
fn to_close_char_test() {
    assert_eq!(
        CharUtil::to_close_char('2'),
        '②',
        "to_close_char('2') = '②' (对齐 Java toCloseCharTest)"
    );
    assert_eq!(
        CharUtil::to_close_char('M'),
        'Ⓜ',
        "to_close_char('M') = 'Ⓜ' (对齐 Java toCloseCharTest)"
    );
    assert_eq!(
        CharUtil::to_close_char('r'),
        'ⓡ',
        "to_close_char('r') = 'ⓡ' (对齐 Java toCloseCharTest)"
    );
}

/// 对齐 Java: `CharUtilTest.toCloseByNumberTest()`
///
/// Java 源(行 59-64):
/// ```java
/// assertEquals('②', CharUtil.toCloseByNumber(2));
/// assertEquals('⑫', CharUtil.toCloseByNumber(12));
/// assertEquals('⑳', CharUtil.toCloseByNumber(20));
/// ```
#[test]
fn to_close_by_number_test() {
    assert_eq!(
        CharUtil::to_close_by_number(2).unwrap(),
        '②',
        "to_close_by_number(2) = '②' (对齐 Java toCloseByNumberTest)"
    );
    assert_eq!(
        CharUtil::to_close_by_number(12).unwrap(),
        '⑫',
        "to_close_by_number(12) = '⑫' (对齐 Java toCloseByNumberTest)"
    );
    assert_eq!(
        CharUtil::to_close_by_number(20).unwrap(),
        '⑳',
        "to_close_by_number(20) = '⑳' (对齐 Java toCloseByNumberTest)"
    );
}

/// 对齐 Java: `CharUtilTest.issueI5UGSQTest()`
///
/// Java 源(行 66-73):
/// ```java
/// char c = '\u3164';
/// assertTrue(CharUtil.isBlankChar(c));
/// c = '\u2800';
/// assertTrue(CharUtil.isBlankChar(c));
/// ```
///
/// 这是 issue I5UGSQ 的回归测试:某些 Unicode 字符(韩文填充符、盲文空白符)
/// 应被识别为 blank。
#[test]
fn issue_i5ugsq_test() {
    assert!(
        CharUtil::is_blank_char('\u{3164}'),
        "is_blank_char('\\u3164') 应为 true (对齐 Java issueI5UGSQTest)"
    );
    assert!(
        CharUtil::is_blank_char('\u{2800}'),
        "is_blank_char('\\u2800') 应为 true (对齐 Java issueI5UGSQTest)"
    );
}

/// 对齐 Java: `CharUtilTest.issueIDFNHETest()`
///
/// Java 源(行 75-80):
/// ```java
/// assertThrows(IllegalArgumentException.class, () -> CharUtil.toCloseByNumber(0));
/// assertThrows(IllegalArgumentException.class, () -> CharUtil.toCloseByNumber(-1));
/// ```
///
/// 这是 issue IDFNHE 的回归测试:数字 0 或负数应抛出 `IllegalArgumentException`。
/// Rust 版本通过 `Result::is_err()` 表达相同语义。
#[test]
fn issue_idfnhe_test() {
    assert!(
        CharUtil::to_close_by_number(0).is_err(),
        "to_close_by_number(0) 应返回 Err (对齐 Java issueIDFNHETest: 抛出 IllegalArgumentException)"
    );
    assert!(
        CharUtil::to_close_by_number(-1).is_err(),
        "to_close_by_number(-1) 应返回 Err (对齐 Java issueIDFNHETest: 抛出 IllegalArgumentException)"
    );
}
