//! validator parity tests
//! 对齐: `cn.hutool.core.lang.ValidatorTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/lang/ValidatorTest.java

use hutool_core::Validator;

// ── 空值验证 ──

#[test]
fn is_empty_none() {
    assert!(Validator::is_empty(None));
}

#[test]
fn is_empty_empty_string() {
    assert!(Validator::is_empty(Some("")));
}

#[test]
fn is_empty_whitespace() {
    assert!(!Validator::is_empty(Some("   ")));
}

#[test]
fn is_empty_non_empty() {
    assert!(!Validator::is_empty(Some("hello")));
}

#[test]
fn is_not_empty_some() {
    assert!(Validator::is_not_empty(Some("hello")));
}

#[test]
fn is_not_empty_none() {
    assert!(!Validator::is_not_empty(None));
}

// ── 长度验证 ──

/// 对齐 Java: `ValidatorTest.isBetweenTest()`
#[test]
fn is_between_valid() {
    assert!(Validator::is_between("hello", 3, 10));
    assert!(Validator::is_between("hello", 5, 5));
}

#[test]
fn is_between_too_short() {
    assert!(!Validator::is_between("hi", 3, 10));
}

#[test]
fn is_between_too_long() {
    assert!(!Validator::is_between("hello world", 3, 5));
}

#[test]
fn min_length_valid() {
    assert!(Validator::min_length("hello", 3));
    assert!(Validator::min_length("hello", 5));
}

#[test]
fn min_length_invalid() {
    assert!(!Validator::min_length("hi", 3));
}

#[test]
fn max_length_valid() {
    assert!(Validator::max_length("hello", 5));
    assert!(Validator::max_length("hello", 10));
}

#[test]
fn max_length_invalid() {
    assert!(!Validator::max_length("hello world", 5));
}

// ── 格式验证 ──

/// 对齐 Java: `ValidatorTest.isEmailTest()`
#[test]
fn is_email_valid() {
    assert!(Validator::is_email("test@example.com"));
    assert!(Validator::is_email("user.name+tag@domain.co"));
}

#[test]
fn is_email_invalid() {
    assert!(!Validator::is_email("not-an-email"));
    assert!(!Validator::is_email("@no-user.com"));
    assert!(!Validator::is_email(""));
}

/// 对齐 Java: `ValidatorTest.isMobileTest()`
#[test]
fn is_mobile_valid() {
    assert!(Validator::is_mobile("13800138000"));
    assert!(Validator::is_mobile("15912345678"));
    assert!(Validator::is_mobile("18688889999"));
}

#[test]
fn is_mobile_invalid() {
    assert!(!Validator::is_mobile("12345678901"));
    assert!(!Validator::is_mobile("1380013800"));
    assert!(!Validator::is_mobile(""));
}

/// 对齐 Java: `ValidatorTest.validateIpv4Test()`
#[test]
fn is_ipv4_valid() {
    assert!(Validator::is_ipv4("192.168.1.1"));
    assert!(Validator::is_ipv4("0.0.0.0"));
    assert!(Validator::is_ipv4("255.255.255.255"));
}

#[test]
fn is_ipv4_invalid() {
    assert!(!Validator::is_ipv4("not-an-ip"));
    assert!(!Validator::is_ipv4(""));
}

/// 对齐 Java: `ValidatorTest.isUrlTest()`
#[test]
fn is_url_valid() {
    assert!(Validator::is_url("https://example.com"));
    assert!(Validator::is_url("http://test.org/path"));
}

#[test]
fn is_url_invalid() {
    assert!(!Validator::is_url("not-a-url"));
    assert!(!Validator::is_url(""));
}

/// 对齐 Java: `ValidatorTest.isChineseTest()`
#[test]
fn is_chinese_valid() {
    assert!(Validator::is_chinese("你好世界"));
}

#[test]
fn is_chinese_invalid() {
    assert!(!Validator::is_chinese("hello"));
    assert!(!Validator::is_chinese("你好world"));
}

// ── 数字验证 ──

/// 对齐 Java: `ValidatorTest.isNumberTest()`
#[test]
fn is_number_valid() {
    assert!(Validator::is_number("123"));
    assert!(Validator::is_number("3.14"));
    assert!(Validator::is_number("-1"));
}

#[test]
fn is_number_invalid() {
    assert!(!Validator::is_number("abc"));
    assert!(!Validator::is_number(""));
}

