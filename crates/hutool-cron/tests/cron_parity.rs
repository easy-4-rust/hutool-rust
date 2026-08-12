//! Cron parity tests —— 对齐 Hutool `hutool-cron` 测试。
//!
//! Inventory: 41 `@Test` methods under `hutool-cron/src/test`.
//! Sources:
//! - `cn.hutool.cron.pattern.*`
//! - `cn.hutool.cron.demo.CronTest`
//! - `cn.hutool.cron.TaskTableTest`

#![allow(missing_docs)]

use std::sync::Arc;

use chrono::{Datelike, Duration as ChronoDuration, NaiveDate, TimeZone, Timelike, Utc};
use hutool_cron::{
    CronPattern, CronPatternBuilder, CronPatternUtil, CronTask, CronUtil, Part, PatternParser,
    RunnableTask, Scheduler, SimpleTaskListener, TaskListener, TaskTable,
};

/// Parse Hutool-style local datetime strings used in Java tests (`yyyy-MM-dd HH:mm:ss` / short forms).
fn parse_local(s: &str) -> chrono::DateTime<Utc> {
    let formats = [
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d",
        "%H:%M:%S",
        "%Y-%m-%d %H:%M",
    ];
    for fmt in formats {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Utc.from_utc_datetime(&naive);
        }
        if fmt == "%Y-%m-%d"
            && let Ok(date) = NaiveDate::parse_from_str(s, fmt)
        {
            return Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap());
        }
        if fmt == "%H:%M:%S" {
            // Bare times in CronPatternTest use an arbitrary day; Hutool DateUtil.parse
            // resolves against "today". Use a fixed anchor day for determinism.
            if let Ok(time) = chrono::NaiveTime::parse_from_str(s, fmt) {
                let naive = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap().and_time(time);
                return Utc.from_utc_datetime(&naive);
            }
        }
    }
    panic!("unparseable datetime: {s}");
}

