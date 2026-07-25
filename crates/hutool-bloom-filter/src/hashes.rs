//! Hutool `HashUtil` algorithms, including Java UTF-16 and wrapping semantics.
//!
//! 对齐: `cn.hutool.bloomFilter.HashUtil`
//! 来源: hutool-bloomFilter/src/main/java/cn/hutool/bloomFilter/HashUtil.java
//! 中文说明: Hutool 哈希工具类，包含多种哈希算法实现（RS、JS、ELF、PJW 等）

fn units(value: &str) -> impl Iterator<Item = i32> + '_ {
    value.encode_utf16().map(i32::from)
}

/// Java's `String.hashCode` algorithm.
#[must_use]
pub fn java_default_hash(value: &str) -> i32 {
    units(value).fold(0_i32, |hash, unit| hash.wrapping_mul(31).wrapping_add(unit))
}

/// RS hash.
#[must_use]
pub fn rs_hash(value: &str) -> i32 {
    let mut multiplier = 63_689_i32;
    let mut hash = 0_i32;
    for unit in units(value) {
        hash = hash.wrapping_mul(multiplier).wrapping_add(unit);
        multiplier = multiplier.wrapping_mul(378_551);
    }
    hash & i32::MAX
}

/// JS hash.
#[must_use]
pub fn js_hash(value: &str) -> i32 {
    let mut hash = 1_315_423_911_i32;
    for unit in units(value) {
        hash ^= hash
            .wrapping_shl(5)
            .wrapping_add(unit)
            .wrapping_add(hash >> 2);
    }
    hash.wrapping_abs() & i32::MAX
}

/// PJW hash.
#[must_use]
pub fn pjw_hash(value: &str) -> i32 {
    let mut hash = 0_i32;
    for unit in units(value) {
        hash = hash.wrapping_shl(4).wrapping_add(unit);
        let high = hash & -268_435_456_i32;
        if high != 0 {
            hash = (hash ^ (high >> 24)) & !high;
        }
    }
    hash & i32::MAX
}

/// ELF hash.
#[must_use]
pub fn elf_hash(value: &str) -> i32 {
    let mut hash = 0_i32;
    for unit in units(value) {
        hash = hash.wrapping_shl(4).wrapping_add(unit);
        let high = hash & -268_435_456_i32;
        if high != 0 {
            hash ^= high >> 24;
            hash &= !high;
        }
    }
    hash & i32::MAX
}

/// BKDR hash.
#[must_use]
pub fn bkdr_hash(value: &str) -> i32 {
    units(value).fold(0_i32, |hash, unit| {
        hash.wrapping_mul(131).wrapping_add(unit)
    }) & i32::MAX
}

/// SDBM hash.
#[must_use]
pub fn sdbm_hash(value: &str) -> i32 {
    units(value).fold(0_i32, |hash, unit| {
        unit.wrapping_add(hash.wrapping_shl(6))
            .wrapping_add(hash.wrapping_shl(16))
            .wrapping_sub(hash)
    }) & i32::MAX
}

/// DJB hash.
#[must_use]
pub fn djb_hash(value: &str) -> i32 {
    units(value).fold(5_381_i32, |hash, unit| {
        hash.wrapping_shl(5).wrapping_add(hash).wrapping_add(unit)
    }) & i32::MAX
}

/// AP hash.
#[must_use]
pub fn ap_hash(value: &str) -> i32 {
    units(value).enumerate().fold(0_i32, |hash, (index, unit)| {
        if index & 1 == 0 {
            hash ^ (hash.wrapping_shl(7) ^ unit ^ (hash >> 3))
        } else {
            hash ^ !(hash.wrapping_shl(11) ^ unit ^ (hash >> 5))
        }
    })
}

/// Improved 32-bit FNV-1 hash.
#[must_use]
pub fn fnv_hash(value: &str) -> i32 {
    let mut hash = -2_128_831_035_i32;
    for unit in units(value) {
        hash = (hash ^ unit).wrapping_mul(16_777_619);
    }
    hash = hash.wrapping_add(hash.wrapping_shl(13));
    hash ^= hash >> 7;
    hash = hash.wrapping_add(hash.wrapping_shl(3));
    hash ^= hash >> 17;
    hash = hash.wrapping_add(hash.wrapping_shl(5));
    hash.wrapping_abs()
}

/// `TianL`'s case-insensitive, tail-bounded hash.
#[must_use]
pub fn tianl_hash(value: &str) -> i64 {
    let chars: Vec<u16> = value.encode_utf16().collect();
    if chars.is_empty() {
        return 0;
    }
    let length = chars.len();
    let mut hash = if length <= 256 {
        16_777_216_i64 * (i64::try_from(length).unwrap_or(i64::MAX) - 1)
    } else {
        4_278_190_080_i64
    };
    let start = length.saturating_sub(96);
    for (offset, unit) in chars[start..].iter().take(96).enumerate() {
        let unit = if (u16::from(b'A')..=u16::from(b'Z')).contains(unit) {
            unit + 32
        } else {
            *unit
        };
        let unit = i64::from(unit);
        let index = i64::try_from(offset).unwrap_or(i64::MAX - 1) + 1;
        hash = hash.wrapping_add(
            (3 * index * unit * unit + 5 * index * unit + 7 * index + 11 * unit) % 16_777_216,
        );
    }
    hash.wrapping_abs()
}

/// HF hash.
#[must_use]
pub fn hf_hash(value: &str) -> i64 {
    value
        .encode_utf16()
        .enumerate()
        .fold(0_i64, |hash, (index, unit)| {
            hash.wrapping_add(i64::from(unit) * 3 * i64::try_from(index).unwrap_or(i64::MAX))
        })
        .wrapping_abs()
}

/// HF-IP hash.
#[must_use]
pub fn hf_ip_hash(value: &str) -> i64 {
    let units: Vec<u16> = value.encode_utf16().collect();
    units.iter().enumerate().fold(0_i64, |hash, (index, unit)| {
        hash.wrapping_add(i64::from(units[index % 4] ^ unit))
    })
}

/// Selects one of the eight algorithms used by Hutool's `BitSetBloomFilter`.
#[must_use]
pub fn indexed_hash(value: &str, index: usize) -> i32 {
    match index {
        0 => rs_hash(value),
        1 => js_hash(value),
        2 => elf_hash(value),
        3 => bkdr_hash(value),
        4 => ap_hash(value),
        5 => djb_hash(value),
        6 => sdbm_hash(value),
        7 => pjw_hash(value),
        _ => 0,
    }
}
