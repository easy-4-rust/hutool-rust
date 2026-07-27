//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

/// 对齐: `cn.hutool.core.collection.IterableIter`
/// 可迭代迭代器

use std::collections::VecDeque;

/// Marker for iterators that are directly usable in `for` loops.
///
/// Rust iterators already implement `IntoIterator`, so this is the zero-cost
/// counterpart of Hutool's `IterableIter` interface.
pub trait IterableIter: Iterator {}

impl<I: Iterator> IterableIter for I {}