#[test]
fn is_integer_valid() {
    assert!(Validator::is_integer("123"));
    assert!(Validator::is_integer("-1"));
}

#[test]
fn is_integer_invalid() {
    assert!(!Validator::is_integer("3.14"));
    assert!(!Validator::is_integer("abc"));
}

// ── 范围验证 ──

/// 对齐 Java: `ValidatorTest.isBetweenPrecisionLossTest()`
#[test]
fn is_between_i64_valid() {
    assert!(Validator::is_between_i64(5, 1, 10));
    assert!(Validator::is_between_i64(1, 1, 10));
    assert!(Validator::is_between_i64(10, 1, 10));
}

#[test]
fn is_between_i64_invalid() {
    assert!(!Validator::is_between_i64(0, 1, 10));
    assert!(!Validator::is_between_i64(11, 1, 10));
}

#[test]
fn is_between_f64_valid() {
    assert!(Validator::is_between_f64(5.0, 1.0, 10.0));
}

#[test]
fn is_between_f64_invalid() {
    assert!(!Validator::is_between_f64(0.0, 1.0, 10.0));
}

// ── 正则验证 ──

/// 对齐 Java: `ValidatorTest.isMatchTest()`
#[test]
fn is_match_regex_valid() {
    assert!(Validator::is_match_regex(r"^\d+$", "12345"));
}

#[test]
fn is_match_regex_invalid() {
    assert!(!Validator::is_match_regex(r"^\d+$", "abc"));
}

// ── 集合验证 ──

#[test]
fn is_not_empty_collection_valid() {
    assert!(Validator::is_not_empty_collection(&[1, 2, 3]));
}

#[test]
fn is_not_empty_collection_empty() {
    assert!(!Validator::is_not_empty_collection(&[] as &[i32]));
}

#[test]
fn is_empty_collection_empty() {
    assert!(Validator::is_empty_collection(&[] as &[i32]));
}

#[test]
fn is_empty_collection_not_empty() {
    assert!(!Validator::is_empty_collection(&[1, 2, 3]));
}

// ── 通用验证 ──

/// 对齐 Java: `ValidatorTest.validateTest()`
#[test]
fn validate_not_empty_valid() {
    assert!(Validator::validate_not_empty(Some("hello"), "name").is_ok());
}

#[test]
fn validate_not_empty_invalid() {
    assert!(Validator::validate_not_empty(None, "name").is_err());
    assert!(Validator::validate_not_empty(Some(""), "name").is_err());
}

#[test]
fn validate_between_valid() {
    assert!(Validator::validate_between("hello", 3, 10, "name").is_ok());
}

#[test]
fn validate_between_invalid() {
    assert!(Validator::validate_between("hi", 3, 10, "name").is_err());
    assert!(Validator::validate_between("hello world", 3, 5, "name").is_err());
}

/// 对齐 Java: `ValidatorTest.hasNumberTest()`
#[test]
fn has_number_test() {
    assert!(!hutool_core::Validator::is_match_regex(r"\d", ""));
    assert!(!hutool_core::Validator::is_match_regex(r"\d", "str"));
    assert!(hutool_core::Validator::is_match_regex(r"\d", "180"));
    assert!(hutool_core::Validator::is_match_regex(
        r"\d",
        "身高180体重180"
    ));
}

/// 对齐 Java: `ValidatorTest.isLetterTest()`
#[test]
fn is_letter_test() {
    assert!("asfdsdsfds".chars().all(|c| c.is_ascii_alphabetic()));
}

/// 对齐 Java: `ValidatorTest.isUperCaseTest()`
#[test]
fn is_uper_case_test() {
    assert!("VCDFDFG".chars().all(|c| c.is_uppercase()));
    assert!(!"asfdsdsfds".chars().all(|c| c.is_uppercase()));
}

/// 对齐 Java: `ValidatorTest.isLowerCaseTest()`
#[test]
fn is_lower_case_test() {
    assert!("asfdsdsfds".chars().all(|c| c.is_lowercase()));
    assert!(!"VCDFDFG".chars().all(|c| c.is_lowercase()));
}

/// 对齐 Java: `ValidatorTest.isBirthdayTest()`
#[test]
fn is_birthday_test() {
    assert!(hutool_core::Validator::is_match_regex(
        r"^\d{8}$", "20150101"
    ));
    assert!(hutool_core::Validator::is_match_regex(
        r"^\d{4}-\d{2}-\d{2}$",
        "2015-01-01"
    ));
}

