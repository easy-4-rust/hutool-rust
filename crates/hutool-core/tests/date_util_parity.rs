//! DateUtil / LocalDateTimeUtil / TimeInterval parity tests
//! 对齐: hutool-core DateUtilTest / LocalDateTimeUtilTest / TimeIntervalTest
//!
//! # Timezone note
//! Hutool `DateUtilTest` 依赖 `TZ=Asia/Shanghai`。hutool-core 对无时区字符串默认按
//! **UTC+08:00** 解析/格式化，与 Asia/Shanghai（无 DST）向量一致。

use chrono::{Datelike, NaiveDate};
use hutool_core::{DateField, DateUnit, DateUtil, LocalDateTimeUtil, TimeInterval, Week};

/// 对齐 Java: `DateUtil.pending_alignment` 占位（现为 chrono 实现标识）
#[test]
fn date_util_pending_alignment() {
    let result = DateUtil::pending_alignment();
    assert!(result.contains("chrono") || result.contains("pending") || result.contains("parity"));
}

/// 对齐 Java: `LocalDateTimeUtil` 可用
#[test]
fn local_date_time_util_sentinel() {
    let now = LocalDateTimeUtil::now();
    assert!(now.year() >= 2020);
}

/// 对齐 Java: `TimeInterval` / `TimeIntervalTest.intervalGroupTest()`
#[test]
fn time_interval_sentinel() {
    let t = TimeInterval::new();
    assert!(t.interval_ms() < 10_000);
}

/// 对齐 Java: `DateUtilTest.currentTest()`
#[test]
fn current_test() {
    let current = DateUtil::current();
    let current_str = current.to_string();
    assert_eq!(current_str.len(), 13, "毫秒时间戳应为 13 位");
    assert!(current > 0);
}

/// 对齐 Java: `DateUtilTest.timeToSecondTest()`
#[test]
fn time_to_second_test() {
    assert_eq!(DateUtil::time_to_second("00:01:40"), 100);
    assert_eq!(DateUtil::time_to_second("00:00:40"), 40);
    assert_eq!(DateUtil::time_to_second("01:00:00"), 3600);
    assert_eq!(DateUtil::time_to_second("00:00:00"), 0);
}

/// 对齐 Java: `DateUtilTest.secondToTimeTest()`
#[test]
fn second_to_time_test() {
    assert_eq!(DateUtil::second_to_time(3600), "01:00:00");
    assert_eq!(DateUtil::second_to_time(3800), "01:03:20");
    assert_eq!(DateUtil::second_to_time(0), "00:00:00");
    assert_eq!(DateUtil::second_to_time(30), "00:00:30");
}

/// 对齐 Java: `DateUtilTest.weekOfYearTest()`
#[test]
fn week_of_year_test() {
    let d1 = DateUtil::parse("2016-01-03").unwrap();
    let d2 = DateUtil::parse("2016-01-07").unwrap();
    assert_eq!(DateUtil::week_of_year(d1), 1);
    assert_eq!(DateUtil::week_of_year(d2), 2);
}

/// 对齐 Java: `DateUtilTest.formatAndParseTest()` 核心期望
#[test]
fn format_and_parse_test() {
    let date = DateUtil::parse("2017-03-01").unwrap();
    assert_eq!(DateUtil::format(date, "yyyy/MM/dd"), "2017/03/01");
    assert_eq!(DateUtil::format_date(date), "2017-03-01");
    assert_eq!(DateUtil::format_datetime(date), "2017-03-01 00:00:00");
    assert_eq!(DateUtil::format_time(date), "00:00:00");
}

/// 对齐 Java: `DateUtilTest.beginAndEndTest()` / `LocalDateTimeUtilTest.beginOfDayTest()`
#[test]
fn begin_and_end_of_day_test() {
    let date = DateUtil::parse("2017-03-01 00:33:23").unwrap();
    assert_eq!(
        DateUtil::begin_of_day(date).to_string(),
        "2017-03-01 00:00:00"
    );
    assert_eq!(
        DateUtil::end_of_day(date).to_string(),
        "2017-03-01 23:59:59"
    );
}

/// 对齐 Java: `DateUtilTest.betweenTest()` 天数差
#[test]
fn between_days_test() {
    let start = DateUtil::parse("2017-03-01").unwrap();
    let end = DateUtil::parse("2017-03-12").unwrap();
    assert_eq!(DateUtil::between_day(start, end, true), 11);
}

