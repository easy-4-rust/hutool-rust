//! `DesensitizedUtil` 对比验证测试 —— 对齐 Hutool `DesensitizedUtilTest`
//!
//! 对齐: `cn.hutool.core.util.DesensitizedUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/DesensitizedUtilTest.java
//!
//! # API 命名映射
//! | Java                                         | Rust                                            |
//! |----------------------------------------------|-------------------------------------------------|
//! | `desensitized(str, DesensitizedType.X)`      | `desensitized(Some(str), DesensitizedType::X)`  |
//! | `desensitized(null, DesensitizedType.X)`     | `desensitized(None, DesensitizedType::X)`       |
//! | `DesensitizedType.USER_ID`                   | `DesensitizedType::UserId`                      |
//! | `DesensitizedType.CLEAR_TO_EMPTY`            | `DesensitizedType::ClearToEmpty`                |
//! | `DesensitizedType.CLEAR_TO_NULL`             | `DesensitizedType::ClearToNull`                 |
//! | `DesensitizedType.CHINESE_NAME`              | `DesensitizedType::ChineseName`                 |
//! | `userId()`                                   | `user_id() -> i64`                              |
//! | `chineseName(str)`                           | `chinese_name(Some(str))`                       |
//! | `idCardNum(str, front, end)`                 | `id_card_num(Some(str), front, end)`            |
//! | `fixedPhone(str)`                            | `fixed_phone(Some(str))`                        |
//! | `mobilePhone(str)`                           | `mobile_phone(Some(str))`                       |
//! | `address(str, size)`                         | `address(Some(str), size)`                      |
//! | `email(str)`                                 | `email(Some(str))`                              |
//! | `password(str)`                              | `password(Some(str))`                           |
//! | `carLicense(str)`                            | `car_license(Some(str))`                        |
//! | `bankCard(str)`                              | `bank_card(Some(str)) -> Option<String>`        |
//! | `passport(str)`                              | `passport(Some(str)) -> Option<String>`         |
//! | `creditCode(str)`                            | `credit_code(Some(str)) -> Option<String>`      |

use hutool_core::{DesensitizedType, DesensitizedUtil};

