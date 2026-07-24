//! 对齐: `cn.hutool.core.comparator.IndexedComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/IndexedComparator.java

use std::collections::HashMap;
use std::hash::Hash;

use super::reversed_array_indexed_comparator::ReversedArrayIndexedComparator;

/// 对齐 Java 测试类 `ArrayIndexedComparator`（按数组索引，允许多次查找同一引用语义用 Eq）。
#[derive(Debug, Clone)]
pub struct ArrayIndexedComparator<T> {
    at_end_if_miss: bool,
    array: Vec<T>,
}

impl<T: PartialEq> ArrayIndexedComparator<T> {
    /// 对齐 Java: `ArrayIndexedComparator(T... objs)`
    pub fn new(objs: impl IntoIterator<Item = T>) -> Self {
        Self {
            at_end_if_miss: false,
            array: objs.into_iter().collect(),
        }
    }

    /// 对齐 Java: `reversed()` —— 返回反序比较器包装。
    pub fn reversed(self) -> ReversedArrayIndexedComparator<T> {
        ReversedArrayIndexedComparator { inner: self }
    }

    /// 对齐 Java: `compare(T, T)`
    pub fn compare(&self, o1: &T, o2: &T) -> i32 {
        let index1 = self.get_order(o1);
        let index2 = self.get_order(o2);
        if index1 == index2 {
            if index1 < 0 || index1 == self.array.len() as i32 {
                return 1;
            }
            return 0;
        }
        index1.cmp(&index2) as i32
    }

    fn get_order(&self, object: &T) -> i32 {
        match self.array.iter().position(|x| x == object) {
            Some(order) => order as i32,
            None => {
                if self.at_end_if_miss {
                    self.array.len() as i32
                } else {
                    -1
                }
            }
        }
    }
}
