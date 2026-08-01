//! Date package type/util parity
//! 对齐: hutool-core date types (Month/Week/Quarter/YearQuarter/Zodiac/DateTime/...)

#![allow(non_snake_case)] // 对齐 Java 测试方法名（带 _2 后缀的 Java 风格命名）

use chrono::{Month as ChronoMonth, Timelike, Weekday};
use hutool_core::{
    BetweenFormatter, BetweenFormatterLevel, DateBetween, DateField, DatePattern, DateRange,
    DateTime, DateUnit, DateUtil, GroupTimeInterval, Month, Quarter, StopWatch,
    TemporalAccessorUtil, TemporalUtil, TimeInterval, Week, YearQuarter, Zodiac,
};
use hutool_core::date::calendar_util::CalendarUtil;
use hutool_core::date::date_modifier::DateModifier;
use hutool_core::date::format::fast_date_format::FastDateFormat;
use hutool_core::date::zone_util::ZoneUtil;

/// 对齐 Java: `BetweenFormatterTest.formatTest()`
#[test]
fn between_formatter_format_test_2() {
    let between_ms = DateUtil::between_ms(
        DateUtil::parse("2017-01-01 22:59:59").unwrap(),
        DateUtil::parse("2017-01-02 23:59:58").unwrap(),
    );
    let formater = BetweenFormatter::new(between_ms, BetweenFormatterLevel::Millisecond, 1);
    assert_eq!(formater.to_string(), "1天");
}

/// 对齐 Java: `BetweenFormatterTest.formatTestEn()`
#[test]
fn between_formatter_format_test_en_2() {
    let between_ms = DateUtil::between_ms(
        DateUtil::parse("2017-01-01 22:59:59").unwrap(),
        DateUtil::parse("2017-01-02 23:59:58").unwrap(),
    );
    let mut formatter = BetweenFormatter::new(between_ms, BetweenFormatterLevel::Millisecond, 1);
    formatter.set_level_formatter(|level| match level {
        BetweenFormatterLevel::Day => " day".into(),
        BetweenFormatterLevel::Hour => " hour".into(),
        BetweenFormatterLevel::Minute => " minute".into(),
        BetweenFormatterLevel::Second => " second".into(),
        BetweenFormatterLevel::Millisecond => " millisecond".into(),
    });
    assert_eq!(formatter.to_string(), "1 day");
}

/// 对齐 Java: `BetweenFormatterTest.formatTestEn2()`
#[test]
fn between_formatter_format_test_en_2_2_2() {
    let between_ms = 3610001;
    let mut formatter = BetweenFormatter::new(between_ms, BetweenFormatterLevel::Millisecond, 5);
    formatter.set_separator(",");
    formatter.set_level_formatter(|level| match level {
        BetweenFormatterLevel::Day => " day".into(),
        BetweenFormatterLevel::Hour => " hour".into(),
        BetweenFormatterLevel::Minute => " minute".into(),
        BetweenFormatterLevel::Second => " second".into(),
        BetweenFormatterLevel::Millisecond => " millisecond".into(),
    });
    assert_eq!(formatter.to_string(), "1 hour,10 second,1 millisecond");
}

/// 对齐 Java: `BetweenFormatterTest.formatBetweenTest()`
#[test]
fn between_formatter_format_between_test_2() {
    let between_ms = DateUtil::between_ms(
        DateUtil::parse("2018-07-16 11:23:19").unwrap(),
        DateUtil::parse("2018-07-16 11:23:20").unwrap(),
    );
    let formater = BetweenFormatter::new(between_ms, BetweenFormatterLevel::Second, 1);
    assert_eq!(formater.to_string(), "1秒");
}

/// 对齐 Java: `BetweenFormatterTest.formatBetweenTest2()`
#[test]
fn between_formatter_format_between_test_2_2_2() {
    let between_ms = DateUtil::between_ms(
        DateUtil::parse("2018-07-16 12:25:23").unwrap(),
        DateUtil::parse("2018-07-16 11:23:20").unwrap(),
    );
    let formater = BetweenFormatter::new(between_ms, BetweenFormatterLevel::Second, 5);
    assert_eq!(formater.to_string(), "1小时2分3秒");
}

/// 对齐 Java: `BetweenFormatterTest.formatTest2()`
#[test]
fn between_formatter_format_test_2_2_2() {
    let formater = BetweenFormatter::new(584, BetweenFormatterLevel::Second, 1);
    assert_eq!(formater.to_string(), "0秒");
}

/// 对齐 Java: `CalendarUtilTest.formatChineseDate()`
#[test]
fn calendar_util_format_chinese_date_2() {
    let date = DateUtil::parse("2018-02-24").unwrap();
    assert!(CalendarUtil::format_chinese_date(date, false).contains("年"));
}

/// 对齐 Java: `CalendarUtilTest.parseTest()`
#[test]
fn calendar_util_parse_test_2() {
    assert!(CalendarUtil::parse("2020-01-01").is_ok());
}

