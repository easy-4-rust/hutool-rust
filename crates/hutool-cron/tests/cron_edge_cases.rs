//! hutool-cron 边界分支覆盖测试（对齐 Java `PartParser` 语义）。
//!
//! 覆盖 `pattern/mod.rs` 的 L 哨兵、负值、逆向范围、别名、步进等
//! 展开分支，以及 `CronPattern`/`CronPatternUtil`/`DayOfMonthMatcher`
//! 的错误与边界路径。Java 语义依据 `PartParser.parseNumber/parseRange/parseAlias`：
//! - 负值 `i += part.getMax()`（hour `-4` → 19）
//! - `L` 表示字段最大值（DOM=32 月末哨兵、DayOfWeek=6 周六）
//! - 逆向范围 `5-2` → `[5..max] + [min..2]`
//! - 周日 `0`/`7` 统一（Hutool 7→0；Quartz 周日=1）

use chrono::{Datelike, TimeZone, Utc};
use hutool_cron::CronPatternUtil;
use hutool_cron::pattern::{CronPattern, DayOfMonthMatcher, Part, PatternParser, PatternUtil};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn parse_local(s: &str) -> chrono::DateTime<Utc> {
    // 与 cron_parity.rs 相同的解析约定
    let (date_part, time_part) = s.split_once(' ').unwrap_or((s, "00:00:00"));
    let mut ymd = date_part.split('-');
    let year: i32 = ymd.next().unwrap().parse().unwrap();
    let month: u32 = ymd.next().unwrap().parse().unwrap();
    let day: u32 = ymd.next().unwrap().parse().unwrap();
    let mut hms = time_part.split(':');
    let hour: u32 = hms.next().unwrap().parse().unwrap();
    let minute: u32 = hms.next().unwrap().parse().unwrap();
    let second: u32 = hms.next().unwrap_or("0").parse().unwrap();
    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .single()
        .unwrap()
}

fn fmt_local(dt: chrono::DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// L 哨兵（月末）：`"0 0 L * *"` 匹配每月最后一天
#[test]
fn last_day_sentinel_expands_to_month_end() {
    let pattern = CronPattern::of("0 0 L * *").unwrap();
    // 1 月 31 日匹配
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    // 2 月（平年）28 日匹配、29 日不匹配
    assert!(pattern.matches(parse_local("2026-02-28 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-02-27 00:00:00"), true));
    // 闰年 2 月 29 日匹配
    assert!(pattern.matches(parse_local("2028-02-29 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-30 00:00:00"), true));

    // next_match_after 经过 L 展开（触发 dom_last 过滤 + is_last_day_of_month）
    let next = pattern
        .next_match_after(parse_local("2026-01-01 00:00:00"), true)
        .unwrap();
    assert_eq!(fmt_local(next), "2026-01-31 00:00:00");
    // 从 2 月中旬起，下一匹配是 2 月 28 日
    let next = pattern
        .next_match_after(parse_local("2026-02-01 00:00:00"), true)
        .unwrap();
    assert_eq!(fmt_local(next), "2026-02-28 00:00:00");
    // 闰年 2 月 → 29 日
    let next = pattern
        .next_match_after(parse_local("2028-02-01 00:00:00"), true)
        .unwrap();
    assert_eq!(fmt_local(next), "2028-02-29 00:00:00");
}

/// DOM 直接写 32（L 哨兵值）：`"0 0 32 * *"` 等价于月末
#[test]
fn dom_sentinel_value_32_is_last_day() {
    let pattern = CronPattern::of("0 0 32 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
}

/// 负值：`-4` 小时 → `max + (-4)` = 23 - 4 = 19（对齐 Java `i += part.getMax()`）
#[test]
fn negative_value_adds_field_max() {
    // 分钟 -4 → 59 - 4 = 55（6 段表达式：秒 分 时 日 月 周）
    let pattern = CronPattern::of("0 -4 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 00:55:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 00:54:00"), true));
    // 小时 -4 → 23 - 4 = 19
    let pattern = CronPattern::of("0 0 -4 * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 19:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 20:00:00"), true));
}

