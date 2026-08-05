//! 多个 Utility 的对比验证测试合集
//!
//! 本文件汇集 Hutool `cn.hutool.core.util` 包下短小测试类的 Rust 等价:
//! - CreditCodeUtilTest (4 个 @Test)
//! - PageUtilTest       (3 个 @Test)
//! - RadixUtilTest      (1 个 @Test)
//!
//! 所有断言严格按 Java 测试用例 1:1 翻译。

use hutool_core::{CreditCodeUtil, PageUtil, RadixUtil};

// ════════════════════════════════════════════════════════════
//  CreditCodeUtil —— 对齐 cn.hutool.core.util.CreditCodeUtilTest
//  来源: hutool-core/src/test/java/cn/hutool/core/util/CreditCodeUtilTest.java
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `CreditCodeUtilTest.isCreditCodeBySimple()`
///
/// Java 源(行 8-12):
/// ```java
/// String testCreditCode = "91310115591693856A";
/// assertTrue(CreditCodeUtil.isCreditCodeSimple(testCreditCode));
/// ```
#[test]
fn credit_code_is_credit_code_by_simple() {
    let code = "91310115591693856A";
    assert!(
        CreditCodeUtil::is_credit_code_simple(code),
        "is_credit_code_simple({code:?}) 应为 true (对齐 Java CreditCodeUtilTest.isCreditCodeBySimple)"
    );
}

/// 对齐 Java: `CreditCodeUtilTest.isCreditCode()`
///
/// Java 源(行 14-18):
/// ```java
/// String testCreditCode = "91310110666007217T";
/// assertTrue(CreditCodeUtil.isCreditCode(testCreditCode));
/// ```
#[test]
fn credit_code_is_credit_code() {
    let code = "91310110666007217T";
    assert!(
        CreditCodeUtil::is_credit_code(code),
        "is_credit_code({code:?}) 应为 true (对齐 Java CreditCodeUtilTest.isCreditCode)"
    );
}

/// 对齐 Java: `CreditCodeUtilTest.isCreditCode2()`
///
/// Java 源(行 20-26):早期试点地区部分代码不符合国家标准,应校验失败。
/// ```java
/// String testCreditCode = "91350211M00013FA1N";
/// assertFalse(CreditCodeUtil.isCreditCode(testCreditCode));
/// ```
#[test]
fn credit_code_is_credit_code_2() {
    // 早期部分试点地区推行"法人和其他组织统一社会信用代码"较早,
    // 会存在部分代码不符合国家标准的情况。
    // 见: https://github.com/bluesky335/IDCheck
    let code = "91350211M00013FA1N";
    assert!(
        !CreditCodeUtil::is_credit_code(code),
        "is_credit_code({code:?}) 应为 false (对齐 Java CreditCodeUtilTest.isCreditCode2)"
    );
}

/// 对齐 Java: `CreditCodeUtilTest.randomCreditCode()`
///
/// Java 源(行 28-32):
/// ```java
/// final String s = CreditCodeUtil.randomCreditCode();
/// assertTrue(CreditCodeUtil.isCreditCode(s));
/// ```
///
/// 注:Hutool 的 `randomCreditCode()` 无入参,内部使用 `ThreadLocalRandom`。
/// hutool-rs 版本同样无入参,内部使用 `OsRng`(密码学安全随机源)。
#[test]
fn credit_code_random_credit_code() {
    let code = CreditCodeUtil::random_credit_code();
    assert!(
        CreditCodeUtil::is_credit_code(&code),
        "random_credit_code 生成的 {code:?} 应通过 is_credit_code 校验 (对齐 Java randomCreditCode)"
    );
}

// ════════════════════════════════════════════════════════════
//  PageUtil —— 对齐 cn.hutool.core.util.PageUtilTest
//  来源: hutool-core/src/test/java/cn/hutool/core/util/PageUtilTest.java
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `PageUtilTest.transToStartEndTest()`
///
/// Java 源(行 13-22):
/// ```java
/// final int[] startEnd1 = PageUtil.transToStartEnd(0, 10);
/// assertEquals(0, startEnd1[0]);
/// assertEquals(10, startEnd1[1]);
///
/// final int[] startEnd2 = PageUtil.transToStartEnd(1, 10);
/// assertEquals(10, startEnd2[0]);
/// assertEquals(20, startEnd2[1]);
/// ```
#[test]
fn page_trans_to_start_end_test() {
    // PageUtil 默认 first_page_no = 0(对齐 Java PageUtil 静态约定)
    let page_util = PageUtil::new(0);
    let start_end1 = page_util.start_end(0, 10);
    assert_eq!(
        start_end1,
        [0, 10],
        "start_end(0, 10) = [0, 10] (对齐 Java transToStartEnd 第 1 组)"
    );

    let start_end2 = page_util.start_end(1, 10);
    assert_eq!(
        start_end2,
        [10, 20],
        "start_end(1, 10) = [10, 20] (对齐 Java transToStartEnd 第 2 组)"
    );
}

/// 对齐 Java: `PageUtilTest.totalPage()`
///
/// Java 源(行 24-28):
/// ```java
/// final int totalPage = PageUtil.totalPage(20, 3);
/// assertEquals(7, totalPage);
/// ```
#[test]
fn page_total_page_test() {
    let total_page = PageUtil::total_page_i32(20, 3).unwrap();
    assert_eq!(
        total_page, 7,
        "total_page_i32(20, 3) = 7 (对齐 Java PageUtilTest.totalPage)"
    );
}

/// 对齐 Java: `PageUtilTest.rainbowTest()`
///
/// Java 源(行 30-34):
/// ```java
/// final int[] rainbow = PageUtil.rainbow(5, 20, 6);
/// assertArrayEquals(new int[]{3, 4, 5, 6, 7, 8}, rainbow);
/// ```
#[test]
fn page_rainbow_test() {
    let rainbow = PageUtil::rainbow(5, 20, 6).unwrap();
    assert_eq!(
        rainbow,
        vec![3, 4, 5, 6, 7, 8],
        "rainbow(5, 20, 6) = [3,4,5,6,7,8] (对齐 Java PageUtilTest.rainbowTest)"
    );
}

// ════════════════════════════════════════════════════════════
//  RadixUtil —— 对齐 cn.hutool.core.util.RadixUtilTest
//  来源: hutool-core/src/test/java/cn/hutool/core/util/RadixUtilTest.java
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `RadixUtilTest.issueIDFPGRTest()`
///
/// Java 源(行 7-14):
/// ```java
/// String radixs = "0123456789ABC"; // base 13
/// String bad = "1X3"; // 'X' 不在 radix 中
/// assertThrows(IllegalArgumentException.class, () -> {
///     RadixUtil.decode(radixs, bad);
/// });
/// ```
///
/// 这是 issue IDFPGR 的回归测试:字符不在字母表中时应抛出异常。
#[test]
fn radix_issue_idfpgr_test() {
    let alphabet = "0123456789ABC"; // base 13
    let bad = "1X3"; // 'X' 不在 radix 中
    let result = RadixUtil::decode(alphabet, bad);
    assert!(
        result.is_err(),
        "decode(\"0123456789ABC\", \"1X3\") 应返回 Err (对齐 Java RadixUtilTest.issueIDFPGRTest: 抛出 IllegalArgumentException)"
    );
}
