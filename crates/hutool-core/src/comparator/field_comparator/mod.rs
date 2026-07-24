//! 对齐: `cn.hutool.core.comparator.FieldComparator` / `BaseFieldComparator`
//! 来源: hutool-core FieldComparator.java / BaseFieldComparator.java
//!
//! Java 反射 Field → Rust 提取闭包。

use std::cmp::Ordering;
use std::marker::PhantomData;

mod base_field_comparator;
mod field_comparator;

pub use base_field_comparator::BaseFieldComparator;
pub use field_comparator::FieldComparator;
