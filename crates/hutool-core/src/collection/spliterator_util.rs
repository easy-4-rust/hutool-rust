//! Stateful and view-based collection adapters aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.SpliteratorUtil`
/// 分割迭代器

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

use super::trans_spliterator::TransSpliterator;

/// Factory for transformed spliterator/iterator views.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpliteratorUtil;

impl SpliteratorUtil {
    /// Lazily transforms an iterator while preserving its size hint and splitting traits.
    pub fn trans<I, F, U>(source: I, transform: F) -> TransSpliterator<I, F>
    where
        I: Iterator,
        F: FnMut(I::Item) -> U,
    {
        source.map(transform)
    }
}
