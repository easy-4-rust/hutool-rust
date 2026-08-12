//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

/// 对齐: `cn.hutool.core.collection.ResettableIter`
/// 可重置迭代器

/// An iterator that can return to its configured starting position.
pub trait ResettableIter: Iterator {
    /// Resets iteration state.
    fn reset(&mut self);
}