fn fmt_local(dt: chrono::DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Assert pattern matches with both second-match modes (Hutool `assertMatch`).
fn assert_match(pattern: &CronPattern, date: &str) {
    let instant = parse_local(date);
    assert!(
        pattern.matches(instant, false),
        "expected match(false) for {date} on {pattern}"
    );
    assert!(
        pattern.matches(instant, true),
        "expected match(true) for {date} on {pattern}"
    );
}

// ---------------------------------------------------------------------------
// CronPatternBuilderTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CronPatternBuilderTest.buildMatchAllTest()`
#[test]
fn cron_pattern_builder_build_match_all_test() {
    let build = CronPatternBuilder::of().build();
    assert_eq!(build, "* * * * *");

    let mut builder = CronPatternBuilder::of();
    builder.set(Part::Second, "*").unwrap();
    assert_eq!(builder.build(), "* * * * * *");

    let mut builder = CronPatternBuilder::of();
    builder.set(Part::Second, "*").unwrap();
    builder.set(Part::Year, "*").unwrap();
    assert_eq!(builder.build(), "* * * * * * *");
}

/// 对齐 Java: `CronPatternBuilderTest.buildRangeTest()`
#[test]
fn cron_pattern_builder_build_range_test() {
    let mut builder = CronPatternBuilder::of();
    builder.set(Part::Second, "*").unwrap();
    builder.set_range(Part::Hour, 2, 9).unwrap();
    assert_eq!(builder.build(), "* * 2-9 * * *");
}

/// 对齐 Java: `CronPatternBuilderTest.buildRangeErrorTest()`
#[test]
fn cron_pattern_builder_build_range_error_test() {
    let mut builder = CronPatternBuilder::of();
    builder.set(Part::Second, "*").unwrap();
    assert!(builder.set_range(Part::Hour, 2, 55).is_err());
}

// ---------------------------------------------------------------------------
// CronPatternTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CronPatternTest.matchAllTest()`
#[test]
fn cron_pattern_match_all_test() {
    let pattern = CronPattern::of("* * * * * *").unwrap();
    assert_match(&pattern, &fmt_local(Utc::now()));
}

/// 对齐 Java: `CronPatternTest.matchAllTest2()`
#[test]
fn cron_pattern_match_all_test2() {
    let pattern = CronPattern::of("* * * * *").unwrap();
    let now = Utc::now();
    let begin_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap();
    assert_match(&pattern, &fmt_local(begin_minute));
}

/// 对齐 Java: `CronPatternTest.cronPatternTest()`
#[test]
fn cron_pattern_cron_pattern_test() {
    let pattern = CronPattern::of("39 11 12 * * *").unwrap();
    assert_match(&pattern, "12:11:39");

    let pattern = CronPattern::of("39 */5 * * * *").unwrap();
    for minute in [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55] {
        assert_match(&pattern, &format!("12:{minute:02}:39"));
    }

    let pattern = CronPattern::of("39 1 2-4 * * *").unwrap();
    assert_match(&pattern, "02:01:39");
    assert_match(&pattern, "03:01:39");
    assert_match(&pattern, "04:01:39");

    let pattern = CronPattern::of("39 1 2,3,4 * * *").unwrap();
    assert_match(&pattern, "02:01:39");
    assert_match(&pattern, "03:01:39");
    assert_match(&pattern, "04:01:39");

    let pattern = CronPattern::of("39 0 0 6,7 8 *").unwrap();
    assert_match(&pattern, "2016-08-07 00:00:39");
    assert_match(&pattern, "2016-08-06 00:00:39");

    let pattern = CronPattern::of("39 0 0 6,7 Aug *").unwrap();
    assert_match(&pattern, "2016-08-06 00:00:39");
    assert_match(&pattern, "2016-08-07 00:00:39");

    let pattern = CronPattern::of("39 0 0 7 aug *").unwrap();
    assert_match(&pattern, "2016-08-07 00:00:39");
}

/// 对齐 Java: `CronPatternTest.matchDayOfWeekTest()`
#[test]
fn cron_pattern_match_day_of_week_test() {
    let pattern = CronPattern::of("39 0 0 * * Thu").unwrap();
    assert_match(&pattern, "2017-02-09 00:00:39");

    let pattern = CronPattern::of("39 0 0 * * Sun").unwrap();
    assert_match(&pattern, "2022-03-27 00:00:39");

    let pattern = CronPattern::of("39 0 0 * * 0").unwrap();
    assert_match(&pattern, "2022-03-27 00:00:39");

    let pattern = CronPattern::of("39 0 0 * * 7").unwrap();
    assert_match(&pattern, "2022-03-27 00:00:39");
}

/// 对齐 Java: `CronPatternTest.CronPatternTest2()`
#[test]
fn cron_pattern_cron_pattern_test2() {
    let pattern = CronPattern::of("0/30 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2018-10-09 12:00:00"), false));
    assert!(pattern.matches(parse_local("2018-10-09 12:30:00"), false));

    let pattern = CronPattern::of("32 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2018-10-09 12:32:00"), false));
}

/// 对齐 Java: `CronPatternTest.patternTest()`
#[test]
fn cron_pattern_pattern_test() {
    let pattern = CronPattern::of("* 0 4 * * ?").unwrap();
    assert_match(&pattern, "2017-02-09 04:00:00");
    assert_match(&pattern, "2017-02-19 04:00:33");

    let pattern = CronPattern::of("* 0 4 * * ?").unwrap();
    assert_match(&pattern, "2017-02-09 04:00:00");
    assert_match(&pattern, "2017-02-19 04:00:33");
}

/// 对齐 Java: `CronPatternTest.patternNegativeTest()`
#[test]
fn cron_pattern_pattern_negative_test() {
    let pattern = CronPattern::of("* 0 -4 * * ?").unwrap();
    assert_match(&pattern, "2017-02-09 19:00:00");
    assert_match(&pattern, "2017-02-19 19:00:33");
}

