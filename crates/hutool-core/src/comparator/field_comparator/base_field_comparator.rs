//! 对齐: `cn.hutool.core.comparator.FieldComparator` / `BaseFieldComparator`
//! 来源: hutool-core FieldComparator.java / BaseFieldComparator.java
//!
//! Java 反射 Field → Rust 提取闭包。

use std::cmp::Ordering;
use std::marker::PhantomData;

/// 对齐 Java 类: `cn.hutool.core.comparator.BaseFieldComparator`
///
/// 提供可比较键提取后的 null/自比较策略骨架。
pub struct BaseFieldComparator<T, K, F>
where
    F: Fn(&T) -> Option<K>,
    K: Ord,
{
    null_greater: bool,
    compare_self: bool,
    extractor: F,
    _marker: PhantomData<(T, K)>,
}

impl<T, K, F> BaseFieldComparator<T, K, F>
where
    F: Fn(&T) -> Option<K>,
    K: Ord,
{
    /// 构造基类比较器。
    #[must_use]
    pub fn new(null_greater: bool, compare_self: bool, extractor: F) -> Self {
        Self {
            null_greater,
            compare_self,
            extractor,
            _marker: PhantomData,
        }
    }

    /// 比较两个对象的字段键。
    #[must_use]
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        let _ = self.compare_self;
        let ka = (self.extractor)(a);
        let kb = (self.extractor)(b);
        match (ka, kb) {
            (None, None) => 0,
            (None, Some(_)) => {
                if self.null_greater {
                    1
                } else {
                    -1
                }
            }
            (Some(_), None) => {
                if self.null_greater {
                    -1
                } else {
                    1
                }
            }
            (Some(x), Some(y)) => match x.cmp(&y) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            },
        }
    }
}
