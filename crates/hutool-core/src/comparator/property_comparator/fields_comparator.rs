//! 对齐: `cn.hutool.core.comparator.PropertyComparator` / `FieldsComparator`
//! 来源: hutool-core PropertyComparator / FieldsComparator / FuncComparator

use std::cmp::Ordering;

/// 多字段比较器 —— 对齐 Java `FieldsComparator`。
pub struct FieldsComparator<T> {
    extractors: Vec<Box<dyn Fn(&T) -> Option<i32> + Send + Sync>>,
}

impl<T> FieldsComparator<T> {
    /// 用多个 i32 字段提取器构造（对齐 Java 字段名列表的测试向量）。
    pub fn new(extractors: Vec<Box<dyn Fn(&T) -> Option<i32> + Send + Sync>>) -> Self {
        Self { extractors }
    }

    /// 对齐 Java: `compare(T, T)`
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        for ex in &self.extractors {
            let v1 = ex(a);
            let v2 = ex(b);
            let c = compare_nullable_i32(v1, v2, true);
            if c != 0 {
                return c;
            }
        }
        0
    }
}

use super::{compare_nullable_i32, compare_nullable_str};
