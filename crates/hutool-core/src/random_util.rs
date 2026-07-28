//! 对齐: `cn.hutool.core.util.RandomUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/RandomUtil.java

use std::collections::HashSet;

use rand::RngExt;
use rust_decimal::Decimal;

/// 对齐 Java: `cn.hutool.core.util.RandomUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct RandomUtil;

impl RandomUtil {
    /// 对齐 Java: `RandomUtil.randomInt()`
    pub fn random_int() -> i32 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomInt(int)`
    pub fn random_int_max(max: i32) -> i32 {
        if max <= 0 {
            return 0;
        }
        rand::rng().random_range(0..max)
    }

    /// 对齐 Java: `RandomUtil.randomInt(int, int)`
    pub fn random_int_range(min: i32, max: i32) -> i32 {
        if min >= max {
            return min;
        }
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomInt(int, int, boolean, boolean)`
    pub fn random_int_bounds(min: i32, max: i32, include_min: bool, include_max: bool) -> i32 {
        let start = if include_min { min } else { min.saturating_add(1) };
        let end = if include_max {
            max.saturating_add(1)
        } else {
            max
        };
        if start >= end {
            return start;
        }
        rand::rng().random_range(start..end)
    }

    /// 对齐 Java: `RandomUtil.randomInts(int)`
    pub fn random_ints(length: usize) -> Vec<i32> {
        (0..length).map(|_| Self::random_int()).collect()
    }

    /// 对齐 Java: `RandomUtil.randomLong()`
    pub fn random_long() -> i64 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomLong(long)`
    pub fn random_long_max(max: i64) -> i64 {
        if max <= 0 {
            return 0;
        }
        rand::rng().random_range(0..max)
    }

    /// 对齐 Java: `RandomUtil.randomLong(long, long)`
    pub fn random_long_range(min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomLong(long, long, boolean, boolean)`
    pub fn random_long_bounds(min: i64, max: i64, include_min: bool, include_max: bool) -> i64 {
        let start = if include_min { min } else { min.saturating_add(1) };
        let end = if include_max {
            max.saturating_add(1)
        } else {
            max
        };
        if start >= end {
            return start;
        }
        rand::rng().random_range(start..end)
    }

