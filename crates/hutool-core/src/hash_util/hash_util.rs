//! Classic non-cryptographic hashes aligned with Hutool's UTF-16 and wrapping rules.

#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use crate::IdKey;
/// 对齐: `cn.hutool.core.util.HashUtil`
/// 哈希工具类
use crate::lang::hash::{CityHash, Number128};

use super::hash_error::HashError;

/// Hutool-aligned classic hash algorithms.
#[derive(Debug, Clone, Copy, Default)]
pub struct HashUtil;

impl HashUtil {
    /// Additive hash over Java UTF-16 units.
    pub fn additive_hash(key: &str, prime: i32) -> Result<i32, HashError> {
        require_prime(prime)?;
        let units = utf16(key);
        let hash = units.iter().fold(units.len() as i32, |hash, unit| {
            hash.wrapping_add(i32::from(*unit))
        });
        Ok(hash % prime)
    }

    /// Rotating hash over Java UTF-16 units.
    pub fn rotating_hash(key: &str, prime: i32) -> Result<i32, HashError> {
        require_prime(prime)?;
        let units = utf16(key);
        let hash = units.iter().fold(units.len() as i32, |hash, unit| {
            hash.wrapping_shl(4) ^ (hash >> 28) ^ i32::from(*unit)
        });
        Ok(hash % prime)
    }

    /// Bob Jenkins' one-at-a-time style hash used by Hutool.
    #[must_use]
    pub fn one_by_one_hash(key: &str) -> i32 {
        let mut hash = 0_i32;
        for unit in utf16(key) {
            hash = hash.wrapping_add(i32::from(unit));
            hash = hash.wrapping_add(hash.wrapping_shl(10));
            hash ^= hash >> 6;
        }
        hash = hash.wrapping_add(hash.wrapping_shl(3));
        hash ^= hash >> 11;
        hash.wrapping_add(hash.wrapping_shl(15))
    }

    /// Bernstein hash over Java UTF-16 units.
    #[must_use]
    pub fn bernstein(key: &str) -> i32 {
        utf16(key).into_iter().fold(0_i32, |hash, unit| {
            hash.wrapping_mul(33).wrapping_add(i32::from(unit))
        })
    }

    /// Universal table hash with bounds validation.
    pub fn universal(key: &[u16], mask: i32, table: &[i32]) -> Result<i32, HashError> {
        let expected = key.len().saturating_mul(8);
        if table.len() < expected {
            return Err(HashError::UniversalTable {
                expected,
                actual: table.len(),
            });
        }
        let mut hash = key.len() as i32;
        for (index, unit) in key.iter().enumerate() {
            for bit in 0..8 {
                if unit & (1 << bit) == 0 {
                    hash ^= table[index * 8 + bit];
                }
            }
        }
        Ok(hash & mask)
    }

    /// Zobrist table hash with row and column validation.
    pub fn zobrist<T: AsRef<[i32]>>(key: &[u16], mask: i32, table: &[T]) -> Result<i32, HashError> {
        if table.len() < key.len() {
            return Err(HashError::ZobristRows {
                expected: key.len(),
                actual: table.len(),
            });
        }
        let mut hash = key.len() as i32;
        for (index, unit) in key.iter().enumerate() {
            let row = table[index].as_ref();
            let column = usize::from(*unit);
            if row.len() <= column {
                return Err(HashError::ZobristColumns {
                    row: index,
                    expected: column + 1,
                    actual: row.len(),
                });
            }
            hash ^= row[column];
        }
        Ok(hash & mask)
    }

    /// Hutool's improved FNV-1 hash for signed Java bytes.
    #[must_use]
    pub fn fnv_hash_bytes(data: &[u8]) -> i32 {
        improved_fnv(
            data.iter()
                .map(|byte| i32::from(i8::from_ne_bytes([*byte]))),
        )
    }

    /// Hutool's improved FNV-1 hash for Java UTF-16 string units.
    #[must_use]
    pub fn fnv_hash(key: &str) -> i32 {
        improved_fnv(utf16(key).into_iter().map(i32::from))
    }

    /// Thomas Wang's 32-bit integer hash.
    #[must_use]
    pub fn int_hash(mut key: i32) -> i32 {
        key = key.wrapping_add(!key.wrapping_shl(15));
        key ^= unsigned_shift(key, 10);
        key = key.wrapping_add(key.wrapping_shl(3));
        key ^= unsigned_shift(key, 6);
        key = key.wrapping_add(!key.wrapping_shl(11));
        key ^ unsigned_shift(key, 16)
    }

