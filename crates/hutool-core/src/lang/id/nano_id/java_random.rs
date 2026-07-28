//! 对齐: `cn.hutool.core.lang.id.NanoId`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/id/NanoId.java

use rand::Rng;

use super::nano_id::NanoId;

/// Java `java.util.Random` LCG，用于种子可复现测试向量。
#[derive(Debug, Clone)]
pub struct JavaRandom {
    seed: u64,
}

impl JavaRandom {
    /// 对齐 `new Random(seed)`
    pub fn new(seed: i64) -> Self {
        Self {
            seed: (seed as u64 ^ 0x5DEECE66D) & ((1u64 << 48) - 1),
        }
    }

    fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(0x5DEECE66D).wrapping_add(0xB)) & ((1u64 << 48) - 1);
        (self.seed >> (48 - bits)) as i32
    }

    /// 对齐 `Random.nextBytes`
    pub fn next_bytes(&mut self, bytes: &mut [u8]) {
        let mut i = 0;
        while i < bytes.len() {
            let mut rnd = self.next(32);
            let mut n = (bytes.len() - i).min(4);
            while n > 0 {
                bytes[i] = rnd as u8;
                i += 1;
                n -= 1;
                rnd >>= 8;
            }
        }
    }
}

impl NanoId {
    /// 对齐 Java: `NanoId.randomNanoId()`
    pub fn random_nano_id() -> String {
        Self::random_nano_id_size(DEFAULT_SIZE)
    }

    /// 对齐 Java: `NanoId.randomNanoId(int size)`
    pub fn random_nano_id_size(size: usize) -> String {
        Self::random_nano_id_with(None, None, size).expect("valid default alphabet/size")
    }

    /// 对齐 Java: `NanoId.randomNanoId(Random, char[], int)`
    ///
    /// `rng_bytes`: 若提供则使用该 RNG 的 `next_bytes`；否则使用 `rand` 安全随机。
    pub fn random_nano_id_with(
        mut java_rng: Option<&mut JavaRandom>,
        alphabet: Option<&[char]>,
        size: usize,
    ) -> Result<String, String> {
        let default_alpha: Vec<char> = DEFAULT_ALPHABET.iter().map(|&b| b as char).collect();
        let alphabet = alphabet.unwrap_or(&default_alpha);
        if alphabet.is_empty() || alphabet.len() >= 256 {
            return Err("Alphabet must contain between 1 and 255 symbols.".into());
        }
        if size == 0 {
            return Err("Size must be greater than zero.".into());
        }
        // 对齐 Hutool/JS：alphabet.len()==1 时 log2(0) 视为移位 0 → mask=1
        let mask = if alphabet.len() <= 1 {
            1usize
        } else {
            (2usize << (((alphabet.len() - 1) as f64).log2().floor() as u32)) - 1
        };
        let step = ((1.6 * mask as f64 * size as f64 / alphabet.len() as f64).ceil() as usize).max(1);
        let mut id = String::with_capacity(size);
        let mut count = 0usize;
        loop {
            let mut bytes = vec![0u8; step];
            if let Some(rng) = java_rng.as_mut() {
                rng.next_bytes(&mut bytes);
            } else {
                rand::rng().fill_bytes(&mut bytes);
            }
            for b in bytes {
                let alphabet_index = (b as usize) & mask;
                if alphabet_index < alphabet.len() {
                    id.push(alphabet[alphabet_index]);
                    count += 1;
                    if count == size {
                        return Ok(id);
                    }
                }
            }
        }
    }
}

use super::{DEFAULT_ALPHABET, DEFAULT_SIZE};
