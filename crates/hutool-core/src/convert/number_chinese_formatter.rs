//! 对齐: `cn.hutool.core.convert.NumberChineseFormatter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/NumberChineseFormatter.java

#![allow(dead_code, clippy::needless_range_loop)]

use rust_decimal::Decimal;
use std::str::FromStr;

/// 对齐 Java 类: `cn.hutool.core.convert.NumberChineseFormatter`
#[derive(Debug, Clone, Default)]
pub struct NumberChineseFormatter;

/// 中文形式，奇数位置是简体，偶数位置是记账繁体，0共用
const DIGITS: [char; 19] = [
    '零', '一', '壹', '二', '贰', '三', '叁', '四', '肆', '五', '伍', '六', '陆', '七', '柒', '八',
    '捌', '九', '玖',
];

struct ChineseUnit {
    name: char,
    value: i32,
    sec_unit: bool,
}

const CHINESE_NAME_VALUE: [ChineseUnit; 9] = [
    ChineseUnit {
        name: ' ',
        value: 1,
        sec_unit: false,
    },
    ChineseUnit {
        name: '十',
        value: 10,
        sec_unit: false,
    },
    ChineseUnit {
        name: '拾',
        value: 10,
        sec_unit: false,
    },
    ChineseUnit {
        name: '百',
        value: 100,
        sec_unit: false,
    },
    ChineseUnit {
        name: '佰',
        value: 100,
        sec_unit: false,
    },
    ChineseUnit {
        name: '千',
        value: 1000,
        sec_unit: false,
    },
    ChineseUnit {
        name: '仟',
        value: 1000,
        sec_unit: false,
    },
    ChineseUnit {
        name: '万',
        value: 10_000,
        sec_unit: true,
    },
    ChineseUnit {
        name: '亿',
        value: 100_000_000,
        sec_unit: true,
    },
];

impl NumberChineseFormatter {
    /// 对齐 Java: `format(double, boolean)`
    pub fn format(amount: f64, is_use_traditional: bool) -> String {
        Self::format_money(amount, is_use_traditional, false)
    }

    /// 对齐 Java: `format(double, boolean, boolean)`
    pub fn format_money(amount: f64, is_use_traditional: bool, is_money_mode: bool) -> String {
        Self::format_full(amount, is_use_traditional, is_money_mode, "负", "元")
    }

    /// 对齐 Java: `format(double, boolean, boolean, String, String)`
    pub fn format_full(
        mut amount: f64,
        is_use_traditional: bool,
        is_money_mode: bool,
        negative_name: &str,
        unit_name: &str,
    ) -> String {
        let unit_name = if unit_name.is_empty() || unit_name == "null" {
            "元"
        } else {
            unit_name
        };
        if amount == 0.0 {
            return if is_money_mode {
                format!("零{unit_name}整")
            } else {
                "零".to_string()
            };
        }
        assert!(
            (-99_9999_9999_9999.99..=99_9999_9999_9999.99).contains(&amount),
            "Number support only: (-99999999999999.99 ~ 99999999999999.99)！"
        );
        let mut chinese_str = String::new();
        if amount < 0.0 {
            chinese_str.push_str(if negative_name.is_empty() {
                "负"
            } else {
                negative_name
            });
            amount = -amount;
        }
        let mut yuan = (amount * 100.0).round() as i64;
        let fen = (yuan % 10) as i32;
        yuan /= 10;
        let jiao = (yuan % 10) as i32;
        yuan /= 10;

        if !is_money_mode || yuan != 0 {
            chinese_str.push_str(&Self::long_to_chinese(yuan, is_use_traditional));
            if is_money_mode {
                chinese_str.push_str(unit_name);
            }
        }
        if jiao == 0 && fen == 0 {
            if is_money_mode {
                chinese_str.push('整');
            }
            return chinese_str;
        }
        if !is_money_mode {
            chinese_str.push('点');
        }
        if yuan == 0 && jiao == 0 {
            if !is_money_mode {
                chinese_str.push('零');
            }
        } else {
            chinese_str.push(Self::number_to_chinese(jiao, is_use_traditional));
            if is_money_mode && jiao != 0 {
                chinese_str.push('角');
            }
        }
        if fen != 0 {
            chinese_str.push(Self::number_to_chinese(fen, is_use_traditional));
            if is_money_mode {
                chinese_str.push('分');
            }
        }
        chinese_str
    }

