use std::cmp::Ordering;

mod version_error;
mod version_util;

pub use version_error::VersionError;
pub use version_util::VersionUtil;

const DEFAULT_DELIMITER: &str = ";";

fn validate_delimiter(delimiter: &str) -> Result<(), VersionError> {
    if delimiter.trim().is_empty()
        || delimiter == "-"
        || matches!(delimiter.chars().next(), Some('>' | '<' | '≥' | '≤'))
    {
        return Err(VersionError::InvalidDelimiter(delimiter.to_owned()));
    }
    Ok(())
}

fn split_operator(expression: &str) -> Option<(&str, &str)> {
    for operator in [">=", "<=", "≥=", "≤=", ">", "<", "≥", "≤"] {
        if let Some(version) = expression.strip_prefix(operator) {
            return Some((operator, version));
        }
    }
    None
}

fn compare_nullable(current: Option<&str>, compare: Option<&str>) -> Ordering {
    let current = current.map(str::trim);
    match (current, compare) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a), Some(b)) => compare_versions(a, b),
    }
}

fn comparison_target(version: &str) -> Option<&str> {
    (!version.eq_ignore_ascii_case("null")).then_some(version)
}

#[derive(Debug)]
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
        // Both token readers stop only at a version separator or at end-of-input.
        if index < chars.len() {
            index += 1;
        }
    }
    version
}

fn take_number(chars: &[char], mut index: usize, output: &mut Vec<Token>) -> usize {
    let mut number = (chars[index] as i32).wrapping_sub('0' as i32);
    index += 1;
    while index < chars.len() && chars[index].is_ascii_digit() {
        number = number
            .wrapping_mul(10)
            .wrapping_add((chars[index] as i32).wrapping_sub('0' as i32));
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
    for (left, right) in left.iter().zip(right) {
        let ordering = match (left, right) {
            (Token::Number(left), Token::Number(right)) => left.cmp(right),
            (Token::Text(left), Token::Text(right)) => java_string_cmp(left, right),
            (Token::Number(left), Token::Text(right)) => java_string_cmp(&left.to_string(), right),
            (Token::Text(left), Token::Number(right)) => java_string_cmp(left, &right.to_string()),
        };
        if ordering != Ordering::Equal {
            return ordering;
        }
    }
    let rest = if left.len() > right.len() {
        left
    } else {
        right
    };
    if rest
        .iter()
        .skip(left.len().min(right.len()))
        .any(|token| !matches!(token, Token::Number(0)))
    {
        left.len().cmp(&right.len())
    } else {
        Ordering::Equal
    }
}

fn java_string_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}
