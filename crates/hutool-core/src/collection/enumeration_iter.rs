//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

/// 对齐: `cn.hutool.core.collection.EnumerationIter`
/// 枚举迭代器

/// Rust's `Iterator` is already the equivalent of an `Enumeration` iterator.
pub type EnumerationIter<I> = I;