/// 逆向范围：`22-2` → `[22..23] + [0..2]`（对齐 Java 反选模式）
#[test]
fn reverse_range_wraps_around() {
    let pattern = CronPattern::of("0 0 22-2 * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 22:00:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 23:00:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 02:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 03:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 21:00:00"), true));
}

/// 月别名：`JAN` → 1 月（对齐 Java parseAlias MONTH）
#[test]
fn month_alias_parses() {
    let pattern = CronPattern::of("0 0 1 JAN *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-02-01 00:00:00"), true));
    // 小写别名同样支持（Java equalsIgnoreCase）
    let pattern = CronPattern::of("0 0 1 dec *").unwrap();
    assert!(pattern.matches(parse_local("2026-12-01 00:00:00"), true));
}

/// 星期 L：字段最大值 = 6（周六，对齐 Java `L` → `part.getMax()`）
#[test]
fn day_of_week_l_is_saturday() {
    let pattern = CronPattern::of("0 0 * * L").unwrap();
    // 2026-08-01 是周六
    assert!(pattern.matches(parse_local("2026-08-01 00:00:00"), true));
    // 2026-08-02 是周日，不匹配
    assert!(!pattern.matches(parse_local("2026-08-02 00:00:00"), true));
}

/// 星期别名范围：`MON-FRI` 工作日（对齐 Java parseAlias `DAY_OF_WEEK` + 逆向/正向范围）
#[test]
fn day_of_week_alias_range() {
    let pattern = CronPattern::of("0 0 * * MON-FRI").unwrap();
    // 2026-08-03 周一
    assert!(pattern.matches(parse_local("2026-08-03 00:00:00"), true));
    // 2026-08-07 周五
    assert!(pattern.matches(parse_local("2026-08-07 00:00:00"), true));
    // 2026-08-08 周六不匹配
    assert!(!pattern.matches(parse_local("2026-08-08 00:00:00"), true));
    // 2026-08-02 周日不匹配
    assert!(!pattern.matches(parse_local("2026-08-02 00:00:00"), true));
}

/// 星期别名步进：`SUN/2` → 周日 0 起每 2 天（0,2,4,6 → 周日、周二、周四、周六）
#[test]
fn day_of_week_alias_step() {
    let pattern = CronPattern::of("0 0 * * SUN/2").unwrap();
    // 2026-08-02 周日（0）
    assert!(pattern.matches(parse_local("2026-08-02 00:00:00"), true));
    // 2026-08-04 周二（2）
    assert!(pattern.matches(parse_local("2026-08-04 00:00:00"), true));
    // 2026-08-03 周一（1）不匹配
    assert!(!pattern.matches(parse_local("2026-08-03 00:00:00"), true));
}

/// 单值步进：`5/10` → 从 5 起每 10（5,15,25；对齐 Java `20/2` 形式）
#[test]
fn single_value_with_step() {
    let pattern = CronPattern::of("0 5/10 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 00:05:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 00:15:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 00:25:00"), true));
    // Java appendRange(5, max=59, 10)：5,15,25,35,45,55
    assert!(pattern.matches(parse_local("2026-01-01 00:35:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 00:55:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 00:59:00"), true));
}

/// 非法步进：`*/0` 与 `5/0` 均报错（Java `Non positive divisor`）
#[test]
fn non_positive_step_rejected() {
    assert!(CronPattern::of("0 0 */0 * *").is_err());
    assert!(CronPattern::of("0 5/0 * * *").is_err());
    assert!(CronPattern::of("0 0 5/0 * *").is_err());
}

/// 5 段表达式（无秒）按秒=0、年=* 补全（对齐 Java 5 段支持）
#[test]
fn five_field_expression_padded() {
    let pattern = CronPattern::of("0 0 1 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 00:00:01"), true));
}

/// 多表达式备选（`|` 分隔）：任一匹配即可
#[test]
fn multi_expression_alternatives() {
    let pattern = CronPattern::of("0 0 6 * * * | 0 0 18 * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 06:00:00"), true));
    assert!(pattern.matches(parse_local("2026-01-01 18:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-01 12:00:00"), true));
    // 空备选报错
    assert!(CronPattern::of("0 0 6 * * |").is_err());
}

