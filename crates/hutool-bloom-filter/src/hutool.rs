//! 对齐: `cn.hutool.bloomFilter` (Hutool 布隆过滤器核心实现)
//! 来源: hutool-bloomFilter/src/main/java/cn/hutool/bloomFilter/
//! 中文说明: Hutool 风格的布隆过滤器实现，包含 FuncFilter、BitMapBloomFilter、BitSetBloomFilter 和 BloomFilterUtil

use crate::{BitMap, BloomFilterError, MachineWord, bitmap::create_bitmap, hashes};
use encoding_rs::Encoding;
use std::{fs, path::Path, sync::Arc};

/// Common string filter contract matching Hutool's `BloomFilter` interface.
///
/// 对齐: `cn.hutool.bloomFilter.BloomFilter` (接口)
/// 中文说明: 通用字符串布隆过滤器接口，匹配 Hutool 的 BloomFilter 接口
pub trait StringBloomFilter: Send {
    /// Returns false only when the string is definitely absent.
    fn contains(&self, value: &str) -> bool;
    /// Adds a string and returns false when it was probably already present.
    fn add(&mut self, value: &str) -> bool;
}

type HashFunction = dyn Fn(&str) -> i64 + Send + Sync;

/// One Hutool-compatible hash function backed by a checked bitmap.
///
/// 对齐: `cn.hutool.bloomFilter.FilterFunc`
/// 中文说明: 基于校验位图的 Hutool 兼容哈希函数过滤器
pub struct FuncFilter {
    bitmap: Box<dyn BitMap>,
    size: u64,
    hash: Arc<HashFunction>,
}

impl FuncFilter {
    /// Creates a 32-bit bitmap filter.
    pub fn new(
        max_value: u64,
        hash: impl Fn(&str) -> i64 + Send + Sync + 'static,
    ) -> Result<Self, BloomFilterError> {
        Self::with_machine(max_value, MachineWord::Bits32, hash)
    }

    /// Creates a filter using the selected bitmap word width.
    pub fn with_machine(
        max_value: u64,
        machine: MachineWord,
        hash: impl Fn(&str) -> i64 + Send + Sync + 'static,
    ) -> Result<Self, BloomFilterError> {
        Ok(Self {
            bitmap: create_bitmap(max_value, machine)?,
            size: max_value,
            hash: Arc::new(hash),
        })
    }

    /// Computes the normalized index used by this filter.
    #[must_use]
    pub fn hash(&self, value: &str) -> u64 {
        ((self.hash)(value) % i64::try_from(self.size).unwrap_or(i64::MAX)).unsigned_abs()
    }
}

impl StringBloomFilter for FuncFilter {
    fn contains(&self, value: &str) -> bool {
        self.bitmap
            .contains(self.hash(value))
            .expect("normalized hash is inside the configured bitmap")
    }

    fn add(&mut self, value: &str) -> bool {
        let index = self.hash(value);
        if self
            .bitmap
            .contains(index)
            .expect("normalized hash is inside the configured bitmap")
        {
            return false;
        }
        self.bitmap
            .add(index)
            .expect("normalized hash is inside the configured bitmap");
        true
    }
}

macro_rules! filter {
    ($name:ident, $hash:path, $java_class:expr) => {
        /// A Hutool-compatible named hash filter.
        ///
        #[doc = concat!("对齐: `", $java_class, "`")]
        /// 中文说明: Hutool 兼容的命名哈希过滤器
        pub struct $name(FuncFilter);

        impl $name {
            /// Creates the filter with 32-bit bitmap words.
            pub fn new(max_value: u64) -> Result<Self, BloomFilterError> {
                Self::with_machine(max_value, MachineWord::Bits32)
            }

            /// Creates the filter with an explicit bitmap word width.
            pub fn with_machine(
                max_value: u64,
                machine: MachineWord,
            ) -> Result<Self, BloomFilterError> {
                FuncFilter::with_machine(max_value, machine, |value| i64::from($hash(value)))
                    .map(Self)
            }

            /// Computes the normalized bit index.
            #[must_use]
            pub fn hash(&self, value: &str) -> u64 {
                self.0.hash(value)
            }
        }

        impl StringBloomFilter for $name {
            fn contains(&self, value: &str) -> bool {
                self.0.contains(value)
            }

            fn add(&mut self, value: &str) -> bool {
                self.0.add(value)
            }
        }
    };
}

