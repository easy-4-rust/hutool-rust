//! Production bloom filters and Hutool-compatible bitmap/hash strategies.
//!
//! 对齐: `cn.hutool.bloomFilter` (crate root)
//! 来源: hutool-bloomFilter/src/main/java/cn/hutool/bloomFilter/
//! 中文说明: 布隆过滤器模块，提供生产级布隆过滤器和 Hutool 兼容的位图/哈希策略

#![forbid(unsafe_code)]

mod bitmap;
pub mod hashes;
mod hutool;

use bloomfilter::Bloom;
use std::{hash::Hash, io};

pub use bitmap::{BitMap, IntMap, LongMap, MachineWord};
pub use hutool::{
    BitMapBloomFilter, BitSetBloomFilter, BloomFilterUtil, DefaultFilter, ELFFilter, FNVFilter,
    FuncFilter, HfFilter, HfIpFilter, JSFilter, PJWFilter, RSFilter, SDBMFilter, StringBloomFilter,
    TianlFilter,
};

/// Errors returned by filter construction, indexing, and file initialization.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 布隆过滤器构造、索引和文件初始化返回的错误类型
#[derive(Debug, thiserror::Error)]
pub enum BloomFilterError {
    /// A capacity or expected item count was zero.
    #[error("capacity must be greater than zero")]
    EmptyCapacity,
    /// A probability was outside the open interval `(0, 1)`.
    #[error("false_positive_rate must be between 0 and 1")]
    InvalidFalsePositiveRate,
    /// Hutool's `BitSet` implementation supports one through eight hash functions.
    #[error("hash function count must be between 1 and 8")]
    InvalidHashFunctionCount,
    /// The requested bit index is outside the configured bitmap.
    #[error("bit index {index} is outside capacity {capacity}")]
    IndexOutOfBounds {
        /// Rejected bit index.
        index: u64,
        /// Configured number of addressable bits.
        capacity: u64,
    },
    /// An arithmetic operation exceeded the platform's addressable capacity.
    #[error("filter capacity is too large")]
    CapacityOverflow,
    /// File initialization failed.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Input bytes were not valid in the requested character encoding.
    #[error("input is not valid {0}")]
    InvalidEncoding(&'static str),
}

impl PartialEq for BloomFilterError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EmptyCapacity, Self::EmptyCapacity)
            | (Self::InvalidFalsePositiveRate, Self::InvalidFalsePositiveRate)
            | (Self::InvalidHashFunctionCount, Self::InvalidHashFunctionCount)
            | (Self::CapacityOverflow, Self::CapacityOverflow) => true,
            (Self::InvalidEncoding(left), Self::InvalidEncoding(right)) => left == right,
            (
                Self::IndexOutOfBounds {
                    index: left_index,
                    capacity: left_capacity,
                },
                Self::IndexOutOfBounds {
                    index: right_index,
                    capacity: right_capacity,
                },
            ) => left_index == right_index && left_capacity == right_capacity,
            (Self::Io(left), Self::Io(right)) => left.kind() == right.kind(),
            _ => false,
        }
    }
}

/// A high-throughput generic filter backed by the mature `bloomfilter` crate.
///
/// 对齐: Rust 扩展，封装 `bloomfilter` 库
/// 中文说明: 基于成熟 bloomfilter 库的高吞吐量泛型布隆过滤器
#[derive(Debug)]
pub struct BloomFilter<T: ?Sized> {
    inner: Bloom<T>,
}

impl<T: Hash + ?Sized> BloomFilter<T> {
    /// Sizes a filter from expected cardinality and false-positive probability.
    ///
    /// # Panics
    ///
    /// Panics only if the mature engine rejects inputs validated by this facade.
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Result<Self, BloomFilterError> {
        if expected_items == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if !(0.0..1.0).contains(&false_positive_rate) || false_positive_rate == 0.0 {
            return Err(BloomFilterError::InvalidFalsePositiveRate);
        }
        Ok(Self {
            inner: Bloom::new_for_fp_rate_with_seed(expected_items, false_positive_rate, &[0; 32])
                .expect("validated bloom layout"),
        })
    }

    /// Inserts a value and returns whether it was probably already present.
    pub fn insert(&mut self, value: &T) -> bool {
        self.inner.check_and_set(value)
    }

    /// Returns false only when the value is definitely absent.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.inner.check(value)
    }

    /// Returns the allocated bit count.
    #[must_use]
    pub fn bit_count(&self) -> u64 {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests;