/// `matches_millis` 非法时间戳报错
#[test]
fn matches_millis_out_of_range() {
    let pattern = CronPattern::of("* * * * *").unwrap();
    assert!(pattern.matches_millis(i64::MAX, true).is_err());
    assert!(pattern.matches_millis(0, true).is_ok());
}

/// `next_match` 起始时刻即匹配时返回该时刻（Java nextMatch 含起点）
#[test]
fn next_match_includes_start_when_matching() {
    let pattern = CronPattern::of("0 0 12 * * *").unwrap();
    let start = parse_local("2026-01-01 12:00:00");
    assert_eq!(pattern.next_match(start, true), Some(start));
    // 不匹配时返回之后的首次匹配
    let next = pattern
        .next_match(parse_local("2026-01-01 11:59:59"), true)
        .unwrap();
    assert_eq!(fmt_local(next), "2026-01-01 12:00:00");
}

/// `matched_dates` 的 start > end 报错（Java 校验）
#[test]
fn matched_dates_rejects_reversed_range() {
    let pattern = CronPattern::of("0 0 12 * * *").unwrap();
    let start = parse_local("2026-02-01 00:00:00");
    let end = parse_local("2026-01-01 00:00:00");
    assert!(CronPatternUtil::matched_dates(&pattern, start, end, 10, true).is_err());
}

/// `next_date_after` 默认按秒精度（Java nextDateAfter 两参重载）
#[test]
fn next_date_after_default_precision() {
    let pattern = CronPattern::of("0 0 12 * * *").unwrap();
    let next = CronPatternUtil::next_date_after(&pattern, parse_local("2026-01-01 00:00:00"));
    assert_eq!(fmt_local(next.unwrap()), "2026-01-01 12:00:00");
}

/// `DayOfMonthMatcher` 的月份上下文方法（Java getMinValue/getMaxValue）
#[test]
fn day_of_month_context_methods() {
    // 具体值 {1, 15} + L
    let matcher = DayOfMonthMatcher::new([1, 15, 32]).unwrap();
    // 4 月（30 天）最小匹配 1
    assert_eq!(matcher.min_value(4, false), 1);
    // 4 月最大匹配：15 与月末 30 → 30
    assert_eq!(matcher.max_value(4, false), 30);
    // 2 月平年：{1,15,28} → 28
    assert_eq!(matcher.max_value(2, false), 28);
    // 2 月闰年：{1,15,29} → 29
    assert_eq!(matcher.max_value(2, true), 29);
    // 1 月：{1,15,31} → 31
    assert_eq!(matcher.max_value(1, false), 31);
}

/// `DayOfMonthMatcher` 的 `PartMatcher` 接口（32 哨兵 + `next_after`）
#[test]
fn day_of_month_part_matcher_interface() {
    let matcher = DayOfMonthMatcher::new([1, 32]).unwrap();
    use hutool_cron::pattern::PartMatcher;
    assert!(matcher.matches(1));
    assert!(matcher.matches(32));
    assert!(!matcher.matches(2));
    // values=[1]（32 为 last 标志）：next_after 在 1 参接口下按具体值回绕
    assert_eq!(matcher.next_after(1), 1);
    assert_eq!(matcher.next_after(2), 1);
    assert_eq!(matcher.next_after(33), 1);
}

/// Part 全字段 `calendar_field/min/max（对齐` Java Part 枚举）
#[test]
fn part_all_fields_ranges() {
    assert_eq!(Part::Minute.calendar_field(), 1);
    assert_eq!(Part::Hour.calendar_field(), 2);
    assert_eq!(Part::DayOfMonth.calendar_field(), 3);
    assert_eq!(Part::Month.calendar_field(), 4);
    assert_eq!(Part::DayOfWeek.calendar_field(), 5);
    assert_eq!(Part::Second.min(), 0);
    assert_eq!(Part::Minute.min(), 0);
    assert_eq!(Part::Hour.min(), 0);
    assert_eq!(Part::DayOfWeek.min(), 0);
    assert_eq!(Part::Month.min(), 1);
    assert_eq!(Part::Month.max(), 12);
}

