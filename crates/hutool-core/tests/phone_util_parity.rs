//! `PhoneUtil` 对比验证测试 —— 对齐 Hutool `PhoneUtilTest`
//!
//! 对齐: `cn.hutool.core.util.PhoneUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/PhoneUtilTest.java
//!
//! # API 命名映射
//! | Java                       | Rust                            |
//! |----------------------------|---------------------------------|
//! | `isMobile(str)`            | `is_mobile(str)`                |
//! | `isTel(str)`               | `is_tel(str)`                   |
//! | `isPhone(str)`             | `is_phone(str)`                 |
//! | `isTel400800(str)`         | `is_tel_400_800(str)`           |
//! | `hideBefore(str)`          | `hide_before(str)`              |
//! | `hideBetween(str)`         | `hide_between(str)`             |
//! | `hideAfter(str)`           | `hide_after(str)`               |
//! | `subBefore(str)`           | `sub_before(str)`               |
//! | `subBetween(str)`          | `sub_between(str)`              |
//! | `subAfter(str)`            | `sub_after(str)`                |
//! | `subTelBefore(str)`        | `sub_tel_before(str) -> Option` |
//! | `subTelAfter(str)`         | `sub_tel_after(str) -> Option`  |

use hutool_core::PhoneUtil;

/// 对齐 Java: `PhoneUtilTest.testCheck()` (行 15-31)
#[test]
fn test_check() {
    let mobile = "13612345678";
    let tel = "010-88993108";
    let err_mobile = "136123456781";
    let err_tel = "010-889931081";

    assert!(
        PhoneUtil::is_mobile(mobile),
        "is_mobile(mobile) (对齐 Java)"
    );
    assert!(PhoneUtil::is_tel(tel), "is_tel(tel) (对齐 Java)");
    assert!(PhoneUtil::is_phone(mobile), "is_phone(mobile) (对齐 Java)");
    assert!(PhoneUtil::is_phone(tel), "is_phone(tel) (对齐 Java)");

    assert!(
        !PhoneUtil::is_mobile(err_mobile),
        "is_mobile(errMobile) 应 false (对齐 Java)"
    );
    assert!(
        !PhoneUtil::is_tel(err_tel),
        "is_tel(errTel) 应 false (对齐 Java)"
    );
    assert!(
        !PhoneUtil::is_phone(err_mobile),
        "is_phone(errMobile) 应 false (对齐 Java)"
    );
    assert!(
        !PhoneUtil::is_phone(err_tel),
        "is_phone(errTel) 应 false (对齐 Java)"
    );
}

/// 对齐 Java: `PhoneUtilTest.testTel()` (行 33-50)
#[test]
fn test_tel() {
    // 有效固定电话
    for s in ["010-12345678", "020-9999999", "0755-7654321"] {
        assert!(
            PhoneUtil::is_tel(s),
            "is_tel({s:?}) 应 true (对齐 Java testTel 有效组)"
        );
    }
    // 无效固定电话
    for s in [
        "010 12345678",
        "A20-9999999",
        "0755-7654.321",
        "13619887123",
    ] {
        assert!(
            !PhoneUtil::is_tel(s),
            "is_tel({s:?}) 应 false (对齐 Java testTel 无效组)"
        );
    }
}

/// 对齐 Java: `PhoneUtilTest.testHide()` (行 52-59)
#[test]
fn test_hide() {
    let mobile = "13612345678";
    assert_eq!(
        PhoneUtil::hide_before(mobile),
        "*******5678",
        "hide_before (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::hide_between(mobile),
        "136****5678",
        "hide_between (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::hide_after(mobile),
        "1361234****",
        "hide_after (对齐 Java)"
    );
}

/// 对齐 Java: `PhoneUtilTest.testSubString()` (行 61-67)
#[test]
fn test_sub_string() {
    let mobile = "13612345678";
    assert_eq!(
        PhoneUtil::sub_before(mobile),
        "136",
        "sub_before (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_between(mobile),
        "1234",
        "sub_between (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_after(mobile),
        "5678",
        "sub_after (对齐 Java)"
    );
}

