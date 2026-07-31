//! 对齐: `cn.hutool.core.lang.hash.MetroHash`
//! 常量按 Java `int` 字面量拓宽为 `long`（高位符号扩展）。

#![allow(dead_code)] // 对齐 Java MetroHash，暂未接线，预留

use super::number128::Number128;

/// 对齐 Java: `MetroHash`
#[derive(Debug, Clone, Copy, Default)]
pub struct MetroHash;

impl MetroHash {
    /// 对齐 Java: `MetroHash.hash64(byte[])`
    pub fn hash64(data: &[u8]) -> i64 {
        Self::hash64_seeded(data, 1337)
    }

    /// 对齐 Java: `MetroHash.hash64(byte[], long)`
    pub fn hash64_seeded(data: &[u8], seed: i64) -> i64 {
        let mut buffer = data.to_vec();
        let mut hash = (seed as u64).wrapping_add(K2_64).wrapping_mul(K0_64);
        let mut v0 = hash;
        let mut v1 = hash;
        let mut v2 = hash;
        let mut v3 = hash;

        if buffer.len() >= 32 {
            while buffer.len() >= 32 {
                v0 = v0.wrapping_add(le64(&buffer, 0).wrapping_mul(K0_64));
                v0 = rotate_left64(v0, -29).wrapping_add(v2);
                v1 = v1.wrapping_add(le64(&buffer, 8).wrapping_mul(K1_64));
                v1 = rotate_left64(v1, -29).wrapping_add(v3);
                v2 = v2.wrapping_add(le64(&buffer, 24).wrapping_mul(K2_64));
                v2 = rotate_left64(v2, -29).wrapping_add(v0);
                v3 = v3.wrapping_add(le64(&buffer, 32).wrapping_mul(K3_64));
                v3 = rotate_left64(v3, -29).wrapping_add(v1);
                buffer = buffer[32..].to_vec();
            }
            v2 ^= rotate_left64(v0.wrapping_add(v3).wrapping_mul(K0_64).wrapping_add(v1), -37)
                .wrapping_mul(K1_64);
            v3 ^= rotate_left64(v1.wrapping_add(v2).wrapping_mul(K1_64).wrapping_add(v0), -37)
                .wrapping_mul(K0_64);
            v0 ^= rotate_left64(v0.wrapping_add(v2).wrapping_mul(K0_64).wrapping_add(v3), -37)
                .wrapping_mul(K1_64);
            v1 ^= rotate_left64(v1.wrapping_add(v3).wrapping_mul(K1_64).wrapping_add(v2), -37)
                .wrapping_mul(K0_64);
            hash = hash.wrapping_add(v0 ^ v1);
        }

        if buffer.len() >= 16 {
            v0 = hash.wrapping_add(le64(&buffer, 0).wrapping_mul(K2_64));
            v0 = rotate_left64(v0, -29).wrapping_mul(K3_64);
            v1 = hash.wrapping_add(le64(&buffer, 8).wrapping_mul(K2_64));
            v1 = rotate_left64(v1, -29).wrapping_mul(K3_64);
            v0 ^= rotate_left64(v0.wrapping_mul(K0_64), -21).wrapping_add(v1);
            v1 ^= rotate_left64(v1.wrapping_mul(K3_64), -21).wrapping_add(v0);
            hash = hash.wrapping_add(v1);
            buffer = buffer[16..].to_vec();
        }
        if buffer.len() >= 8 {
            hash = hash.wrapping_add(le64(&buffer, 0).wrapping_mul(K3_64));
            buffer = buffer[8..].to_vec();
            hash ^= rotate_left64(hash, -55).wrapping_mul(K1_64);
        }
        if buffer.len() >= 4 {
            hash = hash.wrapping_add((le32(&buffer) as i64 as u64).wrapping_mul(K3_64));
            hash ^= rotate_left64(hash, -26).wrapping_mul(K1_64);
            buffer = buffer[4..].to_vec();
        }
        if buffer.len() >= 2 {
            hash = hash.wrapping_add((le16(&buffer) as i64 as u64).wrapping_mul(K3_64));
            buffer = buffer[2..].to_vec();
            hash ^= rotate_left64(hash, -48).wrapping_mul(K1_64);
        }
        if !buffer.is_empty() {
            hash = hash.wrapping_add((buffer[0] as i8 as i64 as u64).wrapping_mul(K3_64));
            hash ^= rotate_left64(hash, -38).wrapping_mul(K1_64);
        }
        hash ^= rotate_left64(hash, -28);
        hash = hash.wrapping_mul(K0_64);
        hash ^= rotate_left64(hash, -29);
        hash as i64
    }

