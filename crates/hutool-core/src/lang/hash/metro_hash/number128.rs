//! 对齐: `cn.hutool.core.lang.hash.MetroHash`
//! 常量按 Java `int` 字面量拓宽为 `long`（高位符号扩展）。

/// 对齐 Java: `Number128`（`getLongArray()` = `[low, high]`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number128 {
    /// 低位（数组下标 0）
    pub low: i64,
    /// 高位（数组下标 1）
    pub high: i64,
}

impl Number128 {
    /// 对齐 Java: `Number128(long lowValue, long highValue)`
    #[must_use]
    pub fn new(low: i64, high: i64) -> Self {
        Self { low, high }
    }

    /// 对齐 `getLowValue`
    #[must_use]
    pub fn get_low_value(&self) -> i64 {
        self.low
    }

    /// 对齐 `setLowValue`
    pub fn set_low_value(&mut self, low: i64) {
        self.low = low;
    }

    /// 对齐 `getHighValue`
    #[must_use]
    pub fn get_high_value(&self) -> i64 {
        self.high
    }

    /// 对齐 `setHighValue`
    pub fn set_high_value(&mut self, high: i64) {
        self.high = high;
    }

    /// 对齐 `getLongArray()` → `[low, high]`
    #[must_use]
    pub fn get_long_array(&self) -> [i64; 2] {
        [self.low, self.high]
    }

    /// 对齐 `intValue` — 取低位截断。
    #[must_use]
    pub fn int_value(&self) -> i32 {
        self.low as i32
    }

    /// 对齐 `longValue` — 低位。
    #[must_use]
    pub fn long_value(&self) -> i64 {
        self.low
    }

    /// 对齐 `floatValue`
    #[must_use]
    pub fn float_value(&self) -> f32 {
        self.low as f32
    }

    /// 对齐 `doubleValue`
    #[must_use]
    pub fn double_value(&self) -> f64 {
        self.low as f64
    }
}