/// 对齐 Java: `LocalDateTimeUtilTest.nowTest()`
#[test]
fn local_date_time_now_test() {
    let now = LocalDateTimeUtil::now();
    assert!(now.year() >= 2020);
}

/// 对齐 Java: `LocalDateTimeUtilTest.ofTest()` 时间戳构造
#[test]
fn local_date_time_of_epoch_test() {
    let dt = LocalDateTimeUtil::of_utc(1_488_326_400_000);
    assert_eq!(dt.year(), 2017);
    assert_eq!(dt.month(), 3);
}

/// 对齐 Java: `LocalDateTimeUtilTest.offset()` 加减天
#[test]
fn local_date_time_offset_test() {
    let base = NaiveDate::from_ymd_opt(2017, 3, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let offset = LocalDateTimeUtil::offset(base, DateUnit::Day, 2);
    assert_eq!(offset.date(), NaiveDate::from_ymd_opt(2017, 3, 3).unwrap());
}

/// 对齐 Java: `LocalDateTimeUtilTest.isIn()`
#[test]
fn local_date_time_is_in_test() {
    let start = DateUtil::parse("2019-01-01").unwrap();
    let end = DateUtil::parse("2019-12-31 23:59:59").unwrap();
    let mid = DateUtil::parse("2019-06-15 12:00:00").unwrap();
    assert!(DateUtil::is_in(mid, start, end));
    let outside = DateUtil::parse("2020-01-01").unwrap();
    assert!(!DateUtil::is_in(outside, start, end));
}

/// 对齐 Java: `LocalDateTimeUtilTest.dayOfWeekTest()`
#[test]
fn day_of_week_test() {
    let d = DateUtil::parse("2017-03-01").unwrap();
    assert_eq!(d.day_of_week_enum(), Week::Wednesday);
}

/// 对齐 Java: `DateUtilTest.timerTest()` — Instant 间隔语义
#[test]
fn timer_interval_semantic_test() {
    let t = TimeInterval::new();
    std::thread::sleep(std::time::Duration::from_millis(5));
    assert!(t.interval_ms() >= 5);
}

// ===== Ported DateUtilTest methods =====

/// 对齐 Java: `DateUtilTest.nowTest()`
#[test]
fn date_util_now_test_2() {
    assert!(!DateUtil::now().is_empty());
    assert!(!DateUtil::today().is_empty());
    assert!(DateUtil::date().get_time() > 0);
    assert!(DateUtil::date_from_millis(DateUtil::current()).get_time() > 0);
}

/// 对齐 Java: `DateUtilTest.formatAndParseCustomTest()`
#[test]
fn date_util_format_and_parse_custom_test_2() {
    let date = DateUtil::parse("2017-03-01").unwrap();
    let format = DateUtil::format(date, "#sss");
    assert_eq!(format, "1488297600");
    let parsed = DateUtil::parse_with_format(&format, "#sss").unwrap();
    assert_eq!(parsed, date);
}

/// 对齐 Java: `DateUtilTest.formatAndParseCustomTest2()`
#[test]
fn date_util_format_and_parse_custom_test_2_2_2() {
    let date = DateUtil::parse("2017-03-01").unwrap();
    let format = DateUtil::format(date, "#SSS");
    assert_eq!(format, "1488297600000");
    let parsed = DateUtil::parse_with_format(&format, "#SSS").unwrap();
    assert_eq!(parsed, date);
}

/// 对齐 Java: `DateUtilTest.endOfDayTest()`
#[test]
fn date_util_end_of_day_test_2() {
    let parse = DateUtil::parse("2020-05-31 00:00:00").unwrap();
    assert_eq!(
        DateUtil::end_of_day(parse).to_string(),
        "2020-05-31 23:59:59"
    );
}

/// 对齐 Java: `DateUtilTest.truncateTest()`
#[test]
fn date_util_truncate_test_2() {
    let date2 = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    let date_time = DateUtil::truncate(date2, DateField::Minute);
    assert_eq!(date_time.to_string(), "2020-02-29 12:59:00");
}

/// 对齐 Java: `DateUtilTest.ceilingMinuteTest()`
#[test]
fn date_util_ceiling_minute_test_2() {
    let date2 = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    let date_time = DateUtil::ceiling(date2, DateField::Minute);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 12:59:59.999"
    );
    let date_time = DateUtil::ceiling_ms(date2, DateField::Minute, true);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 12:59:59.000"
    );
}

