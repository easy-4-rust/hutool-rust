//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

mod compute_iter;
mod line_iter;
mod node_list_iter;
mod trans_collection;
mod trans_spliterator;
mod spliterator_util;
mod collection_util;

pub use compute_iter::ComputeIter;
pub use line_iter::LineIter;
pub use node_list_iter::NodeListIter;
pub use trans_collection::TransCollection;
pub use trans_spliterator::TransSpliterator;
pub use spliterator_util::SpliteratorUtil;
pub use collection_util::CollectionUtil;