    /// 对齐 Java: `format(long, boolean)`
    pub fn format_long(mut amount: i64, is_use_traditional: bool) -> String {
        if amount == 0 {
            return "零".to_string();
        }
        let mut chinese_str = String::new();
        if amount < 0 {
            chinese_str.push('负');
            amount = -amount;
        }
        chinese_str.push_str(&Self::long_to_chinese(amount, is_use_traditional));
        chinese_str
    }

    /// 对齐 Java: `formatSimple(long)`
    pub fn format_simple(amount: i64) -> String {
        if (-10_000..10_000).contains(&amount) {
            return amount.to_string();
        }
        if (-100_000_000..100_000_000).contains(&amount) {
            let v = (amount as f64) / 10_000.0;
            format!("{}万", trim_decimal(v, 2))
        } else if (-1_0000_0000_0000i64..1_0000_0000_0000i64).contains(&amount) {
            let v = (amount as f64) / 100_000_000.0;
            format!("{}亿", trim_decimal(v, 2))
        } else {
            let v = (amount as f64) / 1_0000_0000_0000.0;
            format!("{}万亿", trim_decimal(v, 2))
        }
    }

    /// 对齐 Java: `formatThousand(int, boolean)`
    pub fn format_thousand(amount: i32, is_use_traditional: bool) -> String {
        assert!(
            (-999..=999).contains(&amount),
            "Number support only: (-999 ~ 999)！"
        );
        let chinese = Self::thousand_to_chinese(amount.abs(), is_use_traditional);
        if (10..20).contains(&amount) {
            return chinese.chars().skip(1).collect();
        }
        chinese
    }

    /// 对齐 Java: `format(BigDecimal, boolean, boolean)`
    pub fn format_decimal(
        amount: &Decimal,
        is_use_traditional: bool,
        is_use_colloquial: bool,
    ) -> String {
        let plain = amount.normalize().to_string();
        let mut format_amount = if !plain.contains('.') {
            Self::format_long(
                amount.to_string().parse::<i64>().unwrap_or(0),
                is_use_traditional,
            )
        } else {
            let parts: Vec<&str> = plain.split('.').collect();
            let int_part = parts[0].parse::<i64>().unwrap_or(0);
            let mut decimal_part = String::new();
            for c in parts[1].chars() {
                decimal_part.push_str(&Self::number_char_to_chinese(c, is_use_traditional));
            }
            format!(
                "{}点{}",
                Self::format_long(int_part, is_use_traditional),
                decimal_part
            )
        };
        if is_use_colloquial {
            for (k, v) in [
                ("一十", "十"),
                ("一拾", "拾"),
                ("负一十", "负十"),
                ("负一拾", "负拾"),
            ] {
                if format_amount.starts_with(k) {
                    format_amount = format_amount.replacen(k, v, 1);
                    break;
                }
            }
        }
        format_amount
    }

    /// 对齐 Java: `numberCharToChinese`
    pub fn number_char_to_chinese(c: char, is_use_traditional: bool) -> String {
        if !c.is_ascii_digit() {
            return c.to_string();
        }
        Self::number_to_chinese(c.to_digit(10).unwrap() as i32, is_use_traditional).to_string()
    }

