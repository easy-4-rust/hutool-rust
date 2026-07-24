//! Borrowed and streaming collection partitions aligned with Hutool.

use std::iter::Peekable;

use crate::{CoreError, Result};

use super::avg_partition::AvgPartition;

/// `AvgPartition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessAvgPartition<'a, T> = AvgPartition<'a, T>;

use super::{validate_partition_size};