    /// 对齐 Java: `MetroHash.hash128(byte[])`
    pub fn hash128(data: &[u8]) -> Number128 {
        Self::hash128_seeded(data, 1337)
    }

    /// 对齐 Java: `MetroHash.hash128(byte[], long)`
    pub fn hash128_seeded(data: &[u8], seed: i64) -> Number128 {
        let mut buffer = data.to_vec();
        let mut v0 = (seed as u64).wrapping_sub(K0_128).wrapping_mul(K3_128);
        let mut v1 = (seed as u64).wrapping_add(K1_128).wrapping_mul(K2_128);
        let mut v2;
        let mut v3;

        if buffer.len() >= 32 {
            v2 = (seed as u64).wrapping_add(K0_128).wrapping_mul(K2_128);
            v3 = (seed as u64).wrapping_sub(K1_128).wrapping_mul(K3_128);
            while buffer.len() >= 32 {
                v0 = v0.wrapping_add(le64(&buffer, 0).wrapping_mul(K0_128));
                buffer = buffer[8..].to_vec();
                v0 = rotate_right(v0, 29).wrapping_add(v2);
                v1 = v1.wrapping_add(le64(&buffer, 0).wrapping_mul(K1_128));
                buffer = buffer[8..].to_vec();
                v1 = rotate_right(v1, 29).wrapping_add(v3);
                v2 = v2.wrapping_add(le64(&buffer, 0).wrapping_mul(K2_128));
                buffer = buffer[8..].to_vec();
                v2 = rotate_right(v2, 29).wrapping_add(v0);
                v3 = le64(&buffer, 0).wrapping_mul(K3_128);
                buffer = buffer[8..].to_vec();
                v3 = rotate_right(v3, 29).wrapping_add(v1);
            }
            v2 ^= rotate_right(v0.wrapping_add(v3).wrapping_mul(K0_128).wrapping_add(v1), 21)
                .wrapping_mul(K1_128);
            v3 ^= rotate_right(v1.wrapping_add(v2).wrapping_mul(K1_128).wrapping_add(v0), 21)
                .wrapping_mul(K0_128);
            v0 ^= rotate_right(v0.wrapping_add(v2).wrapping_mul(K0_128).wrapping_add(v3), 21)
                .wrapping_mul(K1_128);
            v1 ^= rotate_right(v1.wrapping_add(v3).wrapping_mul(K1_128).wrapping_add(v2), 21)
                .wrapping_mul(K0_128);
        }

        if buffer.len() >= 16 {
            v0 = v0.wrapping_add(le64(&buffer, 0).wrapping_mul(K2_128));
            buffer = buffer[8..].to_vec();
            v0 = rotate_right(v0, 33).wrapping_mul(K3_128);
            v1 = v1.wrapping_add(le64(&buffer, 0).wrapping_mul(K2_128));
            buffer = buffer[8..].to_vec();
            v1 = rotate_right(v1, 33).wrapping_mul(K3_128);
            v0 ^= rotate_right(v0.wrapping_mul(K2_128).wrapping_add(v1), 45).wrapping_add(K1_128);
            v1 ^= rotate_right(v1.wrapping_mul(K3_128).wrapping_add(v0), 45).wrapping_add(K0_128);
        }
        if buffer.len() >= 8 {
            v0 = v0.wrapping_add(le64(&buffer, 0).wrapping_mul(K2_128));
            buffer = buffer[8..].to_vec();
            v0 = rotate_right(v0, 33).wrapping_mul(K3_128);
            v0 ^= rotate_right(v0.wrapping_mul(K2_128).wrapping_add(v1), 27).wrapping_mul(K1_128);
        }
        if buffer.len() >= 4 {
            v1 = v1.wrapping_add((le32(&buffer) as i64 as u64).wrapping_mul(K2_128));
            buffer = buffer[4..].to_vec();
            v1 = rotate_right(v1, 33).wrapping_mul(K3_128);
            v1 ^= rotate_right(v1.wrapping_mul(K3_128).wrapping_add(v0), 46).wrapping_mul(K0_128);
        }
        if buffer.len() >= 2 {
            v0 = v0.wrapping_add((le16(&buffer) as i64 as u64).wrapping_mul(K2_128));
            buffer = buffer[2..].to_vec();
            v0 = rotate_right(v0, 33).wrapping_mul(K3_128);
            // Hutool 源码为 `(v0 * k2_128) * v1`（非加号）
            v0 ^= rotate_right(v0.wrapping_mul(K2_128).wrapping_mul(v1), 22).wrapping_mul(K1_128);
        }
        if !buffer.is_empty() {
            v1 = v1.wrapping_add((buffer[0] as i8 as i64 as u64).wrapping_mul(K2_128));
            v1 = rotate_right(v1, 33).wrapping_mul(K3_128);
            v1 ^= rotate_right(v1.wrapping_mul(K3_128).wrapping_add(v0), 58).wrapping_mul(K0_128);
        }

        v0 = v0.wrapping_add(rotate_right(v0.wrapping_mul(K0_128).wrapping_add(v1), 13));
        v1 = v1.wrapping_add(rotate_right(v1.wrapping_mul(K1_128).wrapping_add(v0), 37));
        v0 = v0.wrapping_add(rotate_right(v0.wrapping_mul(K2_128).wrapping_add(v1), 13));
        v1 = v1.wrapping_add(rotate_right(v1.wrapping_mul(K3_128).wrapping_add(v0), 37));

        // Number128(v0, v1) => low=v0, high=v1
        Number128 {
            low: v0 as i64,
            high: v1 as i64,
        }
    }
}

