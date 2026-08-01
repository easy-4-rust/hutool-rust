//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本提供算术、比较、最值、解析与 BigDecimal（`rust_decimal::Decimal`）对齐实现。

use crate::{CoreError, Result};
use rand::RngExt;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::collections::HashSet;
use std::str::FromStr;

use super::parsed_number::ParsedNumber;

/// 对齐 Java: `cn.hutool.core.util.NumberUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberUtil;

impl NumberUtil {
    // ── 算术操作 ──

    /// 对齐 Java: `NumberUtil.add(double, double)`
    pub fn add(v1: f64, v2: f64) -> f64 {
        v1 + v2
    }

    /// 对齐 Java: `NumberUtil.sub(double, double)`
    pub fn sub(v1: f64, v2: f64) -> f64 {
        v1 - v2
    }

    /// 对齐 Java: `NumberUtil.mul(double, double)`
    pub fn mul(v1: f64, v2: f64) -> f64 {
        v1 * v2
    }

    /// 对齐 Java: `NumberUtil.div(double, double)`
    pub fn div(v1: f64, v2: f64) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 / v2)
    }

    /// 对齐 Java: `NumberUtil.div(double, double, int)`（默认 HALF_UP）
    pub fn div_with_scale(v1: f64, v2: f64, scale: u32) -> Result<f64> {
        let d = Self::div_decimal(
            &Self::to_big_decimal_f64(v1)?,
            &Self::to_big_decimal_f64(v2)?,
            scale as i32,
        )?;
        Ok(decimal_to_f64(d))
    }

    /// 对齐 Java: `NumberUtil.add(BigDecimal...)`
    pub fn add_decimal(values: &[Decimal]) -> Decimal {
        values.iter().copied().fold(Decimal::ZERO, |a, b| a + b)
    }

    /// 对齐 Java: `NumberUtil.add(String...)`（空白视为 0）
    pub fn add_str(values: &[&str]) -> Decimal {
        values
            .iter()
            .map(|s| Self::to_big_decimal_str(s).unwrap_or(Decimal::ZERO))
            .fold(Decimal::ZERO, |a, b| a + b)
    }

    /// 对齐 Java: `NumberUtil.sub(BigDecimal, BigDecimal)`
    pub fn sub_decimal(v1: Decimal, v2: Decimal) -> Decimal {
        v1 - v2
    }

    /// 对齐 Java: `NumberUtil.mul(BigDecimal, BigDecimal)`；任一为 None 时返回 ZERO
    pub fn mul_decimal_opt(v1: Option<Decimal>, v2: Option<Decimal>) -> Decimal {
        match (v1, v2) {
            (Some(a), Some(b)) => a * b,
            _ => Decimal::ZERO,
        }
    }

    /// 对齐 Java: `NumberUtil.mul(Number, Number)`（f64 → Decimal 字符串路径）
    pub fn mul_f64_as_decimal(v1: f64, v2: f64) -> Decimal {
        let a = Self::to_big_decimal_f64(v1).unwrap_or(Decimal::ZERO);
        let b = Self::to_big_decimal_f64(v2).unwrap_or(Decimal::ZERO);
        a * b
    }

    /// 对齐 Java: `NumberUtil.div(BigDecimal, BigDecimal)`
    pub fn div_decimal(v1: &Decimal, v2: &Decimal, scale: i32) -> Result<Decimal> {
        if *v2 == Decimal::ZERO {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        let scale = if scale < 0 { 0 } else { scale as u32 };
        v1.checked_div(*v2)
            .map(|d| d.round_dp_with_strategy(scale, RoundingStrategy::MidpointAwayFromZero))
            .ok_or(CoreError::InvalidArgument {
                name: "div",
                reason: "division failed",
            })
    }

    /// 对齐 Java: `NumberUtil.div(BigDecimal.ZERO, BigDecimal.ONE)` 默认 scale
    pub fn div_decimal_default(v1: &Decimal, v2: &Decimal) -> Result<Decimal> {
        Self::div_decimal(v1, v2, 10)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `NumberUtil.compare(double, double)`
    pub fn compare_f64(x: f64, y: f64) -> i32 {
        match x.partial_cmp(&y) {
            Some(std::cmp::Ordering::Less) => -1,
            Some(std::cmp::Ordering::Equal) => 0,
            Some(std::cmp::Ordering::Greater) => 1,
            None => 0,
        }
    }

    /// 对齐 Java: `NumberUtil.compare(int, int)`
    pub fn compare_i32(x: i32, y: i32) -> i32 {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `NumberUtil.compare(long, long)`
    pub fn compare_i64(x: i64, y: i64) -> i32 {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `NumberUtil.compare(char, char)`
    pub fn compare_char(x: char, y: char) -> i32 {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    // ── 相等判断 ──

    /// 对齐 Java: `NumberUtil.equals(double, double)`
    pub fn equals_f64(num1: f64, num2: f64) -> bool {
        (num1 - num2).abs() < f64::EPSILON
    }

    /// 对齐 Java: `NumberUtil.equals(float, float)`
    pub fn equals_f32(num1: f32, num2: f32) -> bool {
        (num1 - num2).abs() < f32::EPSILON
    }

    /// 对齐 Java: `NumberUtil.equals(long, long)`
    pub fn equals_i64(num1: i64, num2: i64) -> bool {
        num1 == num2
    }

    /// 对齐 Java: `NumberUtil.equals(BigDecimal, BigDecimal)`（compareTo == 0）
    pub fn equals_decimal(a: &Decimal, b: &Decimal) -> bool {
        a == b
    }

    // ── 最值操作 ──

    /// 对齐 Java: `NumberUtil.min(int[])`
    pub fn min_i32(values: &[i32]) -> Result<i32> {
        values
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.min(long[])`
    pub fn min_i64(values: &[i64]) -> Result<i64> {
        values
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.min(double[])`
    pub fn min_f64(values: &[f64]) -> Result<f64> {
        values
            .iter()
            .copied()
            .reduce(f64::min)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(int[])`
    pub fn max_i32(values: &[i32]) -> Result<i32> {
        values
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(long[])`
    pub fn max_i64(values: &[i64]) -> Result<i64> {
        values
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(double[])`
    pub fn max_f64(values: &[f64]) -> Result<f64> {
        values
            .iter()
            .copied()
            .reduce(f64::max)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    // ── 数值判断 ──

    /// 对齐 Java: `NumberUtil.isNumber(CharSequence)`
    pub fn is_number(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() {
            return false;
        }
        // 与 Hutool 一致：允许前导 +/、小数、科学计数
        Self::parse_number(t).is_some()
    }

    /// 对齐 Java: `NumberUtil.isInteger(CharSequence)`
    pub fn is_integer(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() {
            return false;
        }
        t.parse::<i64>().is_ok()
    }

    /// 对齐 Java: `NumberUtil.isPowerOfTwo(long)`
    pub fn is_power_of_two(n: i64) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }

    /// 对齐 Java: `NumberUtil.isPrimes(int)`
    pub fn is_primes(n: i32) -> bool {
        assert!(n > 1, "The number must be > 1");
        if n <= 3 {
            return true;
        }
        if (n & 1) == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i <= n / i {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    // ── 范围判断 ──

    /// 对齐 Java: `NumberUtil.isInRange(long, long, long)`
    pub fn is_in_range_i64(value: i64, min: i64, max: i64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `NumberUtil.isInRange(double, double, double)`
    pub fn is_in_range_f64(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    // ── 舍入 / 格式化 ──

    /// 对齐 Java: `NumberUtil.round(String, int)` → Decimal（HALF_UP）
    pub fn round_str_to_decimal(number_str: &str, scale: i32) -> Result<Decimal> {
        let d = Self::to_big_decimal_str(number_str)?;
        Ok(Self::round_decimal(d, scale))
    }

    /// 对齐 Java: `NumberUtil.round(BigDecimal, int)` HALF_UP
    pub fn round_decimal(number: Decimal, scale: i32) -> Decimal {
        let scale = if scale < 0 { 0 } else { scale as u32 };
        number.round_dp_with_strategy(scale, RoundingStrategy::MidpointAwayFromZero)
    }

    /// 对齐 Java: `NumberUtil.roundHalfEven(BigDecimal, int)`
    pub fn round_half_even(number: Decimal, scale: i32) -> Decimal {
        let scale = if scale < 0 { 0 } else { scale as u32 };
        number.round_dp_with_strategy(scale, RoundingStrategy::MidpointNearestEven)
    }

    /// 对齐 Java: `NumberUtil.roundStr(double/String, int)` → plain string
    pub fn round_str(number_str: &str, scale: i32) -> Result<String> {
        let d = Self::round_str_to_decimal(number_str, scale)?;
        Ok(plain_fixed(&d, scale))
    }

    /// 对齐 Java: `NumberUtil.roundStr(double, int)`
    pub fn round_str_f64(v: f64, scale: i32) -> Result<String> {
        // Java 走 Double.toString 再解析，避免二进制浮点误差
        Self::round_str(&f64_to_java_string(v), scale)
    }

    /// 对齐 Java: `NumberUtil.roundStr(..., RoundingMode.HALF_EVEN)`
    pub fn round_str_half_even(number_str: &str, scale: i32) -> Result<String> {
        let d = Self::to_big_decimal_str(number_str)?;
        let r = Self::round_half_even(d, scale);
        Ok(plain_fixed(&r, scale))
    }

    /// 对齐 Java: `NumberUtil.decimalFormat(pattern, long)`（支持 `,###` / `0.00` / `,##0.00`）
    pub fn decimal_format(pattern: &str, value: f64) -> Result<String> {
        if !value.is_finite() {
            return Err(CoreError::InvalidArgument {
                name: "value",
                reason: "value is NaN or Infinite!",
            });
        }
        Ok(format_with_pattern(pattern, value))
    }

    /// 对齐 Java: `NumberUtil.decimalFormatMoney(double)`
    pub fn decimal_format_money(value: f64) -> Result<String> {
        Self::decimal_format(",##0.00", value)
    }

    // ── BigDecimal 转换 ──

    /// 对齐 Java: `NumberUtil.toBigDecimal(double)`
    pub fn to_big_decimal_f64(v: f64) -> Result<Decimal> {
        if !v.is_finite() {
            return Err(CoreError::InvalidArgument {
                name: "number",
                reason: "Number is invalid!",
            });
        }
        Decimal::from_str(&f64_to_java_string(v)).map_err(|_| CoreError::InvalidArgument {
            name: "number",
            reason: "invalid decimal",
        })
    }

    /// 对齐 Java: `NumberUtil.toBigDecimal(String)`
    pub fn to_big_decimal_str(number_str: &str) -> Result<Decimal> {
        let t = number_str.trim();
        if t.is_empty() {
            return Ok(Decimal::ZERO);
        }
        if t.eq_ignore_ascii_case("nan") || t.eq_ignore_ascii_case("infinity") {
            return Err(CoreError::InvalidArgument {
                name: "numberStr",
                reason: "Number is invalid!",
            });
        }
        if let Ok(d) = Decimal::from_str(t) {
            return Ok(d);
        }
        // 支持 1,234.55 / 1,234.56D
        Self::parse_number(t)
            .ok_or(CoreError::InvalidArgument {
                name: "numberStr",
                reason: "invalid number",
            })
            .and_then(|n| match n {
                ParsedNumber::Decimal(d) => Ok(d),
                ParsedNumber::F64(f) => Self::to_big_decimal_f64(f),
                ParsedNumber::I64(i) => Ok(Decimal::from(i)),
            })
    }

    /// 对齐 Java: `NumberUtil.toStr(BigDecimal)`（去尾零 + plain）
    pub fn to_str_decimal(d: Decimal) -> String {
        let stripped = d.normalize();
        stripped.to_string()
    }

    /// 对齐 Java: `NumberUtil.toPlainNumber` 语义（科学计数 → 普通字符串）
    pub fn to_plain_number(num: &str) -> Result<String> {
        let d = Self::to_big_decimal_str(num)?;
        Ok(d.to_string())
    }

    // ── 解析 ──

    /// 对齐 Java: `NumberUtil.parseInt(String)`
    pub fn parse_int(number: &str) -> Result<i32> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0);
        }
        if t.len() > 2 && t[..2].eq_ignore_ascii_case("0x") {
            return i32::from_str_radix(&t[2..], 16).map_err(|_| CoreError::InvalidArgument {
                name: "number",
                reason: "invalid hex int",
            });
        }
        if t.to_ascii_lowercase().contains('e') {
            return Err(CoreError::InvalidArgument {
                name: "number",
                reason: "Unsupported int format",
            });
        }
        if let Ok(v) = t.parse::<i32>() {
            return Ok(v);
        }
        Self::parse_number(t)
            .map(|n| n.as_i64() as i32)
            .ok_or(CoreError::InvalidArgument {
                name: "number",
                reason: "NumberFormatException",
            })
    }

    /// 对齐 Java: `NumberUtil.parseInt(String, Integer)`
    pub fn parse_int_or(number: &str, default: Option<i32>) -> Option<i32> {
        match Self::parse_int(number) {
            Ok(v) => Some(v),
            Err(_) => default,
        }
    }

    /// 对齐 Java: `NumberUtil.parseLong(String)`
    pub fn parse_long(number: &str) -> Result<i64> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0);
        }
        if t.len() > 2 && t[..2].eq_ignore_ascii_case("0x") {
            return i64::from_str_radix(&t[2..], 16).map_err(|_| CoreError::InvalidArgument {
                name: "number",
                reason: "invalid hex long",
            });
        }
        if let Ok(v) = t.parse::<i64>() {
            return Ok(v);
        }
        // 后缀 L / 小数截断等走 parseNumber
        Self::parse_number(t)
            .map(|n| n.as_i64())
            .ok_or(CoreError::InvalidArgument {
                name: "number",
                reason: "NumberFormatException",
            })
    }

    /// 对齐 Java: `NumberUtil.parseLong(String, Long)`
    pub fn parse_long_or(number: Option<&str>, default: Option<i64>) -> Option<i64> {
        let Some(number) = number else {
            return default;
        };
        if number.trim().is_empty() {
            return default;
        }
        match Self::parse_long(number) {
            Ok(v) => Some(v),
            Err(_) => default,
        }
    }

    /// 对齐 Java: `NumberUtil.parseFloat(String, Float)`
    pub fn parse_float_or(number: &str, default: Option<f32>) -> Option<f32> {
        Self::parse_number(number)
            .map(|n| n.as_f64() as f32)
            .or(default)
            .or_else(|| {
                if number.trim().is_empty() {
                    default
                } else {
                    None
                }
            })
    }

    /// 对齐 Java: `NumberUtil.parseDouble(String, Double)`
    pub fn parse_double_or(number: &str, default: Option<f64>) -> Option<f64> {
        if number.trim().is_empty() {
            return default;
        }
        Self::parse_number(number)
            .map(|n| n.as_f64())
            .or(default)
            .filter(|_| true)
            .or_else(|| Self::parse_number(number).map(|n| n.as_f64()).or(default))
    }

    /// 对齐 Java: `NumberUtil.parseNumber(String)` → Decimal（千分位/后缀/科学计数）
    pub fn parse_number(number: &str) -> Option<ParsedNumber> {
        let mut t = number.trim().to_string();
        if t.is_empty() {
            return None;
        }
        // 去掉千分位
        if t.contains(',') {
            t = t.replace(',', "");
        }
        // 十六进制
        if t.len() > 2 && t[..2].eq_ignore_ascii_case("0x") {
            return i64::from_str_radix(&t[2..], 16)
                .ok()
                .map(ParsedNumber::I64);
        }
        // 剥离类型后缀 D/F/L（单独一个后缀字母不是数字）
        let mut stripped = false;
        while t
            .chars()
            .last()
            .is_some_and(|c| matches!(c, 'D' | 'd' | 'F' | 'f' | 'L' | 'l'))
        {
            t.pop();
            stripped = true;
        }
        if stripped && (t.is_empty() || t == "+" || t == "-" || t == "." || t == "+." || t == "-.") {
            return None;
        }
        if t.is_empty() || t == "+" || t == "-" || t == "." || t == "+." || t == "-." {
            // ".123" → 0 for int path; for Number keep as 0.123
            if number.trim().starts_with('.') || number.trim().starts_with("+.") || number.trim().starts_with("-.")
            {
                // fallthrough after restoring
            } else {
                return Some(ParsedNumber::I64(0));
            }
        }
        // 重新取原始去掉逗号后的串做部分解析
        let mut s = number.trim().replace(',', "");
        while s
            .chars()
            .last()
            .is_some_and(|c| matches!(c, 'D' | 'd' | 'F' | 'f' | 'L' | 'l'))
        {
            s.pop();
        }
        // 非法前缀
        if s.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
            return None;
        }
        if s.starts_with("..") {
            return None;
        }
        // 科学计数 → Decimal（超大指数超出 rust_decimal 时仍视为可解析）
        if s.to_ascii_lowercase().contains('e') {
            if let Ok(d) = Decimal::from_str(&s) {
                return Some(ParsedNumber::Decimal(d));
            }
            if let Ok(d) = Decimal::from_scientific(&s) {
                return Some(ParsedNumber::Decimal(d));
            }
            if is_scientific_form(&s) {
                return Some(ParsedNumber::Decimal(Decimal::MAX));
            }
            return None;
        }
        if let Ok(d) = Decimal::from_str(&s) {
            return Some(ParsedNumber::Decimal(d));
        }
        // 部分解析：截取前导合法数字片段（对齐 Hutool NumberParser）
        if let Some(prefix) = extract_number_prefix(&s) {
            if let Ok(d) = Decimal::from_str(prefix) {
                return Some(ParsedNumber::Decimal(d));
            }
            if let Ok(i) = prefix.parse::<i64>() {
                return Some(ParsedNumber::I64(i));
            }
            if let Ok(f) = prefix.parse::<f64>() {
                return Some(ParsedNumber::F64(f));
            }
        }
        None
    }

    /// 对齐 Java: `NumberUtil.parseNumber(String, Number)`
    pub fn parse_number_or(number: &str, default: Option<ParsedNumber>) -> Option<ParsedNumber> {
        if number.trim().is_empty() {
            return default;
        }
        Self::parse_number(number).or(default)
    }

    /// 对齐 Java: `NumberUtil.parseInt` 兼容旧签名
    pub fn parse_int_default(s: &str, default_value: i32) -> i32 {
        Self::parse_int(s).unwrap_or(default_value)
    }

    /// 对齐 Java: `NumberUtil.parseLong` 兼容旧签名
    pub fn parse_long_default(s: &str, default_value: i64) -> i64 {
        Self::parse_long(s).unwrap_or(default_value)
    }

    /// 对齐 Java: `NumberUtil.parseDouble` 兼容旧签名
    pub fn parse_double(s: &str, default_value: f64) -> f64 {
        Self::parse_double_or(s, Some(default_value)).unwrap_or(default_value)
    }

    // ── 幂 / 进制 / 随机 ──

    /// 对齐 Java: `NumberUtil.pow(BigDecimal, int)`（负指数默认 scale=2 HALF_UP）
    pub fn pow(number: Decimal, n: i32) -> Decimal {
        Self::pow_with_scale(number, n, 2)
    }

    /// 对齐 Java: `NumberUtil.pow(BigDecimal, int, scale, HALF_UP)`
    pub fn pow_with_scale(number: Decimal, n: i32, scale: i32) -> Decimal {
        fn pow_u(mut base: Decimal, mut exp: u32) -> Decimal {
            let mut result = Decimal::ONE;
            while exp > 0 {
                if exp & 1 == 1 {
                    result *= base;
                }
                base *= base;
                exp >>= 1;
            }
            result
        }
        if n < 0 {
            let pos = pow_u(number, (-n) as u32);
            let one = Decimal::ONE;
            return Self::div_decimal(&one, &pos, scale).unwrap_or(Decimal::ZERO);
        }
        pow_u(number, n as u32)
    }

    /// 对齐 Java: `NumberUtil.getBinaryStr(double)` IEEE754 64-bit
    pub fn get_binary_str_f64(v: f64) -> String {
        let bits = v.to_bits();
        format!("{bits:064b}")
    }

    /// 对齐 Java: `NumberUtil.generateRandomNumber(begin, end, size)`
    pub fn generate_random_number(begin: i32, end: i32, size: usize) -> Result<Vec<i32>> {
        let (mut begin, mut end) = (begin, end);
        if begin > end {
            std::mem::swap(&mut begin, &mut end);
        }
        if (end - begin) < size as i32 {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "Size is larger than range between begin and end!",
            });
        }
        let mut seed: Vec<i32> = (begin..end).collect();
        let mut rng = rand::rng();
        let mut ran = Vec::with_capacity(size);
        for i in 0..size {
            let j = rng.random_range(0..seed.len() - i);
            ran.push(seed[j]);
            let last = seed.len() - 1 - i;
            seed[j] = seed[last];
        }
        Ok(ran)
    }

    /// 对齐 Java: `NumberUtil.generateBySet(begin, end, size)`
    pub fn generate_by_set(begin: i32, end: i32, size: usize) -> Result<Vec<i32>> {
        let (mut begin, mut end) = (begin, end);
        if begin > end {
            std::mem::swap(&mut begin, &mut end);
        }
        if (end - begin) < size as i32 {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "Size is larger than range",
            });
        }
        let mut set = HashSet::new();
        let mut rng = rand::rng();
        while set.len() < size {
            set.insert(rng.random_range(begin..end));
        }
        Ok(set.into_iter().collect())
    }

    /// 对齐 Java: `NumberUtil.multiple(int, int)`（防溢出 LCM）
    pub fn multiple(m: i32, n: i32) -> Result<i32> {
        let gcd = Self::gcd(m as i64, n as i64) as i32;
        let result = (m as i64) / (gcd as i64) * (n as i64);
        if result > i32::MAX as i64 || result < i32::MIN as i64 {
            return Err(CoreError::InvalidArgument {
                name: "multiple",
                reason: "Integer overflow",
            });
        }
        Ok(result as i32)
    }

    // ── 计数操作 ──

    /// 对齐 Java: `NumberUtil.count(int, int)`
    pub fn count(start: i32, end: i32, step: i32) -> Result<Vec<i32>> {
        if step == 0 {
            return Err(CoreError::InvalidArgument {
                name: "step",
                reason: "step cannot be zero",
            });
        }
        let mut result = Vec::new();
        if step > 0 {
            let mut current = start;
            while current <= end {
                result.push(current);
                current += step;
            }
        } else {
            let mut current = start;
            while current >= end {
                result.push(current);
                current += step;
            }
        }
        Ok(result)
    }

    /// 对齐 Java: `NumberUtil.appendRange(int, int)`
    pub fn range(start: i32, end: i32) -> Vec<i32> {
        (start..end).collect()
    }

    /// 对齐 Java: `NumberUtil.factorial(long)`
    pub fn factorial(n: u64) -> Result<u64> {
        if n > 20 {
            return Err(CoreError::InvalidArgument {
                name: "n",
                reason: "factorial overflow for n > 20",
            });
        }
        Ok((1..=n).product())
    }

    /// 对齐 Java: `NumberUtil.gcd(long, long)`
    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// 对齐 Java: `NumberUtil.lcm(long, long)`
    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            return 0;
        }
        (a.abs() / Self::gcd(a, b)) * b.abs()
    }

    /// 对齐 Java: `NumberUtil.divisor(int, int)`（最大公约数）
    pub fn divisor(m: i32, n: i32) -> i32 {
        Self::gcd(m as i64, n as i64) as i32
    }

    /// 对齐 Java: `NumberUtil.ceilDiv(int, int)`
    pub fn ceil_div(v1: i32, v2: i32) -> i32 {
        ((v1 as f64) / (v2 as f64)).ceil() as i32
    }

    /// 对齐 Java: `NumberUtil.isLong(String)`
    pub fn is_long(s: &str) -> bool {
        let t = s.trim();
        !t.is_empty() && t.parse::<i64>().is_ok()
    }

    /// 对齐 Java: `NumberUtil.isDouble(String)`（须含小数点）
    pub fn is_double(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() || !t.contains('.') {
            return false;
        }
        t.parse::<f64>().is_ok()
    }

    /// 对齐 Java: `NumberUtil.isOdd(int)`
    pub fn is_odd(num: i32) -> bool {
        (num & 1) == 1
    }

    /// 对齐 Java: `NumberUtil.isEven(int)`
    pub fn is_even(num: i32) -> bool {
        !Self::is_odd(num)
    }

    /// 对齐 Java: `NumberUtil.isValid(double)`
    pub fn is_valid_f64(number: f64) -> bool {
        number.is_finite()
    }

    /// 对齐 Java: `NumberUtil.isValid(float)`
    pub fn is_valid_f32(number: f32) -> bool {
        number.is_finite()
    }

    /// 对齐 Java: `NumberUtil.isBeside(long, long)`
    pub fn is_beside_i64(number1: i64, number2: i64) -> bool {
        number1.abs_diff(number2) == 1
    }

    /// 对齐 Java: `NumberUtil.isBeside(int, int)`
    pub fn is_beside_i32(number1: i32, number2: i32) -> bool {
        number1.abs_diff(number2) == 1
    }

    /// 对齐 Java: `NumberUtil.zero2One(int)`
    pub fn zero2_one(value: i32) -> i32 {
        if value == 0 { 1 } else { value }
    }

    /// 对齐 Java: `NumberUtil.nullToZero(Integer)` 等（`None` → 0）
    pub fn null_to_zero_i32(number: Option<i32>) -> i32 {
        number.unwrap_or(0)
    }

    /// 对齐 Java: `NumberUtil.nullToZero(Long)`
    pub fn null_to_zero_i64(number: Option<i64>) -> i64 {
        number.unwrap_or(0)
    }

    /// 对齐 Java: `NumberUtil.nullToZero(Double)`
    pub fn null_to_zero_f64(number: Option<f64>) -> f64 {
        number.unwrap_or(0.0)
    }

    /// 对齐 Java: `NumberUtil.nullToZero(Float)`
    pub fn null_to_zero_f32(number: Option<f32>) -> f32 {
        number.unwrap_or(0.0)
    }

    /// 对齐 Java: `NumberUtil.nullToZero(BigDecimal)`
    pub fn null_to_zero_decimal(number: Option<Decimal>) -> Decimal {
        number.unwrap_or(Decimal::ZERO)
    }

    /// 对齐 Java: `NumberUtil.partValue(int, int)`（有余数时每份 +1）
    pub fn part_value(total: i32, part_count: i32) -> i32 {
        Self::part_value_with_rem(total, part_count, true)
    }

    /// 对齐 Java: `NumberUtil.partValue(int, int, boolean)`
    pub fn part_value_with_rem(total: i32, part_count: i32, plus_one_when_rem: bool) -> i32 {
        if part_count == 0 {
            return 0;
        }
        let mut part = total / part_count;
        if plus_one_when_rem && total % part_count > 0 {
            part += 1;
        }
        part
    }

    /// 对齐 Java: `NumberUtil.formatPercent(double, int)`
    pub fn format_percent(number: f64, scale: u32) -> String {
        let pct = number * 100.0;
        let scale_usize = scale as usize;
        let factor = 10f64.powi(scale as i32);
        let rounded = (pct * factor).round() / factor;
        if scale == 0 {
            format!("{}%", rounded as i64)
        } else {
            format!("{rounded:.scale_usize$}%")
        }
    }

    /// 对齐 Java: `NumberUtil.roundDown(BigDecimal, int)`
    pub fn round_down(number: Decimal, scale: i32) -> Decimal {
        let scale = if scale < 0 { 0 } else { scale as u32 };
        number.round_dp_with_strategy(scale, RoundingStrategy::ToZero)
    }

    /// 对齐 Java: `NumberUtil.binaryToInt(String)`
    pub fn binary_to_int(binary_str: &str) -> Result<i32> {
        i32::from_str_radix(binary_str.trim(), 2).map_err(|_| CoreError::InvalidArgument {
            name: "binaryStr",
            reason: "invalid binary int",
        })
    }

    /// 对齐 Java: `NumberUtil.binaryToLong(String)`
    pub fn binary_to_long(binary_str: &str) -> Result<i64> {
        i64::from_str_radix(binary_str.trim(), 2).map_err(|_| CoreError::InvalidArgument {
            name: "binaryStr",
            reason: "invalid binary long",
        })
    }

    /// 对齐 Java: `NumberUtil.sqrt(long)`（整数平方根）
    pub fn sqrt(x: i64) -> i64 {
        if x <= 0 {
            return 0;
        }
        (x as f64).sqrt().floor() as i64
    }

    /// 对齐 Java: `NumberUtil.toBytes(int)`（大端）
    pub fn to_bytes(value: i32) -> [u8; 4] {
        value.to_be_bytes()
    }

    /// 对齐 Java: `NumberUtil.toInt(byte[])`
    pub fn to_int(bytes: &[u8]) -> i32 {
        let mut buf = [0u8; 4];
        let n = bytes.len().min(4);
        buf[4 - n..].copy_from_slice(&bytes[..n]);
        i32::from_be_bytes(buf)
    }

    /// 对齐 Java: `NumberUtil.toDouble(Number)`（f32 经字符串避免精度损失）
    pub fn to_double_f32(value: f32) -> f64 {
        value.to_string().parse().unwrap_or(value as f64)
    }

    /// 对齐 Java: `NumberUtil.compare(short, short)`
    pub fn compare_i16(x: i16, y: i16) -> i32 {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `NumberUtil.compare(byte, byte)`
    pub fn compare_i8(x: i8, y: i8) -> i32 {
        match x.cmp(&y) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `NumberUtil.min(float...)`
    pub fn min_f32(values: &[f32]) -> Result<f32> {
        values
            .iter()
            .copied()
            .reduce(f32::min)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.max(float...)`
    pub fn max_f32(values: &[f32]) -> Result<f32> {
        values
            .iter()
            .copied()
            .reduce(f32::max)
            .ok_or(CoreError::InvalidArgument {
                name: "values",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `NumberUtil.isGreater(BigDecimal, BigDecimal)`
    pub fn is_greater(a: &Decimal, b: &Decimal) -> bool {
        a > b
    }

    /// 对齐 Java: `NumberUtil.isGreaterOrEqual(BigDecimal, BigDecimal)`
    pub fn is_greater_or_equal(a: &Decimal, b: &Decimal) -> bool {
        a >= b
    }

    /// 对齐 Java: `NumberUtil.isLess(BigDecimal, BigDecimal)`
    pub fn is_less(a: &Decimal, b: &Decimal) -> bool {
        a < b
    }

    /// 对齐 Java: `NumberUtil.isLessOrEqual(BigDecimal, BigDecimal)`
    pub fn is_less_or_equal(a: &Decimal, b: &Decimal) -> bool {
        a <= b
    }

    /// 对齐 Java: `NumberUtil.isIn(BigDecimal, BigDecimal, BigDecimal)`（闭区间）
    pub fn is_in_decimal(value: &Decimal, min_include: &Decimal, max_include: &Decimal) -> bool {
        value >= min_include && value <= max_include
    }

    /// 对齐 Java: `NumberUtil.equals(char, char, boolean)`
    pub fn equals_char(c1: char, c2: char, ignore_case: bool) -> bool {
        if ignore_case {
            c1.eq_ignore_ascii_case(&c2)
        } else {
            c1 == c2
        }
    }

    /// 对齐 Java: `NumberUtil.add(float, float)` 等混合重载的 f64 路径
    pub fn add_f32(v1: f32, v2: f32) -> f64 {
        f64::from(v1) + f64::from(v2)
    }

    /// 对齐 Java: `NumberUtil.sub(float, float)`
    pub fn sub_f32(v1: f32, v2: f32) -> f64 {
        f64::from(v1) - f64::from(v2)
    }

    /// 对齐 Java: `NumberUtil.mul(float, float)`
    pub fn mul_f32(v1: f32, v2: f32) -> f64 {
        f64::from(v1) * f64::from(v2)
    }

    /// 对齐 Java: `NumberUtil.div(float, float)`
    pub fn div_f32(v1: f32, v2: f32) -> Result<f64> {
        Self::div(f64::from(v1), f64::from(v2))
    }

    /// 对齐 Java: `NumberUtil.add(long, long)` → double
    pub fn add_i64(v1: i64, v2: i64) -> f64 {
        v1 as f64 + v2 as f64
    }

    /// 对齐 Java: `NumberUtil.sub(BigDecimal...)` 可变参数
    pub fn sub_decimal_values(values: &[Decimal]) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        values[1..]
            .iter()
            .copied()
            .fold(values[0], |a, b| a - b)
    }

    /// 对齐 Java: `NumberUtil.mul(BigDecimal...)`
    pub fn mul_decimal_values(values: &[Decimal]) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        values.iter().copied().fold(Decimal::ONE, |a, b| a * b)
    }

    /// 对齐 Java: `NumberUtil.mul(String...)`
    pub fn mul_str(values: &[&str]) -> Decimal {
        if values.is_empty() {
            return Decimal::ZERO;
        }
        values
            .iter()
            .map(|s| Self::to_big_decimal_str(s).unwrap_or(Decimal::ZERO))
            .fold(Decimal::ONE, |a, b| a * b)
    }
}

use super::{decimal_to_f64, extract_number_prefix, f64_to_java_string, format_with_pattern, is_scientific_form, plain_fixed};