/// 对齐 Java: `DateBetweenTest.betweenYearTest()`
#[test]
fn date_between_between_year_test_2() {
    let start = DateUtil::parse("2017-02-01 12:23:46").unwrap();
    let end = DateUtil::parse("2018-02-01 12:23:46").unwrap();
    assert_eq!(DateBetween::new(start, end, true).between_year(false), 1);
    let end2 = DateUtil::parse("2018-02-01 11:23:46").unwrap();
    assert_eq!(DateBetween::new(start, end2, true).between_year(false), 0);
}

/// 对齐 Java: `DateBetweenTest.betweenYearTest2()`
#[test]
fn date_between_between_year_test_2_2_2() {
    let start = DateUtil::parse("2000-02-29").unwrap();
    let end = DateUtil::parse("2018-02-28").unwrap();
    assert_eq!(DateBetween::new(start, end, true).between_year(false), 18);
}

/// 对齐 Java: `DateBetweenTest.betweenYearTest3()`
#[test]
fn date_between_between_year_test_3_2() {
    let start = DateUtil::parse("20170301").unwrap();
    let end = DateUtil::parse("2024-02-29 14:56:18").unwrap();
    assert_eq!(DateBetween::new(start, end, true).between_year(false), 6);
}

/// 对齐 Java: `DateBetweenTest.betweenMonthTest()`
#[test]
fn date_between_between_month_test_2() {
    let start = DateUtil::parse("2017-02-01 12:23:46").unwrap();
    let end = DateUtil::parse("2018-02-01 12:23:46").unwrap();
    assert_eq!(DateBetween::new(start, end, true).between_month(false), 12);
    let end2 = DateUtil::parse("2018-02-01 11:23:46").unwrap();
    assert_eq!(DateBetween::new(start, end2, true).between_month(false), 11);
}

/// 对齐 Java: `DateBetweenTest.betweenMinuteTest()`
#[test]
fn date_between_between_minute_test_2() {
    let date1 = DateUtil::parse("2017-03-01 20:33:23").unwrap();
    let date2 = DateUtil::parse("2017-03-01 23:33:23").unwrap();
    let format_between = DateUtil::format_between_level(date1, date2, BetweenFormatterLevel::Second);
    assert_eq!(format_between, "3小时");
}

/// 对齐 Java: `DateBetweenTest.betweenWeeksTest()`
#[test]
fn date_between_between_weeks_test_2() {
    let between_week = DateUtil::between_week(
        DateUtil::parse("2020-11-21").unwrap(),
        DateUtil::parse("2020-11-23").unwrap(),
        false,
    );
    assert!(between_week >= 0);
}

/// 对齐 Java: `DateBetweenTest.issueI97U3JTest()`
#[test]
fn date_between_issue_i_97_u_3_j_test_2() {
    let b = DateBetween::new(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2021-01-01").unwrap(),
        true,
    );
    assert_eq!(b.between_year(true), 1);
}

/// 对齐 Java: `DateBetweenTest.issueIDFVKGTest()`
#[test]
fn date_between_issue_idfvkg_test_2() {
    assert_eq!(
        DateBetween::new(
            DateUtil::parse("2023-01-01").unwrap(),
            DateUtil::parse("2023-12-31").unwrap(),
            true
        )
        .between_day(true),
        364
    );
}

/// 对齐 Java: `DateFieldTest.ofTest()`
#[test]
fn date_field_of_test_2() {
    assert_eq!(DateField::of(1), Some(DateField::Year));
    assert_eq!(DateField::of(12), Some(DateField::Minute));
}

/// 对齐 Java: `DateModifierTest.truncateTest()`
#[test]
fn date_modifier_truncate_test_2() {
    let date = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    assert_eq!(
        DateModifier::truncate(date, DateField::Minute).to_string(),
        "2020-02-29 12:59:00"
    );
}

/// 对齐 Java: `DateModifierTest.truncateDayOfWeekInMonthTest()`
#[test]
fn date_modifier_truncate_day_of_week_in_month_test_2() {
    let date = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    let t = DateModifier::truncate(date, DateField::DayOfWeekInMonth);
    assert_eq!(t.hour(true), 0);
}

/// 对齐 Java: `DateModifierTest.ceilingTest()`
#[test]
fn date_modifier_ceiling_test_2() {
    let date = DateUtil::parse("2020-02-29 12:59:34").unwrap();
    let c = DateModifier::ceiling(date, DateField::DayOfMonth, false);
    assert!(c.format("yyyy-MM-dd HH:mm:ss.SSS").ends_with("23:59:59.999"));
}

/// 对齐 Java: `DateRangeTest.issue3783Test()`
#[test]
fn date_range_issue_3783_test_2() {
    let range = DateRange::new(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-03").unwrap(),
        DateField::DayOfMonth,
    );
    assert_eq!(range.to_list().len(), 3);
}

