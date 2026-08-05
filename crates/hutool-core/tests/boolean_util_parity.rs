//! `BooleanUtil` 对比验证测试 —— 对齐 Hutool `BooleanUtilTest`
//!
//! 对齐: `cn.hutool.core.util.BooleanUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/BooleanUtilTest.java
//!
//! 本文件严格按 Hutool Java 测试用例 1:1 翻译为 Rust,验证相同输入下
//! hutool-rs 与 hutool-java 产生相同输出。
//!
//! # 对比策略
//!
//! - 每个 Rust 测试函数对应一个 Java `@Test` 方法,函数名保持一致(snake_case)
//! - 每条 `assertTrue/assertFalse` 断言在 Rust 中用 `assert!` 复现
//! - 多语言词汇(true/yes/是/√/对...)必须与 Java `BooleanUtil.toBoolean` 同语义
//! - 在 Hutool Java 测试源码中已显式断言的输入值,本文件 100% 复制
//!
//! # 已知 Java↔Rust 差异
//!
//! - Java 的 `BooleanUtil.andOfWrap(Boolean...)` 接受装箱 + null,Rust 版本
//!   通过 `and_wrapped(&[Option<bool>])` 表达,`None` 对应 Java `null`
//! - Java 的可变参数 `Boolean...` → Rust `&[bool]` 切片

use hutool_core::BooleanUtil;

/// 对齐 Java: `BooleanUtilTest.toBooleanTest()`
///
/// Java 源(行 11-78):
/// ```java
/// assertTrue(BooleanUtil.toBoolean("true"));
/// assertTrue(BooleanUtil.toBoolean("yes"));
/// // ... 共 18 个 true 用例
/// assertFalse(BooleanUtil.toBoolean("false"));
/// // ... 共 21 个 false 用例
/// ```
#[test]
fn to_boolean_test() {
    // Java assertTrue 用例(英文/中文/符号词汇,true 语义)
    for value in [
        "true", "yes", "y", "t", "OK", "correct", "success", "1", "On",
        // 中文 true 词汇
        "是", "对", "真", "對", "正确", "开", "开启", // 符号
        "√", "☑",
    ] {
        assert!(
            BooleanUtil::to_boolean(value),
            "BooleanUtil::to_boolean({value:?}) 应为 true (对齐 Java BooleanUtilTest.toBooleanTest assertTrue)"
        );
    }

    // Java assertFalse 用例
    for value in [
        "false", "no", "n", "f", "off", "wrong", "fail", "0", "Off",
        // 中文 false 词汇
        "否", "错", "假", "錯", "错误", "关", "关闭", // 符号
        "×", "☒", // 其他非布尔字符串
        "6455434", "",
    ] {
        assert!(
            !BooleanUtil::to_boolean(value),
            "BooleanUtil::to_boolean({value:?}) 应为 false (对齐 Java BooleanUtilTest.toBooleanTest assertFalse)"
        );
    }
}

/// 对齐 Java: `BooleanUtilTest.andTest()`
///
/// Java 源(行 42-45):
/// ```java
/// assertFalse(BooleanUtil.and(true, false));
/// assertFalse(BooleanUtil.andOfWrap(true, false));
/// ```
#[test]
fn and_test() {
    // BooleanUtil.and(true, false) → false
    assert!(
        !BooleanUtil::and(&[true, false]).unwrap(),
        "and(true, false) 应为 false (对齐 Java BooleanUtilTest.andTest)"
    );
    // BooleanUtil.andOfWrap(true, false) → false
    // Rust: and_wrapped(&[Some(true), Some(false)])
    assert!(
        !BooleanUtil::and_wrapped(&[Some(true), Some(false)]).unwrap(),
        "and_wrapped(Some(true), Some(false)) 应为 false (对齐 Java andOfWrap)"
    );
}