/// 对齐 Java: `CronPatternTest.rangePatternTest()`
#[test]
fn cron_pattern_range_pattern_test() {
    let pattern = CronPattern::of("* 20/2 * * * ?").unwrap();
    assert_match(&pattern, "2017-02-09 04:20:00");
    assert_match(&pattern, "2017-02-09 05:20:00");
    assert_match(&pattern, "2017-02-19 04:22:33");

    let pattern = CronPattern::of("* 2-20/2 * * * ?").unwrap();
    assert_match(&pattern, "2017-02-09 04:02:00");
    assert_match(&pattern, "2017-02-09 05:04:00");
    assert_match(&pattern, "2017-02-19 04:20:33");
}

/// 对齐 Java: `CronPatternTest.lastTest()`
#[test]
fn cron_pattern_last_test() {
    let pattern = CronPattern::of("* * * L * ?").unwrap();
    assert_match(&pattern, "2017-07-31 04:20:00");
    assert_match(&pattern, "2017-02-28 04:20:00");

    let pattern = CronPattern::of("* * * * L ?").unwrap();
    assert_match(&pattern, "2017-12-02 04:20:00");

    let pattern = CronPattern::of("L L L * * ?").unwrap();
    assert_match(&pattern, "2017-12-02 23:59:59");
}

/// 对齐 Java: `CronPatternTest.rangeYearTest()`
#[test]
fn cron_pattern_range_year_test() {
    assert!(CronPattern::of("0/1 * * * 1/1 ? 2020-2120").is_err());
}

// ---------------------------------------------------------------------------
// CronPatternNextMatchTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CronPatternNextMatchTest.nextMatchAllAfterTest()`
#[test]
fn cron_pattern_next_match_all_after_test() {
    let pattern = CronPattern::of("* * * * * * *").unwrap();
    let date = Utc::now().with_nanosecond(0).unwrap();
    let calendar = pattern.next_match_after(date, true).unwrap();
    assert_eq!(calendar.timestamp_millis(), date.timestamp_millis() + 1000);

    let pattern = CronPattern::of("0 * * * * * *").unwrap();
    let date = parse_local("2022-04-08 07:44:16");
    assert_eq!(
        fmt_local(pattern.next_match_after(date, true).unwrap()),
        "2022-04-08 07:45:00"
    );

    let pattern = CronPattern::of("0 0 * * * * *").unwrap();
    assert_eq!(
        fmt_local(pattern.next_match_after(date, true).unwrap()),
        "2022-04-08 08:00:00"
    );

    let pattern = CronPattern::of("0 0 0 * * * *").unwrap();
    assert_eq!(
        fmt_local(pattern.next_match_after(date, true).unwrap()),
        "2022-04-09 00:00:00"
    );

    let pattern = CronPattern::of("0 0 0 1 * * *").unwrap();
    assert_eq!(
        fmt_local(pattern.next_match_after(date, true).unwrap()),
        "2022-05-01 00:00:00"
    );
}

/// 对齐 Java: `CronPatternNextMatchTest.nextMatchAfterTest()`
#[test]
fn cron_pattern_next_match_after_test() {
    let pattern = CronPattern::of("23 12 * 12 * * *").unwrap();

    let calendar = pattern
        .next_match_after(parse_local("2022-04-12 09:12:12"), true)
        .unwrap();
    assert!(pattern.matches(calendar, true));
    assert_eq!(fmt_local(calendar), "2022-04-12 09:12:23");

    let calendar = pattern
        .next_match_after(parse_local("2022-04-12 09:09:24"), true)
        .unwrap();
    assert!(pattern.matches(calendar, true));
    assert_eq!(fmt_local(calendar), "2022-04-12 09:12:23");

    let calendar = pattern
        .next_match_after(parse_local("2022-04-12 09:12:24"), true)
        .unwrap();
    assert!(pattern.matches(calendar, true));
    assert_eq!(fmt_local(calendar), "2022-04-12 10:12:23");

    let calendar = pattern
        .next_match_after(parse_local("2022-04-13 09:12:24"), true)
        .unwrap();
    assert!(pattern.matches(calendar, true));
    assert_eq!(fmt_local(calendar), "2022-05-12 00:12:23");

    let calendar = pattern
        .next_match_after(parse_local("2021-12-22 00:00:00"), true)
        .unwrap();
    assert!(pattern.matches(calendar, true));
    assert_eq!(fmt_local(calendar), "2022-01-12 00:12:23");
}

