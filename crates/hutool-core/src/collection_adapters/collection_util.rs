//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

/// Legacy alias retained for Hutool's `CollectionUtil extends CollUtil` surface.
pub type CollectionUtil = CollUtil;
