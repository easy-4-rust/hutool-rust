//! `VersionUtil` 对比验证测试 —— 对齐 Hutool `VersionUtilTest`
//!
//! 对齐: `cn.hutool.core.util.VersionUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/VersionUtilTest.java
//!
//! # API 命名映射
//! | Java                                  | Rust                                       |
//! |---------------------------------------|--------------------------------------------|
//! | `isGreaterThan(a, b)`                 | `is_greater_than(Option, Option)` / `_str`  |
//! | `isGreaterThanOrEqual(a, b)`          | `is_greater_than_or_equal(Option, Option)`  |
//! | `isLessThan(a, b)`                    | `is_less_than(Option, Option)` / `_str`     |
//! | `isLessThanOrEqual(a, b)`             | `is_less_than_or_equal(Option, Option)`     |
//! | `matchEl(cur, el)`                    | `match_el(cur, el) -> Result<bool>`        |
//! | `matchEl(cur, el, delimiter)`         | `match_el_with_delimiter(cur, el, delim)`  |
//! | `anyMatch(cur, list)`                 | `any_match(cur, iter)`                     |
//!
//! # Java null 处理
//!
//! Java `VersionUtil.isGreaterThan("1.0", null)` 将 null 视为"小于任何版本"。
//! Rust 中使用 `Option<&str>` 表达，`None` 与 Java `null` 对齐。

use hutool_core::VersionUtil;

/// 对齐 Java: `VersionUtilTest.isGreaterThan()` (行 14-20)
///
/// 注:Java 字符串含前后空格 `" 1.0.2"`,VersionUtil 内部会 trim。
#[test]
fn is_greater_than_test() {
    let cur = " 1.0.2";
    assert!(
        VersionUtil::is_greater_than_str(cur, "1.0.1"),
        "(对齐 Java isGreaterThan 第 1 组)"
    );
    assert!(
        VersionUtil::is_greater_than_str(cur, "1"),
        "(对齐 Java isGreaterThan 第 2 组)"
    );
    assert!(
        !VersionUtil::is_greater_than_str(cur, "1.1"),
        "(对齐 Java isGreaterThan 第 3 组)"
    );
}

/// 对齐 Java: `VersionUtilTest.isGreaterThanOrEqual()` (行 22-28)
#[test]
fn is_greater_than_or_equal_test() {
    let cur = "1.0.2 ";
    assert!(
        VersionUtil::is_greater_than_or_equal_str(cur, "1.0.1"),
        "(对齐 Java)"
    );
    assert!(
        VersionUtil::is_greater_than_or_equal_str(cur, "1.0.2"),
        "(对齐 Java)"
    );
    assert!(
        !VersionUtil::is_greater_than_or_equal_str(cur, "1.1"),
        "(对齐 Java)"
    );
}

/// 对齐 Java: `VersionUtilTest.isLessThan()` (行 30-37)
#[test]
fn is_less_than_test() {
    let cur = "1.0.2";
    assert!(VersionUtil::is_less_than_str(cur, "1.0.3"), "(对齐 Java)");
    assert!(!VersionUtil::is_less_than_str(cur, "1"), "(对齐 Java)");
    assert!(VersionUtil::is_less_than_str(cur, "1.1"), "(对齐 Java)");
    assert!(!VersionUtil::is_less_than_str(cur, "1.0.2"), "(对齐 Java)");
}

/// 对齐 Java: `VersionUtilTest.isLessThanOrEqual()` (行 39-45)
#[test]
fn is_less_than_or_equal_test() {
    let cur = "1.0.2";
    assert!(
        VersionUtil::is_less_than_or_equal_str(cur, "1.0.2"),
        "(对齐 Java)"
    );
    assert!(
        !VersionUtil::is_less_than_or_equal_str(cur, "1.0.1"),
        "(对齐 Java)"
    );
    assert!(
        VersionUtil::is_less_than_or_equal_str(cur, "1.1"),
        "(对齐 Java)"
    );
}

/// 对齐 Java: `VersionUtilTest.matchEl()` (行 47-56)
#[test]
fn match_el_test() {
    let cur = "1.0.2";
    assert!(
        VersionUtil::match_el(cur, "1.0.1;1.0.2").unwrap(),
        "(对齐 Java 第 1 组)"
    );
    assert!(
        !VersionUtil::match_el(cur, "1.0.1;1.0.3").unwrap(),
        "(对齐 Java 第 2 组)"
    );
    assert!(
        VersionUtil::match_el(cur, "1.0.9;1.0.1-1.0.2").unwrap(),
        "(对齐 Java 第 3 组)"
    );
    assert!(
        VersionUtil::match_el(cur, "1.0.9;1.0.1-1.0.3").unwrap(),
        "(对齐 Java 第 4 组)"
    );
    assert!(
        VersionUtil::match_el_with_delimiter(cur, "1.0.9,1.0.1-1.0.3", ",").unwrap(),
        "自定义分隔符 ,(对齐 Java 第 5 组)"
    );
}

