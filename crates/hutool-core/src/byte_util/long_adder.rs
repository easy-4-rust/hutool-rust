//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil.LongAdder`
/// 长整型累加器
use std::sync::atomic::{AtomicI64, Ordering};

/// Concurrent integer adder equivalent to Java's `LongAdder` result branch.
#[derive(Debug, Default)]
pub struct LongAdder {
    value: AtomicI64,
}

impl LongAdder {
    /// Creates an adder with `value` as its initial sum.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self {
            value: AtomicI64::new(value),
        }
    }

    /// Atomically adds `value`.
    pub fn add(&self, value: i64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    /// Returns the current sum.
    #[must_use]
    pub fn sum(&self) -> i64 {
        self.value.load(Ordering::Relaxed)
    }
}
