//! Borrowed and streaming collection partitions aligned with Hutool.

use std::iter::Peekable;

use crate::{CoreError, Result};

mod partition;
mod random_access_partition;
mod avg_partition;
mod random_access_avg_partition;
mod partition_iter;

pub use partition::Partition;
pub use random_access_partition::RandomAccessPartition;
pub use avg_partition::AvgPartition;
pub use random_access_avg_partition::RandomAccessAvgPartition;
pub use partition_iter::PartitionIter;

fn validate_partition_size(partition_size: usize) -> Result<()> {
    if partition_size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "partition_size",
            reason: "must be greater than zero",
        });
    }
    Ok(())
}