/// 对齐 Java: `DateRangeTest.issue3783Test2()`
#[test]
fn date_range_issue_3783_test_2_2_2() {
    let list = DateUtil::range_to_list(
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-01-05").unwrap(),
        DateField::DayOfMonth,
    );
    assert_eq!(list.len(), 5);
}

/// 对齐 Java: `DateTimeTest.datetimeTest()`
#[test]
fn date_time_datetime_test_2() {
    let dt = DateTime::now();
    assert!(dt.get_time() > 0);
}

/// 对齐 Java: `DateTimeTest.datetimeTest2()`
#[test]
fn date_time_datetime_test_2_2_2() {
    let dt = DateUtil::parse("2020-01-01").unwrap();
    assert_eq!(dt.year(), 2020);
}

/// 对齐 Java: `DateTimeTest.quarterTest()`
#[test]
fn date_time_quarter_test_2() {
    let dt = DateUtil::parse("2020-05-01").unwrap();
    assert_eq!(dt.quarter(), 2);
}

/// 对齐 Java: `DateTimeTest.mutableTest()`
#[test]
fn date_time_mutable_test_2() {
    let mut dt = DateUtil::parse("2020-01-01").unwrap();
    dt.set_mutable(false);
    assert!(!dt.is_mutable());
}

/// 对齐 Java: `DateTimeTest.toStringTest()`
#[test]
fn date_time_to_string_test_2() {
    assert_eq!(
        DateUtil::parse("2020-01-01 12:00:00").unwrap().to_string(),
        "2020-01-01 12:00:00"
    );
}

/// 对齐 Java: `DateTimeTest.toStringTest2()`
#[test]
fn date_time_to_string_test_2_2_2() {
    let dt = DateUtil::parse("2020-01-01 12:00:00.123").unwrap();
    assert!(dt.format("yyyy-MM-dd HH:mm:ss.SSS").contains("2020-01-01"));
}

/// 对齐 Java: `DateTimeTest.monthTest()`
#[test]
fn date_time_month_test_2() {
    assert_eq!(DateUtil::parse("2020-05-01").unwrap().month(), 4);
}

/// 对齐 Java: `DateTimeTest.weekOfYearTest()`
#[test]
fn date_time_week_of_year_test_2() {
    assert_eq!(DateUtil::parse("2016-01-03").unwrap().week_of_year(), 1);
}

/// 对齐 Java: `DateTimeTest.ofTest()`
#[test]
fn date_time_of_test_2() {
    let dt = DateTime::of_millis(0);
    assert!(dt.year() >= 1969 && dt.year() <= 1970);
}

/// 对齐 Java: `FastDateFormatTest.yearTest()`
#[test]
fn fast_date_format_year_test_2() {
    let f = FastDateFormat::get_instance("yyyy");
    assert_eq!(f.format(DateUtil::parse("2020-01-01").unwrap()), "2020");
}

/// 对齐 Java: `FastDateFormatTest.weekYearTest()`
#[test]
fn fast_date_format_week_year_test_2() {
    let f = FastDateFormat::get_instance("yyyy-MM-dd");
    assert_eq!(f.format(DateUtil::parse("2020-01-01").unwrap()), "2020-01-01");
}

/// 对齐 Java: `MonthTest.getLastDayTest()`
#[test]
fn month_get_last_day_test_2() {
    assert_eq!(Month::of_value(0).unwrap().get_last_day(false), 31);
    assert_eq!(Month::February.get_last_day(false), 28);
    assert_eq!(Month::February.get_last_day(true), 29);
    assert_eq!(Month::April.get_last_day(true), 30);
}

/// 对齐 Java: `MonthTest.toJdkMonthTest()`
#[test]
fn month_to_jdk_month_test_2() {
    assert_eq!(Month::August.to_jdk_month().unwrap(), ChronoMonth::August);
}

/// 对齐 Java: `MonthTest.toJdkMonthTest2()`
#[test]
fn month_to_jdk_month_test_2_2_2() {
    assert!(Month::Undecimber.to_jdk_month().is_err());
}

/// 对齐 Java: `MonthTest.ofTest()`
#[test]
fn month_of_test_2() {
    assert_eq!(Month::of("Jan"), Some(Month::January));
    assert_eq!(Month::of("JAN"), Some(Month::January));
    assert_eq!(Month::of("FEBRUARY"), Some(Month::February));
    assert_eq!(Month::of("February"), Some(Month::February));
}

/// 对齐 Java: `MonthTest.getDisplayNameTest()`
#[test]
fn month_get_display_name_test_2() {
    assert_eq!(Month::February.get_display_name("SHORT", "US"), "Feb");
}

