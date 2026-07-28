//! 对齐: `cn.hutool.core.lang.Snowflake`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Snowflake.java

use parking_lot::Mutex;
use rand::RngExt;
use std::time::{SystemTime, UNIX_EPOCH};

/// 默认起始时间（Thu, 04 Nov 2010 01:42:54 GMT）
pub const DEFAULT_TWEPOCH: i64 = 1_288_834_974_657;
/// 默认允许时钟回拨毫秒数
pub const DEFAULT_TIME_OFFSET: i64 = 2000;

const WORKER_ID_BITS: u64 = 5;
const DATA_CENTER_ID_BITS: u64 = 5;
const SEQUENCE_BITS: u64 = 12;
/// 最大 workerId（0~31）
pub const MAX_WORKER_ID: i64 = !(-1_i64 << WORKER_ID_BITS);
/// 最大 dataCenterId（0~31）
pub const MAX_DATA_CENTER_ID: i64 = !(-1_i64 << DATA_CENTER_ID_BITS);
const WORKER_ID_SHIFT: u64 = SEQUENCE_BITS;
const DATA_CENTER_ID_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS;
const TIMESTAMP_LEFT_SHIFT: u64 = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;
const SEQUENCE_MASK: i64 = !(-1_i64 << SEQUENCE_BITS);

struct SnowflakeInner {
    sequence: i64,
    last_timestamp: i64,
}

/// 对齐 Java: `cn.hutool.core.lang.Snowflake`
pub struct Snowflake {
    twepoch: i64,
    worker_id: i64,
    data_center_id: i64,
    time_offset: i64,
    random_sequence_limit: i64,
    inner: Mutex<SnowflakeInner>,
}

impl Snowflake {
    /// 对齐 Java: `Snowflake(long workerId, long dataCenterId)`
    pub fn new(worker_id: i64, data_center_id: i64) -> Self {
        Self::with_options(None, worker_id, data_center_id, DEFAULT_TIME_OFFSET, 0)
    }

    /// 对齐 Java: 完整构造（含随机序号上限）
    pub fn with_options(
        epoch_ms: Option<i64>,
        worker_id: i64,
        data_center_id: i64,
        time_offset: i64,
        random_sequence_limit: i64,
    ) -> Self {
        assert!((0..=MAX_WORKER_ID).contains(&worker_id), "workerId out of range");
        assert!(
            (0..=MAX_DATA_CENTER_ID).contains(&data_center_id),
            "dataCenterId out of range"
        );
        assert!(
            (0..=SEQUENCE_MASK).contains(&random_sequence_limit),
            "randomSequenceLimit out of range"
        );
        Self {
            twepoch: epoch_ms.unwrap_or(DEFAULT_TWEPOCH),
            worker_id,
            data_center_id,
            time_offset,
            random_sequence_limit,
            inner: Mutex::new(SnowflakeInner {
                sequence: 0,
                last_timestamp: -1,
            }),
        }
    }

    fn now_ms() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    }

    fn til_next_millis(last: i64) -> i64 {
        let mut ts = Self::now_ms();
        while ts <= last {
            ts = Self::now_ms();
        }
        ts
    }

    /// 对齐 Java: `Snowflake.nextId()`
    pub fn next_id(&self) -> i64 {
        let mut g = self.inner.lock();
        let mut timestamp = Self::now_ms();
        if timestamp < g.last_timestamp {
            if g.last_timestamp - timestamp < self.time_offset {
                timestamp = g.last_timestamp;
            } else {
                panic!(
                    "Clock moved backwards. Refusing to generate id for {}ms",
                    g.last_timestamp - timestamp
                );
            }
        }
        if timestamp == g.last_timestamp {
            let sequence = (g.sequence + 1) & SEQUENCE_MASK;
            if sequence == 0 {
                timestamp = Self::til_next_millis(g.last_timestamp);
            }
            g.sequence = sequence;
        } else if self.random_sequence_limit > 1 {
            g.sequence = rand::rng().random_range(0..self.random_sequence_limit);
        } else {
            g.sequence = 0;
        }
        g.last_timestamp = timestamp;
        ((timestamp - self.twepoch) << TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << DATA_CENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | g.sequence
    }

    /// 对齐 Java: `Snowflake.nextIdStr()`
    pub fn next_id_str(&self) -> String {
        self.next_id().to_string()
    }

    /// 对齐 Java: `Snowflake.getWorkerId(long id)`
    pub fn get_worker_id(&self, id: i64) -> i64 {
        (id >> WORKER_ID_SHIFT) & !(-1_i64 << WORKER_ID_BITS)
    }

    /// 对齐 Java: `Snowflake.getDataCenterId(long id)`
    pub fn get_data_center_id(&self, id: i64) -> i64 {
        (id >> DATA_CENTER_ID_SHIFT) & !(-1_i64 << DATA_CENTER_ID_BITS)
    }

    /// 对齐 Java: `Snowflake.getGenerateDateTime(long id)`
    pub fn get_generate_date_time(&self, id: i64) -> i64 {
        ((id >> TIMESTAMP_LEFT_SHIFT) & !(-1_i64 << 41)) + self.twepoch
    }

    /// 对齐 Java: `Snowflake.getIdScopeByTimestamp(long, long, boolean)`
    pub fn get_id_scope_by_timestamp(
        &self,
        timestamp_start: i64,
        timestamp_end: i64,
        ignore_center_and_worker: bool,
    ) -> (i64, i64) {
        let start_time_min_id = (timestamp_start - self.twepoch) << TIMESTAMP_LEFT_SHIFT;
        let end_time_min_id = (timestamp_end - self.twepoch) << TIMESTAMP_LEFT_SHIFT;
        if ignore_center_and_worker {
            let end_id = end_time_min_id | !(-1_i64 << TIMESTAMP_LEFT_SHIFT);
            (start_time_min_id, end_id)
        } else {
            let start_id = start_time_min_id
                | (self.data_center_id << DATA_CENTER_ID_SHIFT)
                | (self.worker_id << WORKER_ID_SHIFT);
            let end_id = end_time_min_id
                | (self.data_center_id << DATA_CENTER_ID_SHIFT)
                | (self.worker_id << WORKER_ID_SHIFT)
                | SEQUENCE_MASK;
            (start_id, end_id)
        }
    }
}

#[cfg(test)]
mod snowflake_idiomatic_parity {
    use super::*;

    /// 对齐 Java Snowflake nextId/解析字段可执行证据。
    #[test]
    fn snowflake_next_id_and_field_extract() {
        let sf = Snowflake::new(1, 2);
        let id = sf.next_id();
        assert!(id > 0);
        assert_eq!(sf.get_worker_id(id), 1);
        assert_eq!(sf.get_data_center_id(id), 2);
        assert!(sf.get_generate_date_time(id) > DEFAULT_TWEPOCH);
        let s = sf.next_id_str();
        assert!(!s.is_empty());
        let (lo, hi) = sf.get_id_scope_by_timestamp(DEFAULT_TWEPOCH + 1000, DEFAULT_TWEPOCH + 2000, true);
        assert!(hi >= lo);
    }
}
