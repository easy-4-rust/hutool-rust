//! Stateful and view-based collection adapters aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.TransSpliterator`
/// 转换分割器

/// Rust's lazy `Map` iterator is the counterpart of Hutool's spliterator view.
pub type TransSpliterator<I, F> = std::iter::Map<I, F>;