    /// 对齐 Java: `RandomUtil.randomFloat()`
    pub fn random_float() -> f32 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomFloat(float)`
    pub fn random_float_max(max: f32) -> f32 {
        if max <= 0.0 {
            return 0.0;
        }
        rand::rng().random_range(0.0..max)
    }

    /// 对齐 Java: `RandomUtil.randomFloat(float, float)`
    pub fn random_float_range(min: f32, max: f32) -> f32 {
        if min >= max {
            return min;
        }
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomDouble()`
    pub fn random_double() -> f64 {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomDouble(double)`
    pub fn random_double_max(max: f64) -> f64 {
        if max <= 0.0 {
            return 0.0;
        }
        rand::rng().random_range(0.0..max)
    }

    /// 对齐 Java: `RandomUtil.randomDouble(double, double)`
    pub fn random_double_range(min: f64, max: f64) -> f64 {
        if min >= max {
            return min;
        }
        rand::rng().random_range(min..max)
    }

    /// 对齐 Java: `RandomUtil.randomDouble` 带小数位（简化：四舍五入到 scale）。
    pub fn random_double_scaled(min: f64, max: f64, scale: u32) -> f64 {
        let v = Self::random_double_range(min, max);
        let factor = 10f64.powi(scale as i32);
        (v * factor).round() / factor
    }

    /// 对齐 Java: `RandomUtil.randomBigDecimal()`
    pub fn random_big_decimal() -> Decimal {
        Decimal::from_f64_retain(Self::random_double()).unwrap_or(Decimal::ZERO)
    }

    /// 对齐 Java: `RandomUtil.randomBigDecimal(BigDecimal)`
    pub fn random_big_decimal_max(limit: Decimal) -> Decimal {
        let max = limit.to_string().parse::<f64>().unwrap_or(1.0);
        Decimal::from_f64_retain(Self::random_double_max(max)).unwrap_or(Decimal::ZERO)
    }

    /// 对齐 Java: `RandomUtil.randomBigDecimal(BigDecimal, BigDecimal)`
    pub fn random_big_decimal_range(min: Decimal, max: Decimal) -> Decimal {
        let lo = min.to_string().parse::<f64>().unwrap_or(0.0);
        let hi = max.to_string().parse::<f64>().unwrap_or(1.0);
        Decimal::from_f64_retain(Self::random_double_range(lo, hi)).unwrap_or(min)
    }

    /// 对齐 Java: `RandomUtil.randomBoolean()`
    pub fn random_boolean() -> bool {
        rand::random()
    }

    /// 对齐 Java: `RandomUtil.randomChar(String)` / BASE 字符集随机字符。
    pub fn random_char() -> char {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        CHARS[rand::rng().random_range(0..CHARS.len())] as char
    }

    /// 对齐 Java: `RandomUtil.randomChar(String)`
    pub fn random_char_from(base: &str) -> Option<char> {
        let chars: Vec<char> = base.chars().collect();
        if chars.is_empty() {
            return None;
        }
        Some(chars[rand::rng().random_range(0..chars.len())])
    }

    /// 对齐 Java: `RandomUtil.randomString(int)`
    pub fn random_string(length: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::rng();
        (0..length)
            .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomStringLower(int)`
    pub fn random_string_lower(length: usize) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();
        (0..length)
            .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomStringUpper(int)`
    pub fn random_string_upper(length: usize) -> String {
        Self::random_string_lower(length).to_ascii_uppercase()
    }

    /// 对齐 Java: `RandomUtil.randomStringWithoutStr(int, String)`
    pub fn random_string_without(length: usize, exclude: &str) -> String {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let exclude_set: HashSet<char> = exclude.chars().collect();
        let pool: Vec<char> = CHARS
            .iter()
            .map(|&b| b as char)
            .filter(|c| !exclude_set.contains(c))
            .collect();
        if pool.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        (0..length)
            .map(|_| pool[rng.random_range(0..pool.len())])
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomString(String, int)`
    pub fn random_string_from(base: &str, length: usize) -> String {
        let chars: Vec<char> = base.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        let mut rng = rand::rng();
        (0..length)
            .map(|_| chars[rng.random_range(0..chars.len())])
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomNumbers(int)`
    pub fn random_numbers(length: usize) -> String {
        let mut rng = rand::rng();
        (0..length)
            .map(|_| rng.random_range(0u8..10).to_string())
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomEle(List)` / `randomEle(T[])`
    pub fn random_element<T: Clone>(items: &[T]) -> Option<T> {
        if items.is_empty() {
            return None;
        }
        let idx = rand::rng().random_range(0..items.len());
        Some(items[idx].clone())
    }

    /// 对齐 Java: `RandomUtil.randomEle(List, int)` — limit 截断候选后再抽一个。
    pub fn random_element_limit<T: Clone>(items: &[T], limit: usize) -> Option<T> {
        if items.is_empty() || limit == 0 {
            return None;
        }
        let end = limit.min(items.len());
        Self::random_element(&items[..end])
    }

    /// 对齐 Java: `RandomUtil.randomEles` — 有放回抽样 count 个。
    pub fn random_eles<T: Clone>(items: &[T], count: usize) -> Vec<T> {
        Self::random_elements(items, count)
    }

    /// 对齐 Java: `RandomUtil.randomEle(List, int)` 多元素有放回。
    pub fn random_elements<T: Clone>(items: &[T], count: usize) -> Vec<T> {
        if items.is_empty() || count == 0 {
            return Vec::new();
        }
        let mut rng = rand::rng();
        (0..count)
            .map(|_| items[rng.random_range(0..items.len())].clone())
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomEleList` — 无放回抽样。
    pub fn random_ele_list<T: Clone>(items: &[T], count: usize) -> Vec<T> {
        if items.is_empty() || count == 0 {
            return Vec::new();
        }
        let mut indices: Vec<usize> = (0..items.len()).collect();
        let mut rng = rand::rng();
        // Fisher–Yates partial shuffle
        let take = count.min(items.len());
        for i in 0..take {
            let j = rng.random_range(i..indices.len());
            indices.swap(i, j);
        }
        indices[..take]
            .iter()
            .map(|&i| items[i].clone())
            .collect()
    }

    /// 对齐 Java: `RandomUtil.randomEleSet`
    pub fn random_ele_set<T: Clone + Eq + std::hash::Hash>(
        items: &[T],
        count: usize,
    ) -> HashSet<T> {
        Self::random_ele_list(items, count).into_iter().collect()
    }

    /// 对齐 Java: `RandomUtil.weightRandom` / `RandomUtil.weightRandom(Map)`
    pub fn weighted_random<T: Clone>(items: &[(T, u32)]) -> Option<T> {
        Self::weight_random(items)
    }

    /// 对齐 Java: `RandomUtil.weightRandom`
    pub fn weight_random<T: Clone>(items: &[(T, u32)]) -> Option<T> {
        if items.is_empty() {
            return None;
        }
        let total_weight: u32 = items.iter().map(|(_, w)| w).sum();
        if total_weight == 0 {
            return None;
        }
        let mut rng = rand::rng();
        let mut target = rng.random_range(0..total_weight);
        for (item, weight) in items {
            if target < *weight {
                return Some(item.clone());
            }
            target -= weight;
        }
        items.last().map(|(item, _)| item.clone())
    }

    /// 对齐 Java: `RandomUtil.randomBytes(int)`
    pub fn random_bytes(length: usize) -> Vec<u8> {
        let mut rng = rand::rng();
        (0..length).map(|_| rng.random()).collect()
    }

    /// 对齐 Java: `RandomUtil.randomChinese()` — `randomInt('\u4E00', '\u9FFF')` 上界不含。
    pub fn random_chinese() -> char {
        let code = Self::random_int_range(0x4E00, 0x9FFF);
        char::from_u32(code as u32).expect("CJK unified ideograph range")
    }

    /// 对齐 Java: `RandomUtil.randomDay(int, int)` — 相对今天的随机日偏移（简化为日期毫秒种子）。
    pub fn random_day(min_include: i32, max_include: i32) -> i32 {
        Self::random_int_bounds(min_include, max_include, true, true)
    }
}