/// `PatternUtil::get_fields` 公有函数（对齐 Hutool 字段提取）
#[test]
fn pattern_util_fields_function() {
    let dt = parse_local("2026-08-05 08:09:10");
    let fields = PatternUtil::get_fields(dt.naive_utc(), true);
    assert_eq!(fields, [10, 9, 8, 5, 8, 3, 2026]);
    // 不匹配秒字段时为 0（Java isMatchSecond=false 时 second=-1，Rust 内部以 0 参与）
    let fields = PatternUtil::get_fields(dt.naive_utc(), false);
    assert_eq!(fields[0], -1);
}

/// `PatternParser` 段数校验（5/6/7 段合法，其余报错）
#[test]
fn pattern_parser_field_count_validation() {
    assert!(!PatternParser::parse("0 0 1 * *").unwrap().is_empty());
    assert!(!PatternParser::parse("0 0 1 * * *").unwrap().is_empty());
    assert!(!PatternParser::parse("0 0 1 * * * *").unwrap().is_empty());
    assert!(PatternParser::parse("0 0 1").is_err());
    assert!(PatternParser::parse("").is_err());
}

/// 非法表达式整体报错（Java `CronException` 语义）
#[test]
fn invalid_expressions_rejected() {
    assert!(CronPattern::of("not a cron").is_err());
    assert!(CronPattern::of("").is_err());
    // 小时越界
    assert!(CronPattern::of("0 0 24 * * *").is_err());
    // 星期 8 越界（Java DAY_OF_WEEK max=6，8 无 7→0 转换）
    assert!(CronPattern::of("0 0 * * 8").is_err());
    // "7" 合法：Java parseNumber 把 iso8601 周日 7 转为 0（周日）；
    // 7 段表达式 dow 在第 6 位（sec min hour dom month dow year）
    let sunday = CronPattern::of("0 0 * * * 7 *").unwrap();
    // 2026-08-02 是周日
    assert!(sunday.matches(parse_local("2026-08-02 00:00:00"), true));
    assert!(!sunday.matches(parse_local("2026-08-03 00:00:00"), true));
}

/// 年末边界（CronPatternUtil.matchedDatesCount 的 `end_of_year`）
#[test]
fn matched_dates_count_ends_at_year_end() {
    let start = parse_local("2026-01-01 00:00:00");
    let dates = CronPatternUtil::matched_dates_count("0 0 1 * *", start, 3, true).unwrap();
    assert_eq!(dates.len(), 3);
    // 5 段表达式 = [分, 时, 日, 月, 周]：min=0 hour=0 dom=1 → 每月 1 日 00:00
    assert_eq!(fmt_local(dates[0]), "2026-01-01 00:00:00");
    assert_eq!(fmt_local(dates[1]), "2026-02-01 00:00:00");
    assert_eq!(fmt_local(dates[2]), "2026-03-01 00:00:00");
}

/// 直接从时间提取字段（mod.rs pub fields）
#[test]
fn extract_fields_from_timestamp() {
    let dt = parse_local("2026-08-05 08:09:10");
    let fields = hutool_cron::pattern::fields(&dt, true);
    assert_eq!(fields, [10, 9, 8, 5, 8, 3, 2026]);
    let fields = hutool_cron::pattern::fields(&dt, false);
    assert_eq!(fields, [0, 9, 8, 5, 8, 3, 2026]);
}

/// `next_date_after_with_precision（3` `参重载，match_second=false`）
#[test]
fn next_date_after_with_precision_overload() {
    let pattern = CronPattern::of("0 0 12 * * *").unwrap();
    let next = CronPatternUtil::next_date_after_with_precision(
        &pattern,
        parse_local("2026-01-01 00:00:00"),
        false,
    );
    assert_eq!(fmt_local(next.unwrap()), "2026-01-01 12:00:00");
}

/// `matched_dates` 的窗口结束中断（next > end 且 count 未满）
#[test]
fn matched_dates_stops_at_window_end() {
    let pattern = CronPattern::of("0 0 12 * * *").unwrap();
    let start = parse_local("2026-01-01 00:00:00");
    // 窗口只到 1 月 1 日 12:00:30，count 很大 → 只返回 1 个匹配
    let end = parse_local("2026-01-01 12:00:30");
    let dates = CronPatternUtil::matched_dates(&pattern, start, end, 100, true).unwrap();
    assert_eq!(dates.len(), 1);
    assert_eq!(fmt_local(dates[0]), "2026-01-01 12:00:00");
}

