//! 对齐: `cn.hutool.core.lang.UUID`（fastUUID / randomUUID / fromString）
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/UUID.java

#![allow(dead_code)] // 对齐 Java UUID，暂未接线，预留

use std::cmp::Ordering;
use std::fmt;
use uuid::Uuid;

/// 对齐 Java: `cn.hutool.core.lang.UUID`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UUID {
    inner: Uuid,
}

impl UUID {
    /// 对齐 Java: `UUID(long mostSigBits, long leastSigBits)`
    #[must_use]
    pub fn from_bits(most_sig_bits: i64, least_sig_bits: i64) -> Self {
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&most_sig_bits.to_be_bytes());
        bytes[8..].copy_from_slice(&least_sig_bits.to_be_bytes());
        Self {
            inner: Uuid::from_bytes(bytes),
        }
    }

    /// 对齐 Java: `UUID.fastUUID()`
    #[must_use]
    pub fn fast_uuid() -> Self {
        Self {
            inner: Uuid::new_v4(),
        }
    }

    /// 对齐 Java: `UUID.randomUUID()` / `randomUUID(boolean isSecure)`
    #[must_use]
    pub fn random_uuid() -> Self {
        Self::fast_uuid()
    }

    /// 对齐 Java: `UUID.nameUUIDFromBytes(byte[])`
    #[must_use]
    pub fn name_uuid_from_bytes(name: &[u8]) -> Self {
        // Java 使用 MD5/v3；无 `v3`/`v5` feature 时用确定性字节填充作为可移植近似。
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        let h = hasher.finish();
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&h.to_be_bytes());
        bytes[8..].copy_from_slice(&(!h).to_be_bytes());
        // 设置 version/variant 位近似 RFC4122
        bytes[6] = (bytes[6] & 0x0f) | 0x30; // version 3
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Self {
            inner: Uuid::from_bytes(bytes),
        }
    }

    /// 对齐 Java: `UUID.fromString(String)`
    pub fn from_string(name: &str) -> Option<Self> {
        Uuid::parse_str(name).ok().map(|inner| Self { inner })
    }

    /// 对齐 Java: `getMostSignificantBits`
    #[must_use]
    pub fn get_most_significant_bits(&self) -> i64 {
        let b = self.inner.as_bytes();
        i64::from_be_bytes(b[..8].try_into().unwrap())
    }

    /// 对齐 Java: `getLeastSignificantBits`
    #[must_use]
    pub fn get_least_significant_bits(&self) -> i64 {
        let b = self.inner.as_bytes();
        i64::from_be_bytes(b[8..].try_into().unwrap())
    }

    /// 对齐 Java: `version`
    #[must_use]
    pub fn version(&self) -> i32 {
        self.inner.get_version_num() as i32
    }

    /// 对齐 Java: `variant`
    #[must_use]
    pub fn variant(&self) -> i32 {
        match self.inner.get_variant() {
            uuid::Variant::NCS => 0,
            uuid::Variant::RFC4122 => 2,
            uuid::Variant::Microsoft => 6,
            uuid::Variant::Future => 7,
            _ => 2,
        }
    }

    /// 对齐 Java: `timestamp`（仅 version 1）
    pub fn timestamp(&self) -> Option<u64> {
        self.inner.get_timestamp().map(|t| t.to_unix().0)
    }

    /// 对齐 Java: `clockSequence`（仅 version 1）
    pub fn clock_sequence(&self) -> Option<u16> {
        // uuid crate 不直接暴露 clock_seq；从字节提取 RFC4122 clock_seq
        if self.version() != 1 {
            return None;
        }
        let b = self.inner.as_bytes();
        Some(u16::from_be_bytes([b[8] & 0x3f, b[9]]))
    }

    /// 对齐 Java: `node`（仅 version 1）
    pub fn node(&self) -> Option<u64> {
        if self.version() != 1 {
            return None;
        }
        let b = self.inner.as_bytes();
        Some(
            ((b[10] as u64) << 40)
                | ((b[11] as u64) << 32)
                | ((b[12] as u64) << 24)
                | ((b[13] as u64) << 16)
                | ((b[14] as u64) << 8)
                | (b[15] as u64),
        )
    }

    /// 对齐 Java: `toString()` / `toString(boolean isSimple)`
    #[must_use]
    pub fn to_string_style(&self, is_simple: bool) -> String {
        if is_simple {
            self.inner.as_simple().to_string()
        } else {
            self.inner.to_string()
        }
    }

    /// 对齐 Java: `compareTo`
    pub fn compare_to(&self, other: &UUID) -> i32 {
        match self
            .get_most_significant_bits()
            .cmp(&other.get_most_significant_bits())
        {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => match self
                .get_least_significant_bits()
                .cmp(&other.get_least_significant_bits())
            {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            },
        }
    }
}

impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Default for UUID {
    fn default() -> Self {
        Self::fast_uuid()
    }
}

#[cfg(test)]
mod uuid_idiomatic_parity {
    use super::*;

    /// 对齐 Java UUID 构造/解析/位运算可执行证据。
    #[test]
    fn uuid_bits_from_string_and_compare() {
        let u = UUID::from_bits(0x1234_5678_9abc_def0u64 as i64, 0x1111_2222_3333_4444);
        assert_eq!(
            u.get_most_significant_bits(),
            0x1234_5678_9abc_def0u64 as i64
        );
        assert_eq!(u.get_least_significant_bits(), 0x1111_2222_3333_4444);
        let s = u.to_string_style(false);
        let parsed = UUID::from_string(&s).expect("parse");
        assert_eq!(u, parsed);
        assert_eq!(u.compare_to(&parsed), 0);
        let simple = u.to_string_style(true);
        assert!(!simple.contains('-'));
        let r = UUID::random_uuid();
        assert_eq!(r.version(), 4);
        let named = UUID::name_uuid_from_bytes(b"hutool");
        assert_eq!(named.version(), 3);
    }
}
