//! 对齐: `cn.hutool.core.comparator.FuncComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/FuncComparator.java
//!
//! 用提取函数拿到可比较键再比较；替代 Java Bean 反射。

use std::cmp::Ordering;
use std::marker::PhantomData;

mod func_comparator;
mod func_option_comparator;

pub use func_comparator::FuncComparator;
pub use func_option_comparator::FuncOptionComparator;