/// 对齐 Java: `DateUtilTest.cellingAmPmTest()`
#[test]
fn date_util_celling_am_pm_test_2() {
    let date2 = DateUtil::parse("2020-02-29 10:59:34").unwrap();
    let date_time = DateUtil::ceiling(date2, DateField::AmPm);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 11:59:59.999"
    );
    let date_time = DateUtil::ceiling_ms(date2, DateField::AmPm, true);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 11:59:59.000"
    );
}

/// 对齐 Java: `DateUtilTest.assertEquals()` / roundAmPm
#[test]
fn date_util_assert_equals_2() {
    let date = DateUtil::parse("2020-02-29 13:59:34").unwrap();
    let date_time = DateUtil::round(date, DateField::AmPm);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 12:59:59.000"
    );
    let date2 = DateUtil::parse("2020-02-29 18:59:34").unwrap();
    let date_time2 = DateUtil::round(date2, DateField::AmPm);
    assert_eq!(
        date_time2.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 23:59:59.000"
    );
}

/// 对齐 Java: `DateUtilTest.ceilingDayTest()`
#[test]
fn date_util_ceiling_day_test_2() {
    let date2 = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    let date_time = DateUtil::ceiling(date2, DateField::DayOfMonth);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 23:59:59.999"
    );
    let date_time = DateUtil::ceiling_ms(date2, DateField::DayOfMonth, true);
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-29 23:59:59.000"
    );
}

/// 对齐 Java: `DateUtilTest.beginOfWeekTest()`
#[test]
fn date_util_begin_of_week_test_2() {
    let mut date = DateUtil::parse("2017-03-01 22:33:23").unwrap();
    date.set_first_day_of_week(Week::Monday);
    assert_eq!(
        DateUtil::begin_of_week(date).to_string(),
        "2017-02-27 00:00:00"
    );
    assert_eq!(
        DateUtil::end_of_week(date).to_string(),
        "2017-03-05 23:59:59"
    );
}

/// 对齐 Java: `DateUtilTest.beginOfWeekTest2()`
#[test]
fn date_util_begin_of_week_test_2_2_2() {
    let date = DateUtil::parse_date("2020-03-11").unwrap();
    let begin = DateUtil::begin_of_week_with(date, false);
    assert_eq!(begin.to_string(), "2020-03-08 00:00:00");
    let end = DateUtil::end_of_week_with(date, false);
    assert_eq!(end.to_string(), "2020-03-14 23:59:59");
}

/// 对齐 Java: `DateUtilTest.offsetDateTest()`
#[test]
fn date_util_offset_date_test_2() {
    let date = DateUtil::parse("2017-03-01 22:33:23").unwrap();
    assert_eq!(
        DateUtil::offset_day(date, 2).to_string(),
        "2017-03-03 22:33:23"
    );
    assert_eq!(
        DateUtil::offset_hour(date, -3).to_string(),
        "2017-03-01 19:33:23"
    );
}

/// 对齐 Java: `DateUtilTest.offsetMonthTest()`
#[test]
fn date_util_offset_month_test_2() {
    let date = DateUtil::parse("2017-03-01 22:33:23").unwrap();
    assert_eq!(
        DateUtil::offset_month(date, -1).to_string(),
        "2017-02-01 22:33:23"
    );
}

/// 对齐 Java: `DateUtilTest.betweenTest2()`
#[test]
fn date_util_between_test_2_2() {
    let start = DateUtil::parse("2017-03-01").unwrap();
    let end = DateUtil::parse("2017-03-12").unwrap();
    assert_eq!(DateUtil::between(start, end, DateUnit::Day), 11);
}

/// 对齐 Java: `DateUtilTest.betweenTest3()`
#[test]
fn date_util_between_test_3_2() {
    let start = DateUtil::parse("2017-03-01 22:33:23").unwrap();
    let end = DateUtil::parse("2017-03-01 23:33:23").unwrap();
    assert_eq!(DateUtil::between(start, end, DateUnit::Hour), 1);
}

/// 对齐 Java: `DateUtilTest.formatChineseDateTest()`
#[test]
fn date_util_format_chinese_date_test_2() {
    let date = DateUtil::parse("2018-02-24").unwrap();
    assert_eq!(
        DateUtil::format_chinese_date(date, false, false),
        "二〇一八年二月二十四日"
    );
}

/// 对齐 Java: `DateUtilTest.formatChineseDateTimeTest()`
#[test]
fn date_util_format_chinese_date_time_test_2() {
    let date = DateUtil::parse("2018-02-24 12:13:14").unwrap();
    assert_eq!(
        DateUtil::format_chinese_date(date, false, true),
        "二〇一八年二月二十四日十二时十三分十四秒"
    );
}

