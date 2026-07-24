//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

/// 对齐 Hutool `TableType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    /// 普通表。
    Table,
    /// 视图。
    View,
    /// 其他。
    Other,
}

impl TableType {
    /// 从 JDBC TABLE_TYPE 字符串解析。
    #[must_use]
    pub fn from_jdbc(name: &str) -> Self {
        match name.to_ascii_uppercase().as_str() {
            "TABLE" => Self::Table,
            "VIEW" => Self::View,
            _ => Self::Other,
        }
    }

    /// JDBC 名称。
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Table => "TABLE",
            Self::View => "VIEW",
            Self::Other => "OTHER",
        }
    }
}
