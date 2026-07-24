//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

use super::jdbc_type::JdbcType;

/// 对齐 Hutool `Column`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    table_name: String,
    name: String,
    type_code: i32,
    type_name: String,
    size: i32,
    digit: i32,
    nullable: bool,
    comment: String,
    auto_increment: bool,
    pk: bool,
    column_def: String,
}

impl Column {
    /// 对齐 Java: `Column.create(String tableName, String columnName)`。
    #[must_use]
    pub fn create(table_name: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            name: name.into(),
            type_code: 0,
            type_name: String::new(),
            size: 0,
            digit: 0,
            nullable: true,
            comment: String::new(),
            auto_increment: false,
            pk: false,
            column_def: String::new(),
        }
    }

    /// 对齐 Java: `init` 风格批量设置。
    pub fn init(
        &mut self,
        type_name: impl Into<String>,
        size: i32,
        nullable: bool,
        pk: bool,
    ) -> &mut Self {
        self.type_name = type_name.into();
        self.type_code = match JdbcType::from_type_name(&self.type_name) {
            JdbcType::Varchar => 12,
            JdbcType::Integer => 4,
            JdbcType::BigInt => -5,
            JdbcType::Decimal => 3,
            JdbcType::Timestamp => 93,
            JdbcType::Boolean => 16,
            JdbcType::Other => 1111,
        };
        self.size = size;
        self.nullable = nullable;
        self.pk = pk;
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

    /// 对齐 Java: `getName`。
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 设置列名。
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }

    /// 对齐 Java: `getTypeEnum`。
    #[must_use]
    pub fn type_enum(&self) -> JdbcType {
        JdbcType::from_type_name(&self.type_name)
    }

    /// 对齐 Java: `getType`。
    #[must_use]
    pub fn type_code(&self) -> i32 {
        self.type_code
    }

    /// 设置 JDBC 类型码。
    pub fn set_type(&mut self, type_code: i32) -> &mut Self {
        self.type_code = type_code;
        self
    }

    /// 对齐 Java: `getTypeName`。
    #[must_use]
    pub fn type_name(&self) -> &str {
        &self.type_name
    }

    /// 设置类型名。
    pub fn set_type_name(&mut self, type_name: impl Into<String>) -> &mut Self {
        self.type_name = type_name.into();
        self
    }

    /// 对齐 Java: `getSize`。
    #[must_use]
    pub fn size(&self) -> i32 {
        self.size
    }

    /// 设置大小。
    pub fn set_size(&mut self, size: i32) -> &mut Self {
        self.size = size;
        self
    }

    /// 对齐 Java: `getDigit`。
    #[must_use]
    pub fn digit(&self) -> i32 {
        self.digit
    }

    /// 设置小数位。
    pub fn set_digit(&mut self, digit: i32) -> &mut Self {
        self.digit = digit;
        self
    }

    /// 对齐 Java: `isNullable`。
    #[must_use]
    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    /// 设置可空。
    pub fn set_nullable(&mut self, nullable: bool) -> &mut Self {
        self.nullable = nullable;
        self
    }

    /// 对齐 Java: `getComment`。
    #[must_use]
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// 设置注释。
    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.comment = comment.into();
        self
    }

    /// 对齐 Java: `isAutoIncrement`。
    #[must_use]
    pub fn is_auto_increment(&self) -> bool {
        self.auto_increment
    }

    /// 设置自增。
    pub fn set_auto_increment(&mut self, auto_increment: bool) -> &mut Self {
        self.auto_increment = auto_increment;
        self
    }

    /// 对齐 Java: `isPk`。
    #[must_use]
    pub fn is_pk(&self) -> bool {
        self.pk
    }

    /// 设置主键。
    pub fn set_pk(&mut self, pk: bool) -> &mut Self {
        self.pk = pk;
        self
    }

    /// 对齐 Java: `getColumnDef`。
    #[must_use]
    pub fn column_def(&self) -> &str {
        &self.column_def
    }

    /// 设置默认值定义。
    pub fn set_column_def(&mut self, column_def: impl Into<String>) -> &mut Self {
        self.column_def = column_def.into();
        self
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.table_name, self.name, self.type_name)
    }
}