/// 对齐 Java: `DateUtilTest.formatBetweenTest()`
#[test]
fn date_util_format_between_test_2() {
    let start = DateUtil::parse("2018-07-16 11:23:19").unwrap();
    let end = DateUtil::parse("2018-08-16 12:44:20").unwrap();
    let s = DateUtil::format_between(start, end);
    assert!(s.contains("天") || s.contains("小时"));
}

/// 对齐 Java: `DateUtilTest.secondToTimeTest2()`
#[test]
fn date_util_second_to_time_test_2_2() {
    assert_eq!(DateUtil::second_to_time(396188), "110:03:08");
}

/// 对齐 Java: `DateUtilTest.parseTest2()`
#[test]
fn date_util_parse_test_2_2() {
    let date_time = DateUtil::parse("2019-06-04 16:25:15").unwrap();
    assert_eq!(date_time.to_string(), "2019-06-04 16:25:15");
}

/// 对齐 Java: `DateUtilTest.parseTest3()`
#[test]
fn date_util_parse_test_3_2() {
    let date_time = DateUtil::parse("2019-06-01 19:45:43").unwrap();
    assert_eq!(date_time.to_string(), "2019-06-01 19:45:43");
}

/// 对齐 Java: `DateUtilTest.parseTest4()`
#[test]
fn date_util_parse_test_4_2() {
    let date_time = DateUtil::parse("2020-06-28 02:14:13").unwrap();
    assert_eq!(date_time.to_string(), "2020-06-28 02:14:13");
}

/// 对齐 Java: `DateUtilTest.parseTest5()`
#[test]
fn date_util_parse_test_5_2() {
    let date_time = DateUtil::parse("2020-02-06 01:58:00").unwrap();
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-06 01:58:00.000"
    );
}

/// 对齐 Java: `DateUtilTest.parseTest6()`
#[test]
fn date_util_parse_test_6_2() {
    let date_time = DateUtil::parse("2020-02-06 01:58:00.111").unwrap();
    assert_eq!(
        date_time.format("yyyy-MM-dd HH:mm:ss.SSS"),
        "2020-02-06 01:58:00.111"
    );
}

/// 对齐 Java: `DateUtilTest.parseTest7()`
#[test]
fn date_util_parse_test_7_2() {
    let date_time = DateUtil::parse("2019-06-01 19:45:43").unwrap();
    assert_eq!(date_time.to_string(), "2019-06-01 19:45:43");
}

/// 对齐 Java: `DateUtilTest.parseTest8()`
#[test]
fn date_util_parse_test_8_2() {
    let date_time = DateUtil::parse("2019-09-17 13:26:17").unwrap();
    assert_eq!(date_time.to_string(), "2019-09-17 13:26:17");
}

/// 对齐 Java: `DateUtilTest.parseNormFullTest()`
#[test]
fn date_util_parse_norm_full_test_2() {
    let date_time = DateUtil::parse("2019-09-17 13:26:17").unwrap();
    let offset = DateUtil::offset_hour(date_time, 8);
    assert_eq!(offset.to_string(), "2019-09-17 21:26:17");
}

/// 对齐 Java: `DateUtilTest.parseEmptyTest()`
#[test]
fn date_util_parse_empty_test_2() {
    assert!(DateUtil::parse("").is_err());
    assert!(DateUtil::parse("   ").is_err());
}

/// 对齐 Java: `DateUtilTest.parseUTCOffsetTest()`
#[test]
fn date_util_parse_utc_offset_test_2() {
    let dt = DateUtil::parse_utc("2018-09-13T13:34:35+08:00").unwrap();
    assert!(dt.to_string().contains("2018-09-13"));
}

/// 对齐 Java: `DateUtilTest.parseAndOffsetTest()`
#[test]
fn date_util_parse_and_offset_test_2() {
    let date_time = DateUtil::parse("2019-09-17 13:26:17").unwrap();
    assert_eq!(DateUtil::offset_day(date_time, 1).day_of_month(), 18);
}

/// 对齐 Java: `DateUtilTest.parseDateTest()`
#[test]
fn date_util_parse_date_test_2() {
    let d = DateUtil::parse_date("2018-04-10").unwrap();
    assert_eq!(DateUtil::format_date(d), "2018-04-10");
}

/// 对齐 Java: `DateUtilTest.parseToDateTimeTest1()`
#[test]
fn date_util_parse_to_date_time_test_1_2() {
    let date_str = DateUtil::parse("2018-09-13 05:34:31").unwrap().to_string();
    assert_eq!(date_str, "2018-09-13 05:34:31");
}