fn le64(buf: &[u8], off: usize) -> u64 {
    let mut b = [0u8; 8];
    let n = (buf.len().saturating_sub(off)).min(8);
    b[..n].copy_from_slice(&buf[off..off + n]);
    u64::from_le_bytes(b)
}

fn le32(buf: &[u8]) -> i32 {
    let b0 = buf[0] as i8 as i32;
    let b1 = buf[1] as i8 as i32;
    let b2 = buf[2] as i8 as i32;
    let b3 = buf[3] as i8 as i32;
    b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
}

const K2_128: u64 = 0x7bde_c03b;

fn rotate_right(v: u64, shift: i32) -> u64 {
    let shift = shift as u32;
    ((v as i64) >> shift) as u64 | (v << (64 - shift))
}

fn le16(buf: &[u8]) -> i16 {
    i16::from_le_bytes([buf[0], buf[1]])
}

const K0_128: u64 = 0xffff_ffff_c83a_91e1;

const K1_128: u64 = 0xffff_ffff_8648_dbdb;

const K1_64: u64 = 0xffff_ffff_a2aa_033b;

const K0_64: u64 = 0xffff_ffff_d6d0_18f5;

const K3_128: u64 = 0x2f58_70a5;

const K2_64: u64 = 0x6299_2fc1;

fn rotate_left64(v: u64, k: i32) -> u64 {
    let s = (k as u32) & 63;
    let left = v << s;
    let right = ((v as i64) >> (64 - s)) as u64;
    left | right
}

const K3_64: u64 = 0x30bc_5b29;
