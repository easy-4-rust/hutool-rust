//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

/// XML node lists map to the same resettable borrowed-slice iterator in Rust.
pub type NodeListIter<'a, T> = ArrayIter<'a, T>;