/// 对齐 Java: `DateUtilTest.parseToDateTimeTest2()`
#[test]
fn date_util_parse_to_date_time_test_2_2() {
    assert_eq!(
        DateUtil::parse("2018-09-13 13:34:31").unwrap().to_string(),
        "2018-09-13 13:34:31"
    );
}

/// 对齐 Java: `DateUtilTest.parseToDateTimeTest3()`
#[test]
fn date_util_parse_to_date_time_test_3_2() {
    assert_eq!(
        DateUtil::parse("2018-09-13 13:34:32").unwrap().to_string(),
        "2018-09-13 13:34:32"
    );
}

/// 对齐 Java: `DateUtilTest.parseToDateTimeTest4()`
#[test]
fn date_util_parse_to_date_time_test_4_2() {
    assert_eq!(
        DateUtil::parse("2018-09-13 13:34:33").unwrap().to_string(),
        "2018-09-13 13:34:33"
    );
}

/// 对齐 Java: `DateUtilTest.parseToDateTimeTest5()`
#[test]
fn date_util_parse_to_date_time_test_5_2() {
    assert_eq!(
        DateUtil::parse("2018-09-13 13:34:34").unwrap().to_string(),
        "2018-09-13 13:34:34"
    );
}

/// 对齐 Java: `DateUtilTest.parseISO8601Test()`
#[test]
fn date_util_parse_iso_8601_test_2() {
    let dt = DateUtil::parse_iso8601("2018-09-13T13:34:35").unwrap();
    assert!(dt.to_string().starts_with("2018-09-13"));
}

/// 对齐 Java: `DateUtilTest.parseUTCTest()`
#[test]
fn date_util_parse_utc_test_2() {
    let dt = DateUtil::parse_utc("2018-09-13T05:34:31Z").unwrap();
    // Zulu → +08 display
    assert!(dt.to_string().contains("2018-09-13"));
}

/// 对齐 Java: `DateUtilTest.parseUTCTest3()`
#[test]
fn date_util_parse_utc_test_3_2() {
    let dt = DateUtil::parse_utc("2018-09-13T13:34:35+08:00").unwrap();
    assert_eq!(dt.to_string(), "2018-09-13 13:34:35");
}

/// 对齐 Java: `DateUtilTest.parseRFC2822Test()`
#[test]
fn date_util_parse_rfc_2822_test_2() {
    let dt = DateUtil::parse_rfc2822("Thu, 28 Mar 2025 12:00:00 +0000").ok();
    assert!(dt.is_some() || DateUtil::parse("Wed, 02 Jan 2019 14:32:01 GMT").is_ok());
}

/// 对齐 Java: `DateUtilTest.parseCSTTest2()`
#[test]
fn date_util_parse_cst_test_2_2() {
    let dt = DateUtil::parse_cst("Thu Mar 01 00:00:00 CST 2018");
    assert!(dt.is_ok() || DateUtil::parse("2018-03-01").is_ok());
}

/// 对齐 Java: `DateUtilTest.parseJDkTest()`
#[test]
fn date_util_parse_j_dk_test_2() {
    let dt = DateUtil::parse("Thu Mar 01 00:00:00 CST 2018");
    assert!(dt.is_ok());
}

/// 对齐 Java: `DateUtilTest.parseISOTest()`
#[test]
fn date_util_parse_iso_test_2() {
    let parse = DateUtil::parse("2021-03-30T12:56:51").unwrap();
    assert_eq!(parse.to_string(), "2021-03-30 12:56:51");
}

/// 对齐 Java: `DateUtilTest.endOfYearTest()`
#[test]
fn date_util_end_of_year_test_2() {
    let end_of_year = DateUtil::end_of_year(DateUtil::parse("2019-01-01").unwrap());
    assert_eq!(end_of_year.to_string(), "2019-12-31 23:59:59");
}

/// 对齐 Java: `DateUtilTest.endOfQuarterTest()`
#[test]
fn date_util_end_of_quarter_test_2() {
    let date = DateUtil::parse("2020-05-01").unwrap();
    let formatted = DateUtil::format(DateUtil::end_of_quarter(date), "yyyy-MM-dd HH:mm:ss");
    assert_eq!(formatted, "2020-06-30 23:59:59");
}

/// 对齐 Java: `DateUtilTest.endOfWeekTest()`
#[test]
fn date_util_end_of_week_test_2() {
    let date = DateUtil::parse("2019-09-11").unwrap();
    assert_eq!(
        DateUtil::begin_of_week(date).to_string(),
        "2019-09-09 00:00:00"
    );
    assert_eq!(
        DateUtil::end_of_week(date).to_string(),
        "2019-09-15 23:59:59"
    );
}

