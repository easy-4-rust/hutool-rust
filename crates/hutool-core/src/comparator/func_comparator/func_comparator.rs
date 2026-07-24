//! 对齐: `cn.hutool.core.comparator.FuncComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/FuncComparator.java
//!
//! 用提取函数拿到可比较键再比较；替代 Java Bean 反射。

use std::cmp::Ordering;
use std::marker::PhantomData;

/// 对齐 Java 类: `cn.hutool.core.comparator.FuncComparator`
pub struct FuncComparator<T, K, F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    null_greater: bool,
    compare_self: bool,
    func: F,
    _marker: PhantomData<(T, K)>,
}

impl<T, K, F> FuncComparator<T, K, F>
where
    F: Fn(&T) -> K,
    K: Ord,
{
    /// 对齐 Java: `FuncComparator(boolean nullGreater, Function)`
    #[must_use]
    pub fn new(null_greater: bool, func: F) -> Self {
        Self {
            null_greater,
            compare_self: true,
            func,
            _marker: PhantomData,
        }
    }

    /// 对齐 Java: `FuncComparator(boolean nullGreater, boolean compareSelf, Function)`
    #[must_use]
    pub fn with_compare_self(null_greater: bool, compare_self: bool, func: F) -> Self {
        Self {
            null_greater,
            compare_self,
            func,
            _marker: PhantomData,
        }
    }

    /// 对齐 Java: `compare(T, T)` —— 非空引用。
    #[must_use]
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        let _ = (self.null_greater, self.compare_self);
        let ka = (self.func)(a);
        let kb = (self.func)(b);
        match ka.cmp(&kb) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}