/// 对齐 Java: `PhoneUtilTest.testNewTel()` (行 69-98)
#[test]
fn test_new_tel() {
    // 有效固定电话(含可选 - 分隔符)
    for s in [
        "010-12345678",
        "01012345678",
        "020-9999999",
        "0209999999",
        "0755-7654321",
        "07557654321",
    ] {
        assert!(
            PhoneUtil::is_tel(s),
            "is_tel({s:?}) 应 true (对齐 Java testNewTel 有效组)"
        );
    }
    // 无效
    for s in [
        "010 12345678",
        "A20-9999999",
        "0755-7654.321",
        "13619887123",
    ] {
        assert!(
            !PhoneUtil::is_tel(s),
            "is_tel({s:?}) 应 false (对齐 Java testNewTel 无效组)"
        );
    }

    // subTelBefore / subTelAfter
    // Java 测试对一个特定城市区号字符串返回确定值
    assert_eq!(
        PhoneUtil::sub_tel_before("010-12345678").unwrap(),
        "010",
        "sub_tel_before(\"010-12345678\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_before("01012345678").unwrap(),
        "010",
        "sub_tel_before(\"01012345678\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_after("010-12345678").unwrap(),
        "12345678",
        "sub_tel_after(\"010-12345678\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_after("01012345678").unwrap(),
        "12345678",
        "sub_tel_after(\"01012345678\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_before("0755-7654321").unwrap(),
        "0755",
        "sub_tel_before(\"0755-7654321\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_before("07557654321").unwrap(),
        "0755",
        "sub_tel_before(\"07557654321\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_after("0755-7654321").unwrap(),
        "7654321",
        "sub_tel_after(\"0755-7654321\") (对齐 Java)"
    );
    assert_eq!(
        PhoneUtil::sub_tel_after("07557654321").unwrap(),
        "7654321",
        "sub_tel_after(\"07557654321\") (对齐 Java)"
    );
}

/// 对齐 Java: `PhoneUtilTest.isTel400800Test()` (行 100-107)
#[test]
fn is_tel_400_800_test() {
    assert!(
        PhoneUtil::is_tel_400_800("400-860-8608"),
        "is_tel_400_800(\"400-860-8608\") (对齐 Java)"
    );
    assert!(
        PhoneUtil::is_tel_400_800("400-8608608"),
        "is_tel_400_800(\"400-8608608\") (对齐 Java)"
    );
}
// ── 扩展 phone_util 测试 ──

#[test]
fn is_mobile_valid_various() {
    assert!(PhoneUtil::is_mobile("13800138000"));
    assert!(PhoneUtil::is_mobile("15912345678"));
    assert!(PhoneUtil::is_mobile("18688889999"));
    assert!(PhoneUtil::is_mobile("17700001111"));
}

#[test]
fn is_mobile_invalid_various() {
    assert!(!PhoneUtil::is_mobile("12345678901"));
    assert!(!PhoneUtil::is_mobile("1380013800"));
    assert!(!PhoneUtil::is_mobile("138001380000"));
    assert!(!PhoneUtil::is_mobile("abc"));
    assert!(!PhoneUtil::is_mobile(""));
}

#[test]
fn is_mobile_hk_valid() {
    assert!(PhoneUtil::is_mobile_hk("91234567"));
    assert!(PhoneUtil::is_mobile_hk("61234567"));
}

#[test]
fn is_mobile_hk_invalid() {
    assert!(!PhoneUtil::is_mobile_hk("1234567"));
    assert!(!PhoneUtil::is_mobile_hk("123456789"));
}

#[test]
fn is_mobile_tw_valid() {
    assert!(PhoneUtil::is_mobile_tw("0912345678"));
    assert!(PhoneUtil::is_mobile_tw("0987654321"));
}

#[test]
fn is_mobile_tw_invalid() {
    assert!(!PhoneUtil::is_mobile_tw("0812345678"));
    assert!(!PhoneUtil::is_mobile_tw("091234567"));
}

#[test]
fn is_mobile_mo_valid() {
    assert!(PhoneUtil::is_mobile_mo("66123456"));
    assert!(PhoneUtil::is_mobile_mo("61234567"));
}

#[test]
fn is_tel_valid() {
    assert!(PhoneUtil::is_tel("01012345678"));
    assert!(PhoneUtil::is_tel("02112345678"));
    assert!(PhoneUtil::is_tel("07551234567"));
}
