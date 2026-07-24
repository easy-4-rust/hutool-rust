//! SQL 构建器 —— 对齐 Hutool `cn.hutool.db.sql.SqlBuilder`。

use crate::entity::Entity;
use crate::sql::condition::Condition;
use crate::sql::condition_builder::ConditionBuilder;
use crate::sql::formatter;
use crate::sql::order::Order;
use crate::wrapper::Wrapper;
use serde_json::Value;

use super::join::Join;

/// SQL 构建器。
#[derive(Debug, Clone, Default)]
pub struct SqlBuilder {
    sql: String,
    param_values: Vec<Value>,
    wrapper: Option<Wrapper>,
}

impl SqlBuilder {
    /// 对齐 Java: `SqlBuilder.create()`.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java: `SqlBuilder.create(Wrapper)`.
    #[must_use]
    pub fn create_with_wrapper(wrapper: Wrapper) -> Self {
        Self {
            wrapper: Some(wrapper),
            ..Self::default()
        }
    }

    /// 对齐 Java: `SqlBuilder.of(CharSequence)`.
    #[must_use]
    pub fn of(sql: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            ..Self::default()
        }
    }

    /// 对齐 Java: `SqlBuilder.select(String...)`.
    pub fn select(&mut self, fields: impl IntoIterator<Item = impl AsRef<str>>) -> &mut Self {
        self.sql.push_str("SELECT ");
        let fields: Vec<_> = fields.into_iter().map(|f| f.as_ref().to_string()).collect();
        if fields.is_empty() {
            self.sql.push('*');
        } else {
            self.sql.push_str(&fields.join(",")); // Hutool: no space after comma
        }
        self
    }

    /// 对齐 Java: `SqlBuilder.from(String...)`.
    pub fn from(&mut self, table: impl AsRef<str>) -> &mut Self {
        self.sql.push_str(" FROM ");
        self.sql.push_str(table.as_ref());
        self
    }

    /// 对齐 Java: `SqlBuilder.join(String, Join)`.
    pub fn join(&mut self, table: impl AsRef<str>, join: Join) -> &mut Self {
        self.sql.push(' ');
        self.sql.push_str(&join.to_string());
        self.sql.push_str(" JOIN ");
        self.sql.push_str(table.as_ref());
        self
    }

    /// 对齐 Java: `SqlBuilder.on(String)`.
    pub fn on(&mut self, on: impl AsRef<str>) -> &mut Self {
        self.sql.push_str(" ON ");
        self.sql.push_str(on.as_ref());
        self
    }

    /// 对齐 Java: `SqlBuilder.where(Condition...)`.
    pub fn where_conditions(&mut self, conditions: &[Condition]) -> &mut Self {
        if conditions.is_empty() {
            return self;
        }
        let mut wrapped = conditions.to_vec();
        if let Some(wrapper) = &self.wrapper {
            for condition in &mut wrapped {
                condition.set_field(wrapper.wrap(condition.field()));
            }
        }
        self.sql.push_str(" WHERE ");
        self.sql
            .push_str(&ConditionBuilder::of(&wrapped).build(&mut self.param_values));
        self
    }

    /// 对齐 Java: `SqlBuilder.orderBy(Order...)`.
    pub fn order_by(&mut self, orders: &[Order]) -> &mut Self {
        if orders.is_empty() {
            return self;
        }
        self.sql.push_str(" ORDER BY ");
        for (idx, order) in orders.iter().enumerate() {
            if idx > 0 {
                self.sql.push(',');
            }
            self.sql.push_str(order.field());
            if let Some(direction) = order.direction() {
                self.sql.push(' ');
                self.sql.push_str(&direction.to_string());
            }
        }
        self
    }

    /// 对齐 Java: `SqlBuilder.append(Object)`.
    pub fn append(&mut self, fragment: impl AsRef<str>) -> &mut Self {
        self.sql.push_str(fragment.as_ref());
        self
    }

    /// 对齐 Java: `SqlBuilder.build()`.
    #[must_use]
    pub fn build(&self) -> String {
        self.sql.clone()
    }

    /// 对齐 Java: `SqlBuilder.getParamValues()`.
    #[must_use]
    pub fn param_values(&self) -> &[Value] {
        &self.param_values
    }

    /// 对齐 Java: `SqlBuilder.format()`.
    pub fn format(&mut self) -> &mut Self {
        self.sql = formatter::format(&self.sql);
        self
    }

    /// 对齐 Java: `SqlBuilder.insert(Entity)`.
    pub fn insert(&mut self, entity: &Entity) -> &mut Self {
        entity.validate_for_write();
        self.sql.push_str("INSERT INTO \"");
        self.sql.push_str(entity.table_name().unwrap_or("unknown"));
        self.sql.push_str("\"");
        let mut fields = Vec::new();
        let mut placeholders = Vec::new();
        for (field, value) in entity.iter() {
            fields.push(field.clone());
            placeholders.push('?');
            self.param_values.push(value.clone());
        }
        self.sql.push_str(" (");
        self.sql.push_str(&fields.join(", "));
        self.sql.push_str(") VALUES (");
        self.sql.push_str(
            &std::iter::repeat("?")
                .take(placeholders.len())
                .collect::<Vec<_>>()
                .join(", "),
        );
        self.sql.push(')');
        self
    }

    /// 对齐 Java: `SqlBuilder.update(Entity)`.
    pub fn update(&mut self, entity: &Entity) -> &mut Self {
        entity.validate_for_write();
        self.sql.push_str("UPDATE ");
        self.sql.push_str(entity.table_name().unwrap_or("unknown"));
        self.sql.push_str(" SET ");
        for (idx, (field, value)) in entity.iter().enumerate() {
            if idx > 0 {
                self.sql.push_str(", ");
            }
            self.sql.push_str(field);
            self.sql.push_str(" = ?");
            self.param_values.push(value.clone());
        }
        self
    }
}
