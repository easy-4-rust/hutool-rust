//! Borrowed and streaming collection partitions aligned with Hutool.

use std::iter::Peekable;

use crate::{CoreError, Result};

use super::partition::Partition;

/// `Partition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessPartition<'a, T> = Partition<'a, T>;

use super::{validate_partition_size};