/// 对齐 Java: `BooleanUtilTest.orTest()`
///
/// Java 源(行 47-50):
/// ```java
/// assertTrue(BooleanUtil.or(true, false));
/// assertTrue(BooleanUtil.orOfWrap(true, false));
/// ```
#[test]
fn or_test() {
    assert!(
        BooleanUtil::or(&[true, false]).unwrap(),
        "or(true, false) 应为 true (对齐 Java BooleanUtilTest.orTest)"
    );
    assert!(
        BooleanUtil::or_wrapped(&[Some(true), Some(false)]).unwrap(),
        "or_wrapped(Some(true), Some(false)) 应为 true (对齐 Java orOfWrap)"
    );
}

/// 对齐 Java: `BooleanUtilTest.xorTest()`
///
/// Java 源(行 52-55):
/// ```java
/// assertTrue(BooleanUtil.xor(true, false));
/// assertTrue(BooleanUtil.xorOfWrap(true, false));
/// ```
#[test]
fn xor_test() {
    assert!(
        BooleanUtil::xor(&[true, false]).unwrap(),
        "xor(true, false) 应为 true (对齐 Java BooleanUtilTest.xorTest)"
    );
    assert!(
        BooleanUtil::xor_wrapped(&[Some(true), Some(false)]).unwrap(),
        "xor_wrapped(Some(true), Some(false)) 应为 true (对齐 Java xorOfWrap)"
    );
}

/// 参考 Java BooleanUtilTest.orOfWrapTest（Java 源无 @Test，非 inventory 项；此处保留对等行为校验）
///
/// Java 源(行 57-60):
/// ```java
/// assertFalse(BooleanUtil.orOfWrap(Boolean.FALSE, null));
/// assertTrue(BooleanUtil.orOfWrap(Boolean.TRUE, null));
/// ```
///
/// 注:Java 该方法未标注 `@Test`,但作为对照仍翻译为 Rust 测试。
#[test]
fn or_of_wrap_test() {
    // Boolean.FALSE, null → false (Java null → Rust None)
    assert!(
        !BooleanUtil::or_wrapped(&[Some(false), None]).unwrap(),
        "or_wrapped(Some(false), None) 应为 false (对齐 Java orOfWrapTest)"
    );
    assert!(
        BooleanUtil::or_wrapped(&[Some(true), None]).unwrap(),
        "or_wrapped(Some(true), None) 应为 true (对齐 Java orOfWrapTest)"
    );
}

/// 对齐 Java: `BooleanUtilTest.isTrueIsFalseTest()`
///
/// Java 源(行 63-67):
/// ```java
/// assertFalse(BooleanUtil.isTrue(null));
/// assertFalse(BooleanUtil.isFalse(null));
/// ```
#[test]
fn is_true_is_false_test() {
    // Java null → Rust None
    assert!(
        !BooleanUtil::is_true(None),
        "is_true(None) 应为 false (对齐 Java BooleanUtilTest.isTrueIsFalseTest)"
    );
    assert!(
        !BooleanUtil::is_false(None),
        "is_false(None) 应为 false (对齐 Java BooleanUtilTest.isTrueIsFalseTest)"
    );
}

/// 参考 Java BooleanUtilTest.negateTest（Java 源无 @Test，非 inventory 项；此处保留对等行为校验）
///
/// Java 源(行 81-88):
/// ```java
/// assertFalse(BooleanUtil.negate(Boolean.TRUE));
/// assertTrue(BooleanUtil.negate(Boolean.FALSE));
/// ```
#[test]
fn negate_test() {
    assert!(
        !BooleanUtil::negate(true),
        "negate(true) 应为 false (对齐 Java BooleanUtilTest.negateTest)"
    );
    assert!(
        BooleanUtil::negate(false),
        "negate(false) 应为 true (对齐 Java BooleanUtilTest.negateTest)"
    );
}