/// 对齐 Java: `VersionUtilTest.matchEl_Exception_whenVersionDelimiterIllegal()` (行 58-68)
///
/// 非法分隔符: "-", ">", ">=", "<", "<=", "≥", "≤", null, "", " "
/// 全部应抛 `UtilException`。Rust 中用 `Result::is_err()` 表达。
/// 注:Java null 在 Rust 中跳过。
#[test]
fn match_el_exception_when_version_delimiter_illegal() {
    let cur = "1.0.2";
    for illegal in ["-", ">", ">=", "<", "<=", "\u{2265}", "\u{2264}", "", " "] {
        let result = VersionUtil::match_el_with_delimiter(cur, "1.0.1;1.0.2", illegal);
        assert!(
            result.is_err(),
            "分隔符 {illegal:?} 非法应返回 Err (对齐 Java matchEl_Exception_whenVersionDelimiterIllegal)"
        );
    }
    // Java null 跳过(Rust &str 无法表达 null)
}

/// 对齐 Java: `VersionUtilTest.anyMatch()` (行 70-75)
#[test]
fn any_match_test() {
    let cur = "1.0.2";
    assert!(
        VersionUtil::any_match(cur, ["1.0.1", "1.0.3", "1.0.2"]),
        "List 形式 (对齐 Java anyMatch 第 1 组)"
    );
    assert!(
        VersionUtil::any_match(cur, ["1.0.1", "1.0.2"]),
        "可变参数形式 (对齐 Java anyMatch 第 2 组)"
    );
}

/// 对齐 Java: `VersionUtilTest.testMatchEl()` (行 77-79)
///
/// 该测试方法体为空,仅占位以备未来扩展。Rust 版本同样为空。
#[test]
fn test_match_el() {
    // Java 源为空方法,Rust 版本保持空,不引入断言。
}

/// 对齐 Java: `VersionUtilTest.matchEl_rangeBoundaryCases()` (行 88-106)
///
/// 测试版本范围表达式边界:
/// 1. 左边界为空("-1.0.3")应匹配 <= 1.0.3 的版本
/// 2. 右边界为空("1.0.0-")应匹配 >= 1.0.0 的版本
/// 3. 双边界为空("-")应匹配所有版本
#[test]
fn match_el_range_boundary_cases() {
    let cur = "1.0.2";

    // 左边界为空
    assert!(
        VersionUtil::match_el(cur, "-1.0.3").unwrap(),
        "-1.0.3 (对齐 Java)"
    );
    assert!(
        VersionUtil::match_el(cur, "-1.0.2").unwrap(),
        "-1.0.2 (对齐 Java)"
    );
    assert!(
        !VersionUtil::match_el(cur, "-1.0.0").unwrap(),
        "-1.0.0 (对齐 Java)"
    );

    // 右边界为空
    assert!(
        VersionUtil::match_el(cur, "1.0.0-").unwrap(),
        "1.0.0- (对齐 Java)"
    );
    assert!(
        VersionUtil::match_el(cur, "1.0.2-").unwrap(),
        "1.0.2- (对齐 Java)"
    );
    assert!(
        !VersionUtil::match_el(cur, "1.0.3-").unwrap(),
        "1.0.3- (对齐 Java)"
    );

    // 双边界为空
    assert!(VersionUtil::match_el(cur, "-").unwrap(), "- (对齐 Java)");
    assert!(
        VersionUtil::match_el("0.0.1", "-").unwrap(),
        "0.0.1 vs - (对齐 Java)"
    );
    assert!(
        VersionUtil::match_el("999.999.999", "-").unwrap(),
        "999.999.999 vs - (对齐 Java)"
    );
}

/// 对齐 Java: `VersionUtilTest.issueIJNFQZTest()` (行 108-113)
///
/// `VersionComparator.INSTANCE.compare("1.0", null)` = 1(null 视为最小)
/// `StrUtil.compareVersion("1.0", null)` = 1
/// `VersionUtil.isGreaterThan("1.0", null)` = true
#[test]
fn issue_ijnfqz_test() {
    assert!(
        VersionUtil::is_greater_than(Some("1.0"), None),
        "isGreaterThan(\"1.0\", null) == true (对齐 Java issueIJNFQZTest)"
    );
}
