//! 对齐: `cn.hutool.core.convert.NumberWordFormatter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/NumberWordFormatter.java

#![allow(dead_code)]

/// 对齐 Java 类: `cn.hutool.core.convert.NumberWordFormatter`
#[derive(Debug, Clone, Default)]
pub struct NumberWordFormatter;

const NUMBER: [&str; 10] = [
    "", "ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN", "EIGHT", "NINE",
];
const NUMBER_TEEN: [&str; 10] = [
    "TEN",
    "ELEVEN",
    "TWELVE",
    "THIRTEEN",
    "FOURTEEN",
    "FIFTEEN",
    "SIXTEEN",
    "SEVENTEEN",
    "EIGHTEEN",
    "NINETEEN",
];
const NUMBER_TEN: [&str; 10] = [
    "TEN", "TWENTY", "THIRTY", "FORTY", "FIFTY", "SIXTY", "SEVENTY", "EIGHTY", "NINETY", "",
];
const NUMBER_MORE: [&str; 5] = ["", "THOUSAND", "MILLION", "BILLION", "TRILLION"];
const NUMBER_SUFFIX: [&str; 7] = ["k", "w", "m", "b", "t", "p", "e"];
const STANDARD_UNIT_INDICES: [usize; 6] = [0, 2, 3, 4, 5, 6];

impl NumberWordFormatter {
    /// 对齐 Java: `NumberWordFormatter.format(Object)`
    pub fn format(x: Option<&str>) -> String {
        match x {
            Some(s) => Self::format_str(s),
            None => String::new(),
        }
    }

    /// 对齐 Java: `NumberWordFormatter.format(Object)` — 数字入参
    pub fn format_number(x: impl ToString) -> String {
        Self::format_str(&x.to_string())
    }

    /// 对齐 Java: `NumberWordFormatter.formatSimple(long)`
    pub fn format_simple(value: i64) -> String {
        Self::format_simple_with(value, true)
    }

    /// 对齐 Java: `NumberWordFormatter.formatSimple(long, boolean)`
    pub fn format_simple_with(value: i64, is_two: bool) -> String {
        if value < 1000 {
            return value.to_string();
        }
        let mut res = value as f64;
        let index: usize;
        if is_two {
            if value >= 10000 {
                res = value as f64 / 10000.0;
                index = 1;
            } else {
                res = value as f64 / 1000.0;
                index = 0;
            }
        } else {
            let mut unit_index: i32 = -1;
            while res >= 1000.0 && unit_index < (STANDARD_UNIT_INDICES.len() as i32 - 1) {
                res /= 1000.0;
                unit_index += 1;
            }
            index = STANDARD_UNIT_INDICES[unit_index as usize];
        }
        format!(
            "{}{}",
            decimal_format_hash_dot_hash_hash(res),
            NUMBER_SUFFIX[index]
        )
    }

    fn format_str(x: &str) -> String {
        // 非数字会在 parse 时抛出（对齐 NumberFormatException）
        let z = x.find('.');
        let (lstr, rstr) = if let Some(zi) = z {
            (&x[..zi], &x[zi + 1..])
        } else {
            (x, "")
        };
        // 对齐 Java: 非数字字符串在 parseInt 时抛 NumberFormatException
        let check = if lstr.is_empty() { "0" } else { lstr };
        if check.parse::<i128>().is_err() {
            panic!("NumberFormatException");
        }
        if z.is_some() && !rstr.is_empty() {
            // 小数部分仅用于两位转换；非法字符同样失败
            let _ = rstr; // 交由 trans_two / parseInt 路径处理
        }

        let mut lstrrev: String = lstr.chars().rev().collect();
        match lstrrev.len() % 3 {
            1 => lstrrev.push_str("00"),
            2 => lstrrev.push('0'),
            _ => {}
        }
        let mut lm = String::new();
        let groups = lstrrev.len() / 3;
        for i in 0..groups {
            let chunk: String = lstrrev[3 * i..3 * i + 3].chars().rev().collect();
            if chunk != "000" {
                if i != 0 {
                    lm = format!("{} {} {}", Self::trans_three(&chunk), NUMBER_MORE[i], lm);
                } else {
                    lm = Self::trans_three(&chunk);
                }
            } else {
                lm.push_str(&Self::trans_three(&chunk));
            }
        }
        lm = lm.trim().to_string();

        let mut xs = if lm.is_empty() {
            "ZERO ".to_string()
        } else {
            " ".to_string()
        };
        if z.is_some() {
            xs.push_str("AND CENTS ");
            xs.push_str(&Self::trans_two(rstr));
            xs.push(' ');
        }
        format!("{}{}ONLY", lm.trim(), xs)
    }

    fn parse_teen(s: &str) -> String {
        NUMBER_TEEN[s.parse::<usize>().unwrap() - 10].to_string()
    }

    fn parse_ten(s: &str) -> String {
        NUMBER_TEN[s[..1].parse::<usize>().unwrap() - 1].to_string()
    }

    fn parse_last(s: &str) -> String {
        let last = s.chars().last().unwrap().to_digit(10).unwrap() as usize;
        NUMBER[last].to_string()
    }

    fn trans_two(s: &str) -> String {
        let mut s = s.to_string();
        if s.len() > 2 {
            s = s[..2].to_string();
        } else if s.len() < 2 {
            s.push('0');
        }
        if s.starts_with('0') {
            Self::parse_last(&s)
        } else if s.starts_with('1') {
            Self::parse_teen(&s)
        } else if s.ends_with('0') {
            Self::parse_ten(&s)
        } else {
            format!("{} {}", Self::parse_ten(&s), Self::parse_last(&s))
        }
    }

    fn trans_three(s: &str) -> String {
        if s.starts_with('0') {
            Self::trans_two(&s[1..])
        } else if &s[1..] == "00" {
            format!("{} HUNDRED", Self::parse_last(&s[..1]))
        } else {
            format!(
                "{} HUNDRED AND {}",
                Self::parse_last(&s[..1]),
                Self::trans_two(&s[1..])
            )
        }
    }
}

/// 对齐 Java `NumberUtil.decimalFormat("#.##", res)` 的常见输出形态
fn decimal_format_hash_dot_hash_hash(res: f64) -> String {
    // 去掉无意义的尾随 0，对齐 #.##
    let rounded = (res * 100.0).round() / 100.0;
    if (rounded - rounded.round()).abs() < f64::EPSILON {
        format!("{}", rounded as i64)
    } else {
        let s = format!("{:.2}", rounded);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
