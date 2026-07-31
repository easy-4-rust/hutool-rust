//! 对齐: `cn.hutool.core.lang.id.IdConstants`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/id/IdConstants.java

#![allow(dead_code)] // 对齐 Java IdConstants，暂未接线，预留

use crate::lang::snowflake::{Snowflake, MAX_DATA_CENTER_ID, MAX_WORKER_ID};
use std::sync::OnceLock;

/// 对齐 Java: `IdConstants.DEFAULT_DATACENTER_ID` — 由进程号派生并截断到合法范围。
pub fn default_datacenter_id() -> i64 {
    (std::process::id() as i64) & MAX_DATA_CENTER_ID
}

/// 对齐 Java: `IdConstants.DEFAULT_WORKER_ID`
pub fn default_worker_id() -> i64 {
    ((std::process::id() as i64) >> 5) & MAX_WORKER_ID
}

/// 对齐 Java: `cn.hutool.core.lang.id.IdConstants`
#[derive(Debug, Clone, Copy, Default)]
pub struct IdConstants;

impl IdConstants {
    /// 默认数据中心 ID。
    #[must_use]
    pub fn default_datacenter_id() -> i64 {
        default_datacenter_id()
    }

    /// 默认 Worker ID。
    #[must_use]
    pub fn default_worker_id() -> i64 {
        default_worker_id()
    }

    /// 对齐 Java: `DEFAULT_SNOWFLAKE` 单例。
    pub fn default_snowflake() -> &'static Snowflake {
        static SF: OnceLock<Snowflake> = OnceLock::new();
        SF.get_or_init(|| Snowflake::new(default_worker_id(), default_datacenter_id()))
    }
}

#[cfg(test)]
mod id_constants_idiomatic_parity {
    use super::*;

    #[test]
    fn id_constants_default_snowflake() {
        assert!((0..=MAX_DATA_CENTER_ID).contains(&IdConstants::default_datacenter_id()));
        assert!((0..=MAX_WORKER_ID).contains(&IdConstants::default_worker_id()));
        let id = IdConstants::default_snowflake().next_id();
        assert!(id > 0);
    }
}
