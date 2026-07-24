//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

/// Rust's lazy `Map` iterator is the counterpart of Hutool's spliterator view.
pub type TransSpliterator<I, F> = std::iter::Map<I, F>;
