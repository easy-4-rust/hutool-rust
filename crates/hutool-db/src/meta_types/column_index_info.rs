//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

/// 对齐 Hutool `ColumnIndexInfo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnIndexInfo {
    column_name: String,
    asc_or_desc: String,
}

impl ColumnIndexInfo {
    /// 对齐 Java: `create`。
    #[must_use]
    pub fn create(column_name: impl Into<String>, asc_or_desc: impl Into<String>) -> Self {
        Self {
            column_name: column_name.into(),
            asc_or_desc: asc_or_desc.into(),
        }
    }

    /// 对齐 Java: `getColumnName`。
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// 设置列名。
    pub fn set_column_name(&mut self, column_name: impl Into<String>) -> &mut Self {
        self.column_name = column_name.into();
        self
    }

    /// 对齐 Java: `getAscOrDesc`。
    #[must_use]
    pub fn asc_or_desc(&self) -> &str {
        &self.asc_or_desc
    }

    /// 设置升降序。
    pub fn set_asc_or_desc(&mut self, asc_or_desc: impl Into<String>) -> &mut Self {
        self.asc_or_desc = asc_or_desc.into();
        self
    }
}

impl fmt::Display for ColumnIndexInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.column_name, self.asc_or_desc)
    }
}
