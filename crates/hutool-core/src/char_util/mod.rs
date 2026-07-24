use std::any::{Any, TypeId};

use unicode_general_category::{GeneralCategory, get_general_category};

mod char_error;
mod char_util;

pub use char_error::CharError;
pub use char_util::CharUtil;

fn lower_char(ch: char) -> char {
    ch.to_lowercase().next().unwrap_or(ch)
}

fn java_category(category: GeneralCategory) -> i32 {
    java_category_abbreviation(category.abbreviation())
}

fn java_category_abbreviation(category: &str) -> i32 {
    match category {
        "Cn" => 0,
        "Lu" => 1,
        "Ll" => 2,
        "Lt" => 3,
        "Lm" => 4,
        "Lo" => 5,
        "Mn" => 6,
        "Me" => 7,
        "Mc" => 8,
        "Nd" => 9,
        "Nl" => 10,
        "No" => 11,
        "Zs" => 12,
        "Zl" => 13,
        "Zp" => 14,
        "Cc" => 15,
        "Cf" => 16,
        "Co" => 18,
        "Cs" => 19,
        "Pd" => 20,
        "Ps" => 21,
        "Pe" => 22,
        "Pc" => 23,
        "Po" => 24,
        "Sm" => 25,
        "Sc" => 26,
        "Sk" => 27,
        "So" => 28,
        "Pi" => 29,
        "Pf" => 30,
        _ => 0,
    }
}