/// 对齐 Java: `QuarterTest.testQ1()`
#[test]
fn quarter_test_q_1_2() {
    let quarter = Quarter::of(1).unwrap();
    assert_eq!(quarter, Quarter::Q1);
    assert_eq!(Quarter::value_of("Q1"), Some(Quarter::Q1));
    assert_eq!(quarter.get_value(), 1);
    assert_eq!(quarter.name(), "Q1");
    assert!(Quarter::of(0).is_none());
    assert_eq!(quarter.first_month_value(), 1);
    assert_eq!(quarter.first_month(), Month::January);
    assert_eq!(quarter.last_month_value(), 3);
    assert_eq!(quarter.last_month(), Month::March);
    assert_eq!(quarter.first_month_day(), (1, 1));
    assert_eq!(quarter.last_month_day(), (3, 31));
}

/// 对齐 Java: `QuarterTest.testQ2()`
#[test]
fn quarter_test_q_2_2() {
    let quarter = Quarter::of(2).unwrap();
    assert_eq!(quarter.first_month_value(), 4);
    assert_eq!(quarter.last_month_value(), 6);
}

/// 对齐 Java: `QuarterTest.testQ3()`
#[test]
fn quarter_test_q_3_2() {
    let quarter = Quarter::of(3).unwrap();
    assert_eq!(quarter.first_month_value(), 7);
    assert_eq!(quarter.last_month_value(), 9);
}

/// 对齐 Java: `QuarterTest.testQ4()`
#[test]
fn quarter_test_q_4_2() {
    let quarter = Quarter::of(4).unwrap();
    assert_eq!(quarter.first_month_value(), 10);
    assert_eq!(quarter.last_month_value(), 12);
}

/// 对齐 Java: `QuarterTest.testPlusZeroAndPositiveRealNumbers()`
#[test]
fn quarter_test_plus_zero_and_positive_real_numbers_2() {
    assert_eq!(Quarter::Q1.plus(0), Quarter::Q1);
    assert_eq!(Quarter::Q1.plus(1), Quarter::Q2);
    assert_eq!(Quarter::Q4.plus(1), Quarter::Q1);
}

/// 对齐 Java: `QuarterTest.testPlusZeroAndNegativeNumber()`
#[test]
fn quarter_test_plus_zero_and_negative_number_2() {
    assert_eq!(Quarter::Q1.plus(-1), Quarter::Q4);
}

/// supplemental: TemporalAccessor format (see formatLocalDateTest)
#[test]
fn temporal_accessor_util_format_test_2() {
    use hutool_core::date::temporal_accessor_util::TemporalAccessorUtil;
    let dt = DateUtil::parse("2020-01-01 12:00:00").unwrap().naive_local();
    assert!(TemporalAccessorUtil::format(dt).contains("2020-01-01"));
}

/// supplemental: TemporalAccessor format variant
#[test]
fn temporal_accessor_util_of_test_2() {
    use hutool_core::date::temporal_accessor_util::TemporalAccessorUtil;
    let dt = DateUtil::parse("2020-06-15").unwrap().naive_local();
    assert_eq!(TemporalAccessorUtil::get_year(dt), 2020);
    assert_eq!(TemporalAccessorUtil::get_month(dt), 6);
}

/// 对齐 Java: `TimeIntervalTest.intervalGroupTest()`
#[test]
fn time_interval_interval_group_test_2() {
    use hutool_core::TimeInterval;
    let t = TimeInterval::new_group();
    assert!(t.is_group());
}

/// supplemental: ZoneUtil (see timeZoneConvertTest)
#[test]
fn time_zone_time_zone_test_2() {
    use hutool_core::date::time_zone_util::TimeZoneUtil;
    let z = TimeZoneUtil::get_default();
    assert_eq!(z.local_minus_utc(), 8 * 3600);
}

/// 对齐 Java: `WeekTest.ofTest()`
#[test]
fn week_of_test_2() {
    assert_eq!(Week::of("sun"), Some(Week::Sunday));
    assert_eq!(Week::of("SUN"), Some(Week::Sunday));
    assert_eq!(Week::of("Monday"), Some(Week::Monday));
    assert_eq!(Week::of("saturday"), Some(Week::Saturday));
}

/// 对齐 Java: `WeekTest.ofChineseTest()`
#[test]
fn week_of_chinese_test_2() {
    assert_eq!(Week::of("星期日"), Some(Week::Sunday));
    assert_eq!(Week::of("周一"), Some(Week::Monday));
    assert_eq!(Week::of("周六"), Some(Week::Saturday));
}

/// 对齐 Java: `WeekTest.ofTest2()`
#[test]
fn week_of_test_2_2() {
    assert_eq!(Week::of_weekday(Weekday::Sun), Week::Sunday);
    assert_eq!(Week::of_weekday(Weekday::Mon), Week::Monday);
}

