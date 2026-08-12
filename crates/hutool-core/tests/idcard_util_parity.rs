//! `IdcardUtil` 对比验证测试 —— 对齐 Hutool `IdcardUtilTest`
//!
//! 对齐: `cn.hutool.core.util.IdcardUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/IdcardUtilTest.java
//!
//! # API 命名映射
//! | Java                                | Rust                                    |
//! |-------------------------------------|-----------------------------------------|
//! | `isValidCard(str)`                  | `is_valid_card(str)`                    |
//! | `isValidCard18(str)`                | `is_valid_card_18(str)`                 |
//! | `isValidCard18(str, ignoreCase)`    | `is_valid_card_18_case(str, !strict)`   |
//! | `isValidHKCard(str)`                | `is_valid_hk_card(str)`                 |
//! | `isValidTWCard(str)`                | `is_valid_tw_card(str)`                 |
//! | `convert15To18(str)`                | `convert_15_to_18(str) -> Option<String>` |
//! | `convert18To15(str)`                | `convert_18_to_15(str) -> String`       |
//! | `getBirthByIdCard(str)`             | `get_birth_by_id_card(str) -> Result<Option<String>>` |
//! | `getProvinceByIdCard(str)`          | `get_province_by_id_card(str) -> Option<&'static str>` |
//! | `getCityCodeByIdCard(str)`          | `get_city_code_by_id_card(str) -> Option<String>` |
//! | `getDistrictCodeByIdCard(str)`      | `get_district_code_by_id_card(str) -> Option<String>` |
//! | `getGenderByIdCard(str)`            | `get_gender_by_id_card(str) -> Result<u8>` (1=男) |
//! | `getAgeByIdCard(str, Date)`         | `get_age_by_id_card_at(str, NaiveDate)` |
//! | `isValidCard10(str)`                | `is_valid_card_10(str) -> Option<Card10Info>` |

use chrono::NaiveDate;
use hutool_core::IdcardUtil;

const ID_18: &str = "321083197812162119";
const FOREIGN_ID_18: &str = "932682198501010017";
const ID_15: &str = "150102880730303";

/// 对齐 Java: `IdcardUtilTest.isValidCardTest()` (行 24-45)
#[test]
fn is_valid_card_test() {
    assert!(IdcardUtil::is_valid_card(ID_18), "ID_18 valid (对齐 Java)");
    assert!(IdcardUtil::is_valid_card(ID_15), "ID_15 valid (对齐 Java)");
    assert!(
        IdcardUtil::is_valid_card(FOREIGN_ID_18),
        "FOREIGN_ID_18 valid (对齐 Java)"
    );

    // 无效
    assert!(
        !IdcardUtil::is_valid_card("360198910283844"),
        "无效卡 (对齐 Java)"
    );
    // 生日无效
    assert!(
        !IdcardUtil::is_valid_card("201511221897205960"),
        "生日无效 (对齐 Java)"
    );
    assert!(
        !IdcardUtil::is_valid_card("815727834224151"),
        "生日无效 (对齐 Java)"
    );
}

/// 对齐 Java: `IdcardUtilTest.convert15To18Test()` (行 47-54)
#[test]
fn convert_15_to_18_test() {
    let r = IdcardUtil::convert_15_to_18(ID_15).unwrap();
    assert_eq!(
        r, "150102198807303035",
        "convert_15_to_18 ID_15 (对齐 Java)"
    );

    let r2 = IdcardUtil::convert_15_to_18("330102200403064").unwrap();
    assert_eq!(
        r2, "33010219200403064X",
        "convert_15_to_18 含 X (对齐 Java)"
    );
}

