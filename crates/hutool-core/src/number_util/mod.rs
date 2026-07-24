//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本提供算术、比较、最值、解析与 BigDecimal（`rust_decimal::Decimal`）对齐实现。

use crate::{CoreError, Result};
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use std::collections::HashSet;
use std::str::FromStr;

mod number_util;
mod parsed_number;

pub use number_util::NumberUtil;
pub use parsed_number::ParsedNumber;

fn decimal_to_f64(d: Decimal) -> f64 {
    d.to_string().parse().unwrap_or(0.0)
}

fn f64_to_java_string(v: f64) -> String {
    // 对常见用例足够；科学计数交由 Decimal 解析
    let s = format!("{}", v);
    if s.contains('e') || s.contains('E') {
        s
    } else {
        s
    }
}

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

fn is_scientific_form(s: &str) -> bool {
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
    if !saw_digit || i >= bytes.len() || (bytes[i] != b'e' && bytes[i] != b'E') {
        return false;
    }
    i += 1;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let exp_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    i == bytes.len() && i > exp_start
}

fn extract_number_prefix(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();
    let mut i = 0;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let start = i;
    let mut saw_digit = false;
    let mut saw_dot = false;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_digit() {
            saw_digit = true;
            i += 1;
        } else if c == b'.' && !saw_dot {
            saw_dot = true;
            i += 1;
        } else {
            break;
        }
    }
    if !saw_digit && !(saw_dot && i > start) {
        // ".123"
        if s.starts_with('.') || s.starts_with("+.") || s.starts_with("-.") {
            let mut j = if s.as_bytes()[0] == b'+' || s.as_bytes()[0] == b'-' {
                1
            } else {
                0
            };
            if j < bytes.len() && bytes[j] == b'.' {
                j += 1;
                let d0 = j;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                if j > d0 {
                    return Some(&s[..j]);
                }
            }
        }
        return None;
    }
    // 仅 "." 无数字 → 当作 0（parseInt(".123") 走 Number 后 intValue=0）
    Some(&s[..i])
}
