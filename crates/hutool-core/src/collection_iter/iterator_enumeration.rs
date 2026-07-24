//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// Converting a Rust iterator to an enumeration is a zero-cost identity.
pub type IteratorEnumeration<I> = I;
