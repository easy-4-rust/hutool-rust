//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供关联函数实现。
//! 使用 `rust_decimal::Decimal` 对齐 Java `BigDecimal` 精度语义。
//!
//! 重载的 Java 方法通过 `<name>_<n>` 后缀区分,避免 Rust 关联函数重名冲突。

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    non_snake_case
)]

use crate::{CoreError, Result};
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::str::FromStr;

/// 占位类型,对齐 Java 中尚无 Rust 等价的 BigDecimal / BigInteger / Number 等。
type OPAQUE = *const ();

/// 舍入模式,对齐 Java `java.math.RoundingMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    Up,
    Down,
    Ceiling,
    Floor,
    HalfUp,
    HalfDown,
    HalfEven,
    Unnecessary,
}

impl RoundingMode {
    /// 映射到 `rust_decimal::RoundingStrategy`
    fn to_strategy(self) -> RoundingStrategy {
        match self {
            Self::Up => RoundingStrategy::AwayFromZero,
            Self::Down => RoundingStrategy::ToZero,
            Self::Ceiling => RoundingStrategy::AwayFromZero,
            Self::Floor => RoundingStrategy::ToZero,
            Self::HalfUp => RoundingStrategy::MidpointAwayFromZero,
            Self::HalfDown => RoundingStrategy::MidpointTowardZero,
            Self::HalfEven => RoundingStrategy::MidpointNearestEven,
            Self::Unnecessary => RoundingStrategy::MidpointAwayFromZero,
        }
    }
}

// ── 内部辅助 ──

/// f64 → Decimal（字符串中转避免二进制浮点误差）
fn f64_to_decimal(v: f64) -> Result<Decimal> {
    if !v.is_finite() {
        return Err(CoreError::InvalidArgument {
            name: "value",
            reason: "NaN or Infinite",
        });
    }
    Decimal::from_str(&v.to_string()).map_err(|_| CoreError::InvalidArgument {
        name: "value",
        reason: "invalid decimal conversion",
    })
}

/// 以 Decimal 精度做除法,返回 Decimal
fn div_decimal(v1: f64, v2: f64, scale: i32, mode: RoundingMode) -> Result<Decimal> {
    if v2 == 0.0 {
        return Err(CoreError::InvalidArgument {
            name: "v2",
            reason: "division by zero",
        });
    }
    let d1 = f64_to_decimal(v1)?;
    let d2 = f64_to_decimal(v2)?;
    let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
    d1.checked_div(d2)
        .map(|d| d.round_dp_with_strategy(scale_u, mode.to_strategy()))
        .ok_or(CoreError::InvalidArgument {
            name: "div",
            reason: "division failed",
        })
}

/// Decimal 保留指定位数,格式化为 plain 字符串（补齐尾部零）
fn plain_fixed(d: &Decimal, scale: i32) -> String {
    let scale = if scale < 0 { 0 } else { scale as u32 };
    let rounded = d.round_dp_with_strategy(scale, RoundingStrategy::MidpointAwayFromZero);
    if scale == 0 {
        return rounded.trunc().to_string();
    }
    let s = rounded.to_string();
    if let Some(dot) = s.find('.') {
        let frac = &s[dot + 1..];
        if frac.len() < scale as usize {
            return format!("{s}{}", "0".repeat(scale as usize - frac.len()));
        }
        s
    } else {
        format!("{s}.{}", "0".repeat(scale as usize))
    }
}

/// 千分位分组
fn group_thousands(int_part: &str) -> String {
    let neg = int_part.starts_with('-');
    let digits: String = int_part.trim_start_matches('-').chars().collect();
    let mut out = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    let s: String = out.chars().rev().collect();
    if neg {
        format!("-{s}")
    } else {
        s
    }
}

/// 格式化数值为千分位 / 小数位（简易 `,###.00` 模式）
fn format_with_pattern(pattern: &str, value: f64) -> String {
    if pattern.contains('%') {
        let pct = value * 100.0;
        return format!("{pct}%");
    }
    let use_comma = pattern.contains(',');
    let decimals = pattern
        .rsplit('.')
        .next()
        .filter(|_| pattern.contains('.'))
        .map(|p| p.chars().filter(|c| *c == '0' || *c == '#').count())
        .unwrap_or(0);
    let factor = 10f64.powi(decimals as i32);
    let rounded = (value * factor).round() / factor;
    let mut body = if decimals > 0 {
        format!("{rounded:.decimals$}")
    } else {
        format!("{}", rounded as i64)
    };
    if use_comma {
        if let Some((int_part, frac)) = body.split_once('.') {
            body = format!("{}.{}", group_thousands(int_part), frac);
        } else {
            body = group_thousands(&body);
        }
    }
    body
}

/// 对齐 Java: `cn.hutool.core.util.NumberUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct NumberUtil;