    /// 对齐 Java: `chineseMoneyToNumber`
    pub fn chinese_money_to_number(chinese_money_amount: &str) -> Option<Decimal> {
        if chinese_money_amount.trim().is_empty() {
            return None;
        }
        let chars: Vec<char> = chinese_money_amount.chars().collect();
        let find = |c: char| chars.iter().position(|&x| x == c);

        let mut yi = find('元');
        if yi.is_none() {
            yi = find('圆');
        }
        let ji = find('角');
        let fi = find('分');

        let slice = |start: usize, end: usize| -> String { chars[start..end].iter().collect() };

        let y_str = yi.filter(|&i| i > 0).map(|i| slice(0, i));
        let j_str = if let Some(ji) = ji {
            if ji > 0 {
                if let Some(yi) = yi {
                    if ji > yi {
                        Some(slice(yi + 1, ji))
                    } else {
                        None
                    }
                } else {
                    Some(slice(0, ji))
                }
            } else {
                None
            }
        } else {
            None
        };
        let f_str = if let Some(fi) = fi {
            if fi > 0 {
                if let Some(ji) = ji {
                    if fi > ji {
                        Some(slice(ji + 1, fi))
                    } else {
                        None
                    }
                } else if let Some(yi) = yi {
                    if yi > 0 && fi > yi {
                        Some(slice(yi + 1, fi))
                    } else {
                        None
                    }
                } else {
                    Some(slice(0, fi))
                }
            } else {
                None
            }
        } else {
            None
        };

        let y = y_str
            .filter(|s| !s.is_empty())
            .map(|s| Self::chinese_to_number(&s))
            .unwrap_or(0);
        let j = j_str
            .filter(|s| !s.is_empty())
            .map(|s| Self::chinese_to_number(&s))
            .unwrap_or(0);
        let f = f_str
            .filter(|s| !s.is_empty())
            .map(|s| Self::chinese_to_number(&s))
            .unwrap_or(0);

        let mut amount = Decimal::from(y);
        amount += Decimal::from(j) / Decimal::from(10);
        amount += Decimal::from(f) / Decimal::from(100);
        Some(amount.round_dp(2))
    }

    /// 对齐 Java: `chineseToNumber(String)`
    pub fn chinese_to_number(chinese: &str) -> i32 {
        let chars: Vec<char> = chinese.chars().collect();
        let length = chars.len();
        let mut result = 0i32;
        let mut section = 0i32;
        let mut number = 0i32;
        let mut unit: Option<&ChineseUnit> = None;
        for i in 0..length {
            let c = chars[i];
            let num = Self::chinese_char_to_number(c);
            if num >= 0 {
                if num == 0 {
                    if number > 0 {
                        if let Some(u) = unit {
                            section += number * (u.value / 10);
                        }
                    }
                    unit = None;
                } else if number > 0 {
                    panic!("Bad number at: {i}");
                }
                number = num;
            } else {
                let u = Self::chinese_to_unit(c)
                    .unwrap_or_else(|| panic!("Unknown unit '{c}' at: {i}"));
                unit = Some(u);
                if u.sec_unit {
                    section = (section + number) * u.value;
                    result += section;
                    section = 0;
                } else {
                    let mut unit_number = number;
                    if number == 0 && i == 0 {
                        unit_number = 1;
                    }
                    section += unit_number * u.value;
                }
                number = 0;
            }
        }
        let mut number = number;
        if number > 0 {
            if let Some(u) = unit {
                number *= u.value / 10;
            }
        }
        result + section + number
    }