/// 对齐 Java: `CronPatternNextMatchTest.nextMatchAfterByWeekTest()`
#[test]
fn cron_pattern_next_match_after_by_week_test() {
    let pattern = CronPattern::of("1 1 1 * * Sat *").unwrap();
    let time = parse_local("2022-04-03");
    let calendar = pattern.next_match_after(time, true).unwrap();
    assert_eq!(fmt_local(calendar), "2022-04-09 01:01:01");
}

fn last_day_of_month(dt: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let (y, m) = (dt.year(), dt.month());
    let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
    let first_next = Utc.with_ymd_and_hms(ny, nm, 1, 0, 0, 0).unwrap();
    first_next - ChronoDuration::days(1)
}

fn advance_last_day_result(result: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let next = result + ChronoDuration::days(1);
    let last = last_day_of_month(next);
    Utc.with_ymd_and_hms(last.year(), last.month(), last.day(), 3, 2, 1)
        .unwrap()
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryMonth1()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_month1() {
    let mut date = parse_local("2023-01-08 07:44:16");
    let mut result = parse_local("2023-01-31 03:02:01");
    let pattern = CronPattern::of("1 2 3 L * ?").unwrap();
    for _ in 0..30 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);
        result = advance_last_day_result(result);
    }
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryMonth2()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_month2() {
    let mut date = parse_local("2023-03-08 07:44:16");
    let mut result = parse_local("2023-03-31 03:02:01");
    let pattern = CronPattern::of("1 2 3 L * ?").unwrap();
    for _ in 0..30 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);
        result = advance_last_day_result(result);
    }
}

fn advance_feb_last_year(result: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let year = result.year() + 1;
    let last = last_day_of_month(Utc.with_ymd_and_hms(year, 2, 1, 0, 0, 0).unwrap());
    Utc.with_ymd_and_hms(last.year(), last.month(), last.day(), 3, 2, 1)
        .unwrap()
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryYear1()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_year1() {
    let mut date = parse_local("2023-01-08 07:44:16");
    let mut result = parse_local("2023-02-28 03:02:01");
    let pattern = CronPattern::of("1 2 3 L 2 ?").unwrap();
    for _ in 0..10 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);
        result = advance_feb_last_year(result);
    }
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryYear2()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_year2() {
    let mut date = parse_local("2022-03-08 07:44:16");
    let mut result = parse_local("2023-02-28 03:02:01");
    let pattern = CronPattern::of("1 2 3 L 2 ?").unwrap();
    for _ in 0..30 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);
        result = advance_feb_last_year(result);
    }
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryYear3()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_year3() {
    let date = parse_local("2022-03-08 07:44:16");
    let pattern = CronPattern::of("1 2 3 L 2 ?").unwrap();
    let calendar = pattern.next_match_after(date, true).unwrap();
    // Java only logs; assert the known next February last-day fire.
    assert_eq!(fmt_local(calendar), "2023-02-28 03:02:01");
}

/// 对齐 Java: `CronPatternNextMatchTest.testEveryHour()`
#[test]
fn cron_pattern_test_every_hour() {
    let mut date = parse_local("2022-02-28 07:44:16");
    let mut result = parse_local("2022-02-28 08:02:01");
    let pattern = CronPattern::of("1 2 */1 * * ?").unwrap();
    for _ in 0..30 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);
        result += ChronoDuration::hours(1);
    }
}

