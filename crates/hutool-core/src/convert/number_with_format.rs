//! 对齐: `cn.hutool.core.convert.NumberWithFormat`

#![allow(dead_code)]

/// 对齐 Java 类: `cn.hutool.core.convert.NumberWithFormat`
#[derive(Debug, Clone)]
pub struct NumberWithFormat {
    value: i64,
    format: Option<String>,
}

impl Default for NumberWithFormat {
    fn default() -> Self {
        Self {
            value: 0,
            format: None,
        }
    }
}

impl NumberWithFormat {
    /// 兼容 sentinel。
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    /// 对齐 Java 构造: `new NumberWithFormat(number, format)`
    pub fn new(value: i64, format: Option<String>) -> Self {
        Self { value, format }
    }

    /// 数值。
    pub fn value(&self) -> i64 {
        self.value
    }

    /// 格式。
    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }
}
