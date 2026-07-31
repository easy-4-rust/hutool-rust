//! 对齐: `cn.hutool.core.lang.hash.KetamaHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/hash/KetamaHash.java

#![allow(dead_code)] // 对齐 Java KetamaHash，暂未接线，预留

use ::md5::{Digest, Md5};

/// 对齐 Java: `cn.hutool.core.lang.hash.KetamaHash`
#[derive(Debug, Clone, Copy, Default)]
pub struct KetamaHash;

impl KetamaHash {
    /// UTF-8 MD5 摘要。
    fn md5(key: &str) -> [u8; 16] {
        let mut hasher = Md5::new();
        hasher.update(key.as_bytes());
        hasher.finalize().into()
    }

    /// 对齐 Java: `hash64(String)`
    pub fn hash64(key: &str) -> i64 {
        let b = Self::md5(key);
        ((b[3] as i64 & 0xff) << 24)
            | ((b[2] as i64 & 0xff) << 16)
            | ((b[1] as i64 & 0xff) << 8)
            | (b[0] as i64 & 0xff)
    }

    /// 对齐 Java: `hash32(String)`
    pub fn hash32(key: &str) -> i32 {
        (Self::hash64(key) & 0xffff_ffff) as i32
    }

    /// 对齐 Java: `hash(String)` → 返回 hash64。
    pub fn hash(key: &str) -> i64 {
        Self::hash64(key)
    }
}

#[cfg(test)]
mod ketama_hash_idiomatic_parity {
    use super::*;

    #[test]
    fn ketama_hash32_matches_hash64_low_bits() {
        let h64 = KetamaHash::hash64("node-1");
        assert_eq!(KetamaHash::hash32("node-1"), (h64 & 0xffff_ffff) as i32);
        assert_eq!(KetamaHash::hash("node-1"), h64);
    }
}