/// 对齐 Java: `DesensitizedUtilTest.desensitizedTest()` (行 16-41)
///
/// 注:Java 测试在末尾重复了一组断言,Rust 版本仅保留一次。
#[test]
fn desensitized_test() {
    // USER_ID:数字 mod 10
    assert_eq!(
        DesensitizedUtil::desensitized(Some("100"), DesensitizedType::UserId),
        Some("0".to_owned()),
        "USER_ID (对齐 Java)"
    );
    // CLEAR_TO_EMPTY
    assert_eq!(
        DesensitizedUtil::desensitized(Some("100"), DesensitizedType::ClearToEmpty),
        Some(String::new()),
        "CLEAR_TO_EMPTY (对齐 Java)"
    );
    // CLEAR_TO_NULL (Java assertNull → Rust None)
    assert_eq!(
        DesensitizedUtil::desensitized(Some("100"), DesensitizedType::ClearToNull),
        None,
        "CLEAR_TO_NULL (对齐 Java assertNull)"
    );
    // 中文姓名
    assert_eq!(
        DesensitizedUtil::desensitized(Some("段正淳"), DesensitizedType::ChineseName),
        Some("段**".to_owned()),
        "CHINESE_NAME (对齐 Java)"
    );
    // 身份证
    assert_eq!(
        DesensitizedUtil::desensitized(Some("51343620000320711X"), DesensitizedType::IdCard),
        Some("5***************1X".to_owned()),
        "ID_CARD (对齐 Java)"
    );
    // 座机
    assert_eq!(
        DesensitizedUtil::desensitized(Some("09157518479"), DesensitizedType::FixedPhone),
        Some("0915*****79".to_owned()),
        "FIXED_PHONE (对齐 Java)"
    );
    // 手机
    assert_eq!(
        DesensitizedUtil::desensitized(Some("18049531999"), DesensitizedType::MobilePhone),
        Some("180****1999".to_owned()),
        "MOBILE_PHONE (对齐 Java)"
    );
    // 地址
    assert_eq!(
        DesensitizedUtil::desensitized(
            Some("北京市海淀区马连洼街道289号"),
            DesensitizedType::Address,
        ),
        Some("北京市海淀区马********".to_owned()),
        "ADDRESS (对齐 Java)"
    );
    // 邮箱
    assert_eq!(
        DesensitizedUtil::desensitized(
            Some("duandazhi-jack@gmail.com.cn"),
            DesensitizedType::Email,
        ),
        Some("d*************@gmail.com.cn".to_owned()),
        "EMAIL (对齐 Java)"
    );
    // 密码
    assert_eq!(
        DesensitizedUtil::desensitized(Some("1234567890"), DesensitizedType::Password),
        Some("**********".to_owned()),
        "PASSWORD (对齐 Java)"
    );
    // 银行卡
    assert_eq!(
        DesensitizedUtil::desensitized(Some("11011111222233333256"), DesensitizedType::BankCard),
        Some("1101 **** **** **** 3256".to_owned()),
        "BANK_CARD 第 1 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::desensitized(Some("6227880100100105123"), DesensitizedType::BankCard),
        Some("6227 **** **** **** 123".to_owned()),
        "BANK_CARD 第 2 组 (对齐 Java)"
    );
    // IPv4
    assert_eq!(
        DesensitizedUtil::desensitized(Some("192.168.1.1"), DesensitizedType::Ipv4),
        Some("192.*.*.*".to_owned()),
        "IPV4 (对齐 Java)"
    );
    // IPv6
    assert_eq!(
        DesensitizedUtil::desensitized(
            Some("2001:0db8:86a3:08d3:1319:8a2e:0370:7344"),
            DesensitizedType::Ipv6,
        ),
        Some("2001:*:*:*:*:*:*:*".to_owned()),
        "IPV6 (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.userIdTest()` (行 43-46)
#[test]
fn user_id_test() {
    assert_eq!(DesensitizedUtil::user_id(), 0_i64, "user_id (对齐 Java)");
}

/// 对齐 Java: `DesensitizedUtilTest.chineseNameTest()` (行 48-51)
#[test]
fn chinese_name_test() {
    assert_eq!(
        DesensitizedUtil::chinese_name(Some("段正淳")),
        "段**",
        "chinese_name (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.idCardNumTest()` (行 53-56)
#[test]
fn id_card_num_test() {
    assert_eq!(
        DesensitizedUtil::id_card_num(Some("51343620000320711X"), 1, 2),
        "5***************1X",
        "id_card_num(str, 1, 2) (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.fixedPhoneTest()` (行 58-61)
#[test]
fn fixed_phone_test() {
    assert_eq!(
        DesensitizedUtil::fixed_phone(Some("09157518479")),
        "0915*****79",
        "fixed_phone (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.mobilePhoneTest()` (行 63-66)
#[test]
fn mobile_phone_test() {
    assert_eq!(
        DesensitizedUtil::mobile_phone(Some("18049531999")),
        "180****1999",
        "mobile_phone (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.addressTest()` (行 68-74)
#[test]
fn address_test() {
    assert_eq!(
        DesensitizedUtil::address(Some("北京市海淀区马连洼街道289号"), 5),
        "北京市海淀区马连洼街*****",
        "address(str, 5) (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::address(Some("北京市海淀区马连洼街道289号"), 50),
        "***************",
        "address(str, 50) 全脱敏 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::address(Some("北京市海淀区马连洼街道289号"), 0),
        "北京市海淀区马连洼街道289号",
        "address(str, 0) 不脱敏 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::address(Some("北京市海淀区马连洼街道289号"), -1),
        "北京市海淀区马连洼街道289号",
        "address(str, -1) 不脱敏 (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.emailTest()` (行 76-81)
#[test]
fn email_test() {
    assert_eq!(
        DesensitizedUtil::email(Some("duandazhi@126.com")),
        "d********@126.com",
        "email 第 1 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::email(Some("duandazhi@gmail.com.cn")),
        "d********@gmail.com.cn",
        "email 第 2 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::email(Some("duandazhi-jack@gmail.com.cn")),
        "d*************@gmail.com.cn",
        "email 第 3 组 (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.passwordTest()` (行 83-86)
#[test]
fn password_test() {
    assert_eq!(
        DesensitizedUtil::password(Some("1234567890")),
        "**********",
        "password (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.carLicenseTest()` (行 88-95)
#[test]
fn car_license_test() {
    // Java carLicense(null) → "",Rust None → ""
    assert_eq!(
        DesensitizedUtil::car_license(None),
        "",
        "carLicense(null) (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::car_license(Some("")),
        "",
        "carLicense(\"\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::car_license(Some("苏D40000")),
        "苏D4***0",
        "carLicense(\"苏D40000\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::car_license(Some("陕A12345D")),
        "陕A1****D",
        "carLicense(\"陕A12345D\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::car_license(Some("京A123")),
        "京A123",
        "carLicense(\"京A123\") (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.bankCardTest()` (行 97-106)
#[test]
fn bank_card_test() {
    // Java bankCard(null) → null
    assert_eq!(
        DesensitizedUtil::bank_card(None),
        None,
        "bankCard(null) → None (对齐 Java assertNull)"
    );
    assert_eq!(
        DesensitizedUtil::bank_card(Some("")),
        Some(String::new()),
        "bankCard(\"\") → Some(\"\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 6789 9")),
        Some("1234 **** **** **** **** 9".to_owned()),
        "bankCard 第 1 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 6789 91")),
        Some("1234 **** **** **** **** 91".to_owned()),
        "bankCard 第 2 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 6789")),
        Some("1234 **** **** **** 6789".to_owned()),
        "bankCard 第 3 组 (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::bank_card(Some("1234 2222 3333 4444 678")),
        Some("1234 **** **** **** 678".to_owned()),
        "bankCard 第 4 组 (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.passportTest()` (行 108-114)
#[test]
fn passport_test() {
    assert_eq!(
        DesensitizedUtil::passport(None),
        None,
        "passport(null) → None (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::passport(Some("")),
        Some(String::new()),
        "passport(\"\") → Some(\"\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::passport(Some("EM1234567")),
        Some("EM*****67".to_owned()),
        "passport(\"EM1234567\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::passport(Some("3")),
        Some("*".to_owned()),
        "passport(\"3\") (对齐 Java)"
    );
}

/// 对齐 Java: `DesensitizedUtilTest.creditCodeTest()` (行 116-121)
#[test]
fn credit_code_test() {
    assert_eq!(
        DesensitizedUtil::credit_code(None),
        None,
        "creditCode(null) → None (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::credit_code(Some("")),
        Some(String::new()),
        "creditCode(\"\") → Some(\"\") (对齐 Java)"
    );
    assert_eq!(
        DesensitizedUtil::credit_code(Some("91110108MA01ABCDE7")),
        Some("9111**********CDE7".to_owned()),
        "creditCode(\"91110108MA01ABCDE7\") (对齐 Java)"
    );
}
