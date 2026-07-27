//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

/// 对齐: `cn.hutool.core.collection.IteratorEnumeration`
/// 迭代器枚举

use std::collections::VecDeque;

/// Converting a Rust iterator to an enumeration is a zero-cost identity.
pub type IteratorEnumeration<I> = I;