/// 对齐 Java: `IdcardUtilTest.convert18To15Test()` (行 56-60)
#[test]
fn convert_18_to_15_test() {
    let r = IdcardUtil::convert_18_to_15("150102198807303035");
    assert_eq!(r, ID_15, "convert_18_to_15 (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getAgeByIdCardTest()` (行 62-72)
#[test]
fn get_age_by_id_card_test() {
    let date = NaiveDate::from_ymd_opt(2017, 4, 10).unwrap();
    let age = IdcardUtil::get_age_by_id_card_at(ID_18, date).unwrap();
    assert_eq!(age, 38, "ID_18 age at 2017-04-10 (对齐 Java)");

    let age_f = IdcardUtil::get_age_by_id_card_at(FOREIGN_ID_18, date).unwrap();
    assert_eq!(age_f, 32, "FOREIGN_ID_18 age (对齐 Java)");

    let age_15 = IdcardUtil::get_age_by_id_card_at(ID_15, date).unwrap();
    assert_eq!(age_15, 28, "ID_15 age (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.issue3651Test()` (行 74-83)
#[test]
fn issue_3651_test() {
    let date = NaiveDate::from_ymd_opt(2014, 7, 11).unwrap();
    let age = IdcardUtil::get_age_by_id_card_at("321083200807112111", date).unwrap();
    assert_eq!(age, 5, "issue3651 第 1 组 (对齐 Java)");

    let date2 = NaiveDate::from_ymd_opt(2014, 7, 31).unwrap();
    let age2 = IdcardUtil::get_age_by_id_card_at("321083200807312113", date2).unwrap();
    assert_eq!(age2, 5, "issue3651 第 2 组 (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getBirthByIdCardTest()` (行 85-92)
#[test]
fn get_birth_by_id_card_test() {
    let birth = IdcardUtil::get_birth_by_id_card(ID_18).unwrap().unwrap();
    assert_eq!(birth, "19781216", "ID_18 birth (对齐 Java)");

    let birth2 = IdcardUtil::get_birth_by_id_card(ID_15).unwrap().unwrap();
    assert_eq!(birth2, "19880730", "ID_15 birth (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getProvinceByIdCardTest()` (行 94-101)
#[test]
fn get_province_by_id_card_test() {
    let province = IdcardUtil::get_province_by_id_card(ID_18).unwrap();
    assert_eq!(province, "江苏", "ID_18 province (对齐 Java)");

    let province2 = IdcardUtil::get_province_by_id_card(ID_15).unwrap();
    assert_eq!(province2, "内蒙古", "ID_15 province (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getCityCodeByIdCardTest()` (行 103-107)
#[test]
fn get_city_code_by_id_card_test() {
    let code = IdcardUtil::get_city_code_by_id_card(ID_18).unwrap();
    assert_eq!(code, "3210", "city code (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getDistrictCodeByIdCardTest()` (行 109-113)
#[test]
fn get_district_code_by_id_card_test() {
    let code = IdcardUtil::get_district_code_by_id_card(ID_18).unwrap();
    assert_eq!(code, "321083", "district code (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.getGenderByIdCardTest()` (行 115-119)
///
/// 注:Hutool `getGenderByIdCard` 返回 1=男,0=女;hutool 同。
#[test]
fn get_gender_by_id_card_test() {
    let gender = IdcardUtil::get_gender_by_id_card(ID_18).unwrap();
    assert_eq!(gender, 1, "ID_18 gender = 1 (男) (对齐 Java)");
}

/// 对齐 Java: `IdcardUtilTest.isValidCard18Test()` (行 121-153)
#[test]
fn is_valid_card_18_test() {
    assert!(
        !IdcardUtil::is_valid_card_18("3301022011022000D6"),
        "非法字符 D (对齐 Java)"
    );

    // 不忽略大小写情况下,X 严格校验必须大写
    // Java `isValidCard18(str, false)` 第二参数 ignoreCase=false → 严格
    // hutool `is_valid_card_18_case(str, ignore_case)`,严格时传 false
    assert!(
        !IdcardUtil::is_valid_card_18_case("33010219200403064x", false),
        "严格模式 小写 x 应 false (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_18_case("33010219200403064X", false),
        "严格模式 大写 X 应 true (对齐 Java)"
    );

    // 非严格校验下大小写皆可
    assert!(
        IdcardUtil::is_valid_card_18("33010219200403064x"),
        "非严格 小写 x (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_18("33010219200403064X"),
        "非严格 大写 X (对齐 Java)"
    );

    // 港澳台
    assert!(
        IdcardUtil::is_valid_card_18("81000019980902013X"),
        "香港人在大陆身份证 (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_18("820000200009100032"),
        "澳门人在大陆身份证 (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_18("830000200209060065"),
        "台湾人在大陆身份证 (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_18("932682198501010017"),
        "外国人永久居留 (对齐 Java)"
    );
}

/// 对齐 Java: `IdcardUtilTest.isValidHKCardIdTest()` (行 155-160)
#[test]
fn is_valid_hk_card_id_test() {
    assert!(
        IdcardUtil::is_valid_hk_card("P174468(6)"),
        "香港身份证 (对齐 Java)"
    );
}

/// 对齐 Java: `IdcardUtilTest.isValidTWCardIdTest()` (行 162-173)
#[test]
fn is_valid_tw_card_id_test() {
    assert!(
        IdcardUtil::is_valid_tw_card("B221690311"),
        "台湾身份证 有效 (对齐 Java)"
    );
    assert!(
        !IdcardUtil::is_valid_tw_card("M517086311"),
        "台湾身份证 无效 1 (对齐 Java)"
    );
    assert!(
        !IdcardUtil::is_valid_tw_card("B2216903112"),
        "台湾身份证 无效 2 (对齐 Java)"
    );
}

/// 对齐 Java: `IdcardUtilTest.issueI88YKMTest()` (行 175-178)
#[test]
fn issue_i88ykm_test() {
    assert!(
        IdcardUtil::is_valid_card("111111111111111"),
        "15 位全 1 (对齐 Java issueI88YKM)"
    );
}

/// 对齐 Java: `IdcardUtilTest.issueIAFOLITest()` (行 180-186)
#[test]
fn issue_iafoli_test() {
    let idcard = "H01487002";
    assert!(
        !IdcardUtil::is_valid_hk_card(idcard),
        "非 HK 卡 (对齐 Java)"
    );
    assert!(
        IdcardUtil::is_valid_card_10(idcard).is_none(),
        "Card10 应返回 None (对齐 Java assertNull)"
    );
    assert!(!IdcardUtil::is_valid_card(idcard), "非有效卡 (对齐 Java)");
}
// ── 扩展 idcard_util 测试 ──

#[test]
fn is_valid_card_18_valid() {
    // 合法的18位身份证号（校验码正确）
    assert!(IdcardUtil::is_valid_card_18("11010519491231002X"));
}

#[test]
fn is_valid_card_18_invalid_length() {
    assert!(!IdcardUtil::is_valid_card_18("1101051949123100"));
}

#[test]
fn is_valid_card_18_invalid_checksum() {
    assert!(!IdcardUtil::is_valid_card_18("110105194912310020"));
}

#[test]
fn is_valid_card_15_valid() {
    assert!(IdcardUtil::is_valid_card_15("110101490101001"));
}

#[test]
fn is_valid_card_15_invalid_length() {
    assert!(!IdcardUtil::is_valid_card_15("11010149010100"));
}

#[test]
fn is_valid_card_validates_both() {
    // 18位有效
    assert!(IdcardUtil::is_valid_card("11010519491231002X"));
    // 15位有效
    assert!(IdcardUtil::is_valid_card("110101490101001"));
    // 无效
    assert!(!IdcardUtil::is_valid_card("invalid"));
}

#[test]
fn get_birth_by_id_card_18() {
    let birth = IdcardUtil::get_birth_by_id_card("11010519491231002X").unwrap();
    assert_eq!(birth, Some("19491231".to_string()));
}

#[test]
fn get_birth_by_id_card_15() {
    let birth = IdcardUtil::get_birth_by_id_card("110101490101001").unwrap();
    assert_eq!(birth, Some("19490101".to_string()));
}

#[test]
fn get_year_by_id_card() {
    let year = IdcardUtil::get_year_by_id_card("11010519491231002X").unwrap();
    assert_eq!(year, Some(1949));
}

#[test]
fn get_month_by_id_card() {
    let month = IdcardUtil::get_month_by_id_card("11010519491231002X").unwrap();
    assert_eq!(month, Some(12));
}

#[test]
fn get_day_by_id_card() {
    let day = IdcardUtil::get_day_by_id_card("11010519491231002X").unwrap();
    assert_eq!(day, Some(31));
}

#[test]
fn get_gender_male() {
    // 倒数第二位为奇数 → 男
    let gender = IdcardUtil::get_gender_by_id_card("11010519491231001X").unwrap();
    assert_eq!(gender, 1);
}

#[test]
fn get_gender_female() {
    // 倒数第二位为偶数 → 女
    let gender = IdcardUtil::get_gender_by_id_card("11010519491231002X").unwrap();
    assert_eq!(gender, 0);
}

#[test]
fn get_province_by_id_card() {
    let province = IdcardUtil::get_province_by_id_card("11010519491231002X");
    assert!(province.is_some());
}

#[test]
fn get_province_code_by_id_card() {
    let code = IdcardUtil::get_province_code_by_id_card("11010519491231002X");
    assert_eq!(code, Some("11".to_string()));
}

#[test]
fn convert_15_to_18_roundtrip() {
    let id15 = "110101490101001";
    let id18 = IdcardUtil::convert_15_to_18(id15).unwrap();
    assert_eq!(id18.len(), 18);
    // 转回15位
    let id15_again = IdcardUtil::convert_18_to_15(&id18);
    assert_eq!(id15_again, id15);
}

#[test]
fn is_valid_card_10_tw() {
    // 台湾身份证号
    assert!(IdcardUtil::is_valid_tw_card("A123456789"));
}

#[test]
fn get_age_by_id_card() {
    let age = IdcardUtil::get_age_by_id_card("11010519491231002X").unwrap();
    assert!(age > 0);
}

#[test]
fn get_age_by_id_card_at_specific_date() {
    let comparison = chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let age = IdcardUtil::get_age_by_id_card_at("11010519491231002X", comparison).unwrap();
    assert_eq!(age, 50);
}

#[test]
fn idcard_struct_methods() {
    let idcard = IdcardUtil::is_valid_card_10("K123456(7)");
    assert!(idcard.is_some());
}
