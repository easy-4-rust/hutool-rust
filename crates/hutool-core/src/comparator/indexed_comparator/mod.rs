//! 对齐: `cn.hutool.core.comparator.IndexedComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/IndexedComparator.java

use std::collections::HashMap;
use std::hash::Hash;

mod indexed_comparator;
mod array_indexed_comparator;
mod reversed_array_indexed_comparator;

pub use indexed_comparator::IndexedComparator;
pub use array_indexed_comparator::ArrayIndexedComparator;
pub use reversed_array_indexed_comparator::ReversedArrayIndexedComparator;
