//! 对齐: `cn.hutool.core.io.unit.DataSizeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/unit/DataSizeUtil.java

use super::data_unit::DataUnit;

/// 对齐 Java 类: `cn.hutool.core.io.unit.DataSizeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct DataSizeUtil;

impl DataSizeUtil {
    /// 对齐 Java: `DataSizeUtil.parse(String)`
    pub fn parse(text: &str) -> Result<i64, String> {
        parse_data_size(text)
    }

    /// 对齐 Java: `DataSizeUtil.format(long)`
    pub fn format(size: i64) -> String {
        if size <= 0 {
            return "0".to_string();
        }
        let digit_groups = ((size as f64).log10() / 1024_f64.log10()).floor() as usize;
        let digit_groups = digit_groups.min(DataUnit::UNIT_NAMES.len() - 1);
        let value = size as f64 / 1024_f64.powi(digit_groups as i32);
        format!(
            "{} {}",
            format_decimal(value),
            DataUnit::UNIT_NAMES[digit_groups]
        )
    }

    /// 对齐 Java: `DataSizeUtil.format(Long, DataUnit)`
    pub fn format_with_unit(size: i64, unit: DataUnit) -> String {
        if size <= 0 {
            return "0".to_string();
        }
        let digit_groups = unit.digit_group();
        let value = size as f64 / 1024_f64.powi(digit_groups as i32);
        format!(
            "{} {}",
            format_decimal(value),
            DataUnit::UNIT_NAMES[digit_groups]
        )
    }
}

/// 对齐 Java `DecimalFormat("##0.##")` 的可读数值格式。
fn format_decimal(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    if (rounded - rounded.round()).abs() < 1e-9 {
        return format!("{}", rounded as i64);
    }
    let mut s = format!("{rounded:.2}");
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

/// 对齐 Java `DataSize.parse` 的简化解析（parity 测试所需子集）。
fn parse_data_size(s: &str) -> Result<i64, String> {
    let s_trim = s.trim();
    if s_trim.is_empty() {
        return Err(format!("'{s}' is not a valid data size"));
    }
    let mut sign = 1i64;
    let mut rest = s_trim;
    if let Some(r) = rest.strip_prefix('+') {
        rest = r;
    } else if let Some(r) = rest.strip_prefix('-') {
        sign = -1;
        rest = r;
    }
    let bytes = rest.as_bytes();
    let mut num_end = 0;
    let mut dots = 0;
    while num_end < bytes.len() {
        let c = bytes[num_end];
        if c.is_ascii_digit() {
            num_end += 1;
        } else if c == b'.' {
            dots += 1;
            if dots > 1 {
                return Err(format!("'{s}' is not a valid data size"));
            }
            num_end += 1;
        } else {
            break;
        }
    }
    if num_end == 0 {
        return Err(format!("'{s}' is not a valid data size"));
    }
    let num: f64 = rest[..num_end]
        .parse()
        .map_err(|_| format!("'{s}' is not a valid data size"))?;
    let unit = rest[num_end..].trim().to_ascii_lowercase();
    let mult: f64 = match unit.as_str() {
        "" | "b" => 1.0,
        "k" | "kb" | "kib" => 1024.0,
        "m" | "mb" | "mib" => 1024.0 * 1024.0,
        "g" | "gb" | "gib" => 1024.0_f64.powi(3),
        "t" | "tb" | "tib" => 1024.0_f64.powi(4),
        "p" | "pb" | "pib" => 1024.0_f64.powi(5),
        "e" | "eb" | "eib" => 1024.0_f64.powi(6),
        _ => return Err(format!("'{s}' is not a valid data size")),
    };
    Ok(sign * (num * mult) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_with_unit_matches_java_long_max_tb() {
        assert_eq!(
            DataSizeUtil::format_with_unit(i64::MAX, DataUnit::Terabytes),
            "8388608 TB"
        );
    }
}