    /// RS hash.
    #[must_use]
    pub fn rs_hash(value: &str) -> i32 {
        let mut multiplier = 63_689_i32;
        let mut hash = 0_i32;
        for unit in utf16(value) {
            hash = hash.wrapping_mul(multiplier).wrapping_add(i32::from(unit));
            multiplier = multiplier.wrapping_mul(378_551);
        }
        hash & I32_MASK
    }

    /// JS hash.
    #[must_use]
    pub fn js_hash(value: &str) -> i32 {
        let mut hash = 1_315_423_911_i32;
        for unit in utf16(value) {
            hash ^= hash
                .wrapping_shl(5)
                .wrapping_add(i32::from(unit))
                .wrapping_add(hash >> 2);
        }
        hash.wrapping_abs() & I32_MASK
    }

    /// PJW hash.
    #[must_use]
    pub fn pjw_hash(value: &str) -> i32 {
        let high_bits = i32::from_ne_bytes(0xf000_0000_u32.to_ne_bytes());
        let mut hash = 0_i32;
        for unit in utf16(value) {
            hash = hash.wrapping_shl(4).wrapping_add(i32::from(unit));
            let test = hash & high_bits;
            if test != 0 {
                hash = (hash ^ (test >> 24)) & !high_bits;
            }
        }
        hash & I32_MASK
    }

    /// ELF hash.
    #[must_use]
    pub fn elf_hash(value: &str) -> i32 {
        let mut hash = 0_i32;
        for unit in utf16(value) {
            hash = hash.wrapping_shl(4).wrapping_add(i32::from(unit));
            let high = hash & i32::from_ne_bytes(0xf000_0000_u32.to_ne_bytes());
            if high != 0 {
                hash ^= high >> 24;
                hash &= !high;
            }
        }
        hash & I32_MASK
    }

    /// BKDR hash using Hutool's seed 131.
    #[must_use]
    pub fn bkdr_hash(value: &str) -> i32 {
        utf16(value).into_iter().fold(0_i32, |hash, unit| {
            hash.wrapping_mul(131).wrapping_add(i32::from(unit))
        }) & I32_MASK
    }

    /// SDBM hash.
    #[must_use]
    pub fn sdbm_hash(value: &str) -> i32 {
        utf16(value).into_iter().fold(0_i32, |hash, unit| {
            i32::from(unit)
                .wrapping_add(hash.wrapping_shl(6))
                .wrapping_add(hash.wrapping_shl(16))
                .wrapping_sub(hash)
        }) & I32_MASK
    }

    /// DJB hash.
    #[must_use]
    pub fn djb_hash(value: &str) -> i32 {
        utf16(value).into_iter().fold(5_381_i32, |hash, unit| {
            hash.wrapping_shl(5)
                .wrapping_add(hash)
                .wrapping_add(i32::from(unit))
        }) & I32_MASK
    }

    /// DEK hash.
    #[must_use]
    pub fn dek_hash(value: &str) -> i32 {
        let units = utf16(value);
        units.iter().fold(units.len() as i32, |hash, unit| {
            hash.wrapping_shl(5) ^ (hash >> 27) ^ i32::from(*unit)
        }) & I32_MASK
    }

    /// AP hash.
    #[must_use]
    pub fn ap_hash(value: &str) -> i32 {
        let mut hash = 0_i32;
        for (index, unit) in utf16(value).into_iter().enumerate() {
            let unit = i32::from(unit);
            hash ^= if index & 1 == 0 {
                hash.wrapping_shl(7) ^ unit ^ (hash >> 3)
            } else {
                !(hash.wrapping_shl(11) ^ unit ^ (hash >> 5))
            };
        }
        hash
    }

    /// `TianL` hash over Java UTF-16 units.
    #[must_use]
    pub fn tianl_hash(value: &str) -> i64 {
        let units = utf16(value);
        if units.is_empty() {
            return 0;
        }
        let length = units.len();
        let mut hash = if length <= 256 {
            16_777_216_i64.wrapping_mul(length.saturating_sub(1) as i64)
        } else {
            4_278_190_080
        };
        let selected = if length <= 96 {
            &units[..]
        } else {
            &units[length - 96..]
        };
        for (index, unit) in selected.iter().enumerate() {
            let position = (index + 1) as i64;
            let unit = i64::from(lower_ascii_unit(*unit));
            let contribution = 3_i64
                .wrapping_mul(position)
                .wrapping_mul(unit)
                .wrapping_mul(unit)
                .wrapping_add(5_i64.wrapping_mul(position).wrapping_mul(unit))
                .wrapping_add(7_i64.wrapping_mul(position))
                .wrapping_add(11_i64.wrapping_mul(unit))
                % 16_777_216;
            hash = hash.wrapping_add(contribution);
        }
        java_abs_i64(hash)
    }

