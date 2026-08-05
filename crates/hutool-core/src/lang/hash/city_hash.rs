//! 对齐: `cn.hutool.core.lang.hash.CityHash`
//! hash32/64 使用 cityhasher crate；hash128 按 Java 私有实现移植，测试向量与 Hutool 一致。

use super::metro_hash::Number128;

const K0: i64 = 0xc3a5_c85c_97cb_3127_u64 as i64;
const K1: i64 = 0xb492_b66f_be98_f273_u64 as i64;
const K2: i64 = 0x9ae1_6a3b_2f90_404f_u64 as i64;
const K_MUL: i64 = 0x9ddf_ea08_eb38_2d69_u64 as i64;

/// 对齐 Java: `cn.hutool.core.lang.hash.CityHash`
#[derive(Debug, Clone, Copy, Default)]
pub struct CityHash;

impl CityHash {
    /// 对齐 Java: `CityHash.hash32(byte[])`
    pub fn hash32(data: &[u8]) -> i32 {
        cityhasher::hash::<u32>(data) as i32
    }

    /// 对齐 Java: `CityHash.hash64(byte[])`
    pub fn hash64(data: &[u8]) -> i64 {
        cityhasher::hash::<u64>(data) as i64
    }

    /// 对齐 Java: `CityHash.hash128(byte[])`
    pub fn hash128(data: &[u8]) -> Number128 {
        let len = data.len();
        if len >= 16 {
            hash128_from(
                data,
                16,
                Number128 {
                    low: fetch64(data, 0),
                    high: fetch64(data, 8).wrapping_add(K0),
                },
            )
        } else {
            hash128_from(data, 0, Number128 { low: K0, high: K1 })
        }
    }

    /// 对齐 Java: `CityHash.hash128(byte[], Number128 seed)`
    pub fn hash128_seeded(data: &[u8], seed: Number128) -> Number128 {
        hash128_from(data, 0, seed)
    }
}

/// 对齐 Java: `CityHash.hash128(byte[], int start, Number128 seed)`
fn hash128_from(data: &[u8], start: usize, seed: Number128) -> Number128 {
    let mut len = data.len().saturating_sub(start);
    if len < 128 {
        return city_murmur(&data[start..], seed);
    }

    let mut v = Number128 { low: 0, high: 0 };
    let mut w = Number128 { low: 0, high: 0 };
    let mut x = seed.low;
    let mut y = seed.high;
    let mut z = (len as i64).wrapping_mul(K1);
    v.low = rotate64(y ^ K1, 49).wrapping_mul(K1) + fetch64(data, start);
    v.high = rotate64(v.low, 42).wrapping_mul(K1) + fetch64(data, start + 8);
    w.low = rotate64(y.wrapping_add(z), 35).wrapping_mul(K1) + x;
    w.high = rotate64(x.wrapping_add(fetch64(data, start + 88)), 53).wrapping_mul(K1);

    let mut pos = start;
    while len >= 128 {
        x = rotate64(
            x.wrapping_add(y)
                .wrapping_add(v.low)
                .wrapping_add(fetch64(data, pos + 8)),
            37,
        )
        .wrapping_mul(K1);
        y = rotate64(
            y.wrapping_add(v.high).wrapping_add(fetch64(data, pos + 48)),
            42,
        )
        .wrapping_mul(K1);
        x ^= w.high;
        y = y.wrapping_add(v.low).wrapping_add(fetch64(data, pos + 40));
        z = rotate64(z.wrapping_add(w.low), 33).wrapping_mul(K1);
        v = weak_hash_len32_with_seeds(data, pos, v.high.wrapping_mul(K1), x.wrapping_add(w.low));
        w = weak_hash_len32_with_seeds(
            data,
            pos + 32,
            z.wrapping_add(w.high),
            y.wrapping_add(fetch64(data, pos + 16)),
        );

        let swap = x;
        x = z;
        z = swap;
        pos += 64;

        x = rotate64(
            x.wrapping_add(y)
                .wrapping_add(v.low)
                .wrapping_add(fetch64(data, pos + 8)),
            37,
        )
        .wrapping_mul(K1);
        y = rotate64(
            y.wrapping_add(v.high).wrapping_add(fetch64(data, pos + 48)),
            42,
        )
        .wrapping_mul(K1);
        x ^= w.high;
        y = y.wrapping_add(v.low).wrapping_add(fetch64(data, pos + 40));
        z = rotate64(z.wrapping_add(w.low), 33).wrapping_mul(K1);
        v = weak_hash_len32_with_seeds(data, pos, v.high.wrapping_mul(K1), x.wrapping_add(w.low));
        w = weak_hash_len32_with_seeds(
            data,
            pos + 32,
            z.wrapping_add(w.high),
            y.wrapping_add(fetch64(data, pos + 16)),
        );

        let swap = x;
        x = z;
        z = swap;
        pos += 64;
        len -= 128;
    }

    x = x.wrapping_add(rotate64(v.low.wrapping_add(z), 49).wrapping_mul(K0));
    y = y.wrapping_mul(K0).wrapping_add(rotate64(w.high, 37));
    z = z.wrapping_mul(K0).wrapping_add(rotate64(w.low, 27));
    w.low = w.low.wrapping_mul(9);
    v.low = v.low.wrapping_mul(K0);

    let mut tail_done = 0;
    while tail_done < len {
        tail_done += 32;
        y = rotate64(x.wrapping_add(y), 42)
            .wrapping_mul(K0)
            .wrapping_add(v.high);
        w.low = w
            .low
            .wrapping_add(fetch64(data, pos + len - tail_done + 16));
        x = x.wrapping_mul(K0).wrapping_add(w.low);
        z = z
            .wrapping_add(w.high)
            .wrapping_add(fetch64(data, pos + len - tail_done));
        w.high = w.high.wrapping_add(v.low);
        v = weak_hash_len32_with_seeds(data, pos + len - tail_done, v.low.wrapping_add(z), v.high);
        v.low = v.low.wrapping_mul(K0);
    }

    x = hash_len16_pair(x, v.low);
    y = hash_len16_pair(y.wrapping_add(z), w.low);
    Number128 {
        low: hash_len16_pair(x.wrapping_add(v.high), w.high).wrapping_add(y),
        high: hash_len16_pair(x.wrapping_add(w.high), y.wrapping_add(v.high)),
    }
}

