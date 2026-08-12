use chrono::NaiveDate;

mod card10_info;
mod idcard;
mod idcard_error;
mod idcard_util;

pub use card10_info::Card10Info;
pub use idcard::Idcard;
pub use idcard_error::IdcardError;
pub use idcard_util::IdcardUtil;

const CHINA_ID_MIN_LENGTH: usize = 15;

const CHINA_ID_MAX_LENGTH: usize = 18;

const POWER: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];

const CHECK_CODES: [char; 11] = ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

const CITY_CODES: [(&str, &str); 35] = [
    ("11", "北京"),
    ("12", "天津"),
    ("13", "河北"),
    ("14", "山西"),
    ("15", "内蒙古"),
    ("21", "辽宁"),
    ("22", "吉林"),
    ("23", "黑龙江"),
    ("31", "上海"),
    ("32", "江苏"),
    ("33", "浙江"),
    ("34", "安徽"),
    ("35", "福建"),
    ("36", "江西"),
    ("37", "山东"),
    ("41", "河南"),
    ("42", "湖北"),
    ("43", "湖南"),
    ("44", "广东"),
    ("45", "广西"),
    ("46", "海南"),
    ("50", "重庆"),
    ("51", "四川"),
    ("52", "贵州"),
    ("53", "云南"),
    ("54", "西藏"),
    ("61", "陕西"),
    ("62", "甘肃"),
    ("63", "青海"),
    ("64", "宁夏"),
    ("65", "新疆"),
    ("71", "台湾"),
    ("81", "香港"),
    ("82", "澳门"),
    ("83", "台湾"),
];

const TW_FIRST_CODES: [(char, u32); 26] = [
    ('A', 10),
    ('B', 11),
    ('C', 12),
    ('D', 13),
    ('E', 14),
    ('F', 15),
    ('G', 16),
    ('H', 17),
    ('J', 18),
    ('K', 19),
    ('L', 20),
    ('M', 21),
    ('N', 22),
    ('P', 23),
    ('Q', 24),
    ('R', 25),
    ('S', 26),
    ('T', 27),
    ('U', 28),
    ('V', 29),
    ('X', 30),
    ('Y', 31),
    ('W', 32),
    ('Z', 33),
    ('I', 34),
    ('O', 35),
];

fn parse_two_digits(value: &[char]) -> u32 {
    let tens = value[0].to_digit(10).unwrap_or(0);
    let ones = value[1].to_digit(10).unwrap_or(0);
    tens * 10 + ones
}

fn check_code_18(code17: &[u8]) -> Option<char> {
    if code17.len() != POWER.len() || !code17.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(weighted_check_code(code17))
}

fn weighted_check_code(code17: &[u8]) -> char {
    let sum = code17
        .iter()
        .zip(POWER)
        .map(|(value, power)| u32::from(value - b'0') * power)
        .sum::<u32>();
    CHECK_CODES[(sum % 11) as usize]
}

fn province_name(code: &str) -> Option<&'static str> {
    CITY_CODES
        .iter()
        .find_map(|(candidate, name)| (*candidate == code).then_some(*name))
}

fn tw_first_code(value: char) -> Option<u32> {
    TW_FIRST_CODES
        .iter()
        .find_map(|(candidate, code)| (*candidate == value).then_some(*code))
}

fn parse_birth(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "%Y%m%d").ok()
}

fn parse_birth_component(
    idcard: &str,
    range: std::ops::Range<usize>,
) -> Result<Option<i16>, IdcardError> {
    let Some(birth) = IdcardUtil::get_birth(idcard)? else {
        return Ok(None);
    };
    birth[range]
        .parse()
        .map(Some)
        .map_err(|_| IdcardError::InvalidCard)
}

fn prefix_for_supported_length(idcard: &str, length: usize) -> Option<String> {
    let chars: Vec<char> = idcard.chars().collect();
    matches!(chars.len(), CHINA_ID_MIN_LENGTH | CHINA_ID_MAX_LENGTH)
        .then(|| chars[..length].iter().collect())
}

fn compact_parenthesized_card(value: &str) -> Option<String> {
    if value.contains(['(', ')']) {
        let chars: Vec<char> = value.chars().collect();
        if chars.len() < 3
            || chars[chars.len() - 3] != '('
            || chars[chars.len() - 1] != ')'
            || chars[..chars.len() - 3]
                .iter()
                .any(|value| matches!(value, '(' | ')'))
        {
            return None;
        }
        Some(
            chars[..chars.len() - 3]
                .iter()
                .chain(std::iter::once(&chars[chars.len() - 2]))
                .collect(),
        )
    } else {
        Some(value.to_owned())
    }
}

fn is_blank(value: &str) -> bool {
    value.chars().all(crate::CharUtil::is_blank_char)
}