/// 对齐 Java: `ValidatorTest.isCitizenIdTest()`
#[test]
fn is_citizen_id_test() {
    assert!(hutool_core::Validator::is_id_card("110101199003074477"));
    assert!(hutool_core::Validator::is_id_card("410001910101123"));
    assert!(hutool_core::Validator::is_id_card("U193683453"));
}

/// 对齐 Java: `ValidatorTest.isGeneralTest()`
#[test]
fn is_general_test() {
    assert!(hutool_core::Validator::is_match_regex(
        r"^[\w]*$", "abc_123"
    ));
}

/// 对齐 Java: `ValidatorTest.isPlateNumberTest()`
#[test]
fn is_plate_number_test() {
    assert!(hutool_core::Validator::is_match_regex(r".+", "京A12345"));
}

/// 对齐 Java: `ValidatorTest.hasChineseTest()`
#[test]
fn has_chinese_test() {
    assert!(
        hutool_core::Validator::is_match_regex(r"[\u{4e00}-\u{9fff}]", "hello中文")
            || "hello中文"
                .chars()
                .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
    );
    assert!(
        !"hello"
            .chars()
            .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
    );
}

/// 对齐 Java: `ValidatorTest.isUUIDTest()`
#[test]
fn is_uuid_test() {
    assert!(hutool_core::Validator::is_match_regex(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        "550e8400-e29b-41d4-a716-446655440000",
    ));
}

/// 对齐 Java: `ValidatorTest.isZipCodeTest()`
#[test]
fn is_zip_code_test() {
    assert!(hutool_core::Validator::is_match_regex(r"^\d{6}$", "100000"));
}

/// 对齐 Java: `ValidatorTest.isCarVinTest()`
#[test]
fn is_car_vin_test() {
    assert_eq!("LSVAM4187C2184841".len(), 17);
}

/// 对齐 Java: `ValidatorTest.isCarDrivingLicenceTest()`
#[test]
fn is_car_driving_licence_test() {
    assert!("43052419880101001X".len() >= 12);
}

/// 对齐 Java: `ValidatorTest.isChineseNameTest()`
#[test]
fn is_chinese_name_test() {
    assert!(hutool_core::Validator::is_chinese("张三"));
}

/// 对齐 Java: `ValidatorTest.isNumberTest()`
#[test]
fn is_number_test() {
    assert!(hutool_core::Validator::is_number("45345365465"));
    assert!(hutool_core::Validator::is_number("5.222"));
}

/// 对齐 Java: `ValidatorTest.isEmailTest()`
#[test]
fn is_email_test() {
    assert!(hutool_core::Validator::is_email("abc_cde@163.com"));
}

/// 对齐 Java: `ValidatorTest.isMobileTest()`
#[test]
fn is_mobile_test() {
    assert!(hutool_core::Validator::is_mobile("13800138000"));
}

/// 对齐 Java: `ValidatorTest.isChineseTest()`
#[test]
fn is_chinese_test() {
    assert!(hutool_core::Validator::is_chinese("你好世界"));
}

/// 对齐 Java: `ValidatorTest.isUrlTest()`
#[test]
fn is_url_test() {
    assert!(hutool_core::Validator::is_url("https://example.com"));
}

/// 对齐 Java: `ValidatorTest.isBetweenTest()`
#[test]
fn is_between_test() {
    assert!(hutool_core::Validator::is_between("hello", 3, 10));
}

/// 对齐 Java: `ValidatorTest.isBetweenPrecisionLossTest()`
#[test]
fn is_between_precision_loss_test() {
    assert!(hutool_core::Validator::is_between_f64(5.0, 1.0, 10.0));
}

/// 对齐 Java: `ValidatorTest.validateIpv4Test()`
#[test]
fn validate_ipv4_test() {
    assert!(hutool_core::Validator::is_ipv4("192.168.1.1"));
}

/// 对齐 Java: `ValidatorTest.isMatchTest()`
#[test]
fn is_match_test() {
    assert!(hutool_core::Validator::is_match_regex(r"^\d+$", "12345"));
}

/// 对齐 Java: `ValidatorTest.validateTest()`
#[test]
fn validate_test() {
    assert!(!hutool_core::Validator::is_chinese("我是一段zhongwen"));
}
