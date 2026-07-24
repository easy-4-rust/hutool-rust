//! 对齐: `cn.hutool.core.comparator.FieldComparator` / `BaseFieldComparator`
//! 来源: hutool-core FieldComparator.java / BaseFieldComparator.java
//!
//! Java 反射 Field → Rust 提取闭包。

use std::cmp::Ordering;
use std::marker::PhantomData;

use super::base_field_comparator::BaseFieldComparator;

/// 对齐 Java 类: `cn.hutool.core.comparator.FieldComparator`
pub struct FieldComparator<T, K, F>
where
    F: Fn(&T) -> Option<K>,
    K: Ord,
{
    inner: BaseFieldComparator<T, K, F>,
}

impl<T, K, F> FieldComparator<T, K, F>
where
    F: Fn(&T) -> Option<K>,
    K: Ord,
{
    /// 对齐 Java: `FieldComparator(Class, String)` —— Rust 用提取器。
    #[must_use]
    pub fn new(extractor: F) -> Self {
        Self {
            inner: BaseFieldComparator::new(true, true, extractor),
        }
    }

    /// 对齐 Java: `FieldComparator(boolean nullGreater, boolean compareSelf, Field)`
    #[must_use]
    pub fn with_flags(null_greater: bool, compare_self: bool, extractor: F) -> Self {
        Self {
            inner: BaseFieldComparator::new(null_greater, compare_self, extractor),
        }
    }

    /// 对齐 Java: `compare`
    #[must_use]
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        self.inner.compare(a, b)
    }
}
