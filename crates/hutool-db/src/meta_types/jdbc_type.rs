//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

/// 对齐 Hutool `JdbcType` 的常用子集。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JdbcType {
    /// VARCHAR。
    Varchar,
    /// INTEGER。
    Integer,
    /// BIGINT。
    BigInt,
    /// DECIMAL。
    Decimal,
    /// TIMESTAMP。
    Timestamp,
    /// BOOLEAN。
    Boolean,
    /// OTHER / 未知。
    Other,
}

impl JdbcType {
    /// 从类型名解析。
    #[must_use]
    pub fn from_type_name(name: &str) -> Self {
        match name.to_ascii_uppercase().as_str() {
            "VARCHAR" | "TEXT" | "CHAR" | "NVARCHAR" => Self::Varchar,
            "INT" | "INTEGER" | "SMALLINT" | "TINYINT" => Self::Integer,
            "BIGINT" => Self::BigInt,
            "DECIMAL" | "NUMERIC" | "REAL" | "FLOAT" | "DOUBLE" => Self::Decimal,
            "TIMESTAMP" | "DATETIME" | "DATE" | "TIME" => Self::Timestamp,
            "BOOLEAN" | "BOOL" | "BIT" => Self::Boolean,
            _ => Self::Other,
        }
    }
}