    fn long_to_chinese(mut amount: i64, is_use_traditional: bool) -> String {
        if amount == 0 {
            return "零".to_string();
        }
        let mut parts = [0i32; 4];
        let mut i = 0;
        while amount != 0 {
            parts[i] = (amount % 10000) as i32;
            amount /= 10000;
            i += 1;
        }
        let mut chinese_str = String::new();
        // 千
        let part_value = parts[0];
        if part_value > 0 {
            let part_chinese = Self::thousand_to_chinese(part_value, is_use_traditional);
            chinese_str.insert_str(0, &part_chinese);
            if part_value < 1000 {
                Self::add_pre_zero(&mut chinese_str);
            }
        }
        // 万
        let part_value = parts[1];
        if part_value > 0 {
            if part_value % 10 == 0 && parts[0] > 0 {
                Self::add_pre_zero(&mut chinese_str);
            }
            let part_chinese = Self::thousand_to_chinese(part_value, is_use_traditional);
            chinese_str.insert_str(0, &format!("{part_chinese}万"));
            if part_value < 1000 {
                Self::add_pre_zero(&mut chinese_str);
            }
        } else {
            Self::add_pre_zero(&mut chinese_str);
        }
        // 亿
        let part_value = parts[2];
        if part_value > 0 {
            if part_value % 10 == 0 && parts[1] > 0 {
                Self::add_pre_zero(&mut chinese_str);
            }
            let part_chinese = Self::thousand_to_chinese(part_value, is_use_traditional);
            chinese_str.insert_str(0, &format!("{part_chinese}亿"));
            if part_value < 1000 {
                Self::add_pre_zero(&mut chinese_str);
            }
        } else {
            Self::add_pre_zero(&mut chinese_str);
        }
        // 万亿
        let part_value = parts[3];
        if part_value > 0 {
            if parts[2] == 0 {
                chinese_str.insert(0, '亿');
            }
            let part_chinese = Self::thousand_to_chinese(part_value, is_use_traditional);
            chinese_str.insert_str(0, &format!("{part_chinese}万"));
        }
        if !chinese_str.is_empty() && chinese_str.starts_with('零') {
            chinese_str = chinese_str.chars().skip(1).collect();
        }
        chinese_str
    }

    fn thousand_to_chinese(amount_part: i32, is_use_traditional: bool) -> String {
        if amount_part == 0 {
            return DIGITS[0].to_string();
        }
        let mut temp = amount_part;
        let mut chinese_str = String::new();
        let mut last_is_zero = true;
        let mut i = 0;
        while temp > 0 {
            let digit = temp % 10;
            if digit == 0 {
                if !last_is_zero {
                    chinese_str.insert(0, '零');
                }
                last_is_zero = true;
            } else {
                let s = format!(
                    "{}{}",
                    Self::number_to_chinese(digit, is_use_traditional),
                    Self::get_unit_name(i, is_use_traditional)
                );
                chinese_str.insert_str(0, &s);
                last_is_zero = false;
            }
            temp /= 10;
            i += 1;
        }
        chinese_str
    }

    fn chinese_to_unit(chinese: char) -> Option<&'static ChineseUnit> {
        CHINESE_NAME_VALUE.iter().find(|u| u.name == chinese)
    }

    fn chinese_char_to_number(mut chinese: char) -> i32 {
        if chinese == '两' {
            chinese = '二';
        }
        let i = DIGITS
            .iter()
            .position(|&d| d == chinese)
            .map(|x| x as i32)
            .unwrap_or(-1);
        if i > 0 { (i + 1) / 2 } else { i }
    }

    fn number_to_chinese(number: i32, is_use_traditional: bool) -> char {
        if number == 0 {
            return DIGITS[0];
        }
        DIGITS[(number * 2 - if is_use_traditional { 0 } else { 1 }) as usize]
    }

    fn get_unit_name(index: i32, is_use_traditional: bool) -> String {
        if index == 0 {
            return String::new();
        }
        let u = &CHINESE_NAME_VALUE[(index * 2 - if is_use_traditional { 0 } else { 1 }) as usize];
        u.name.to_string()
    }

    fn add_pre_zero(chinese_str: &mut String) {
        if chinese_str.is_empty() {
            return;
        }
        if !chinese_str.starts_with('零') {
            chinese_str.insert(0, '零');
        }
    }
}

fn trim_decimal(v: f64, scale: i32) -> String {
    let factor = 10f64.powi(scale);
    let rounded = (v * factor).round() / factor;
    let s = format!("{:.2}", rounded);
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

#[allow(dead_code)]
fn _unused_from_str() {
    let _ = Decimal::from_str("0");
}
