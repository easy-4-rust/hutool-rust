//! 对齐: `cn.hutool.core.comparator.VersionComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/VersionComparator.java

use std::cmp::Ordering;

/// 版本比较器 —— 对齐 Java `VersionComparator`。
///
/// 不实现 `PartialEq`：对齐 Java 默认引用相等（`equalsTest`）。
#[derive(Debug, Clone, Default)]
pub struct VersionComparator;

impl VersionComparator {
    /// 对齐 Java: `VersionComparator.INSTANCE`
    pub const INSTANCE: VersionComparator = VersionComparator;

    /// 对齐 Java: `compare(String, String)`
    pub fn compare(&self, version1: Option<&str>, version2: Option<&str>) -> i32 {
        match (version1, version2) {
            (None, None) => 0,
            (None, Some(_)) => -1,
            (Some(_), None) => 1,
            (Some(a), Some(b)) if a == b => 0,
            (Some(a), Some(b)) => match compare_versions(a, b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(i32),
    Text(String),
}

#[derive(Debug, Default)]
struct LooseVersion {
    sequence: Vec<Token>,
    pre: Vec<Token>,
    build: Vec<Token>,
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let left = parse_version(left);
    let right = parse_version(right);
    compare_tokens(&left.sequence, &right.sequence)
        .then_with(|| match (left.pre.is_empty(), right.pre.is_empty()) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal,
        })
        .then_with(|| compare_tokens(&left.pre, &right.pre))
        .then_with(|| compare_tokens(&left.build, &right.build))
}

fn parse_version(value: &str) -> LooseVersion {
    let chars: Vec<char> = value.chars().collect();
    if chars.is_empty() {
        return LooseVersion::default();
    }
    let mut version = LooseVersion::default();
    let mut index = take_number(&chars, 0, &mut version.sequence);
    let mut separator = chars[0];
    while index < chars.len() {
        separator = chars[index];
        if separator == '.' {
            index += 1;
        } else if separator == '-' || separator == '+' {
            index += 1;
            break;
        } else if separator.is_ascii_digit() {
            index = take_number(&chars, index, &mut version.sequence);
        } else {
            index = take_text(&chars, index, &mut version.sequence);
        }
    }
    if separator == '-' && index >= chars.len() {
        return version;
    }
    while index < chars.len() {
        if chars[index].is_ascii_digit() {
            index = take_number(&chars, index, &mut version.pre);
        } else {
            index = take_text(&chars, index, &mut version.pre);
        }
        if index >= chars.len() {
            break;
        }
        separator = chars[index];
        if separator == '.' || separator == '-' {
            index += 1;
        } else if separator == '+' {
            index += 1;
            break;
        }
    }
    if separator == '+' && index >= chars.len() {
        return version;
    }
    while index < chars.len() {
        if chars[index].is_ascii_digit() {
            index = take_number(&chars, index, &mut version.build);
        } else {
            index = take_text(&chars, index, &mut version.build);
        }
        if index < chars.len() {
            index += 1;
        }
    }
    version
}

fn take_number(chars: &[char], mut index: usize, output: &mut Vec<Token>) -> usize {
    let mut number = (chars[index] as i32) - ('0' as i32);
    index += 1;
    while index < chars.len() && chars[index].is_ascii_digit() {
        number = number
            .wrapping_mul(10)
            .wrapping_add((chars[index] as i32) - ('0' as i32));
        index += 1;
    }
    output.push(Token::Number(number));
    index
}

fn take_text(chars: &[char], mut index: usize, output: &mut Vec<Token>) -> usize {
    let start = index;
    index += 1;
    while index < chars.len()
        && !matches!(chars[index], '.' | '-' | '+')
        && !chars[index].is_ascii_digit()
    {
        index += 1;
    }
    output.push(Token::Text(chars[start..index].iter().collect()));
    index
}

fn compare_tokens(left: &[Token], right: &[Token]) -> Ordering {
    let len = left.len().max(right.len());
    for i in 0..len {
        let l = left.get(i);
        let r = right.get(i);
        let ordering = match (l, r) {
            (None, None) => Ordering::Equal,
            (None, Some(Token::Number(0))) | (Some(Token::Number(0)), None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(Token::Number(a)), Some(Token::Number(b))) => a.cmp(b),
            (Some(Token::Text(a)), Some(Token::Text(b))) => java_string_cmp(a, b),
            (Some(Token::Number(_)), Some(Token::Text(_))) => Ordering::Less,
            (Some(Token::Text(_)), Some(Token::Number(_))) => Ordering::Greater,
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    Ordering::Equal
}

fn java_string_cmp(left: &str, right: &str) -> Ordering {
    left.cmp(right)
}