/// 对齐 Java: `BooleanUtilTest.toStringTest()`
///
/// Java 源(行 90-100):
/// ```java
/// assertEquals("true", BooleanUtil.toStringTrueFalse(true));
/// assertEquals("false", BooleanUtil.toStringTrueFalse(false));
/// assertEquals("yes", BooleanUtil.toStringYesNo(true));
/// assertEquals("no", BooleanUtil.toStringYesNo(false));
/// assertEquals("on", BooleanUtil.toStringOnOff(true));
/// assertEquals("off", BooleanUtil.toStringOnOff(false));
/// ```
#[test]
fn to_string_test() {
    assert_eq!(
        BooleanUtil::to_string_true_false(true),
        "true",
        "to_string_true_false(true) = \"true\" (对齐 Java toStringTrueFalse)"
    );
    assert_eq!(
        BooleanUtil::to_string_true_false(false),
        "false",
        "to_string_true_false(false) = \"false\" (对齐 Java toStringTrueFalse)"
    );
    assert_eq!(
        BooleanUtil::to_string_yes_no(true),
        "yes",
        "to_string_yes_no(true) = \"yes\" (对齐 Java toStringYesNo)"
    );
    assert_eq!(
        BooleanUtil::to_string_yes_no(false),
        "no",
        "to_string_yes_no(false) = \"no\" (对齐 Java toStringYesNo)"
    );
    assert_eq!(
        BooleanUtil::to_string_on_off(true),
        "on",
        "to_string_on_off(true) = \"on\" (对齐 Java toStringOnOff)"
    );
    assert_eq!(
        BooleanUtil::to_string_on_off(false),
        "off",
        "to_string_on_off(false) = \"off\" (对齐 Java toStringOnOff)"
    );
}

/// 对齐 Java: `BooleanUtilTest.issue3587Test()`
///
/// Java 源(行 102-108):
/// ```java
/// Boolean boolean1 = true;
/// Boolean boolean2 = null;
/// Boolean result = BooleanUtil.andOfWrap(boolean1, boolean2);
/// assertFalse(result);
/// ```
///
/// 这是 issue #3587 的回归测试:`andOfWrap(true, null)` 应返回 false
/// 而不是抛出 NPE。
#[test]
fn issue_3587_test() {
    let result = BooleanUtil::and_wrapped(&[Some(true), None]).unwrap();
    assert!(
        !result,
        "and_wrapped(Some(true), None) 应为 false (对齐 Java issue3587Test: andOfWrap(true, null))"
    );
}

/// 对齐 Java: `BooleanUtilTest.testXorSemantics()`
///
/// Java 源(行 109-114):
/// ```java
/// assertTrue(BooleanUtil.xor(true, true, true));
/// assertFalse(BooleanUtil.xor(true, true));
/// ```
///
/// xor 的语义是 true 的数量为奇数。
#[test]
fn test_xor_semantics() {
    assert!(
        BooleanUtil::xor(&[true, true, true]).unwrap(),
        "xor(true, true, true) 应为 true(奇数个 true,对齐 Java testXorSemantics)"
    );
    assert!(
        !BooleanUtil::xor(&[true, true]).unwrap(),
        "xor(true, true) 应为 false(偶数个 true,对齐 Java testXorSemantics)"
    );
}

/// 对齐 Java: `BooleanUtilTest.testExactlyOneTrue()`
///
/// Java 源(行 116-127):
/// ```java
/// assertTrue(BooleanUtil.exactlyOneTrue(true, false, false));
/// assertFalse(BooleanUtil.exactlyOneTrue(true, true, false));
/// assertFalse(BooleanUtil.exactlyOneTrue(true, true, true));
/// assertFalse(BooleanUtil.exactlyOneTrue(false, false, false));
/// ```
#[test]
fn test_exactly_one_true() {
    // 恰好只有一个 true
    assert!(
        BooleanUtil::exactly_one_true(&[true, false, false]).unwrap(),
        "exactly_one_true(true, false, false) 应为 true (对齐 Java testExactlyOneTrue)"
    );
    // 多个 true,不符合互斥语义
    assert!(
        !BooleanUtil::exactly_one_true(&[true, true, false]).unwrap(),
        "exactly_one_true(true, true, false) 应为 false (对齐 Java testExactlyOneTrue)"
    );
    assert!(
        !BooleanUtil::exactly_one_true(&[true, true, true]).unwrap(),
        "exactly_one_true(true, true, true) 应为 false (对齐 Java testExactlyOneTrue)"
    );
    // 没有 true
    assert!(
        !BooleanUtil::exactly_one_true(&[false, false, false]).unwrap(),
        "exactly_one_true(false, false, false) 应为 false (对齐 Java testExactlyOneTrue)"
    );
}