impl NumberUtil {
    // ══════════════════════════════════════════════
    //  算术操作 - add / sub / mul / div
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (float v1, float v2)`
    pub fn add(v1: f32, v2: f32) -> Result<f64> {
        Ok(v1 as f64 + v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (float v1, double v2)`
    pub fn add_2(v1: f32, v2: f64) -> Result<f64> {
        Ok(v1 as f64 + v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, float v2)`
    pub fn add_3(v1: f64, v2: f32) -> Result<f64> {
        Ok(v1 + v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, double v2)`
    pub fn add_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 + v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (long v1, double v2)`
    pub fn add_5(v1: i64, v2: f64) -> Result<f64> {
        Ok(v1 as f64 + v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (double v1, long v2)`
    pub fn add_6(v1: f64, v2: i64) -> Result<f64> {
        Ok(v1 + v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (long v1, long v2)`
    pub fn add_7(v1: i64, v2: i64) -> Result<f64> {
        Ok(v1 as f64 + v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#double (Double v1, Double v2)`
    pub fn add_8(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (Number v1, Number v2)`
    pub fn add_9(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (Number... values)`
    pub fn add_10(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (String... values)`
    pub fn add_11(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::add#BigDecimal (BigDecimal... values)`
    pub fn add_12(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("add"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (float v1, float v2)`
    pub fn sub(v1: f32, v2: f32) -> Result<f64> {
        Ok(v1 as f64 - v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (float v1, double v2)`
    pub fn sub_2(v1: f32, v2: f64) -> Result<f64> {
        Ok(v1 as f64 - v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (double v1, float v2)`
    pub fn sub_3(v1: f64, v2: f32) -> Result<f64> {
        Ok(v1 - v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (double v1, double v2)`
    pub fn sub_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 - v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#double (Double v1, Double v2)`
    pub fn sub_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (Number v1, Number v2)`
    pub fn sub_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (Number... values)`
    pub fn sub_7(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (String... values)`
    pub fn sub_8(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sub#BigDecimal (BigDecimal... values)`
    pub fn sub_9(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("sub"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (float v1, float v2)`
    pub fn mul(v1: f32, v2: f32) -> Result<f64> {
        Ok(v1 as f64 * v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (float v1, double v2)`
    pub fn mul_2(v1: f32, v2: f64) -> Result<f64> {
        Ok(v1 as f64 * v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (double v1, float v2)`
    pub fn mul_3(v1: f64, v2: f32) -> Result<f64> {
        Ok(v1 * v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (double v1, double v2)`
    pub fn mul_4(v1: f64, v2: f64) -> Result<f64> {
        Ok(v1 * v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#double (Double v1, Double v2)`
    pub fn mul_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (Number v1, Number v2)`
    pub fn mul_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (Number... values)`
    pub fn mul_7(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (String v1, String v2)`
    pub fn mul_8(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (String... values)`
    pub fn mul_9(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::mul#BigDecimal (BigDecimal... values)`
    pub fn mul_10(values: &[OPAQUE]) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("mul"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2)`
    pub fn div(v1: f32, v2: f32) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 as f64 / v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2)`
    pub fn div_2(v1: f32, v2: f64) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 as f64 / v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2)`
    pub fn div_3(v1: f64, v2: f32) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 / v2 as f64)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2)`
    pub fn div_4(v1: f64, v2: f64) -> Result<f64> {
        if v2 == 0.0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        Ok(v1 / v2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2)`
    pub fn div_5(_v1: *const (), _v2: *const ()) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2)`
    pub fn div_6(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2)`
    pub fn div_7(_v1: *const (), _v2: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2, int scale)`
    pub fn div_8(v1: f32, v2: f32, scale: i32) -> Result<f64> {
        let d = div_decimal(v1 as f64, v2 as f64, scale, RoundingMode::HalfUp)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2, int scale)`
    pub fn div_9(v1: f32, v2: f64, scale: i32) -> Result<f64> {
        let d = div_decimal(v1 as f64, v2, scale, RoundingMode::HalfUp)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2, int scale)`
    pub fn div_10(v1: f64, v2: f32, scale: i32) -> Result<f64> {
        let d = div_decimal(v1, v2 as f64, scale, RoundingMode::HalfUp)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2, int scale)`
    pub fn div_11(v1: f64, v2: f64, scale: i32) -> Result<f64> {
        let d = div_decimal(v1, v2, scale, RoundingMode::HalfUp)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2, int scale)`
    pub fn div_12(_v1: *const (), _v2: *const (), scale: i32) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2, int scale)`
    pub fn div_13(_v1: *const (), _v2: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2, int scale)`
    pub fn div_14(_v1: *const (), _v2: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, float v2, int scale, RoundingMode roundingMode)`
    pub fn div_15(v1: f32, v2: f32, scale: i32, rounding_mode: RoundingMode) -> Result<f64> {
        let d = div_decimal(v1 as f64, v2 as f64, scale, rounding_mode)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (float v1, double v2, int scale, RoundingMode roundingMode)`
    pub fn div_16(v1: f32, v2: f64, scale: i32, rounding_mode: RoundingMode) -> Result<f64> {
        let d = div_decimal(v1 as f64, v2, scale, rounding_mode)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, float v2, int scale, RoundingMode roundingMode)`
    pub fn div_17(v1: f64, v2: f32, scale: i32, rounding_mode: RoundingMode) -> Result<f64> {
        let d = div_decimal(v1, v2 as f64, scale, rounding_mode)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (double v1, double v2, int scale, RoundingMode roundingMode)`
    pub fn div_18(v1: f64, v2: f64, scale: i32, rounding_mode: RoundingMode) -> Result<f64> {
        let d = div_decimal(v1, v2, scale, rounding_mode)?;
        Ok(d.to_string().parse::<f64>().unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#double (Double v1, Double v2, int scale, RoundingMode roundingMode)`
    pub fn div_19(
        _v1: *const (),
        _v2: *const (),
        scale: i32,
        _rounding_mode: RoundingMode,
    ) -> Result<f64> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (Number v1, Number v2, int scale, RoundingMode roundingMode)`
    pub fn div_20(
        _v1: *const (),
        _v2: *const (),
        scale: i32,
        _rounding_mode: RoundingMode,
    ) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (String v1, String v2, int scale, RoundingMode roundingMode)`
    pub fn div_21(
        _v1: *const (),
        _v2: *const (),
        scale: i32,
        _rounding_mode: RoundingMode,
    ) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::div#BigDecimal (BigDecimal v1, BigDecimal v2, int scale, RoundingMode roundingMode)`
    pub fn div_22(
        _v1: *const (),
        _v2: *const (),
        scale: i32,
        _rounding_mode: RoundingMode,
    ) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("div"))
    }

    // ══════════════════════════════════════════════
    //  向上取整除法
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::ceilDiv#int (int v1, int v2)`
    pub fn ceilDiv(v1: i32, v2: i32) -> Result<i32> {
        if v2 == 0 {
            return Err(CoreError::InvalidArgument {
                name: "v2",
                reason: "division by zero",
            });
        }
        // 对齐 Java: (v1 + v2 - 1) / v2 for positive; 直接用浮点 ceil
        Ok((v1 as f64 / v2 as f64).ceil() as i32)
    }

    // ══════════════════════════════════════════════
    //  舍入 / round / roundStr
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (double v, int scale)`
    /// 返回 Decimal 以保留精度（对齐 Java BigDecimal）
    pub fn round(v: f64, scale: i32) -> Result<Decimal> {
        let d = f64_to_decimal(v)?;
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(d.round_dp_with_strategy(scale_u, RoundingStrategy::MidpointAwayFromZero))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (double v, int scale)`
    pub fn roundStr(v: f64, scale: i32) -> Result<String> {
        let d = Self::round(v, scale)?;
        Ok(plain_fixed(&d, scale))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (String numberStr, int scale)`
    pub fn round_2(number_str: &str, scale: i32) -> Result<Decimal> {
        let d = Decimal::from_str(number_str.trim()).map_err(|_| CoreError::InvalidArgument {
            name: "numberStr",
            reason: "invalid decimal",
        })?;
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(d.round_dp_with_strategy(scale_u, RoundingStrategy::MidpointAwayFromZero))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (BigDecimal number, int scale)`
    pub fn round_3(number: Decimal, scale: i32) -> Result<Decimal> {
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(number.round_dp_with_strategy(scale_u, RoundingStrategy::MidpointAwayFromZero))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (String numberStr, int scale)`
    pub fn roundStr_2(number_str: &str, scale: i32) -> Result<String> {
        let d = Self::round_2(number_str, scale)?;
        Ok(plain_fixed(&d, scale))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (double v, int scale, RoundingMode roundingMode)`
    pub fn round_4(v: f64, scale: i32, rounding_mode: RoundingMode) -> Result<Decimal> {
        let d = f64_to_decimal(v)?;
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(d.round_dp_with_strategy(scale_u, rounding_mode.to_strategy()))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (double v, int scale, RoundingMode roundingMode)`
    pub fn roundStr_3(v: f64, scale: i32, rounding_mode: RoundingMode) -> Result<String> {
        let d = Self::round_4(v, scale, rounding_mode)?;
        Ok(plain_fixed(&d, scale))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (String numberStr, int scale, RoundingMode roundingMode)`
    pub fn round_5(number_str: &str, scale: i32, rounding_mode: RoundingMode) -> Result<Decimal> {
        let d = Decimal::from_str(number_str.trim()).map_err(|_| CoreError::InvalidArgument {
            name: "numberStr",
            reason: "invalid decimal",
        })?;
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(d.round_dp_with_strategy(scale_u, rounding_mode.to_strategy()))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::round#BigDecimal (BigDecimal number, int scale, RoundingMode roundingMode)`
    pub fn round_6(number: Decimal, scale: i32, rounding_mode: RoundingMode) -> Result<Decimal> {
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(number.round_dp_with_strategy(scale_u, rounding_mode.to_strategy()))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundStr#String (String numberStr, int scale, RoundingMode roundingMode)`
    pub fn roundStr_4(number_str: &str, scale: i32, rounding_mode: RoundingMode) -> Result<String> {
        let d = Self::round_5(number_str, scale, rounding_mode)?;
        Ok(plain_fixed(&d, scale))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundHalfEven#BigDecimal (Number number, int scale)`
    pub fn roundHalfEven(_number: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundHalfEven"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundHalfEven#BigDecimal (BigDecimal value, int scale)`
    pub fn roundHalfEven_2(value: Decimal, scale: i32) -> Result<Decimal> {
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(value.round_dp_with_strategy(scale_u, RoundingStrategy::MidpointNearestEven))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundDown#BigDecimal (Number number, int scale)`
    pub fn roundDown(_number: *const (), scale: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("roundDown"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::roundDown#BigDecimal (BigDecimal value, int scale)`
    pub fn roundDown_2(value: Decimal, scale: i32) -> Result<Decimal> {
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        Ok(value.round_dp_with_strategy(scale_u, RoundingStrategy::ToZero))
    }

    // ══════════════════════════════════════════════
    //  格式化 decimalFormat / formatPercent / decimalFormatMoney
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, double value)`
    pub fn decimalFormat(pattern: &str, value: f64) -> Result<String> {
        if !value.is_finite() {
            return Err(CoreError::InvalidArgument {
                name: "value",
                reason: "value is NaN or Infinite!",
            });
        }
        Ok(format_with_pattern(pattern, value))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, long value)`
    pub fn decimalFormat_2(pattern: &str, value: i64) -> Result<String> {
        Ok(format_with_pattern(pattern, value as f64))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, Object value)`
    pub fn decimalFormat_3(_pattern: *const (), _value: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormat#String (String pattern, Object value, RoundingMode roundingMode)`
    pub fn decimalFormat_4(
        _pattern: *const (),
        _value: *const (),
        _rounding_mode: RoundingMode,
    ) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("decimalFormat"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::decimalFormatMoney#String (double value)`
    pub fn decimalFormatMoney(value: f64) -> Result<String> {
        Self::decimalFormat(",##0.00", value)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::formatPercent#String (double number, int scale)`
    pub fn formatPercent(number: f64, scale: i32) -> Result<String> {
        if !number.is_finite() {
            return Err(CoreError::InvalidArgument {
                name: "number",
                reason: "number is NaN or Infinite!",
            });
        }
        let pct = number * 100.0;
        let scale_u = if scale < 0 { 0 } else { scale as usize };
        let factor = 10f64.powi(scale);
        let rounded = (pct * factor).round() / factor;
        if scale <= 0 {
            Ok(format!("{}%", rounded as i64))
        } else {
            Ok(format!("{rounded:.scale_u$}%"))
        }
    }

    // ══════════════════════════════════════════════
    //  数值判断 isNumber / isInteger / isDouble / isOdd / isEven / isPrimes / isValid
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isNumber#boolean (CharSequence str)`
    pub fn isNumber(s: &str) -> Result<bool> {
        let t = s.trim();
        if t.is_empty() {
            return Ok(false);
        }
        // 允许前导 +/- 、小数、科学计数
        Ok(t.parse::<f64>().is_ok() || Self::is_number_pattern(t))
    }

    /// 辅助: 更宽泛的数字模式匹配（科学计数 / 千分位）
    fn is_number_pattern(s: &str) -> bool {
        let bytes = s.as_bytes();
        let mut i = 0;
        if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
            i += 1;
        }
        let mut saw_digit = false;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            saw_digit = true;
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'.' {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                saw_digit = true;
                i += 1;
            }
        }
        if saw_digit && i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
            i += 1;
            if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
                i += 1;
            }
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
        }
        saw_digit && i == bytes.len()
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isInteger#boolean (String s)`
    pub fn isInteger(s: &str) -> Result<bool> {
        let t = s.trim();
        if t.is_empty() {
            return Ok(false);
        }
        Ok(t.parse::<i64>().is_ok())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLong#boolean (String s)`
    pub fn isLong(s: &str) -> Result<bool> {
        let t = s.trim();
        if t.is_empty() {
            return Ok(false);
        }
        Ok(t.parse::<i64>().is_ok())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isDouble#boolean (String s)`
    pub fn isDouble(s: &str) -> Result<bool> {
        let t = s.trim();
        if t.is_empty() {
            return Ok(false);
        }
        // 对齐 Java: 必须包含小数点
        if !t.contains('.') {
            return Ok(false);
        }
        Ok(t.parse::<f64>().is_ok())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isPrimes#boolean (int n)`
    /// 试除法判断素数
    pub fn isPrimes(n: i32) -> Result<bool> {
        if n <= 1 {
            return Ok(false);
        }
        if n <= 3 {
            return Ok(true);
        }
        if n % 2 == 0 || n % 3 == 0 {
            return Ok(false);
        }
        let mut i = 5i32;
        while i <= n / i {
            if n % i == 0 || n % (i + 2) == 0 {
                return Ok(false);
            }
            i += 6;
        }
        Ok(true)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isOdd#boolean (int num)`
    pub fn isOdd(num: i32) -> Result<bool> {
        Ok(num % 2 != 0)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isEven#boolean (int num)`
    pub fn isEven(num: i32) -> Result<bool> {
        Ok(num % 2 == 0)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValid#boolean (double number)`
    pub fn isValid(number: f64) -> Result<bool> {
        Ok(number.is_finite())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValid#boolean (float number)`
    pub fn isValid_2(number: f32) -> Result<bool> {
        Ok(number.is_finite())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isValidNumber#boolean (Number number)`
    pub fn isValidNumber(_number: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("isValidNumber"))
    }

    // ══════════════════════════════════════════════
    //  随机数 generateRandomNumber / generateBySet
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateRandomNumber#int[] (int begin, int end, int size)`
    pub fn generateRandomNumber(begin: i32, end: i32, size: i32) -> Result<Vec<i32>> {
        let (mut begin, mut end) = (begin, end);
        if begin > end {
            std::mem::swap(&mut begin, &mut end);
        }
        let range = end - begin;
        if range < size {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "Size is larger than range between begin and end!",
            });
        }
        let mut seed: Vec<i32> = (begin..end).collect();
        let mut rng = rand::thread_rng();
        let mut ran = Vec::with_capacity(size as usize);
        for i in 0..size as usize {
            let j = rng.gen_range(0..seed.len() - i);
            ran.push(seed[j]);
            let last = seed.len() - 1 - i;
            seed.swap(j, last);
        }
        Ok(ran)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateRandomNumber#int[] (int begin, int end, int size, int[] seed)`
    pub fn generateRandomNumber_2(
        begin: i32,
        end: i32,
        size: i32,
        seed: Vec<i32>,
    ) -> Result<Vec<i32>> {
        Err(CoreError::PendingEngine("generateRandomNumber"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::generateBySet#Integer[] (int begin, int end, int size)`
    pub fn generateBySet(begin: i32, end: i32, size: i32) -> Result<Vec<i32>> {
        let (mut begin, mut end) = (begin, end);
        if begin > end {
            std::mem::swap(&mut begin, &mut end);
        }
        let range = end - begin;
        if range < size {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "Size is larger than range",
            });
        }
        let mut set = std::collections::HashSet::new();
        let mut rng = rand::thread_rng();
        while (set.len() as i32) < size {
            set.insert(rng.gen_range(begin..end));
        }
        Ok(set.into_iter().collect())
    }

    // ══════════════════════════════════════════════
    //  范围 range / appendRange
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int stop)`
    pub fn range(stop: i32) -> Result<Vec<i32>> {
        Ok((0..stop).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int start, int stop)`
    pub fn range_2(start: i32, stop: i32) -> Result<Vec<i32>> {
        Ok((start..stop).collect())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::range#int[] (int start, int stop, int step)`
    pub fn range_3(start: i32, stop: i32, step: i32) -> Result<Vec<i32>> {
        if step == 0 {
            return Err(CoreError::InvalidArgument {
                name: "step",
                reason: "step cannot be zero",
            });
        }
        let mut result = Vec::new();
        if step > 0 {
            let mut current = start;
            while current < stop {
                result.push(current);
                current += step;
            }
        } else {
            let mut current = start;
            while current > stop {
                result.push(current);
                current += step;
            }
        }
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::appendRange#Collection<Integer> (int start, int stop, Collection<Integer> values)`
    pub fn appendRange(start: i32, stop: i32, mut values: Vec<i32>) -> Result<Vec<i32>> {
        values.extend(start..stop);
        Ok(values)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::appendRange#Collection<Integer> (int start, int stop, int step, Collection<Integer> values)`
    pub fn appendRange_2(
        start: i32,
        stop: i32,
        step: i32,
        mut values: Vec<i32>,
    ) -> Result<Vec<i32>> {
        if step == 0 {
            return Err(CoreError::InvalidArgument {
                name: "step",
                reason: "step cannot be zero",
            });
        }
        if step > 0 {
            let mut current = start;
            while current < stop {
                values.push(current);
                current += step;
            }
        } else {
            let mut current = start;
            while current > stop {
                values.push(current);
                current += step;
            }
        }
        Ok(values)
    }

    // ══════════════════════════════════════════════
    //  阶乘 factorial
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#BigInteger (BigInteger n)`
    pub fn factorial(_n: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#BigInteger (BigInteger start, BigInteger end)`
    pub fn factorial_2(_start: *const (), _end: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("factorial"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#long (long start, long end)`
    /// 范围阶乘: product(start..=end)
    pub fn factorial_3(start: i64, end: i64) -> Result<i64> {
        if start > end {
            return Err(CoreError::InvalidArgument {
                name: "start",
                reason: "start must be <= end",
            });
        }
        if start < 0 {
            return Err(CoreError::InvalidArgument {
                name: "start",
                reason: "factorial not defined for negative numbers",
            });
        }
        let mut result: i64 = 1;
        for i in start..=end {
            result = result.checked_mul(i).ok_or(CoreError::InvalidArgument {
                name: "factorial",
                reason: "long overflow",
            })?;
        }
        Ok(result)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::factorial#long (long n)`
    pub fn factorial_4(n: i64) -> Result<i64> {
        if n < 0 {
            return Err(CoreError::InvalidArgument {
                name: "n",
                reason: "factorial not defined for negative numbers",
            });
        }
        if n > 20 {
            return Err(CoreError::InvalidArgument {
                name: "n",
                reason: "factorial overflow for n > 20",
            });
        }
        Self::factorial_3(1, n)
    }

    // ══════════════════════════════════════════════
    //  数学 sqrt / processMultiple / divisor / multiple
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::sqrt#long (long x)`
    /// 整数平方根
    pub fn sqrt(x: i64) -> Result<i64> {
        if x < 0 {
            return Err(CoreError::InvalidArgument {
                name: "x",
                reason: "square root of negative number",
            });
        }
        if x == 0 {
            return Ok(0);
        }
        // 牛顿法
        let mut guess = (x as f64).sqrt() as i64;
        while guess > x / guess {
            guess -= 1;
        }
        while (guess + 1) <= x / (guess + 1) {
            guess += 1;
        }
        Ok(guess)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::processMultiple#int (int selectNum, int minNum)`
    pub fn processMultiple(selectNum: i32, minNum: i32) -> Result<i32> {
        Err(CoreError::PendingEngine("processMultiple"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::divisor#int (int m, int n)`
    /// 最大公约数
    pub fn divisor(m: i32, n: i32) -> Result<i32> {
        let mut a = m.abs() as i64;
        let mut b = n.abs() as i64;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        Ok(a as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::multiple#int (int m, int n)`
    /// 最小公倍数
    pub fn multiple(m: i32, n: i32) -> Result<i32> {
        if m == 0 || n == 0 {
            return Ok(0);
        }
        let gcd = Self::divisor(m, n)? as i64;
        let result = (m.abs() as i64 / gcd) * n.abs() as i64;
        if result > i32::MAX as i64 {
            return Err(CoreError::InvalidArgument {
                name: "multiple",
                reason: "Integer overflow",
            });
        }
        Ok(result as i32)
    }

    // ══════════════════════════════════════════════
    //  进制转换 getBinaryStr / binaryToInt / binaryToLong
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::getBinaryStr#String (Number number)`
    /// 返回 i64 的二进制字符串表示
    pub fn getBinaryStr(number: i64) -> Result<String> {
        Ok(format!("{number:b}"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::binaryToInt#int (String binaryStr)`
    pub fn binaryToInt(binary_str: &str) -> Result<i32> {
        let t = binary_str.trim();
        if t.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "binaryStr",
                reason: "empty binary string",
            });
        }
        i32::from_str_radix(t, 2).map_err(|_| CoreError::InvalidArgument {
            name: "binaryStr",
            reason: "invalid binary string",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::binaryToLong#long (String binaryStr)`
    pub fn binaryToLong(binary_str: &str) -> Result<i64> {
        let t = binary_str.trim();
        if t.is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "binaryStr",
                reason: "empty binary string",
            });
        }
        i64::from_str_radix(t, 2).map_err(|_| CoreError::InvalidArgument {
            name: "binaryStr",
            reason: "invalid binary string",
        })
    }

    // ══════════════════════════════════════════════
    //  比较 compare
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (char x, char y)`
    pub fn compare(x: char, y: char) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (double x, double y)`
    pub fn compare_2(x: f64, y: f64) -> Result<i32> {
        Ok(x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (int x, int y)`
    pub fn compare_3(x: i32, y: i32) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (long x, long y)`
    pub fn compare_4(x: i64, y: i64) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (short x, short y)`
    pub fn compare_5(x: i16, y: i16) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::compare#int (byte x, byte y)`
    pub fn compare_6(x: i8, y: i8) -> Result<i32> {
        Ok(x.cmp(&y) as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isGreater#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isGreater(a: Decimal, b: Decimal) -> Result<bool> {
        Ok(a > b)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isGreaterOrEqual#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isGreaterOrEqual(a: Decimal, b: Decimal) -> Result<bool> {
        Ok(a >= b)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLess#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isLess(a: Decimal, b: Decimal) -> Result<bool> {
        Ok(a < b)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isLessOrEqual#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn isLessOrEqual(a: Decimal, b: Decimal) -> Result<bool> {
        Ok(a <= b)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isIn#boolean (final BigDecimal value, final BigDecimal minInclude, final BigDecimal maxInclude)`
    pub fn isIn(value: Decimal, min_include: Decimal, max_include: Decimal) -> Result<bool> {
        Ok(value >= min_include && value <= max_include)
    }

    // ══════════════════════════════════════════════
    //  相等判断 equals
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (double num1, double num2)`
    pub fn equals(num1: f64, num2: f64) -> Result<bool> {
        Ok((num1 - num2).abs() < f64::EPSILON)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (float num1, float num2)`
    pub fn equals_2(num1: f32, num2: f32) -> Result<bool> {
        Ok((num1 - num2).abs() < f32::EPSILON)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (long num1, long num2)`
    pub fn equals_3(num1: i64, num2: i64) -> Result<bool> {
        Ok(num1 == num2)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (final Number number1, final Number number2)`
    pub fn equals_4(_number1: *const (), _number2: *const ()) -> Result<bool> {
        Err(CoreError::PendingEngine("equals"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (BigDecimal bigNum1, BigDecimal bigNum2)`
    pub fn equals_5(a: Decimal, b: Decimal) -> Result<bool> {
        Ok(a == b)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::equals#boolean (char c1, char c2, boolean ignoreCase)`
    pub fn equals_6(c1: char, c2: char, ignoreCase: bool) -> Result<bool> {
        if ignoreCase {
            Ok(c1.eq_ignore_ascii_case(&c2))
        } else {
            Ok(c1 == c2)
        }
    }

    // ══════════════════════════════════════════════
    //  最值 min / max
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#T (T[] numberArray)`
    pub fn min(numberArray: Vec<i64>) -> Result<i64> {
        numberArray
            .into_iter()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#long (long... numberArray)`
    pub fn min_2(numberArray: &[i64]) -> Result<i64> {
        numberArray
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#int (int... numberArray)`
    pub fn min_3(numberArray: &[i32]) -> Result<i32> {
        numberArray
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#short (short... numberArray)`
    pub fn min_4(numberArray: &[i16]) -> Result<i16> {
        numberArray
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#double (double... numberArray)`
    pub fn min_5(numberArray: &[f64]) -> Result<f64> {
        numberArray
            .iter()
            .copied()
            .reduce(f64::min)
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#float (float... numberArray)`
    pub fn min_6(numberArray: &[f32]) -> Result<f32> {
        numberArray
            .iter()
            .copied()
            .reduce(f32::min)
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::min#BigDecimal (BigDecimal... numberArray)`
    pub fn min_7(numberArray: &[Decimal]) -> Result<Decimal> {
        numberArray
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#T (T[] numberArray)`
    pub fn max(numberArray: Vec<i64>) -> Result<i64> {
        numberArray
            .into_iter()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#long (long... numberArray)`
    pub fn max_2(numberArray: &[i64]) -> Result<i64> {
        numberArray
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#int (int... numberArray)`
    pub fn max_3(numberArray: &[i32]) -> Result<i32> {
        numberArray
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#short (short... numberArray)`
    pub fn max_4(numberArray: &[i16]) -> Result<i16> {
        numberArray
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#double (double... numberArray)`
    pub fn max_5(numberArray: &[f64]) -> Result<f64> {
        numberArray
            .iter()
            .copied()
            .reduce(f64::max)
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#float (float... numberArray)`
    pub fn max_6(numberArray: &[f32]) -> Result<f32> {
        numberArray
            .iter()
            .copied()
            .reduce(f32::max)
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::max#BigDecimal (BigDecimal... numberArray)`
    pub fn max_7(numberArray: &[Decimal]) -> Result<Decimal> {
        numberArray
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "numberArray",
                reason: "empty array",
            })
    }

    // ══════════════════════════════════════════════
    //  字符串转换 toStr
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number, String defaultValue)`
    pub fn toStr(_number: *const (), _defaultValue: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toStr"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number)`
    pub fn toStr_2(number: f64) -> Result<String> {
        Ok(number.to_string())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (Number number, boolean isStripTrailingZeros)`
    pub fn toStr_3(number: f64, isStripTrailingZeros: bool) -> Result<String> {
        if isStripTrailingZeros {
            // 去除尾部零: 用 Decimal normalize
            let d = f64_to_decimal(number)?;
            Ok(d.normalize().to_string())
        } else {
            Ok(number.to_string())
        }
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (BigDecimal bigDecimal)`
    pub fn toStr_4(bigDecimal: Decimal) -> Result<String> {
        Ok(bigDecimal.normalize().to_string())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toStr#String (BigDecimal bigDecimal, boolean isStripTrailingZeros)`
    pub fn toStr_5(bigDecimal: Decimal, isStripTrailingZeros: bool) -> Result<String> {
        if isStripTrailingZeros {
            Ok(bigDecimal.normalize().to_string())
        } else {
            Ok(bigDecimal.to_string())
        }
    }

    // ══════════════════════════════════════════════
    //  BigDecimal / BigInteger 转换
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigDecimal#BigDecimal (Number number)`
    pub fn toBigDecimal(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigDecimal"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigDecimal#BigDecimal (String numberStr)`
    pub fn toBigDecimal_2(numberStr: &str) -> Result<Decimal> {
        let t = numberStr.trim();
        if t.is_empty() {
            return Ok(Decimal::ZERO);
        }
        Decimal::from_str(t).map_err(|_| CoreError::InvalidArgument {
            name: "numberStr",
            reason: "invalid decimal string",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigInteger#BigInteger (Number number)`
    pub fn toBigInteger(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigInteger"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBigInteger#BigInteger (String number)`
    pub fn toBigInteger_2(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toBigInteger"))
    }

    // ══════════════════════════════════════════════
    //  计数 count / null2Zero / zero2One / nullToZero
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::count#int (int total, int part)`
    /// 向上取整除法: ceil(total / part)
    pub fn count(total: i32, part: i32) -> Result<i32> {
        if part == 0 {
            return Err(CoreError::InvalidArgument {
                name: "part",
                reason: "division by zero",
            });
        }
        Ok((total as f64 / part as f64).ceil() as i32)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::null2Zero#BigDecimal (BigDecimal decimal)`
    pub fn null2Zero(decimal: Option<Decimal>) -> Result<Decimal> {
        Ok(decimal.unwrap_or(Decimal::ZERO))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::zero2One#int (int value)`
    pub fn zero2One(value: i32) -> Result<i32> {
        Ok(if value == 0 { 1 } else { value })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#int (Integer number)`
    pub fn nullToZero(number: Option<i32>) -> Result<i32> {
        Ok(number.unwrap_or(0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#long (Long number)`
    pub fn nullToZero_2(number: Option<i64>) -> Result<i64> {
        Ok(number.unwrap_or(0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#double (Double number)`
    pub fn nullToZero_3(number: Option<f64>) -> Result<f64> {
        Ok(number.unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#float (Float number)`
    pub fn nullToZero_4(number: Option<f32>) -> Result<f32> {
        Ok(number.unwrap_or(0.0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#short (Short number)`
    pub fn nullToZero_5(number: Option<i16>) -> Result<i16> {
        Ok(number.unwrap_or(0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#byte (Byte number)`
    pub fn nullToZero_6(number: Option<i8>) -> Result<i8> {
        Ok(number.unwrap_or(0))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#BigDecimal (BigDecimal number)`
    pub fn nullToZero_7(number: Option<Decimal>) -> Result<Decimal> {
        Ok(number.unwrap_or(Decimal::ZERO))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::nullToZero#BigInteger (BigInteger number)`
    pub fn nullToZero_8(_number: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("nullToZero"))
    }

    // ══════════════════════════════════════════════
    //  newBigInteger
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::newBigInteger#BigInteger (String str)`
    pub fn newBigInteger(_str: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("newBigInteger"))
    }

    // ══════════════════════════════════════════════
    //  相邻判断 isBeside / 分段 partValue
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isBeside#boolean (long number1, long number2)`
    pub fn isBeside(number1: i64, number2: i64) -> Result<bool> {
        Ok(number1.abs_diff(number2) == 1)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isBeside#boolean (int number1, int number2)`
    pub fn isBeside_2(number1: i32, number2: i32) -> Result<bool> {
        Ok((number1 as i64 - number2 as i64).unsigned_abs() == 1)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::partValue#int (int total, int partCount)`
    pub fn partValue(total: i32, partCount: i32) -> Result<i32> {
        Self::partValue_2(total, partCount, true)
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::partValue#int (int total, int partCount, boolean isPlusOneWhenHasRem)`
    pub fn partValue_2(total: i32, partCount: i32, isPlusOneWhenHasRem: bool) -> Result<i32> {
        if partCount == 0 {
            return Err(CoreError::InvalidArgument {
                name: "partCount",
                reason: "partCount cannot be zero",
            });
        }
        let mut part = total / partCount;
        if isPlusOneWhenHasRem && total % partCount != 0 {
            part += 1;
        }
        Ok(part)
    }

    // ══════════════════════════════════════════════
    //  幂运算 pow / isPowerOfTwo
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (Number number, int n)`
    pub fn pow(number: Decimal, n: i32) -> Result<Decimal> {
        Ok(Self::pow_decimal(number, n))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (BigDecimal number, int n)`
    pub fn pow_2(number: Decimal, n: i32) -> Result<Decimal> {
        Ok(Self::pow_decimal(number, n))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::pow#BigDecimal (BigDecimal number, int n, int scale, RoundingMode roundingMode)`
    pub fn pow_3(
        number: Decimal,
        n: i32,
        scale: i32,
        rounding_mode: RoundingMode,
    ) -> Result<Decimal> {
        if n >= 0 {
            return Ok(Self::pow_decimal(number, n));
        }
        // 负指数: 1 / x^|n|, 按指定 scale 和舍入
        let pos = Self::pow_decimal(number, (-n) as u32);
        let one = Decimal::ONE;
        let scale_u = if scale < 0 { 0u32 } else { scale as u32 };
        one.checked_div(pos)
            .map(|d| d.round_dp_with_strategy(scale_u, rounding_mode.to_strategy()))
            .ok_or(CoreError::InvalidArgument {
                name: "pow",
                reason: "division failed in negative exponent",
            })
    }

    /// 内部: Decimal 快速幂
    fn pow_decimal(mut base: Decimal, mut exp: u32) -> Decimal {
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

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::isPowerOfTwo#boolean (long n)`
    pub fn isPowerOfTwo(n: i64) -> Result<bool> {
        Ok(n > 0 && (n & (n - 1)) == 0)
    }

    // ══════════════════════════════════════════════
    //  解析 parseInt / parseLong / parseFloat / parseDouble / parseNumber
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseInt#int (String number)`
    pub fn parseInt(number: &str) -> Result<i32> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0);
        }
        // 十六进制
        if t.len() > 2 && t[..2].eq_ignore_ascii_case("0x") {
            return i32::from_str_radix(&t[2..], 16).map_err(|_| CoreError::InvalidArgument {
                name: "number",
                reason: "invalid hex int",
            });
        }
        t.parse::<i32>().map_err(|_| CoreError::InvalidArgument {
            name: "number",
            reason: "NumberFormatException",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseLong#long (String number)`
    pub fn parseLong(number: &str) -> Result<i64> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0);
        }
        // 十六进制
        if t.len() > 2 && t[..2].eq_ignore_ascii_case("0x") {
            return i64::from_str_radix(&t[2..], 16).map_err(|_| CoreError::InvalidArgument {
                name: "number",
                reason: "invalid hex long",
            });
        }
        t.parse::<i64>().map_err(|_| CoreError::InvalidArgument {
            name: "number",
            reason: "NumberFormatException",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseFloat#float (String number)`
    pub fn parseFloat(number: &str) -> Result<f32> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0.0);
        }
        t.parse::<f32>().map_err(|_| CoreError::InvalidArgument {
            name: "number",
            reason: "NumberFormatException",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseDouble#double (String number)`
    pub fn parseDouble(number: &str) -> Result<f64> {
        let t = number.trim();
        if t.is_empty() {
            return Ok(0.0);
        }
        t.parse::<f64>().map_err(|_| CoreError::InvalidArgument {
            name: "number",
            reason: "NumberFormatException",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseNumber#Number (String numberStr)`
    pub fn parseNumber(numberStr: &str) -> Result<Decimal> {
        let t = numberStr.trim();
        if t.is_empty() {
            return Ok(Decimal::ZERO);
        }
        // 去千分位
        let cleaned = t.replace(',', "");
        Decimal::from_str(&cleaned).map_err(|_| CoreError::InvalidArgument {
            name: "numberStr",
            reason: "invalid number",
        })
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseInt#Integer (String numberStr, Integer defaultValue)`
    pub fn parseInt_2(numberStr: &str, defaultValue: i32) -> Result<i32> {
        Self::parseInt(numberStr).or(Ok(defaultValue))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseLong#Long (String numberStr, Long defaultValue)`
    pub fn parseLong_2(numberStr: &str, defaultValue: i64) -> Result<i64> {
        Self::parseLong(numberStr).or(Ok(defaultValue))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseFloat#Float (String numberStr, Float defaultValue)`
    pub fn parseFloat_2(numberStr: &str, defaultValue: f32) -> Result<f32> {
        Self::parseFloat(numberStr).or(Ok(defaultValue))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseDouble#Double (String numberStr, Double defaultValue)`
    pub fn parseDouble_2(numberStr: &str, defaultValue: f64) -> Result<f64> {
        Self::parseDouble(numberStr).or(Ok(defaultValue))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::parseNumber#Number (String numberStr, Number defaultValue)`
    pub fn parseNumber_2(numberStr: &str, defaultValue: Decimal) -> Result<Decimal> {
        Self::parseNumber(numberStr).or(Ok(defaultValue))
    }

    // ══════════════════════════════════════════════
    //  字节转换 toBytes / toInt
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toBytes#byte[] (int value)`
    /// 大端序
    pub fn toBytes(value: i32) -> Result<Vec<u8>> {
        Ok(value.to_be_bytes().to_vec())
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toInt#int (byte[] bytes)`
    /// 大端序
    pub fn toInt(bytes: &[u8]) -> Result<i32> {
        if bytes.len() < 4 {
            return Err(CoreError::InvalidArgument {
                name: "bytes",
                reason: "need at least 4 bytes",
            });
        }
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&bytes[..4]);
        Ok(i32::from_be_bytes(buf))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toUnsignedByteArray#byte[] (BigInteger value)`
    pub fn toUnsignedByteArray(_value: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toUnsignedByteArray#byte[] (int length, BigInteger value)`
    pub fn toUnsignedByteArray_2(length: i32, _value: *const ()) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("toUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::fromUnsignedByteArray#BigInteger (byte[] buf)`
    pub fn fromUnsignedByteArray(buf: Vec<i8>) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("fromUnsignedByteArray"))
    }

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::fromUnsignedByteArray#BigInteger (byte[] buf, int off, int length)`
    pub fn fromUnsignedByteArray_2(buf: Vec<i8>, off: i32, length: i32) -> Result<OPAQUE> {
        Err(CoreError::PendingEngine("fromUnsignedByteArray"))
    }

    // ══════════════════════════════════════════════
    //  计算器 calculate
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::calculate#double (String expression)`
    pub fn calculate(_expression: &str) -> Result<f64> {
        Err(CoreError::PendingEngine("calculate"))
    }

    // ══════════════════════════════════════════════
    //  类型转换 toDouble
    // ══════════════════════════════════════════════

    /// 对齐 Java: `cn.hutool.core.util::NumberUtil::toDouble#double (Number value)`
    pub fn toDouble(value: f64) -> Result<f64> {
        Ok(value)
    }
}
