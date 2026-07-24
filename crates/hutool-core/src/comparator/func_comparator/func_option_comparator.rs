//! 对齐: `cn.hutool.core.comparator.FuncComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/FuncComparator.java
//!
//! 用提取函数拿到可比较键再比较；替代 Java Bean 反射。

use std::cmp::Ordering;
use std::marker::PhantomData;

/// 带 Option 键的函数比较器。
pub struct FuncOptionComparator<T, V, F>
where
    F: Fn(&T) -> Option<V>,
    V: Ord,
{
    null_greater: bool,
    func: F,
    _marker: PhantomData<(T, V)>,
}

impl<T, V, F> FuncOptionComparator<T, V, F>
where
    F: Fn(&T) -> Option<V>,
    V: Ord,
{
    /// 构造。
    #[must_use]
    pub fn new(null_greater: bool, func: F) -> Self {
        Self {
            null_greater,
            func,
            _marker: PhantomData,
        }
    }

    /// 比较。
    #[must_use]
    pub fn compare(&self, a: &T, b: &T) -> i32 {
        let ka = (self.func)(a);
        let kb = (self.func)(b);
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