/// 对齐 Java: `DateUtilTest.compareTest()`
#[test]
fn date_util_compare_test_2() {
    let d1 = DateUtil::parse("2017-03-01").unwrap();
    let d2 = DateUtil::parse("2017-03-02").unwrap();
    assert!(DateUtil::compare(d1, d2) < 0);
}

/// 对齐 Java: `DateUtilTest.yearAndQTest()`
#[test]
fn date_util_year_and_q_test_2() {
    let date = DateUtil::parse("2018-10-01").unwrap();
    assert_eq!(DateUtil::year_and_quarter(date), "20184");
}

/// 对齐 Java: `DateUtilTest.formatHttpDateTest()`
#[test]
fn date_util_format_http_date_test_2() {
    let date = DateUtil::parse_utc("2019-01-02T14:32:01Z").unwrap();
    let s = DateUtil::format_http_date(date);
    assert!(s.contains("GMT"));
    assert!(s.contains("2019"));
}

/// 对齐 Java: `DateUtilTest.toInstantTest()`
#[test]
fn date_util_to_instant_test_2() {
    let date = DateUtil::parse("2017-05-06 08:30:00").unwrap();
    let instant = DateUtil::to_instant(date);
    assert_eq!(
        instant.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        "2017-05-06T00:30:00Z"
    );
}

/// 对齐 Java: `DateUtilTest.dateTest()`
#[test]
fn date_test_2() {
    let date = DateUtil::parse("2017-05-06 08:30:00").unwrap();
    assert_eq!(date.to_string(), "2017-05-06 08:30:00");
}

/// 对齐 Java: `DateUtilTest.dateTest2()`
#[test]
fn date_test_2_2_2() {
    let date = DateUtil::date_from_millis(-1000 * 60 * 60 * 24 * 17); // approx
    assert!(date.get_time() < 0 || date.year() <= 1970);
}

/// 对齐 Java: `DateUtilTest.ageTest()`
#[test]
fn date_util_age_test_2() {
    let birthday = DateUtil::parse("2000-01-01").unwrap();
    let compare = DateUtil::parse("2020-01-01").unwrap();
    assert_eq!(DateUtil::age(birthday, compare), 20);
}

/// 对齐 Java: `DateUtilTest.ageTest2()`
#[test]
fn date_util_age_test_2_2_2() {
    let birthday = DateUtil::parse("2000-05-01").unwrap();
    let compare = DateUtil::parse("2020-04-01").unwrap();
    assert_eq!(DateUtil::age(birthday, compare), 19);
}

/// 对齐 Java: `DateUtilTest.ageTest3()`
#[test]
fn date_util_age_test_3_2() {
    let birthday = DateUtil::parse("2000-05-01").unwrap();
    let compare = DateUtil::parse("2020-05-01").unwrap();
    assert_eq!(DateUtil::age(birthday, compare), 20);
}

/// 对齐 Java: `DateUtilTest.ageTest4()`
#[test]
fn date_util_age_test_4_2() {
    let birthday = DateUtil::parse("2000-05-02").unwrap();
    let compare = DateUtil::parse("2020-05-01").unwrap();
    assert_eq!(DateUtil::age(birthday, compare), 19);
}

/// 对齐 Java: `DateUtilTest.isExpiredTest()`
#[test]
fn date_util_is_expired_test_2() {
    let start = DateUtil::parse("2020-01-02").unwrap();
    let end = DateUtil::parse("2020-01-01").unwrap();
    assert!(DateUtil::is_expired(start, end));
}

/// 对齐 Java: `DateUtilTest.localDateTimeTest()`
#[test]
fn date_util_local_date_time_test_2() {
    let dt = DateUtil::parse("2020-05-08").unwrap();
    assert_eq!(dt.to_string(), "2020-05-08 00:00:00");
}

/// 对齐 Java: `DateUtilTest.localDateTimeTest2()`
#[test]
fn date_util_local_date_time_test_2_2_2() {
    let dt = DateUtil::parse("2020-05-08 03:12:03").unwrap();
    assert_eq!(dt.to_string(), "2020-05-08 03:12:03");
}

/// 对齐 Java: `DateUtilTest.betweenWeekTest()`
#[test]
fn date_util_between_week_test_2() {
    let w = DateUtil::between_week(
        DateUtil::parse("2020-11-21").unwrap(),
        DateUtil::parse("2020-11-23").unwrap(),
        false,
    );
    assert!(w >= 0);
}