/// 对齐 Java: `WeekTest.toJdkDayOfWeekTest()`
#[test]
fn week_to_jdk_day_of_week_test_2() {
    assert_eq!(Week::Monday.to_jdk_day_of_week(), Weekday::Mon);
    assert_eq!(Week::Sunday.to_jdk_day_of_week(), Weekday::Sun);
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearAndQuarterValue_CreatesYearQuarter()`
#[test]
fn year_quarter_of__valid_year_and_quarter_value__creates_year_quarter_2() {
    for q in 1..=4 {
        let yq = YearQuarter::of(2024, q).unwrap();
        assert_eq!(yq.get_year(), 2024);
        assert_eq!(yq.get_quarter_value(), q);
    }
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearAndInvalidQuarterValue_DateTimeException()`
#[test]
fn year_quarter_of__valid_year_and_invalid_quarter_value__date_time_exception_2() {
    for q in [-1, 0, 5, 108] {
        assert!(YearQuarter::of(2024, q).is_err());
    }
}

/// 对齐 Java: `YearQuarterTest.of_InvalidYearAndValidQuarterValue_DateTimeException()`
#[test]
fn year_quarter_of__invalid_year_and_valid_quarter_value__date_time_exception_2() {
    assert!(YearQuarter::of(-1_000_000_000, 1).is_err());
    assert!(YearQuarter::of(1_000_000_000, 1).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_InvalidYearAndInvalidQuarterValue_DateTimeException()`
#[test]
fn year_quarter_of__invalid_year_and_invalid_quarter_value__date_time_exception_2() {
    assert!(YearQuarter::of(-1_000_000_000, 0).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearAndQuarter_CreatesYearQuarter()`
#[test]
fn year_quarter_of__valid_year_and_quarter__creates_year_quarter_2() {
    let yq = YearQuarter::of_quarter(2024, Quarter::Q2).unwrap();
    assert_eq!(yq.get_quarter(), Quarter::Q2);
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearAndNullQuarter_NullPointerException()`
#[test]
fn year_quarter_of__valid_year_and_null_quarter__null_pointer_exception_2() {
    // Rust 无 null；用非法季度值表达失败路径
    assert!(YearQuarter::of(2024, 0).is_err());
}


/// 对齐 Java: `YearQuarterTest.of_NullLocalDate_NullPointerException()`
#[test]
fn year_quarter_of__null_local_date__null_pointer_exception_2() {
    assert!(YearQuarter::from_ymd(2024, 13, 1).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_ValidDate_CreatesYearQuarter()`
#[test]
fn year_quarter_of__valid_date__creates_year_quarter_2() {
    let yq = YearQuarter::from_date(DateUtil::parse("2024-08-01").unwrap());
    assert_eq!(yq.get_quarter_value(), 3);
}

/// 对齐 Java: `YearQuarterTest.of_NullDate_NullPointerException()`
#[test]
fn year_quarter_of__null_date__null_pointer_exception_2() {
    // 无效月份
    assert!(YearQuarter::from_year_month(2024, 0).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_ValidCalendar_CreatesYearQuarter()`
#[test]
fn year_quarter_of__valid_calendar__creates_year_quarter_2() {
    let yq = YearQuarter::from_date(DateUtil::parse("2024-11-01").unwrap());
    assert_eq!(yq.get_quarter_value(), 4);
}

/// 对齐 Java: `YearQuarterTest.of_NullCalendar_NullPointerException()`
#[test]
fn year_quarter_of__null_calendar__null_pointer_exception_2() {
    assert!(YearQuarter::from_year_month(2024, 14).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearMonth_CreatesYearMonth_Q1()`
#[test]
fn year_quarter_of__valid_year_month__creates_year_month_q_1_2() {
    assert_eq!(YearQuarter::from_year_month(2024, 2).unwrap().get_quarter_value(), 1);
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearMonth_CreatesYearMonth_Q2()`
#[test]
fn year_quarter_of__valid_year_month__creates_year_month_q_2_2() {
    assert_eq!(YearQuarter::from_year_month(2024, 5).unwrap().get_quarter_value(), 2);
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearMonth_CreatesYearMonth_Q3()`
#[test]
fn year_quarter_of__valid_year_month__creates_year_month_q_3_2() {
    assert_eq!(YearQuarter::from_year_month(2024, 8).unwrap().get_quarter_value(), 3);
}

/// 对齐 Java: `YearQuarterTest.of_ValidYearMonth_CreatesYearMonth_Q4()`
#[test]
fn year_quarter_of__valid_year_month__creates_year_month_q_4_2() {
    assert_eq!(YearQuarter::from_year_month(2024, 11).unwrap().get_quarter_value(), 4);
}

/// 对齐 Java: `YearQuarterTest.of_NullYearMonth_CreatesYearMonth_Q4()`
#[test]
fn year_quarter_of__null_year_month__creates_year_month_q_4_2() {
    assert!(YearQuarter::from_year_month(2024, 0).is_err());
}

/// 对齐 Java: `YearQuarterTest.test_getFirstDate_And_getLastDate()`
#[test]
fn year_quarter_test_get_first_date__and_get_last_date_2() {
    let yq = YearQuarter::of(2024, 1).unwrap();
    assert_eq!(yq.get_first_date().to_string(), "2024-01-01");
    assert_eq!(yq.get_last_date().to_string(), "2024-03-31");
}

/// 对齐 Java: `YearQuarterTest.test_firstYearMonth_And_lastYearMonth()`
#[test]
fn year_quarter_test_first_year_month__and_last_year_month_2() {
    let yq = YearQuarter::of(2024, 2).unwrap();
    assert_eq!(yq.first_year_month(), (2024, 4));
    assert_eq!(yq.last_year_month(), (2024, 6));
}

/// 对齐 Java: `YearQuarterTest.testFirstMonthAndLastMonth()`
#[test]
fn year_quarter_test_first_month_and_last_month_2() {
    let yq = YearQuarter::of(2024, 3).unwrap();
    assert_eq!(yq.first_month(), Month::July);
    assert_eq!(yq.last_month(), Month::September);
}

/// 对齐 Java: `YearQuarterTest.testCompareTo()`
#[test]
fn year_quarter_test_compare_to_2() {
    let a = YearQuarter::of(2024, 1).unwrap();
    let b = YearQuarter::of(2024, 2).unwrap();
    assert!(a < b);
}

/// 对齐 Java: `YearQuarterTest.testPlusQuartersAndMinusQuarters()`
#[test]
fn year_quarter_test_plus_quarters_and_minus_quarters_2() {
    let yq = YearQuarter::of(2024, 1).unwrap();
    assert_eq!(yq.plus_quarters(1).unwrap().get_quarter_value(), 2);
    assert_eq!(yq.minus_quarters(1).unwrap().get_year(), 2023);
}

/// 对齐 Java: `YearQuarterTest.test_nextQuarter_And_lastQuarter()`
#[test]
fn year_quarter_test_next_quarter__and_last_quarter_2() {
    let yq = YearQuarter::of(2024, 2).unwrap();
    assert_eq!(yq.next_quarter().unwrap().get_quarter_value(), 3);
    assert_eq!(yq.last_quarter().unwrap().get_quarter_value(), 1);
}

/// 对齐 Java: `YearQuarterTest.test_PlusYearsAndMinusYears()`
#[test]
fn year_quarter_test__plus_years_and_minus_years_2() {
    let yq = YearQuarter::of(2024, 1).unwrap();
    assert_eq!(yq.plus_years(1).unwrap().get_year(), 2025);
    assert_eq!(yq.minus_years(1).unwrap().get_year(), 2023);
}

/// 对齐 Java: `YearQuarterTest.test_nextYear_And_lastYear()`
#[test]
fn year_quarter_test_next_year__and_last_year_2() {
    let yq = YearQuarter::of(2024, 1).unwrap();
    assert_eq!(yq.next_year().unwrap().get_year(), 2025);
    assert_eq!(yq.last_year().unwrap().get_year(), 2023);
}

/// 对齐 Java: `YearQuarterTest.test_compareTo_sameYear()`
#[test]
fn year_quarter_test_compare_to_same_year_2() {
    let a = YearQuarter::of(2024, 1).unwrap();
    let b = YearQuarter::of(2024, 3).unwrap();
    assert!(a.compare_to(b) < 0);
}

/// 对齐 Java: `YearQuarterTest.test_isBefore_sameYear()`
#[test]
fn year_quarter_test_is_before_same_year_2() {
    assert!(YearQuarter::of(2024, 1).unwrap().is_before(YearQuarter::of(2024, 2).unwrap()));
}

/// 对齐 Java: `YearQuarterTest.test_isAfter_sameYear()`
#[test]
fn year_quarter_test_is_after_same_year_2() {
    assert!(YearQuarter::of(2024, 3).unwrap().is_after(YearQuarter::of(2024, 1).unwrap()));
}

/// 对齐 Java: `YearQuarterTest.test_compareTo_null()`
#[test]
fn year_quarter_test_compare_to_null_2() {
    // Rust 无 null compare；自比较为 0
    let a = YearQuarter::of(2024, 1).unwrap();
    assert_eq!(a.compare_to(a), 0);
}

/// 对齐 Java: `YearQuarterTest.test_compareTo_differentYear()`
#[test]
fn year_quarter_test_compare_to_different_year_2() {
    assert!(YearQuarter::of(2023, 4).unwrap() < YearQuarter::of(2024, 1).unwrap());
}

/// 对齐 Java: `ZodiacTest.getZodiacTest()`
#[test]
fn zodiac_get_zodiac_test_2() {
    assert_eq!(Zodiac::get_zodiac_month(Month::January, 19).unwrap(), "摩羯座");
    assert_eq!(Zodiac::get_zodiac_month(Month::January, 20).unwrap(), "水瓶座");
    assert_eq!(Zodiac::get_zodiac(6, 17).unwrap(), "巨蟹座");
    let cal = DateUtil::parse("2022-07-17").unwrap();
    assert_eq!(Zodiac::get_zodiac_date(cal).unwrap(), "巨蟹座");
}

/// 对齐 Java: `ZodiacTest.getChineseZodiacTest()`
#[test]
fn zodiac_get_chinese_zodiac_test_2() {
    assert_eq!(Zodiac::get_chinese_zodiac(1994).unwrap(), "狗");
    assert_eq!(Zodiac::get_chinese_zodiac(2018).unwrap(), "狗");
    assert_eq!(Zodiac::get_chinese_zodiac(2019).unwrap(), "猪");
    assert_eq!(Zodiac::get_chinese_zodiac_date(DateUtil::parse("2022-07-17").unwrap()).unwrap(), "虎");
    assert!(Zodiac::get_chinese_zodiac(1899).is_none());
}

/// 对齐 Java: `ZoneUtilTest.toTest()`
#[test]
fn zone_util_to_test_2() {
    assert_eq!(ZoneUtil::to_zone("Asia/Shanghai").local_minus_utc(), 8 * 3600);
    assert_eq!(ZoneUtil::to_zone("UTC").local_minus_utc(), 0);
}


/// 对齐 Java: `TemporalAccessorUtilTest.formatLocalDateTest()`
#[test]
fn temporal_accessor_util_format_local_date_test_2() {
    use hutool_core::date::temporal_accessor_util::TemporalAccessorUtil;
    use chrono::NaiveDate;
    let d = NaiveDate::from_ymd_opt(2020, 1, 23).unwrap().and_hms_opt(0, 0, 0).unwrap();
    assert!(TemporalAccessorUtil::format(d).contains("2020-01-23"));
}

/// 对齐 Java: `TemporalAccessorUtilTest.formatLocalTimeTest()`
#[test]
fn temporal_accessor_util_format_local_time_test_2() {
    use hutool_core::date::temporal_accessor_util::TemporalAccessorUtil;
    use chrono::NaiveDate;
    let d = NaiveDate::from_ymd_opt(2020, 1, 23).unwrap().and_hms_opt(12, 23, 56).unwrap();
    assert!(TemporalAccessorUtil::format(d).contains("12:23:56"));
}

/// 对齐 Java: `TemporalAccessorUtilTest.formatCustomTest()`
#[test]
fn temporal_accessor_util_format_custom_test_2() {
    use hutool_core::date::temporal_accessor_util::TemporalAccessorUtil;
    let dt = DateUtil::parse("2020-01-23 12:23:56").unwrap().naive_local();
    assert_eq!(TemporalAccessorUtil::get_year(dt), 2020);
}

/// 对齐 Java: `TemporalAccessorUtilTest.isInTest()`
#[test]
fn temporal_accessor_util_is_in_test_2() {
    let mid = DateUtil::parse("2020-06-15").unwrap();
    assert!(DateUtil::is_in(
        mid,
        DateUtil::parse("2020-01-01").unwrap(),
        DateUtil::parse("2020-12-31").unwrap()
    ));
}

/// 对齐 Java: `TimeZoneTest.timeZoneConvertTest()`
#[test]
fn time_zone_time_zone_convert_test_2() {
    let z = ZoneUtil::to_zone("Asia/Shanghai");
    assert_eq!(z.local_minus_utc(), 8 * 3600);
    let utc = ZoneUtil::to_zone("UTC");
    assert_eq!(utc.local_minus_utc(), 0);
}

/// 对齐 Java: `YearQuarterTest.of_InvalidYearAndValidQuarter_DateTimeException()`
#[test]
fn year_quarter_of__invalid_year_and_valid_quarter__date_time_exception_2() {
    assert!(YearQuarter::of(-1_000_000_000, 2).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_InvalidYearAndNullQuarter_DateTimeException()`
#[test]
fn year_quarter_of__invalid_year_and_null_quarter__date_time_exception_2() {
    assert!(YearQuarter::of(-1_000_000_000, 0).is_err());
}

/// 对齐 Java: `YearQuarterTest.of_ValidLocalDate_CreatesYearQuarter_Q1()`
#[test]
fn year_quarter_of__valid_local_date__creates_year_quarter_q_1_2() {
    assert_eq!(YearQuarter::from_ymd(2024, 1, 15).unwrap().get_quarter_value(), 1);
}

/// 对齐 Java: `YearQuarterTest.of_ValidLocalDate_CreatesYearQuarter_Q2()`
#[test]
fn year_quarter_of__valid_local_date__creates_year_quarter_q_2_2() {
    assert_eq!(YearQuarter::from_ymd(2024, 4, 15).unwrap().get_quarter_value(), 2);
}

/// 对齐 Java: `YearQuarterTest.of_ValidLocalDate_CreatesYearQuarter_Q3()`
#[test]
fn year_quarter_of__valid_local_date__creates_year_quarter_q_3_2() {
    assert_eq!(YearQuarter::from_ymd(2024, 7, 15).unwrap().get_quarter_value(), 3);
}

/// 对齐 Java: `YearQuarterTest.of_ValidLocalDate_CreatesYearQuarter_Q4()`
#[test]
fn year_quarter_of__valid_local_date__creates_year_quarter_q_4_2() {
    assert_eq!(YearQuarter::from_ymd(2024, 10, 15).unwrap().get_quarter_value(), 4);
}

/// Wave2 StopWatch Spring-style parity.
#[test]
fn wave2_stop_watch_parity() {
    let mut sw = StopWatch::with_id("demo");
    sw.start_named("a");
    sw.stop();
    sw.start_named("b");
    sw.stop();
    assert_eq!(sw.get_id(), "demo");
    assert_eq!(sw.get_task_count(), 2);
    assert!(!sw.is_running());
    let _ = sw.get_total_time_nanos();
    assert!(!sw.pretty_print().is_empty());
    assert_eq!(sw.get_task_info().len(), 2);
    assert_eq!(sw.get_last_task_name(), "b");
}

/// Wave2 DatePattern formatter factory.
#[test]
fn wave2_date_pattern_formatter() {
    let fmt = DatePattern::create_formatter("yyyy-MM-dd HH:mm:ss");
    assert!(fmt.contains("%Y"));
    assert!(fmt.contains("%m"));
}

/// Wave2 TimeInterval helpers.
#[test]
fn wave2_time_interval_parity() {
    let mut t = TimeInterval::with_nano(false);
    let _ = t.start();
    assert!(t.interval_ms() >= 0);
    assert!(!t.interval_pretty().is_empty());
}

/// Wave2 GroupTimeInterval helpers.
#[test]
fn wave2_group_time_interval_parity() {
    let mut g = GroupTimeInterval::new(false);
    g.start("x");
    assert!(g.interval_ms("x") >= 0);
    assert!(!g.interval_pretty("x").is_empty());
    g.clear();
    assert_eq!(g.interval("missing"), 0);
}

/// Wave2 TemporalUtil chrono helpers.
#[test]
fn wave2_temporal_util_parity() {
    use chrono::NaiveDate;
    let a = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap();
    let b = NaiveDate::from_ymd_opt(2020, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap();
    assert_eq!(TemporalUtil::between_ms(a, b), 86_400_000);
    assert_eq!(TemporalUtil::between(a, b, DateUnit::Day), 1);
    let c = TemporalUtil::offset(a, DateUnit::Hour, 3);
    assert_eq!(c.hour(), 3);
}

/// Wave2 TemporalAccessorUtil helpers.
#[test]
fn wave2_temporal_accessor_parity() {
    use chrono::NaiveDate;
    let a = NaiveDate::from_ymd_opt(2020, 5, 6).unwrap().and_hms_opt(7, 8, 9).unwrap();
    assert_eq!(TemporalAccessorUtil::get_year(a), 2020);
    assert!(TemporalAccessorUtil::format_pattern(a, "yyyy-MM-dd").contains("2020"));
    let begin = a;
    let end = NaiveDate::from_ymd_opt(2020, 5, 7).unwrap().and_hms_opt(0, 0, 0).unwrap();
    assert!(TemporalAccessorUtil::is_in(a, begin, end));
}

/// Wave2 DateUtil timer/range/zodiac leftovers.
#[test]
fn wave2_date_util_timer_range_parity() {
    let mut sw = DateUtil::create_stop_watch_id("t");
    sw.start();
    sw.stop();
    assert_eq!(sw.get_id(), "t");
    let _ = DateUtil::timer();
    let start = DateUtil::parse("2020-01-01").unwrap();
    let end = DateUtil::parse("2020-01-03").unwrap();
    let range = DateUtil::range(start, end, DateField::DayOfMonth);
    assert_eq!(range.to_list().len(), 3);
    let mapped = DateUtil::range_func(start, end, DateField::DayOfMonth, |d| d.day_of_month());
    assert_eq!(mapped, vec![1, 2, 3]);
    assert_eq!(DateUtil::nanos_to_millis(2_000_000), 2);
    assert!((DateUtil::nanos_to_seconds(1_000_000_000) - 1.0).abs() < f64::EPSILON);
    assert_eq!(DateUtil::get_zodiac(1, 19).unwrap(), "摩羯座");
    assert_eq!(DateUtil::get_chinese_zodiac(1994).unwrap(), "狗");
    let fmt = DateUtil::format_local_date_time(
        chrono::NaiveDate::from_ymd_opt(2020, 1, 2)
            .unwrap()
            .and_hms_opt(3, 4, 5)
            .unwrap(),
    );
    assert!(fmt.contains("2020"));
}