filter!(
    DefaultFilter,
    hashes::java_default_hash,
    "cn.hutool.bloomFilter.filter.DefaultFilter"
);
filter!(
    ELFFilter,
    hashes::elf_hash,
    "cn.hutool.bloomFilter.filter.ELFFilter"
);
filter!(
    FNVFilter,
    hashes::fnv_hash,
    "cn.hutool.bloomFilter.filter.FNVFilter"
);
filter!(
    JSFilter,
    hashes::js_hash,
    "cn.hutool.bloomFilter.filter.JSFilter"
);
filter!(
    PJWFilter,
    hashes::pjw_hash,
    "cn.hutool.bloomFilter.filter.PJWFilter"
);
filter!(
    RSFilter,
    hashes::rs_hash,
    "cn.hutool.bloomFilter.filter.RSFilter"
);
filter!(
    SDBMFilter,
    hashes::sdbm_hash,
    "cn.hutool.bloomFilter.filter.SDBMFilter"
);

macro_rules! wide_filter {
    ($name:ident, $hash:path, $java_class:expr) => {
        /// A Hutool-compatible named 64-bit hash filter.
        ///
        #[doc = concat!("对齐: `", $java_class, "`")]
        /// 中文说明: Hutool 兼容的命名 64 位哈希过滤器
        pub struct $name(FuncFilter);

        impl $name {
            /// Creates the filter with 32-bit bitmap words.
            pub fn new(max_value: u64) -> Result<Self, BloomFilterError> {
                Self::with_machine(max_value, MachineWord::Bits32)
            }

            /// Creates the filter with an explicit bitmap word width.
            pub fn with_machine(
                max_value: u64,
                machine: MachineWord,
            ) -> Result<Self, BloomFilterError> {
                FuncFilter::with_machine(max_value, machine, $hash).map(Self)
            }

            /// Computes the normalized bit index.
            #[must_use]
            pub fn hash(&self, value: &str) -> u64 {
                self.0.hash(value)
            }
        }

        impl StringBloomFilter for $name {
            fn contains(&self, value: &str) -> bool {
                self.0.contains(value)
            }

            fn add(&mut self, value: &str) -> bool {
                self.0.add(value)
            }
        }
    };
}

wide_filter!(
    HfFilter,
    hashes::hf_hash,
    "cn.hutool.bloomFilter.filter.HfFilter"
);
wide_filter!(
    HfIpFilter,
    hashes::hf_ip_hash,
    "cn.hutool.bloomFilter.filter.HfIpFilter"
);
wide_filter!(
    TianlFilter,
    hashes::tianl_hash,
    "cn.hutool.bloomFilter.filter.TianlFilter"
);

/// An AND-composed group of independently hashed bitmap filters.
///
/// 对齐: `cn.hutool.bloomFilter.BitMapBloomFilter`
/// 中文说明: 由多个独立哈希位图过滤器 AND 组合而成的布隆过滤器
pub struct BitMapBloomFilter {
    filters: Vec<Box<dyn StringBloomFilter>>,
}

impl BitMapBloomFilter {
    /// Builds Hutool's default five-filter layout from a size in MiB.
    ///
    /// # Panics
    ///
    /// Panics only if an already validated internal bitmap size is rejected.
    pub fn new(mebibytes: usize) -> Result<Self, BloomFilterError> {
        let per_filter = mebibytes / 5;
        let size = u64::try_from(per_filter)
            .ok()
            .and_then(|value| value.checked_mul(1024 * 1024 * 8))
            .ok_or(BloomFilterError::CapacityOverflow)?;
        if size == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if size > i32::MAX as u64 {
            return Err(BloomFilterError::CapacityOverflow);
        }
        Ok(Self {
            filters: vec![
                Box::new(DefaultFilter::new(size).expect("validated filter size")),
                Box::new(ELFFilter::new(size).expect("validated filter size")),
                Box::new(JSFilter::new(size).expect("validated filter size")),
                Box::new(PJWFilter::new(size).expect("validated filter size")),
                Box::new(SDBMFilter::new(size).expect("validated filter size")),
            ],
        })
    }

    /// Builds a composed filter from explicit hash filters.
    pub fn with_filters(
        filters: Vec<Box<dyn StringBloomFilter>>,
    ) -> Result<Self, BloomFilterError> {
        if filters.is_empty() {
            return Err(BloomFilterError::EmptyCapacity);
        }
        Ok(Self { filters })
    }
}

impl StringBloomFilter for BitMapBloomFilter {
    fn contains(&self, value: &str) -> bool {
        self.filters.iter().all(|filter| filter.contains(value))
    }

    fn add(&mut self, value: &str) -> bool {
        let mut added = false;
        for filter in &mut self.filters {
            added |= filter.add(value);
        }
        added
    }
}

/// Hutool's fixed one-to-eight-hash `BitSet` implementation.
///
/// 对齐: `cn.hutool.bloomFilter.BitSetBloomFilter`
/// 中文说明: Hutool 的固定一至八个哈希函数的 BitSet 布隆过滤器实现
#[derive(Debug, Clone)]
pub struct BitSetBloomFilter {
    words: Vec<u64>,
    bit_count: usize,
    expected_elements: u32,
    hash_count: u8,
}