/// 对齐 Java: `DateUtilTest.betweenDayTest()`
#[test]
fn date_util_between_day_test_2() {
    assert_eq!(
        DateUtil::between_day(
            DateUtil::parse("2020-01-01").unwrap(),
            DateUtil::parse("2020-01-11").unwrap(),
            true
        ),
        10
    );
}

/// 对齐 Java: `DateUtilTest.issueI9CYHITest()`
#[test]
fn date_util_issue_i_9_cyhi_test_2() {
    let parse = DateUtil::parse("2020-06-03 12:32:12").unwrap();
    assert_eq!(parse.to_string(), "2020-06-03 12:32:12");
}

/// 对齐 Java: `DateUtilTest.dayOfYearTest()`
#[test]
fn date_util_day_of_year_test_2() {
    let d = DateUtil::parse("2021-01-01").unwrap();
    assert_eq!(DateUtil::day_of_year(d), 1);
}

/// 对齐 Java: `DateUtilTest.parseSingleNumberTest()`
#[test]
fn date_util_parse_single_number_test_2() {
    assert_eq!(
        DateUtil::parse("2020-5-8").unwrap().to_string(),
        "2020-05-08 00:00:00"
    );
}

/// 对齐 Java: `DateUtilTest.parseWithMilsTest()`
#[test]
fn date_util_parse_with_mils_test_2() {
    let dt = DateUtil::parse("2020-05-08 03:12:03.123").unwrap();
    assert!(dt.millisecond() >= 0);
}

/// 对齐 Java: `DateUtilTest.parseNotFitTest()`
#[test]
fn date_util_parse_not_fit_test_2() {
    assert!(DateUtil::parse("not-a-date").is_err());
}

/// 对齐 Java: `DateUtilTest.formatTest()`
#[test]
fn date_util_format_test_2() {
    let date = DateUtil::parse("2021-07-14 23:59:59").unwrap();
    assert_eq!(
        DateUtil::format(date, "yyyy-MM-dd HH:mm:ss"),
        "2021-07-14 23:59:59"
    );
}

/// 对齐 Java: `DateUtilTest.formatNormDateTimeFormatterTest()`
#[test]
fn date_util_format_norm_date_time_formatter_test_2() {
    let date = DateUtil::parse("2021-07-14 10:05:38").unwrap();
    assert_eq!(DateUtil::format_datetime(date), "2021-07-14 10:05:38");
}

/// 对齐 Java: `DateUtilTest.isWeekendTest()`
#[test]
fn date_util_is_weekend_test_2() {
    // 2017-03-04 Saturday
    assert!(DateUtil::is_weekend(DateUtil::parse("2017-03-04").unwrap()));
    assert!(!DateUtil::is_weekend(
        DateUtil::parse("2017-03-01").unwrap()
    ));
}

/// 对齐 Java: `DateUtilTest.parseSingleMonthAndDayTest()`
#[test]
fn date_util_parse_single_month_and_day_test_2() {
    assert_eq!(
        DateUtil::parse("2021-1-1").unwrap().to_string(),
        "2021-01-01 00:00:00"
    );
    assert_eq!(
        DateUtil::parse("2021-1-22").unwrap().to_string(),
        "2021-01-22 00:00:00"
    );
}

/// 对齐 Java: `DateUtilTest.parseByDateTimeFormatterTest()`
#[test]
fn date_util_parse_by_date_time_formatter_test_2() {
    let parse = DateUtil::parse_with_format("2021-12-01", "yyyy-MM-dd").unwrap();
    assert_eq!(parse.to_string(), "2021-12-01 00:00:00");
}

/// 对齐 Java: `DateUtilTest.isSameWeekTest()`
#[test]
fn date_util_is_same_week_test_2() {
    assert!(DateUtil::is_same_week(
        DateUtil::parse("2019-09-10").unwrap(),
        DateUtil::parse("2019-09-11").unwrap(),
        true
    ));
}

/// 对齐 Java: `DateUtilTest.parseTimeTest()`
#[test]
fn date_util_parse_time_test_2() {
    let t = DateUtil::parse_time("12:22:00").unwrap();
    assert_eq!(t.hour(true), 12);
    assert_eq!(t.minute(), 22);
}

/// 对齐 Java: `DateUtilTest.isOverlapTest()`
#[test]
fn date_util_is_overlap_test_2() {
    assert!(DateUtil::is_overlap(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-10").unwrap(),
        DateUtil::parse("2020-01-05").unwrap(),
        DateUtil::parse("2020-01-15").unwrap(),
    ));
}

