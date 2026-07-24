//! 对齐: `cn.hutool.core.comparator.PropertyComparator` / `FieldsComparator`
//! 来源: hutool-core PropertyComparator / FieldsComparator / FuncComparator

use std::cmp::Ordering;

use super::property_comparator::PropertyComparator;

/// 反序属性比较器。
pub struct ReversedPropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    pub(crate) inner: PropertyComparator<T, F>,
}

impl<T, F> ReversedPropertyComparator<T, F>
where
    F: Fn(&T) -> Option<String>,
{
    /// 反序比较。
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        -self.inner.compare(a, b)
    }
}

use super::{compare_nullable_i32, compare_nullable_str};
