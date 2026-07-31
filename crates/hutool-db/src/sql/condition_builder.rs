//! 多条件构建 —— 对齐 Hutool `cn.hutool.db.sql.ConditionBuilder`。

use crate::sql::condition::{Condition, ConditionGroup};
use crate::sql::logical_operator::LogicalOperator;
use serde_json::Value;

/// 条件 SQL 片段构建器。
#[derive(Debug, Clone)]
pub struct ConditionBuilder {
    items: Vec<ConditionItem>,
}

#[derive(Debug, Clone)]
pub(crate) enum ConditionItem {
    Condition(Condition),
    Group(ConditionGroup),
}

impl ConditionBuilder {
    /// 对齐 Java: `ConditionBuilder.of(Condition...)`.
    #[must_use]
    pub fn of(conditions: &[Condition]) -> Self {
        Self {
            items: conditions
                .iter()
                .cloned()
                .map(ConditionItem::Condition)
                .collect(),
        }
    }

    /// 支持条件组与条件的混合构建（ConditionGroupTest 用例）。
    #[must_use]
    #[allow(private_bounds)]
    pub fn of_mixed(items: impl IntoIterator<Item = ConditionItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }

    /// 对齐 Java: `ConditionBuilder.build()`.
    pub fn build(&self, param_values: &mut Vec<Value>) -> String {
        if self.items.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (idx, item) in self.items.iter().enumerate() {
            if idx > 0 {
                let link = match item {
                    ConditionItem::Condition(c) => c.link_operator(),
                    ConditionItem::Group(_) => LogicalOperator::And,
                };
                out.push(' ');
                out.push_str(link.as_sql());
                out.push(' ');
            }
            match item {
                ConditionItem::Condition(c) => out.push_str(&c.to_sql(param_values)),
                ConditionItem::Group(g) => out.push_str(&g.to_sql(param_values)),
            }
        }
        out
    }

    /// 对齐 Java: `ConditionBuilder.getParamValues()` — 需先 `build()`。
    #[must_use]
    pub fn param_values(&self) -> Vec<Value> {
        let mut params = Vec::new();
        let _ = self.build(&mut params);
        params
    }
}

impl From<Condition> for ConditionItem {
    fn from(value: Condition) -> Self {
        Self::Condition(value)
    }
}

impl From<ConditionGroup> for ConditionItem {
    fn from(value: ConditionGroup) -> Self {
        Self::Group(value)
    }
}
