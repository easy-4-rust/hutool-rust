//! 对齐: `cn.hutool.core.lang.ObjectId`

use rand::RngExt;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static NEXT_INC: AtomicU32 = AtomicU32::new(0xA5A5A5A5);
static MACHINE: std::sync::OnceLock<u32> = std::sync::OnceLock::new();

/// 对齐 Java: `ObjectId`
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectId;

impl ObjectId {
    fn machine() -> u32 {
        *MACHINE.get_or_init(|| {
            let mut rng = rand::rng();
            let m: u32 = rng.random();
            let pid = std::process::id() & 0xffff;
            (m & 0xffff_0000) | pid
        })
    }

    /// 对齐 `ObjectId.isValid`
    #[allow(dead_code)] // 对齐 Java ObjectId.isValid，暂未接线，预留
    pub fn is_valid(s: &str) -> bool {
        let s: String = s.chars().filter(|c| *c != '-').collect();
        if s.len() != 24 {
            return false;
        }
        s.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// 对齐 `ObjectId.nextBytes`
    pub fn next_bytes() -> [u8; 12] {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as u32)
            .unwrap_or(0);
        let mut out = [0u8; 12];
        out[0..4].copy_from_slice(&secs.to_be_bytes());
        out[4..8].copy_from_slice(&Self::machine().to_be_bytes());
        let inc = NEXT_INC.fetch_add(1, Ordering::SeqCst);
        out[8..12].copy_from_slice(&inc.to_be_bytes());
        out
    }

    /// 对齐 `ObjectId.next()` — 无连字符 24 hex。
    pub fn next() -> String {
        Self::next_with_hyphen(false)
    }

    /// 对齐 `ObjectId.next(boolean withHyphen)`
    pub fn next_with_hyphen(with_hyphen: bool) -> String {
        let hex: String = Self::next_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        if !with_hyphen {
            return hex;
        }
        // 8-4-4-4-8 风格分组（与 Hutool 带连字符输出对齐）
        format!(
            "{}-{}-{}-{}-{}",
            &hex[0..8],
            &hex[8..12],
            &hex[12..16],
            &hex[16..20],
            &hex[20..24]
        )
    }
}

#[cfg(test)]
mod object_id_idiomatic_parity {
    use super::*;

    /// 对齐 Java ObjectId.isValid/next 可执行证据。
    #[test]
    fn object_id_valid_and_next() {
        let id = ObjectId::next();
        assert_eq!(id.len(), 24);
        assert!(ObjectId::is_valid(&id));
        let hy = ObjectId::next_with_hyphen(true);
        assert!(hy.contains('-'));
        assert!(ObjectId::is_valid(&hy));
        assert!(!ObjectId::is_valid("zz"));
    }
}

// init NEXT_INC randomly once
#[allow(dead_code)]
fn init_inc() {
    let v: u32 = rand::rng().random();
    NEXT_INC.store(v, Ordering::SeqCst);
}