    /// Java `String.hashCode` over UTF-16 units.
    #[must_use]
    pub fn java_default_hash(value: &str) -> i32 {
        utf16(value).into_iter().fold(0_i32, |hash, unit| {
            hash.wrapping_mul(31).wrapping_add(i32::from(unit))
        })
    }

    /// Hutool's mixed Java/FNV 64-bit hash.
    #[must_use]
    pub fn mix_hash(value: &str) -> i64 {
        i64::from(Self::java_default_hash(value)).wrapping_shl(32)
            | i64::from(Self::fnv_hash(value))
    }

    /// Safe lifetime-bound reference identity hash.
    #[must_use]
    pub fn identity_hash_code<T>(value: &T) -> i32 {
        IdKey::new(value).hash_code()
    }

    /// HF hash over Java UTF-16 units.
    #[must_use]
    pub fn hf_hash(value: &str) -> i64 {
        let mut hash = 0_i64;
        for (index, unit) in utf16(value).into_iter().enumerate() {
            hash = hash.wrapping_add(i64::from(unit).wrapping_mul(3).wrapping_mul(index as i64));
        }
        java_abs_i64(hash)
    }

    /// HF-IP hash over Java UTF-16 units.
    #[must_use]
    pub fn hf_ip_hash(value: &str) -> i64 {
        let units = utf16(value);
        let mut hash = 0_i64;
        for (index, unit) in units.iter().enumerate() {
            hash = hash.wrapping_add(i64::from(units[index % 4] ^ unit));
        }
        hash
    }

    /// 对齐 Java: `HashUtil.cityHash32(byte[])`
    #[must_use]
    pub fn city_hash32(data: &[u8]) -> i32 {
        CityHash::hash32(data)
    }

    /// 对齐 Java: `HashUtil.cityHash64(byte[])`
    #[must_use]
    pub fn city_hash64(data: &[u8]) -> i64 {
        CityHash::hash64(data)
    }

    /// 对齐 Java: `HashUtil.cityHash128(byte[])`
    #[must_use]
    pub fn city_hash128(data: &[u8]) -> [i64; 2] {
        CityHash::hash128(data).get_long_array()
    }

    /// 对齐 Java: `HashUtil.cityHash128(byte[], Number128 seed)`
    #[must_use]
    pub fn city_hash128_seeded(data: &[u8], seed: Number128) -> [i64; 2] {
        CityHash::hash128_seeded(data, seed).get_long_array()
    }
}

fn require_prime(prime: i32) -> Result<(), HashError> {
    if prime == 0 {
        Err(HashError::ZeroPrime)
    } else {
        Ok(())
    }
}

fn java_abs_i64(value: i64) -> i64 {
    value.wrapping_abs()
}

fn lower_ascii_unit(unit: u16) -> u16 {
    if (u16::from(b'A')..=u16::from(b'Z')).contains(&unit) {
        unit + 32
    } else {
        unit
    }
}

fn utf16(value: &str) -> Vec<u16> {
    value.encode_utf16().collect()
}

fn unsigned_shift(value: i32, bits: u32) -> i32 {
    let value = u32::from_ne_bytes(value.to_ne_bytes()) >> bits;
    i32::from_ne_bytes(value.to_ne_bytes())
}

fn improved_fnv(values: impl IntoIterator<Item = i32>) -> i32 {
    let mut hash = i32::from_ne_bytes(2_166_136_261_u32.to_ne_bytes());
    for value in values {
        hash = (hash ^ value).wrapping_mul(16_777_619);
    }
    hash = hash.wrapping_add(hash.wrapping_shl(13));
    hash ^= hash >> 7;
    hash = hash.wrapping_add(hash.wrapping_shl(3));
    hash ^= hash >> 17;
    hash = hash.wrapping_add(hash.wrapping_shl(5));
    hash.wrapping_abs()
}

const I32_MASK: i32 = i32::MAX;