/// 对齐 Java: `CityHash.cityMurmur(byte[], Number128)`
fn city_murmur(data: &[u8], seed: Number128) -> Number128 {
    let len = data.len();
    let mut a = seed.low;
    let mut b = seed.high;
    let mut l = len as i32 - 16;
    let (c, d) = if l <= 0 {
        a = shift_mix(a.wrapping_mul(K1)).wrapping_mul(K1);
        let c = b.wrapping_mul(K1).wrapping_add(hash_len0to16(data));
        let d = shift_mix(a.wrapping_add(if len >= 8 { fetch64(data, 0) } else { c }));
        (c, d)
    } else {
        let mut c = hash_len16_pair(fetch64(data, len - 8).wrapping_add(K1), a);
        let mut d = hash_len16_pair(
            b.wrapping_add(len as i64),
            c.wrapping_add(fetch64(data, len - 16)),
        );
        a = a.wrapping_add(d);
        let mut pos = 0;
        while l > 0 {
            a ^= shift_mix(fetch64(data, pos).wrapping_mul(K1)).wrapping_mul(K1);
            a = a.wrapping_mul(K1);
            b ^= a;
            c ^= shift_mix(fetch64(data, pos + 8).wrapping_mul(K1)).wrapping_mul(K1);
            c = c.wrapping_mul(K1);
            d ^= c;
            pos += 16;
            l -= 16;
        }
        (c, d)
    };
    a = hash_len16_pair(a, c);
    b = hash_len16_pair(d, b);
    Number128 {
        low: a ^ b,
        high: hash_len16_pair(b, a),
    }
}

fn weak_hash_len32_with_seeds(data: &[u8], start: usize, a: i64, b: i64) -> Number128 {
    weak_hash_len32_with_seeds_values(
        fetch64(data, start),
        fetch64(data, start + 8),
        fetch64(data, start + 16),
        fetch64(data, start + 24),
        a,
        b,
    )
}

