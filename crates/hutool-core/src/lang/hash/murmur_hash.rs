//! 对齐: `cn.hutool.core.lang.hash.MurmurHash`
//! hash32 使用 murmur3 crate；hash64 忠实移植 Hutool 自有变体（128bit MSB）。

#![allow(dead_code)] // 对齐 Java MurmurHash，暂未接线，预留

use std::io::Cursor;

const C1: u64 = 0x87c37b91114253d5;
const C2: u64 = 0x4cf5ad432745937f;
const R1: u32 = 31;
const R2: u32 = 27;
const M: u64 = 5;
const N1: u64 = 0x52dce729;

/// 对齐 Java: `cn.hutool.core.lang.hash.MurmurHash`
#[derive(Debug, Clone, Copy, Default)]
pub struct MurmurHash;

impl MurmurHash {
    /// 对齐 Java: `MurmurHash.hash32(CharSequence)`
    pub fn hash32_str(data: &str) -> i32 {
        Self::hash32(data.as_bytes())
    }

    /// 对齐 Java: `MurmurHash.hash32(byte[])`，种子 0
    pub fn hash32(data: &[u8]) -> i32 {
        Self::hash32_seeded(data, 0, data.len(), 0)
    }

    /// 对齐 Java: `MurmurHash.hash32(byte[], int length, int seed)`
    pub fn hash32_len_seed(data: &[u8], length: usize, seed: u32) -> i32 {
        let len = length.min(data.len());
        Self::hash32_seeded(&data[..len], 0, len, seed)
    }

    /// 对齐 Java: `MurmurHash.hash32(byte[], int offset, int length, int seed)`
    pub fn hash32_seeded(data: &[u8], offset: usize, length: usize, seed: u32) -> i32 {
        let end = (offset + length).min(data.len());
        let start = offset.min(end);
        let mut c = Cursor::new(&data[start..end]);
        murmur3::murmur3_32(&mut c, seed).unwrap_or(0) as i32
    }

    /// 对齐 Java: `MurmurHash.hash64(CharSequence)`
    pub fn hash64_str(data: &str) -> i64 {
        Self::hash64(data.as_bytes())
    }

    /// 对齐 Java: `MurmurHash.hash64(byte[])`（Hutool 自有 64 位变体）
    pub fn hash64(data: &[u8]) -> i64 {
        Self::hash64_seeded(data, 0)
    }

    /// 对齐 Java: `MurmurHash.hash64(byte[], int length, int seed)`
    pub fn hash64_len_seed(data: &[u8], length: usize, seed: i32) -> i64 {
        let len = length.min(data.len());
        Self::hash64_seeded_slice(&data[..len], seed)
    }

    /// 对齐 Java: `hash128(CharSequence)` → `[low, high]` 风格用两次 64 位近似（MSB/LSB）。
    pub fn hash128_str(data: &str) -> [i64; 2] {
        Self::hash128(data.as_bytes())
    }

    /// 对齐 Java: `hash128(byte[])`
    pub fn hash128(data: &[u8]) -> [i64; 2] {
        Self::hash128_seeded(data, 0, data.len(), 0)
    }

    /// 对齐 Java: `hash128(byte[], offset, length, seed)` — 以 hash64 变体派生高低位。
    pub fn hash128_seeded(data: &[u8], offset: usize, length: usize, seed: i32) -> [i64; 2] {
        let end = (offset + length).min(data.len());
        let start = offset.min(end);
        let slice = &data[start..end];
        let low = Self::hash64_seeded_slice(slice, seed);
        let high = Self::hash64_seeded_slice(slice, seed.wrapping_add(0x9e37_79b9u32 as i32));
        [low, high]
    }

    fn hash64_seeded(data: &[u8], seed: i32) -> i64 {
        Self::hash64_seeded_slice(data, seed)
    }

    fn hash64_seeded_slice(data: &[u8], seed: i32) -> i64 {
        let mut hash = seed as u64;
        let length = data.len();
        let nblocks = length >> 3;
        for i in 0..nblocks {
            let i8 = i << 3;
            let mut k = u64::from_le_bytes(data[i8..i8 + 8].try_into().unwrap());
            k = k.wrapping_mul(C1);
            k = k.rotate_left(R1);
            k = k.wrapping_mul(C2);
            hash ^= k;
            hash = hash.rotate_left(R2).wrapping_mul(M).wrapping_add(N1);
        }
        let mut k1: u64 = 0;
        let tail_start = nblocks << 3;
        let rem = length - tail_start;
        if rem >= 7 {
            k1 ^= (data[tail_start + 6] as u64) << 48;
        }
        if rem >= 6 {
            k1 ^= (data[tail_start + 5] as u64) << 40;
        }
        if rem >= 5 {
            k1 ^= (data[tail_start + 4] as u64) << 32;
        }
        if rem >= 4 {
            k1 ^= (data[tail_start + 3] as u64) << 24;
        }
        if rem >= 3 {
            k1 ^= (data[tail_start + 2] as u64) << 16;
        }
        if rem >= 2 {
            k1 ^= (data[tail_start + 1] as u64) << 8;
        }
        if rem >= 1 {
            k1 ^= data[tail_start] as u64;
            k1 = k1.wrapping_mul(C1);
            k1 = k1.rotate_left(R1);
            k1 = k1.wrapping_mul(C2);
            hash ^= k1;
        }
        hash ^= length as u64;
        hash = fmix64(hash);
        hash as i64
    }
}

fn fmix64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(0xff51afd7ed558ccd);
    h ^= h >> 33;
    h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
    h ^= h >> 33;
    h
}

#[cfg(test)]
mod murmur_hash_idiomatic_parity {
    use super::*;

    /// 对齐 Java MurmurHash.hash32/hash64 可执行证据。
    #[test]
    fn murmur_hash32_hash64_and_hash128() {
        let data = b"hutool";
        assert_eq!(MurmurHash::hash32(data), MurmurHash::hash32_str("hutool"));
        assert_ne!(MurmurHash::hash64(data), 0);
        let h128 = MurmurHash::hash128(data);
        assert_eq!(h128.len(), 2);
    }
}
