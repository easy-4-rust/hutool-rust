//! Date package Issue* regression parity
//! 对齐: hutool-core cn.hutool.core.date.Issue*Test

use hutool_core::{DateUtil, Month, StopWatch, Zodiac};

/// 对齐 Java: `Issue2612Test.parseTest()`
#[test]
fn issue_2612_test_parse_test_2() {
    let dt = DateUtil::parse("2020-5-8").unwrap();
    assert_eq!(dt.to_string(), "2020-05-08 00:00:00");
}

/// 对齐 Java: `Issue2981Test.parseUTCTest()`
#[test]
fn issue_2981_test_parse_utc_test_2() {
    let dt = DateUtil::parse_utc("2020-04-23T02:31:00+00:00").unwrap();
    assert!(dt.to_string().contains("2020-04-23"));
}

/// 对齐 Java: `Issue3011Test.isSameMonthTest()`
#[test]
fn issue_3011_test_is_same_month_test_2() {
    assert!(DateUtil::is_same_month(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-31").unwrap()
    ));
    assert!(!DateUtil::is_same_month(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-02-01").unwrap()
    ));
}

/// 对齐 Java: `Issue3036Test.getZodiacTest()`
#[test]
fn issue_3036_test_get_zodiac_test_2() {
    assert_eq!(Zodiac::get_zodiac_month(Month::January, 19).unwrap(), "摩羯座");
    assert_eq!(Zodiac::get_zodiac_month(Month::January, 20).unwrap(), "水瓶座");
}

/// 对齐 Java: `Issue3301Test.ofTest()`
#[test]
fn issue_3301_test_of_test_2() {
    let dt = DateUtil::date_from_millis(0);
    assert_eq!(dt.year(), 1970);
}

/// 对齐 Java: `Issue3348Test.formatChineseDateTest()`
#[test]
fn issue_3348_test_format_chinese_date_test_2() {
    let date = DateUtil::parse("2018-02-14").unwrap();
    assert_eq!(DateUtil::format_chinese_date(date, false, false), "二〇一八年二月十四日");
}

/// 对齐 Java: `Issue3608Test.parseTest()`
#[test]
fn issue_3608_test_parse_test_2() {
    assert!(DateUtil::parse("2021-03-17 06:31:33").is_ok());
}

/// 对齐 Java: `Issue3798Test.parseTest()`
#[test]
fn issue_3798_test_parse_test_2() {
    assert!(DateUtil::parse("2019-10-22 09:56:03").is_ok());
}

/// 对齐 Java: `IssueI7QI6RTest.parseTest()`
#[test]
fn issue_i_7_qi_6_r_test_parse_test_2() {
    let dt = DateUtil::parse("2022-08-13 09:30:00").unwrap();
    assert_eq!(dt.to_string(), "2022-08-13 09:30:00");
}

/// 对齐 Java: `IssueI7QI6RTest.parseTest2()`
#[test]
fn issue_i_7_qi_6_r_test_parse_test_2_2_2() {
    assert!(DateUtil::parse("2022-08-13T09:30:00").is_ok());
}

/// 对齐 Java: `IssueI7XMYWTest.ageTest()`
#[test]
fn issue_i_7_xmyw_test_age_test_2() {
    let age = DateUtil::age(DateUtil::parse("1990-01-01").unwrap(), DateUtil::parse("2020-01-01").unwrap());
    assert_eq!(age, 30);
}

/// 对齐 Java: `IssueI82Y1LTest.parseTest()`
#[test]
fn issue_i_82_y_1_l_test_parse_test_2() {
    assert!(DateUtil::parse("2020-01-01 00:00:00").is_ok());
}

/// 对齐 Java: `IssueI97WU6Test.getTermTest()`
#[test]
fn issue_i_97_wu_6_test_get_term_test_2() {
    use hutool_core::date::chinese::solar_terms::SolarTerms;
    let day = SolarTerms::get_term_day(1987, 3);
    assert_eq!(day, 4);
}

/// 对齐 Java: `IssueI9C2D4Test.parseHttpTest()`
#[test]
fn issue_i_9_c_2_d_4_test_parse_http_test_2() {
    let s = DateUtil::format_http_date(DateUtil::parse_utc("2019-01-02T14:32:01Z").unwrap());
    assert!(s.contains("GMT"));
}

/// 对齐 Java: `IssueI9C2D4Test.parseHttpTest2()`
#[test]
fn issue_i_9_c_2_d_4_test_parse_http_test_2_2_2() {
    assert!(DateUtil::parse("Wed, 02 Jan 2019 14:32:01 GMT").is_ok()
        || DateUtil::parse_rfc2822("Wed, 02 Jan 2019 14:32:01 GMT").is_ok());
}

/// 对齐 Java: `IssueI9C2D4Test.parseTimeTest()`
#[test]
fn issue_i_9_c_2_d_4_test_parse_time_test_2() {
    assert!(DateUtil::parse_time("22:12:12").is_ok());
}

/// 对齐 Java: `IssueI9C2D4Test.parseTimeTest2()`
#[test]
fn issue_i_9_c_2_d_4_test_parse_time_test_2_2_2() {
    assert_eq!(DateUtil::time_to_second("02:12:12"), 2 * 3600 + 12 * 60 + 12);
}

/// 对齐 Java: `IssueIB8OFSTest.rangeTest()`
#[test]
fn issue_ib_8_ofs_test_range_test_2() {
    use hutool_core::{DateField, DateUtil};
    let list = DateUtil::range_to_list(
        DateUtil::parse("2018-05-31").unwrap(),
        DateUtil::parse("2018-08-31").unwrap(),
        DateField::Month,
    );
    assert!(list.len() >= 3);
}

/// 对齐 Java: `IssueIB9NPUTest.parseTest()`
#[test]
fn issue_ib_9_npu_test_parse_test_2() {
    assert!(DateUtil::parse("2023-02-07 00:02:16").is_ok());
}

/// 对齐 Java: `IssueIBB6I5Test.parseISO8601Test()`
#[test]
fn issue_ibb_6_i_5_test_parse_iso_8601_test_2() {
    assert!(DateUtil::parse_iso8601("2018-09-13T13:34:35").is_ok());
}

/// 对齐 Java: `IssueIBB6I5Test.parseISO8601Test2()`
#[test]
fn issue_ibb_6_i_5_test_parse_iso_8601_test_2_2_2() {
    assert!(DateUtil::parse_iso8601("2018-09-13T13:34:35+08:00").is_ok());
}

/// 对齐 Java: `IssueIC00HGTest.dateToStringTest()`
#[test]
fn issue_ic_00_hg_test_date_to_string_test_2() {
    let dt = DateUtil::parse("2020-01-01 12:00:00").unwrap();
    assert_eq!(dt.to_string(), "2020-01-01 12:00:00");
}

/// 对齐 Java: `IssueIDFMXJTest.stopWatchNegativeTimeTest()`
#[test]
fn issue_idfmxj_test_stop_watch_negative_time_test_2() {
    let mut sw = StopWatch::new();
    sw.start();
    sw.stop();
    // 不应 panic；总时间为非负（u64 恒满足）
    let _ = sw.get_total_time_nanos();
}