fn weak_hash_len32_with_seeds_values(
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    mut a: i64,
    mut b: i64,
) -> Number128 {
    a = a.wrapping_add(w);
    b = rotate64(b.wrapping_add(a).wrapping_add(z), 21);
    let c = a;
    a = a.wrapping_add(x).wrapping_add(y);
    b = b.wrapping_add(rotate64(a, 44));
    Number128 {
        low: a.wrapping_add(z),
        high: b.wrapping_add(c),
    }
}

fn hash_len0to16(data: &[u8]) -> i64 {
    let len = data.len();
    if len >= 8 {
        let mul = K2.wrapping_add((len as i64).wrapping_mul(2));
        let a = fetch64(data, 0).wrapping_add(K2);
        let b = fetch64(data, len - 8);
        let c = rotate64(b, 37).wrapping_mul(mul).wrapping_add(a);
        let d = rotate64(a, 25).wrapping_add(b).wrapping_mul(mul);
        return hash_len16_mul(c, d, mul);
    }
    if len >= 4 {
        let mul = K2.wrapping_add((len as i64).wrapping_mul(2));
        let a = fetch32(data, 0) as i64 & 0xffff_ffff;
        return hash_len16_mul(
            (len as i64).wrapping_add(a << 3),
            fetch32(data, len - 4) as i64 & 0xffff_ffff,
            mul,
        );
    }
    if len > 0 {
        let a = i64::from(data[0] & 0xff);
        let b = i64::from(data[len >> 1] & 0xff);
        let c = i64::from(data[len - 1] & 0xff);
        let y = a.wrapping_add(b << 8);
        let z = (len as i64).wrapping_add(c << 2);
        return shift_mix(y.wrapping_mul(K2) ^ z.wrapping_mul(K0)).wrapping_mul(K2);
    }
    K2
}

fn hash_len16_mul(u: i64, v: i64, mul: i64) -> i64 {
    let mut a = (u ^ v).wrapping_mul(mul);
    a ^= unsigned_shift64(a, 47);
    let mut b = (v ^ a).wrapping_mul(mul);
    b ^= unsigned_shift64(b, 47);
    b.wrapping_mul(mul)
}

fn hash_len16_pair(u: i64, v: i64) -> i64 {
    hash128_to64(Number128 { low: u, high: v })
}

fn hash128_to64(number128: Number128) -> i64 {
    let mut a = (number128.low ^ number128.high).wrapping_mul(K_MUL);
    a ^= unsigned_shift64(a, 47);
    let mut b = (number128.high ^ a).wrapping_mul(K_MUL);
    b ^= unsigned_shift64(b, 47);
    b.wrapping_mul(K_MUL)
}

fn shift_mix(val: i64) -> i64 {
    val ^ unsigned_shift64(val, 47)
}

fn fetch64(data: &[u8], start: usize) -> i64 {
    let mut bytes = [0_u8; 8];
    let end = (start + 8).min(data.len());
    if start < data.len() {
        bytes[..end.saturating_sub(start)].copy_from_slice(&data[start..end]);
    }
    i64::from_le_bytes(bytes)
}

fn fetch32(data: &[u8], start: usize) -> i32 {
    let mut bytes = [0_u8; 4];
    let end = (start + 4).min(data.len());
    if start < data.len() {
        bytes[..end.saturating_sub(start)].copy_from_slice(&data[start..end]);
    }
    i32::from_le_bytes(bytes)
}

fn rotate64(val: i64, shift: i32) -> i64 {
    if shift == 0 {
        return val;
    }
    let shift = (shift as u32) & 63;
    let val = val as u64;
    ((val >> shift) | (val << (64 - shift))) as i64
}

fn unsigned_shift64(val: i64, bits: u32) -> i64 {
    ((val as u64) >> bits) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 对齐 Java: `HashUtilTest` cityHash 向量
    #[test]
    fn hash128_matches_hutool_vector() {
        let s = "Google发布的Hash计算算法：CityHash64 与 CityHash128";
        let hash = CityHash::hash128(s.as_bytes()).get_long_array();
        assert_eq!(hash[0], 0x5944_f1e7_88a1_8db0_u64 as i64);
        assert_eq!(hash[1], 0xc2f6_8d8b_2bf4_a5cf_u64 as i64);
    }
}
