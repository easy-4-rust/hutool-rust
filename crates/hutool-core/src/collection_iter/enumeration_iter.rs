//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// Rust's `Iterator` is already the equivalent of an `Enumeration` iterator.
pub type EnumerationIter<I> = I;