/// 对齐 Java: `CronPatternNextMatchTest.testLastDayOfMonthForEveryHour()`
#[test]
fn cron_pattern_test_last_day_of_month_for_every_hour() {
    let mut date = parse_local("2023-01-28 07:44:16");
    let mut result = parse_local("2023-01-31 00:00:00");
    let pattern = CronPattern::of("0 0 */1 L * ?").unwrap();
    for _ in 0..400 {
        let calendar = pattern.next_match_after(date, true).unwrap();
        date = calendar;
        assert_eq!(fmt_local(date), fmt_local(result));
        date += ChronoDuration::seconds(1);

        let t = result + ChronoDuration::hours(1);
        if t.day() == result.day() {
            result = t;
        } else {
            let next = result + ChronoDuration::days(1);
            let last = last_day_of_month(next);
            result = Utc
                .with_ymd_and_hms(last.year(), last.month(), last.day(), 0, 0, 0)
                .unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// CronPatternUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CronPatternUtilTest.matchedDatesTest()`
#[test]
fn cron_pattern_util_matched_dates_test() {
    let matched = CronPatternUtil::matched_dates_count(
        "0/30 * 8-18 * * ?",
        parse_local("2018-10-15 14:33:22"),
        5,
        true,
    )
    .unwrap();
    assert_eq!(matched.len(), 5);
    assert_eq!(fmt_local(matched[0]), "2018-10-15 14:33:30");
    assert_eq!(fmt_local(matched[1]), "2018-10-15 14:34:00");
    assert_eq!(fmt_local(matched[2]), "2018-10-15 14:34:30");
    assert_eq!(fmt_local(matched[3]), "2018-10-15 14:35:00");
    assert_eq!(fmt_local(matched[4]), "2018-10-15 14:35:30");
}

/// 对齐 Java: `CronPatternUtilTest.matchedDatesTest2()`
#[test]
fn cron_pattern_util_matched_dates_test2() {
    let matched = CronPatternUtil::matched_dates_count(
        "0 0 */1 * * *",
        parse_local("2018-10-15 14:33:22"),
        5,
        true,
    )
    .unwrap();
    assert_eq!(matched.len(), 5);
    assert_eq!(fmt_local(matched[0]), "2018-10-15 15:00:00");
    assert_eq!(fmt_local(matched[1]), "2018-10-15 16:00:00");
    assert_eq!(fmt_local(matched[2]), "2018-10-15 17:00:00");
    assert_eq!(fmt_local(matched[3]), "2018-10-15 18:00:00");
    assert_eq!(fmt_local(matched[4]), "2018-10-15 19:00:00");
}

/// 对齐 Java: `CronPatternUtilTest.matchedDatesTest3()`
#[test]
fn cron_pattern_util_matched_dates_test3() {
    let matched = CronPatternUtil::matched_dates_count(
        "0 0 */1 L * *",
        parse_local("2018-10-30 23:33:22"),
        5,
        true,
    )
    .unwrap();
    assert_eq!(matched.len(), 5);
    assert_eq!(fmt_local(matched[0]), "2018-10-31 00:00:00");
    assert_eq!(fmt_local(matched[1]), "2018-10-31 01:00:00");
    assert_eq!(fmt_local(matched[2]), "2018-10-31 02:00:00");
    assert_eq!(fmt_local(matched[3]), "2018-10-31 03:00:00");
    assert_eq!(fmt_local(matched[4]), "2018-10-31 04:00:00");
}

/// 对齐 Java: `CronPatternUtilTest.issue4056Test()`
#[test]
fn cron_pattern_util_issue4056_test() {
    let cron = "0 0 0 */5 * ? *";
    let pattern = CronPattern::of(cron).unwrap();
    assert!(!pattern.matches(parse_local("2025-02-28 00:00:00"), true));
    assert!(pattern.matches(parse_local("2025-03-01 00:00:00"), true));
    assert!(pattern.matches(parse_local("2025-03-31 00:00:00"), true));
}

/// 对齐 Java: `CronPatternUtilTest.issue4056Test2()`
#[test]
fn cron_pattern_util_issue4056_test2() {
    let pattern = CronPattern::of("0 0 0 */5 * ? *").unwrap();
    let next =
        CronPatternUtil::next_date_after(&pattern, parse_local("2025-02-27 23:59:59")).unwrap();
    assert_eq!(fmt_local(next), "2025-03-01 00:00:00");
}

// ---------------------------------------------------------------------------
// Issue* tests
// ---------------------------------------------------------------------------

/// 对齐 Java: `Issue3685Test.nextDateAfterTest()`
#[test]
fn issue3685_next_date_after_test() {
    let pattern = CronPattern::of("0 0 * * MON").unwrap();
    for (start, expected) in [
        ("2024-08-01", "2024-08-05 00:00:00"),
        ("2024-08-02", "2024-08-05 00:00:00"),
        ("2024-08-03", "2024-08-05 00:00:00"),
        ("2024-08-04", "2024-08-05 00:00:00"),
        ("2024-08-05", "2024-08-12 00:00:00"),
    ] {
        let date = CronPatternUtil::next_date_after(&pattern, parse_local(start)).unwrap();
        assert_eq!(fmt_local(date), expected);
    }
}

/// 对齐 Java: `Issue4006Test.testCron()`
#[test]
fn issue4006_test_cron() {
    let cron = "0 0 0 */1 * ? *";
    let pattern = CronPattern::of(cron).unwrap();
    let next = CronPatternUtil::next_date_after(&pattern, Utc::now());
    assert!(next.is_some(), "nextDateAfter should resolve for {cron}");
}

/// 对齐 Java: `Issue4056Test.testCronAll()`
#[test]
fn issue4056_test_cron_all() {
    let crons = [
        "0 0 0 * * ? *",
        "0 0 12 * * ? *",
        "0 0 18 * * ? *",
        "0 0 6,12,18 * * ? *",
        "0 0 */6 * * ? *",
        "0 30 */8 * * ? *",
        "0 */15 * * * ? *",
        "0 */5 9-17 * * ? *",
        "0 0 0-23/2 * * ? *",
        "0 0 0 */8 * ? *",
        "0 0 12 15 * ? *",
        "0 0 0 L * ? *",
        "0 0 0 29 2 ? *",
        "0 0 0 1 1 ? *",
        "0 0/30 * * * ? *",
        "0 0 */4 * * ? *",
        "0 0 0 1/3 * ? *",
        "0 0 2 28-31 * ? *",
        "0 0 0 1,15 * ? *",
        "0 0 0 1/5 * ? *",
        "0 0 0 1/10 * ? *",
        "0 0 0 1 */3 ? *",
        "0 0 0 25 12 ? *",
        "0 0 12 31 12 ? *",
        "0 0 0 14 2 ? *",
        "0 0 10 1 5 ? *",
        "0 0 9 8 3 ? *",
        "0 0 0 1 4 ? *",
        "0 0 12 4 7 ? *",
        "0 0 0 31 10 ? *",
        "0 7,19,31,43,55 * * * ? *",
        "0 */7 * * * ? *",
        "0 15-45/5 * * * ? *",
        "0 0-30/2 * * * ? *",
        "0 45 23 * * ? *",
        "0 59 23 * * ? *",
        "0 0 */3 * * ? *",
        "0 0 9-18/2 * * ? *",
        "0 0 22-2 * * ? *",
        "0 30 16 L * ? *",
    ];
    let judge_times = [
        "2025-02-01 18:20:10",
        "2024-02-29 10:00:00",
        "2025-12-31 23:59:59",
        "2025-01-01 00:00:00",
        "2025-06-15 12:00:00",
        "2025-03-30 00:00:00",
        "2025-02-28 23:59:59",
        "2025-03-01 00:00:00",
        "2025-01-31 23:59:59",
        "2025-04-30 23:59:59",
        "2025-06-30 23:59:59",
        "2025-09-30 23:59:59",
        "2026-01-01 00:00:00",
        "2024-02-28 00:00:00",
        "2024-02-29 00:00:00",
        "2024-02-29 23:59:59",
        "2023-02-28 23:59:59",
        "2028-02-29 12:00:00",
        "2025-06-15 00:00:00",
        "2025-06-15 23:59:59",
        "2025-03-31 23:59:59",
        "2025-04-01 00:00:00",
        "2025-07-01 00:00:00",
        "2025-10-01 00:00:00",
        "2025-01-06 09:00:00",
        "2025-01-10 17:00:00",
        "2025-01-11 12:00:00",
        "2025-01-12 12:00:00",
        "2025-03-09 01:59:59",
        "2025-03-09 03:00:00",
        "2025-11-02 01:59:59",
        "2025-11-02 01:00:00",
        "2024-12-31 23:59:59",
        "2024-01-01 00:00:00",
        "2026-12-31 23:59:59",
        "2026-01-01 00:00:00",
        "2025-05-15 08:45:30",
        "2025-08-22 14:20:15",
        "2025-11-03 19:10:45",
        "2025-02-14 09:30:00",
        "2025-07-07 07:07:07",
        "2025-09-09 09:09:09",
        "2025-10-10 10:10:10",
        "2025-12-12 12:12:12",
        "2025-03-03 03:03:03",
        "2025-06-06 06:06:06",
        "2025-04-16 00:00:00",
        "2025-04-30 23:59:59",
        "2025-05-01 00:00:00",
        "2025-05-01 00:00:01",
    ];

    for cron in crons {
        let pattern = CronPattern::of(cron).unwrap_or_else(|e| panic!("parse {cron}: {e}"));
        for judge in judge_times {
            let start = parse_local(judge);
            let next = CronPatternUtil::next_date_after(&pattern, start)
                .unwrap_or_else(|| panic!("no next for cron={cron} after={judge}"));
            assert!(
                next > start,
                "next must be after start: cron={cron} start={judge} next={}",
                fmt_local(next)
            );
            assert!(
                pattern.matches(next, true),
                "next must match pattern: cron={cron} next={}",
                fmt_local(next)
            );
        }
    }
}

/// 对齐 Java: `Issue4056Test.issue4056Test()`
#[test]
fn issue4056_issue4056_test() {
    let pattern = CronPattern::of("0 0 0 1/3 * ? *").unwrap();
    let next =
        CronPatternUtil::next_date_after(&pattern, parse_local("2025-02-28 00:00:00")).unwrap();
    // Java only prints; assert a deterministic next.
    assert_eq!(fmt_local(next), "2025-03-01 00:00:00");
}

/// 对齐 Java: `IssueI7SMP7Test.parseTest()`
#[test]
fn issue_i7smp7_parse_test() {
    let parse = PatternParser::parse("0 0 3 1 1 ? */1").unwrap();
    assert!(!parse.is_empty());
}

/// 对齐 Java: `IssueI82CSHTest.test()`
#[test]
fn issue_i82csh_test() {
    let begin = parse_local("2023-09-20");
    let end = parse_local("2025-09-20");
    let dates = CronPatternUtil::matched_dates_str("0 0 1 3-3,9 *", begin, end, 20, false).unwrap();
    assert_eq!(dates.len(), 4);
}

/// 对齐 Java: `IssueI9FQUATest.nextDateAfterTest()`
#[test]
fn issue_i9fqua_next_date_after_test() {
    let cron = "0/5 * * * * ?";
    let calendar = CronPattern::of(cron)
        .unwrap()
        .next_match_after(parse_local("2024-01-01 00:00:00"), true)
        .unwrap();
    assert_eq!(fmt_local(calendar), "2024-01-01 00:00:05");
}

// ---------------------------------------------------------------------------
// TaskTableTest / CronTest (Disabled demos → non-hanging API parity)
// ---------------------------------------------------------------------------

/// 对齐 Java: `TaskTableTest.toStringTest()`
#[test]
fn task_table_to_string_test() {
    let mut table = TaskTable::new();
    let task = Arc::new(RunnableTask::new(|| ()));
    table
        .add(CronTask::new(
            "id-1",
            CronPattern::of("*/10 * * * * *").unwrap(),
            Arc::clone(&task) as Arc<dyn hutool_cron::Task>,
        ))
        .unwrap();
    table
        .add(CronTask::new(
            "id-2",
            CronPattern::of("*/20 * * * * *").unwrap(),
            Arc::clone(&task) as Arc<dyn hutool_cron::Task>,
        ))
        .unwrap();
    table
        .add(CronTask::new(
            "id-3",
            CronPattern::of("*/30 * * * * *").unwrap(),
            task as Arc<dyn hutool_cron::Task>,
        ))
        .unwrap();
    let rendered = table.to_string();
    assert!(rendered.contains("id-1"));
    assert!(rendered.contains("*/10 * * * * *"));
    assert_eq!(table.len(), 3);
}

/// 对齐 Java: `CronTest.emptyScheduleTest()`
#[tokio::test]
async fn cron_test_empty_schedule_test() {
    let mut scheduler = Scheduler::new();
    scheduler
        .set_runtime(tokio::runtime::Handle::current())
        .unwrap();
    scheduler.set_match_second(true).unwrap();
    scheduler.start().unwrap();
    scheduler.stop(true);
}

/// 对齐 Java: `CronTest.customCronTest()`
#[tokio::test]
async fn cron_test_custom_cron_test() {
    let mut util = CronUtil::new();
    util.scheduler_mut()
        .set_runtime(tokio::runtime::Handle::current())
        .unwrap();
    util.schedule("*/2 * * * * *", RunnableTask::new(|| ()))
        .unwrap();
    util.set_match_second(true).unwrap();
    util.start().unwrap();
    util.stop();
}

/// 对齐 Java: `CronTest.cronTest()`
#[tokio::test]
async fn cron_test_cron_test() {
    let mut util = CronUtil::new();
    util.scheduler_mut()
        .set_runtime(tokio::runtime::Handle::current())
        .unwrap();
    util.set_match_second(true).unwrap();
    util.scheduler_mut().set_daemon(false);
    util.start().unwrap();
    util.stop();
}

/// 对齐 Java: `CronTest.cronWithListenerTest()`
#[tokio::test]
async fn cron_test_cron_with_listener_test() {
    let mut util = CronUtil::new();
    util.scheduler_mut()
        .set_runtime(tokio::runtime::Handle::current())
        .unwrap();
    let listener: Arc<dyn TaskListener> = Arc::new(SimpleTaskListener);
    util.scheduler().add_listener(listener);
    util.set_match_second(true).unwrap();
    util.start().unwrap();
    util.stop();
}

/// 对齐 Java: `CronTest.addAndRemoveTest()`
#[tokio::test]
async fn cron_test_add_and_remove_test() {
    let mut util = CronUtil::new();
    util.scheduler_mut()
        .set_runtime(tokio::runtime::Handle::current())
        .unwrap();
    let id = util
        .schedule("*/2 * * * * *", RunnableTask::new(|| ()))
        .unwrap();
    assert!(util.remove(&id));
    util.set_match_second(true).unwrap();
    util.start().unwrap();
    util.stop();
}

/// Keep smoke coverage from the original scaffold.
#[test]
fn cron_schedule_parse_test() {
    assert!(hutool_cron::CronSchedule::parse("* * * * * *").is_ok());
}

/// Keep smoke coverage from the original scaffold.
#[test]
fn invoke_registry_test() {
    let _registry = hutool_cron::InvokeRegistry::new();
}