/// 星期逗号列表别名："MON,TUE"（对齐 Java parseAlias 列表）
#[test]
fn day_of_week_alias_list() {
    // 6 段：sec min hour dom month dow，DOW 在第 6 位
    let pattern = CronPattern::of("0 0 * * * MON,TUE").unwrap();
    // 2026-08-03 周一
    assert!(pattern.matches(parse_local("2026-08-03 00:00:00"), true));
    // 2026-08-04 周二
    assert!(pattern.matches(parse_local("2026-08-04 00:00:00"), true));
    // 2026-08-05 周三不匹配
    assert!(!pattern.matches(parse_local("2026-08-05 00:00:00"), true));
}

/// 混合字段："*/5,L" 触发 base=="*" 展开路径（mod.rs `expand_field`）
#[test]
fn mixed_star_step_and_last_day() {
    // dom 字段 "*/5,L"：从 min=1 起每 5 天（1,6,11,16,21,26,31）+ 月末（28..=31）
    let pattern = CronPattern::of("0 0 0 */5,L * *").unwrap();
    // 2026-01-06（1+5）
    assert!(pattern.matches(parse_local("2026-01-06 00:00:00"), true));
    // 2026-01-11
    assert!(pattern.matches(parse_local("2026-01-11 00:00:00"), true));
    // 月末：2026-01-31
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    // 非匹配日
    assert!(!pattern.matches(parse_local("2026-01-05 00:00:00"), true));
}

/// DOM "L,32" 组合：收集循环中 32 再次出现（mod.rs `has_last` 去重）
#[test]
fn dom_last_and_sentinel_combo() {
    let pattern = CronPattern::of("0 0 0 L,32 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-02-28 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-01-30 00:00:00"), true));
}

/// `PartParser` 直接解析非法步进（Java Non positive divisor）
#[test]
fn part_parser_rejects_non_positive_step() {
    use hutool_cron::pattern::PartParser;
    assert!(PartParser::new(Part::Minute).parse("5/0").is_err());
    assert!(PartParser::new(Part::Minute).parse("5/-2").is_err());
}

/// `SystemTimer::default()（Java` 无参构造）
#[test]
fn system_timer_default_constructs() {
    let timer = hutool_cron::timingwheel::SystemTimer::default();
    assert!(!timer.is_started());
}

/// `SystemTimer` 多任务：不同截止时间触发 `ScheduledTask` 排序（BinaryHeap cmp）
#[test]
fn system_timer_multiple_tasks_ordered() {
    use hutool_cron::timingwheel::{SystemTimer, TimerTask};
    let order = Arc::new(Mutex::new(Vec::new()));
    let mut timer = SystemTimer::new();
    for (label, delay_ms) in [("third", 60_u64), ("first", 10), ("second", 30)] {
        let slot = Arc::clone(&order);
        timer
            .add_task(TimerTask::new(
                move || slot.lock().unwrap().push(label.to_string()),
                Duration::from_millis(delay_ms),
            ))
            .unwrap();
    }
    timer.start().unwrap();
    awaitility::at_most(Duration::from_secs(3))
        .poll_interval(Duration::from_millis(5))
        .until(|| order.lock().unwrap().len() >= 3);
    timer.stop();
    let executed = order.lock().unwrap().clone();
    assert_eq!(executed, vec!["first", "second", "third"]);
}

/// Java `DayOfMonthMatcher.match(day, month, leap) = super.match(day) || matchLastDay`：
/// L 与显式值是 OR 关系——显式值在任何月份直接匹配，只有 L 候选按当月最后一天过滤。
#[test]
fn dom_last_or_semantics_with_explicit_values() {
    // "*/5,L" 的显式集 {1,6,11,16,21,26,31}：4 月 6 日（非月末）必须匹配
    let pattern = CronPattern::of("0 0 0 */5,L * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-06 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-16 00:00:00"), true));
    // 4 月 26 也是显式值
    assert!(pattern.matches(parse_local("2026-04-26 00:00:00"), true));
    // L 候选：4 月 30 是最后一天
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    // 显式值之外的 L 候选（4 月 29 非最后一天）不匹配
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
    // 闰年 2 月 29 是最后一天
    assert!(pattern.matches(parse_local("2024-02-29 00:00:00"), true));
    // 闰年 2 月 26 是显式值
    assert!(pattern.matches(parse_local("2024-02-26 00:00:00"), true));
}