impl BitSetBloomFilter {
    /// Creates a filter with `capacity * hash_count` bits.
    pub fn new(
        capacity: usize,
        expected_elements: usize,
        hash_count: usize,
    ) -> Result<Self, BloomFilterError> {
        if capacity == 0 || expected_elements == 0 {
            return Err(BloomFilterError::EmptyCapacity);
        }
        if !(1..=8).contains(&hash_count) {
            return Err(BloomFilterError::InvalidHashFunctionCount);
        }
        let bit_count = capacity
            .checked_mul(hash_count)
            .ok_or(BloomFilterError::CapacityOverflow)?;
        if bit_count > i32::MAX as usize {
            return Err(BloomFilterError::CapacityOverflow);
        }
        let expected_elements =
            u32::try_from(expected_elements).map_err(|_| BloomFilterError::CapacityOverflow)?;
        #[allow(clippy::cast_possible_truncation)]
        let hash_count = hash_count as u8;
        let words = bit_count.div_ceil(64);
        Ok(Self {
            words: vec![0; words],
            bit_count,
            expected_elements,
            hash_count,
        })
    }

    /// Loads one value per line as UTF-8.
    pub fn init(&mut self, path: impl AsRef<Path>) -> Result<(), BloomFilterError> {
        let content = fs::read_to_string(path)?;
        self.add_lines(&content);
        Ok(())
    }

    /// Loads one value per line with an explicit `encoding_rs` decoder.
    pub fn init_with_encoding(
        &mut self,
        path: impl AsRef<Path>,
        encoding: &'static Encoding,
    ) -> Result<(), BloomFilterError> {
        let bytes = fs::read(path)?;
        let (content, had_errors) = encoding.decode_without_bom_handling(&bytes);
        if had_errors {
            return Err(BloomFilterError::InvalidEncoding(encoding.name()));
        }
        self.add_lines(&content);
        Ok(())
    }

    fn add_lines(&mut self, content: &str) {
        for line in content.lines() {
            self.add(line);
        }
    }

    /// Returns Hutool's configured false-positive estimate.
    #[must_use]
    pub fn false_positive_probability(&self) -> f64 {
        let bit_count = u32::try_from(self.bit_count).unwrap_or(u32::MAX);
        (1.0 - (-f64::from(self.hash_count) * f64::from(self.expected_elements)
            / f64::from(bit_count))
        .exp())
        .powi(i32::from(self.hash_count))
    }

    /// Computes the selected number of Hutool hashes.
    #[must_use]
    pub fn create_hashes(value: &str, hash_count: usize) -> Vec<i32> {
        (0..hash_count)
            .map(|index| hashes::indexed_hash(value, index))
            .collect()
    }

    /// Computes one indexed Hutool hash, returning zero outside `0..8`.
    #[must_use]
    pub fn hash(value: &str, index: usize) -> i32 {
        hashes::indexed_hash(value, index)
    }

    fn positions(&self, value: &str) -> impl Iterator<Item = usize> + '_ {
        Self::create_hashes(value, usize::from(self.hash_count))
            .into_iter()
            .map(|hash| {
                let bit_count = i32::try_from(self.bit_count).unwrap_or(i32::MAX);
                usize::try_from((hash % bit_count).unsigned_abs()).unwrap_or(usize::MAX)
            })
    }
}

impl StringBloomFilter for BitSetBloomFilter {
    fn contains(&self, value: &str) -> bool {
        self.positions(value)
            .all(|position| self.words[position / 64] & (1_u64 << (position % 64)) != 0)
    }

    fn add(&mut self, value: &str) -> bool {
        if self.contains(value) {
            return false;
        }
        for position in self.positions(value).collect::<Vec<_>>() {
            self.words[position / 64] |= 1_u64 << (position % 64);
        }
        true
    }
}

/// Constructors corresponding to Hutool's `BloomFilterUtil`.
///
/// 对齐: `cn.hutool.bloomFilter.BloomFilterUtil`
/// 中文说明: 布隆过滤器工具类，提供 BitSetBloomFilter 和 BitMapBloomFilter 的便捷构造方法
pub struct BloomFilterUtil;

impl BloomFilterUtil {
    /// Creates a `BitSetBloomFilter`.
    pub fn create_bit_set(
        capacity: usize,
        expected_elements: usize,
        hash_count: usize,
    ) -> Result<BitSetBloomFilter, BloomFilterError> {
        BitSetBloomFilter::new(capacity, expected_elements, hash_count)
    }

    /// Creates Hutool's default bitmap composition.
    pub fn create_bit_map(mebibytes: usize) -> Result<BitMapBloomFilter, BloomFilterError> {
        BitMapBloomFilter::new(mebibytes)
    }
}
