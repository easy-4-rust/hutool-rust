//! 对齐: `cn.hutool.bloomFilter.BitMap` (内部位图实现)
//! 来源: hutool-bloomFilter/src/main/java/cn/hutool/bloomFilter/BitMap.java
//! 中文说明: Hutool 兼容的位图操作，提供带边界检查的 Rust 索引

use crate::BloomFilterError;
use std::collections::HashMap;

/// Hutool-compatible bitmap operations with checked Rust indexing.
///
/// 对齐: `cn.hutool.bloomFilter.BitMap`
/// 中文说明: Hutool 兼容的位图操作接口，支持添加、查询和删除位
pub trait BitMap: Send {
    /// Sets a bit.
    fn add(&mut self, index: u64) -> Result<(), BloomFilterError>;
    /// Tests a bit.
    fn contains(&self, index: u64) -> Result<bool, BloomFilterError>;
    /// Clears a bit.
    fn remove(&mut self, index: u64) -> Result<(), BloomFilterError>;
    /// Returns the addressable bit capacity.
    fn capacity(&self) -> u64;
}

/// Machine word width used by Hutool's bitmap filters.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: Hutool 位图过滤器使用的机器字宽度枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineWord {
    /// 32-bit words (`IntMap`).
    Bits32,
    /// 64-bit words (`LongMap`).
    Bits64,
}

impl MachineWord {
    pub(crate) const fn bits(self) -> u64 {
        match self {
            Self::Bits32 => 32,
            Self::Bits64 => 64,
        }
    }
}

const HUTOOL_DEFAULT_WORDS: usize = 93_750_000;

macro_rules! bitmap {
    ($name:ident, $word:ty, $bits:expr, $java_class:expr) => {
        /// A sparse, bounded equivalent of Hutool's large eager bitmap.
        ///
        #[doc = concat!("对齐: `", $java_class, "`")]
        /// 中文说明: 稀疏有界位图，等效于 Hutool 的大容量预分配位图
        #[derive(Debug, Clone)]
        pub struct $name {
            words: HashMap<usize, $word>,
            word_count: usize,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::with_size(HUTOOL_DEFAULT_WORDS)
            }
        }

        impl $name {
            /// Creates the Hutool-default logical capacity without eagerly allocating it.
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            /// Creates a bitmap with `word_count` addressable machine words.
            #[must_use]
            pub fn with_size(word_count: usize) -> Self {
                Self {
                    words: HashMap::new(),
                    word_count,
                }
            }

            fn position(&self, index: u64) -> Result<(usize, u32), BloomFilterError> {
                let capacity = self.capacity();
                if index >= capacity {
                    return Err(BloomFilterError::IndexOutOfBounds { index, capacity });
                }
                Ok(((index / $bits) as usize, (index % $bits) as u32))
            }
        }

        impl BitMap for $name {
            fn add(&mut self, index: u64) -> Result<(), BloomFilterError> {
                let (word, bit) = self.position(index)?;
                *self.words.entry(word).or_default() |= (1 as $word) << bit;
                Ok(())
            }

            fn contains(&self, index: u64) -> Result<bool, BloomFilterError> {
                let (word, bit) = self.position(index)?;
                Ok(self
                    .words
                    .get(&word)
                    .is_some_and(|value| ((value >> bit) & 1) == 1))
            }

            fn remove(&mut self, index: u64) -> Result<(), BloomFilterError> {
                let (word, bit) = self.position(index)?;
                if let Some(value) = self.words.get_mut(&word) {
                    *value &= !((1 as $word) << bit);
                    if *value == 0 {
                        self.words.remove(&word);
                    }
                }
                Ok(())
            }

            fn capacity(&self) -> u64 {
                (self.word_count as u64).saturating_mul($bits)
            }
        }
    };
}

bitmap!(IntMap, u32, 32, "cn.hutool.bloomFilter.IntMap");
bitmap!(LongMap, u64, 64, "cn.hutool.bloomFilter.LongMap");

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn create_bitmap(
    max_value: u64,
    machine: MachineWord,
) -> Result<Box<dyn BitMap>, BloomFilterError> {
    if max_value == 0 {
        return Err(BloomFilterError::EmptyCapacity);
    }
    if max_value > i32::MAX as u64 {
        return Err(BloomFilterError::CapacityOverflow);
    }
    // `max_value <= i32::MAX`, so this addition and the usize conversion are
    // safe on every target supported by Rust 1.85.
    let words = max_value.div_ceil(machine.bits()) as usize;
    Ok(match machine {
        MachineWord::Bits32 => Box::new(IntMap::with_size(words)),
        MachineWord::Bits64 => Box::new(LongMap::with_size(words)),
    })
}
