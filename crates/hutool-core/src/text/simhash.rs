//! 对齐: `cn.hutool.core.text.Simhash`

use std::collections::HashSet;

/// 对齐 Java: `Simhash`
#[derive(Debug, Default)]
pub struct Simhash {
    stored: HashSet<u64>,
    hamming_thresh: u32,
}

impl Simhash {
    /// 默认构造
    pub fn new() -> Self {
        Self {
            stored: HashSet::new(),
            hamming_thresh: 3,
        }
    }

    /// 带参数
    pub fn with_params(frac_count: usize, hamming_thresh: u32) -> Self {
        let _ = frac_count;
        Self {
            stored: HashSet::new(),
            hamming_thresh,
        }
    }

    /// 对齐 `hash(Collection)`
    pub fn hash(&self, segs: &[&str]) -> u64 {
        // 简化 simhash：对各 token 的哈希按位投票
        let mut bits = [0i32; 64];
        for s in segs {
            let mut h: u64 = 0xcbf29ce484222325;
            for b in s.as_bytes() {
                h ^= *b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
            for i in 0..64 {
                if (h >> i) & 1 == 1 {
                    bits[i] += 1;
                } else {
                    bits[i] -= 1;
                }
            }
        }
        let mut out = 0u64;
        for i in 0..64 {
            if bits[i] > 0 {
                out |= 1u64 << i;
            }
        }
        out
    }

    /// 对齐 `store`
    pub fn store(&mut self, simhash: u64) {
        self.stored.insert(simhash);
    }

    /// 对齐 `equals(segList)` — 与已存指纹比较汉明距离
    pub fn equals_segs(&self, segs: &[&str]) -> bool {
        let h = self.hash(segs);
        self.stored
            .iter()
            .any(|s| hamming(*s, h) <= self.hamming_thresh)
    }
}

fn hamming(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
