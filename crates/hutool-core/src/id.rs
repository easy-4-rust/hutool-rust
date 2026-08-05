//! 对齐: `cn.hutool.core.util.IdUtil`（门面层）
//!
//! UUID / ObjectId / Snowflake 等 ID 生成入口；底层引擎在 `lang::{ObjectId, Snowflake}`。

use crate::lang::{ObjectId, Snowflake};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use uuid::Uuid;

type SnowflakeKey = (i64, i64);

static SNOWFLAKE_SINGLETONS: OnceLock<Mutex<HashMap<SnowflakeKey, Arc<Snowflake>>>> =
    OnceLock::new();

fn snowflake_cache() -> &'static Mutex<HashMap<SnowflakeKey, Arc<Snowflake>>> {
    SNOWFLAKE_SINGLETONS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 对齐 Java: `cn.hutool.core.util.IdUtil`
pub struct IdUtil;

impl IdUtil {
    /// 创建带连字符的随机 UUID v4。
    #[must_use]
    pub fn uuid() -> String {
        Uuid::new_v4().to_string()
    }

    /// 创建无连字符的随机 UUID v4。
    #[must_use]
    pub fn simple_uuid() -> String {
        Uuid::new_v4().simple().to_string()
    }

    /// 校验 UUID 字符串是否合法。
    #[must_use]
    pub fn is_valid(value: &str) -> bool {
        Uuid::parse_str(value).is_ok()
    }

    /// 对齐 Java: `IdUtil.objectId()` → `ObjectId.next()`
    #[must_use]
    pub fn object_id() -> String {
        ObjectId::next()
    }

    /// 对齐 Java: `IdUtil.createSnowflake(long, long)` — 每次新建实例（非单例）。
    #[must_use]
    pub fn create_snowflake(worker_id: i64, data_center_id: i64) -> Snowflake {
        Snowflake::new(worker_id, data_center_id)
    }

    /// 对齐 Java: `IdUtil.getSnowflake(long workerId, long datacenterId)` — 按 (worker, dc) 单例。
    #[must_use]
    pub fn get_snowflake(worker_id: i64, data_center_id: i64) -> Arc<Snowflake> {
        let mut cache = snowflake_cache().lock();
        cache
            .entry((worker_id, data_center_id))
            .or_insert_with(|| Arc::new(Snowflake::new(worker_id, data_center_id)))
            .clone()
    }

    /// 对齐 Java: `IdUtil.getSnowflakeNextId()`
    pub fn get_snowflake_next_id() -> i64 {
        Self::get_snowflake(0, 0).next_id()
    }

    /// 对齐 Java: `IdUtil.getSnowflakeNextIdStr()`
    #[must_use]
    pub fn get_snowflake_next_id_str() -> String {
        Self::get_snowflake(0, 0).next_id_str()
    }

    /// 对齐 Java: `IdUtil.getDataCenterId(long maxDatacenterId)` — MAC 末字节推导；无 MAC 时返回 1。
    pub fn get_data_center_id(max_datacenter_id: i64) -> i64 {
        assert!(max_datacenter_id > 0, "maxDatacenterId must be > 0");
        let mut max = max_datacenter_id;
        if max == i64::MAX {
            max -= 1;
        }
        let mut id = 1_i64;
        if let Some(mac) = local_hardware_address() {
            let len = mac.len();
            if len >= 2 {
                let raw = (0x0000_00FF_i64 & mac[len - 2] as i64)
                    | (0x0000_FF00_i64 & ((mac[len - 1] as i64) << 8));
                id = (raw >> 6) % (max + 1);
            }
        }
        id
    }

    /// 对齐 Java: `IdUtil.getWorkerId(long datacenterId, long maxWorkerId)` — PID + datacenter 哈希。
    pub fn get_worker_id(datacenter_id: i64, max_worker_id: i64) -> i64 {
        assert!(max_worker_id > 0, "maxWorkerId must be > 0");
        let pid = std::process::id();
        let key = format!("{datacenter_id}{pid}");
        let hash = java_string_hash(&key);
        (hash & 0xffff) as i64 % (max_worker_id + 1)
    }
}

/// 尝试读取本机 MAC（对齐 `NetUtil.getLocalHardwareAddress`）；失败返回 `None`。
fn local_hardware_address() -> Option<Vec<u8>> {
    // 无 unsafe 时暂用平台命令；完整实现待 `NetUtil` 导出后委托。
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let out = Command::new("ifconfig").arg("en0").output().ok()?;
        if !out.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("ether ") {
                let mac: Vec<u8> = rest
                    .split(':')
                    .filter_map(|p| u8::from_str_radix(p, 16).ok())
                    .collect();
                if mac.len() == 6 {
                    return Some(mac);
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

/// Java `String.hashCode()`（用于 workerId 推导）。
fn java_string_hash(s: &str) -> i32 {
    let mut hash: i32 = 0;
    for ch in s.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(ch as i32);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_identifiers_are_valid_and_unique() {
        let first = IdUtil::uuid();
        let second = IdUtil::uuid();
        assert!(IdUtil::is_valid(&first));
        assert_ne!(first, second);
        assert_eq!(IdUtil::simple_uuid().len(), 32);
    }

    #[test]
    fn object_id_length_is_24() {
        assert_eq!(IdUtil::object_id().len(), 24);
    }

    #[test]
    fn snowflake_singleton_returns_positive_id() {
        let id = IdUtil::get_snowflake(1, 1).next_id();
        assert!(id > 0);
    }
}