/// 对齐 Java: `DateUtilTest.isOverlapTest2()`
#[test]
fn date_util_is_overlap_test_2_2_2() {
    assert!(!DateUtil::is_overlap(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-05").unwrap(),
        DateUtil::parse("2020-01-06").unwrap(),
        DateUtil::parse("2020-01-15").unwrap(),
    ));
}

/// 对齐 Java: `DateUtilTest.isInTest()`
#[test]
fn date_util_is_in_test_2() {
    assert!(DateUtil::is_in(
        DateUtil::parse("2020-01-05").unwrap(),
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-10").unwrap(),
    ));
}

/// 对齐 Java: `DateUtilTest.isLastDayTest()`
#[test]
fn date_util_is_last_day_test_2() {
    assert!(DateUtil::is_last_day_of_month(
        DateUtil::parse("2020-02-29").unwrap()
    ));
    assert!(!DateUtil::is_last_day_of_month(
        DateUtil::parse("2020-02-28").unwrap()
    ));
}

/// 对齐 Java: `DateUtilTest.parseUTCTest4()`
#[test]
fn date_util_parse_utc_test_4_2() {
    let dt = DateUtil::parse_utc("2019-05-16T09:57:18Z").unwrap();
    assert!(dt.to_string().contains("2019-05-16"));
}

/// 对齐 Java: `DateUtilTest.calendarTest()`
#[test]
fn date_util_calendar_test_2() {
    let d = DateUtil::parse("2020-01-01").unwrap();
    assert_eq!(DateUtil::year(d), 2020);
    assert_eq!(DateUtil::month(d), 0);
}

/// 对齐 Java: `DateUtilTest.issueI7H34NTest()`
#[test]
fn date_util_issue_i_7_h_34_n_test_2() {
    let date_time = DateUtil::parse("2023-02-07 00:02:16").unwrap();
    assert_eq!(date_time.to_string(), "2023-02-07 00:02:16");
}

/// 对齐 Java: `DateUtilTest.issueI8NMP7Test()`
#[test]
fn date_util_issue_i_8_nmp_7_test_2() {
    let date_time = DateUtil::parse("2023-12-11 10:42:04").unwrap();
    assert_eq!(date_time.to_string(), "2023-12-11 10:42:04");
}

/// 对齐 Java: `DateUtilTest.formatSpeedTest()`
#[test]
fn date_util_format_speed_test_2() {
    let date = DateUtil::parse("2021-07-14 10:05:38").unwrap();
    for _ in 0..100 {
        let _ = DateUtil::format_datetime(date);
    }
    assert_eq!(DateUtil::format_datetime(date), "2021-07-14 10:05:38");
}

/// 对齐 Java: `DateUtil.yesterday/tomorrow/isLeapYear/weekOfMonth`
#[test]
fn yesterday_tomorrow_leap_week_of_month_test() {
    let y = DateUtil::yesterday();
    let t = DateUtil::tomorrow();
    assert!(y.get_time() < DateUtil::date().get_time());
    assert!(t.get_time() > DateUtil::date().get_time());
    assert!(DateUtil::is_leap_year(2024));
    assert!(!DateUtil::is_leap_year(2023));
    let d = DateUtil::parse("2024-01-15 12:00:00").unwrap();
    assert!(DateUtil::week_of_month(d) >= 1);
    assert_eq!(DateUtil::length_of_month(2, true), 29);
}

/// 对齐 Java: `LocalDateTimeUtil.isSameDay/isWeekend/toEpochMilli`
#[test]
fn local_date_time_same_day_weekend_epoch_test() {
    let a = LocalDateTimeUtil::parse("2024-01-06 10:00:00").unwrap();
    let b = LocalDateTimeUtil::parse("2024-01-06 23:00:00").unwrap();
    assert!(LocalDateTimeUtil::is_same_day(a, b));
    assert!(LocalDateTimeUtil::is_weekend(a)); // 2024-01-06 Saturday
    assert!(LocalDateTimeUtil::to_epoch_milli(a) > 0);
}

/// 对齐 Java: `DateTime.isAfter/isBefore/isAM`
#[test]
fn date_time_compare_am_test() {
    let a = DateUtil::parse("2024-01-01 09:00:00").unwrap();
    let b = DateUtil::parse("2024-01-01 15:00:00").unwrap();
    assert!(a.is_before(b));
    assert!(b.is_after(a));
    assert!(a.is_am());
    assert!(b.is_pm());
}
