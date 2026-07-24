//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

use super::column_index_info::ColumnIndexInfo;

/// 对齐 Hutool `IndexInfo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexInfo {
    non_unique: bool,
    index_name: String,
    table_name: String,
    schema: String,
    catalog: String,
    column_index_info_list: Vec<ColumnIndexInfo>,
}

impl IndexInfo {
    /// 对齐 Java 构造。
    #[must_use]
    pub fn new(index_name: impl Into<String>, table_name: impl Into<String>) -> Self {
        Self {
            non_unique: true,
            index_name: index_name.into(),
            table_name: table_name.into(),
            schema: String::new(),
            catalog: String::new(),
            column_index_info_list: Vec::new(),
        }
    }

    /// 对齐 Java: `isNonUnique`。
    #[must_use]
    pub fn is_non_unique(&self) -> bool {
        self.non_unique
    }

    /// 设置非唯一。
    pub fn set_non_unique(&mut self, non_unique: bool) -> &mut Self {
        self.non_unique = non_unique;
        self
    }

    /// 对齐 Java: `getIndexName`。
    #[must_use]
    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    /// 设置索引名。
    pub fn set_index_name(&mut self, index_name: impl Into<String>) -> &mut Self {
        self.index_name = index_name.into();
        self
    }

    /// 对齐 Java: `getTableName`。
    #[must_use]
    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    /// 设置表名。
    pub fn set_table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
        self.table_name = table_name.into();
        self
    }

    /// 对齐 Java: `getSchema`。
    #[must_use]
    pub fn schema(&self) -> &str {
        &self.schema
    }

    /// 设置 schema。
    pub fn set_schema(&mut self, schema: impl Into<String>) -> &mut Self {
        self.schema = schema.into();
        self
    }

    /// 对齐 Java: `getCatalog`。
    #[must_use]
    pub fn catalog(&self) -> &str {
        &self.catalog
    }

    /// 设置 catalog。
    pub fn set_catalog(&mut self, catalog: impl Into<String>) -> &mut Self {
        self.catalog = catalog.into();
        self
    }

    /// 对齐 Java: `getColumnIndexInfoList`。
    #[must_use]
    pub fn column_index_info_list(&self) -> &[ColumnIndexInfo] {
        &self.column_index_info_list
    }

    /// 设置列索引信息。
    pub fn set_column_index_info_list(&mut self, list: Vec<ColumnIndexInfo>) -> &mut Self {
        self.column_index_info_list = list;
        self
    }
}

impl fmt::Display for IndexInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} on {}", self.index_name, self.table_name)
    }
}
