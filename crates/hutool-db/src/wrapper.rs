//! 字段包装器 —— 对齐 Hutool `cn.hutool.db.sql.Wrapper`。

use crate::entity::Entity;
use crate::sql::condition::Condition;

/// SQL 标识符包装器。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wrapper {
    pre: char,
    suf: char,
}

impl Wrapper {
    /// 对齐 Java: `Wrapper(Character wrapQuote)`.
    #[must_use]
    pub fn new(wrap_quote: char) -> Self {
        Self {
            pre: wrap_quote,
            suf: wrap_quote,
        }
    }

    /// 对齐 Java: `Wrapper.wrap(String)`.
    #[must_use]
    pub fn wrap(&self, field: &str) -> String {
        if field.is_empty() {
            return field.to_string();
        }
        if field.starts_with(self.pre) && field.ends_with(self.suf) {
            return field.to_string();
        }
        let lower = field.to_ascii_lowercase();
        if lower.contains('*')
            || field.contains('(')
            || field.contains(' ')
            || lower.contains(" as ")
        {
            return field.to_string();
        }
        if field.contains('.') {
            return field
                .split('.')
                .map(|part| format!("{}{}{}", self.pre, part, self.suf))
                .collect::<Vec<_>>()
                .join(".");
        }
        format!("{}{}{}", self.pre, field, self.suf)
    }

    /// 对齐 Java: `Wrapper.unWrap(String)`.
    #[must_use]
    pub fn unwrap(&self, field: &str) -> String {
        if field.is_empty() {
            return field.to_string();
        }
        if !(field.starts_with(self.pre) && field.ends_with(self.suf)) {
            return field.to_string();
        }
        let lower = field.to_ascii_lowercase();
        if lower.contains('*')
            || field.contains('(')
            || field.contains(' ')
            || lower.contains(" as ")
        {
            return field.to_string();
        }
        if field.contains('.') {
            return field
                .split('.')
                .map(|part| {
                    part.strip_prefix(&self.pre.to_string())
                        .and_then(|p| p.strip_suffix(&self.suf.to_string()))
                        .unwrap_or(part)
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(".");
        }
        field
            .strip_prefix(&self.pre.to_string())
            .and_then(|p| p.strip_suffix(&self.suf.to_string()))
            .unwrap_or(field)
            .to_string()
    }

    /// 对齐 Java: `Wrapper.wrap(Condition...)`.
    #[must_use]
    pub fn wrap_conditions(&self, conditions: &[Condition]) -> Vec<Condition> {
        conditions
            .iter()
            .cloned()
            .map(|mut condition| {
                condition.set_field(self.wrap(condition.field()));
                condition
            })
            .collect()
    }

    /// 对齐 Java: `Wrapper.wrap(Entity)`.
    #[must_use]
    pub fn wrap_entity(&self, entity: &Entity) -> Entity {
        let mut wrapped = Entity::create_table(self.wrap(entity.table_name().unwrap_or("")));
        for (key, value) in entity.iter() {
            wrapped.set(self.wrap(key), value.clone());
        }
        wrapped
    }
}
