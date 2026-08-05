//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil.DoubleAdder`
/// 双精度累加器
use parking_lot::Mutex;

/// Concurrent floating-point adder equivalent to Java's `DoubleAdder` result branch.
#[derive(Debug, Default)]
pub struct DoubleAdder {
    value: Mutex<f64>,
}

impl DoubleAdder {
    /// Creates an adder with `value` as its initial sum.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self {
            value: Mutex::new(value),
        }
    }

    /// Adds `value` under a non-poisoning lock.
    pub fn add(&self, value: f64) {
        *self.value.lock() += value;
    }

    /// Returns the current sum.
    #[must_use]
    pub fn sum(&self) -> f64 {
        *self.value.lock()
    }
}