/// "28,L"：显式 28 任意月份直接匹配（4 月 28 非月末也匹配），
/// L 候选 29/30/31 仅在对应月为最后一天时匹配。
#[test]
fn dom_last_explicit_28_matches_any_month() {
    let pattern = CronPattern::of("0 0 0 28,L * *").unwrap();
    // 显式 28：4 月 28 不是月末，但仍匹配
    assert!(pattern.matches(parse_local("2026-04-28 00:00:00"), true));
    // L 候选 29：4 月 29 不是最后一天
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
    // L：4 月 30 是最后一天
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    // L：1 月 31 是最后一天
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    // L：2 月 28（非闰年最后一天）与显式 28 重合
    assert!(pattern.matches(parse_local("2026-02-28 00:00:00"), true));
}

/// `next_match` 走 `next_after_filtered：显式值（4` 月 6 日）不被月末过滤误杀。
#[test]
fn dom_last_next_match_respects_explicit_values() {
    let pattern = CronPattern::of("0 0 0 */5,L * *").unwrap();
    // 从 2026-04-02 起（4 月 1 日是显式值，直接给 2 日）应命中 4 月 6 日
    let next = pattern
        .next_match(parse_local("2026-04-02 00:00:00"), true)
        .unwrap();
    assert_eq!(next.day(), 6, "显式值 6 应被 next_match 命中，got {next}");
    // 从 4 月 27 起：28 不是显式值也不是 4 月末 → 应跳过到 4 月 30（L）
    let next2 = pattern
        .next_match(parse_local("2026-04-27 00:00:00"), true)
        .unwrap();
    assert_eq!(next2.day(), 30, "L 候选应命中 4 月 30，got {next2}");
    // 从 4 月 30 起：跳到 5 月 1 日（显式值）
    let next3 = pattern
        .next_match(parse_local("2026-04-30 01:00:00"), true)
        .unwrap();
    assert_eq!((next3.month(), next3.day()), (5, 1), "got {next3}");
}

/// Java parseRange 全匹配步进 `*/n` 使用 part.getMax()（DOM=32 哨兵）：
/// `*/31` → {1, 32} = 1 日 + 每月最后一天（31%31==0，不能交给调度引擎）。
#[test]
fn dom_star_step_hits_sentinel_31() {
    let pattern = CronPattern::of("0 0 0 */31 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-01 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    // 30 天月份的最后一天
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-15 00:00:00"), true));
}

/// Java 单值步进 `20/2` → NumberUtil.appendRange(20, 32, 2) = {20..32 step2}，
/// 32 哨兵 → 最后一天。非末月的 29/30 中只有 30 是显式值。
#[test]
fn dom_single_step_hits_sentinel() {
    let pattern = CronPattern::of("0 0 0 20/2 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-20 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-28 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-21 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
}

/// Java 范围 `5-32` → appendRange(5, 32, 1) = {5..32}：5..31 全显式 + 最后一天。
#[test]
fn dom_range_end_sentinel() {
    let pattern = CronPattern::of("0 0 0 5-32 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-05 00:00:00"), true));
    // 29 是显式值（5..31），任意月份匹配
    assert!(pattern.matches(parse_local("2026-04-29 00:00:00"), true));
    // 2 月 28 是最后一天（哨兵）
    assert!(pattern.matches(parse_local("2026-02-28 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-03 00:00:00"), true));
}

/// Java 单独 `32` = L 哨兵：仅最后一天。
#[test]
fn dom_standalone_sentinel() {
    let pattern = CronPattern::of("0 0 0 32 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-01-31 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(pattern.matches(parse_local("2024-02-29 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
}

/// Java 逆向 `32-5` → appendRange(32, 32) + appendRange(1, 5) = {32, 1..5}。
#[test]
fn dom_reverse_range_begin_sentinel() {
    let pattern = CronPattern::of("0 0 0 32-5 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-01 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-05 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-06 00:00:00"), true));
}

