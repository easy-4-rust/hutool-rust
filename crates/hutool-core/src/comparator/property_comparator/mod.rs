//! 对齐: `cn.hutool.core.comparator.PropertyComparator` / `FieldsComparator`
//! 来源: hutool-core PropertyComparator / FieldsComparator / FuncComparator

use std::cmp::Ordering;

mod property_comparator;
mod reversed_property_comparator;
mod fields_comparator;

pub use property_comparator::PropertyComparator;
pub use reversed_property_comparator::ReversedPropertyComparator;
pub use fields_comparator::FieldsComparator;

fn compare_nullable_str(a: Option<&str>, b: Option<&str>, null_greater: bool) -> i32 {
    match (a, b) {
        (None, None) => 0,
        (None, Some(_)) => {
            if null_greater {
                1
            } else {
                -1
            }
        }
        (Some(_), None) => {
            if null_greater {
                -1
            } else {
                1
            }
        }
        (Some(x), Some(y)) => match x.cmp(y) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        },
    }
}

fn compare_nullable_i32(a: Option<i32>, b: Option<i32>, null_greater: bool) -> i32 {
    match (a, b) {
        (None, None) => 0,
        (None, Some(_)) => {
            if null_greater {
                1
            } else {
                -1
            }
        }
        (Some(_), None) => {
            if null_greater {
                -1
            } else {
                1
            }
        }
        (Some(x), Some(y)) => x.cmp(&y) as i32,
    }
}