/// 非 DOM 的 `*/n` 步进（31%n!=0 时早退给调度引擎）：
/// Minute `*/31` → {0, `31}，expand_range` 走 `part.max()` else 分支。
#[test]
fn minute_star_step_preserved() {
    let pattern = CronPattern::of("0 */31 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-01 00:31:00"), true));
    assert!(pattern.matches(parse_local("2026-04-01 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-01 00:15:00"), true));
    let pattern = CronPattern::of("0 */5 * * * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-01 00:05:00"), true));
}

/// `expand_field` 展开路径的非正步进拒绝（Java `Non positive divisor`）：
/// 列表项 `L/0`、`5/0` 在 step<=0 检查处报错。
#[test]
fn expand_field_rejects_non_positive_step_in_list() {
    assert!(CronPattern::of("0 0 0 L/0 * *").is_err());
    assert!(CronPattern::of("0 0 0 5/0,L * *").is_err());
    assert!(CronPattern::of("0 0 0 5/-2 * *").is_err());
}

/// `split_numeric_range` 端点非数字 → 不展开 → cron 拒绝（InvalidPattern）。
#[test]
fn non_numeric_range_endpoint_rejected() {
    assert!(CronPattern::of("0 0 0 1-A * *").is_err());
    assert!(CronPattern::of("0 0 0 A-1 * *").is_err());
}

/// `expand_range` 的 DOM 越界拒绝（begin/end 不在 1..=32）。
#[test]
fn dom_range_out_of_bounds_rejected() {
    assert!(CronPattern::of("0 0 0 34-33 * *").is_err());
    assert!(CronPattern::of("0 0 0 33-5 * *").is_err());
    assert!(CronPattern::of("0 0 0 0-1 * *").is_err());
}

/// `checked_schedule_value` 对 DOM 32 哨兵兜底（Java checkValue 通过）。
#[test]
fn checked_dom_sentinel_value_ok() {
    let pattern = CronPattern::of("0 0 0 32 * *").unwrap();
    // 32 哨兵等价于 L：4 月最后一天
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
}

/// `matched_dates` 循环中 `next_match_after` 无候选时 break（2 月 30 日不存在）。
#[test]
fn matched_dates_breaks_when_no_next() {
    let pattern = CronPattern::of("0 0 0 30 FEB * *").unwrap();
    let dates = CronPatternUtil::matched_dates(
        &pattern,
        parse_local("2026-01-01 00:00:00"),
        parse_local("2026-12-31 00:00:00"),
        5,
        true,
    )
    .unwrap();
    assert!(dates.is_empty());
}

/// 单值负值 + 步进走展开路径（Java `-4/2` → applyNegative 后步进到 max）。
#[test]
fn dom_negative_single_step_expands() {
    let pattern = CronPattern::of("0 0 0 -4/2 * *").unwrap();
    // -4 + 32 = 28 → {28, 30, 32} → 28/30 显式 + 最后一天
    assert!(pattern.matches(parse_local("2026-04-28 00:00:00"), true));
    assert!(pattern.matches(parse_local("2026-04-30 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-27 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-29 00:00:00"), true));
}

/// 负值越界（-33 + 32 = -1）在 `checked_schedule_value` 被拒绝。
#[test]
fn dom_negative_out_of_bounds_rejected() {
    assert!(CronPattern::of("0 0 0 -33 * *").is_err());
}

/// `split_numeric_range` 端点为空（`1-`）→ 不展开 → cron 拒绝。
/// `-5` 是合法负值（Java `i += part.getMax()` → 27 日），不在此列。
#[test]
fn empty_range_endpoint_rejected() {
    assert!(CronPattern::of("0 0 0 1- * *").is_err());
    let pattern = CronPattern::of("0 0 0 -5 * *").unwrap();
    assert!(pattern.matches(parse_local("2026-04-27 00:00:00"), true));
    assert!(!pattern.matches(parse_local("2026-04-26 00:00:00"), true));
}
