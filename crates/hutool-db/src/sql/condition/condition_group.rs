//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

use crate::sql::logical_operator::LogicalOperator;
use rust_decimal::Decimal;
use serde_json::Value;
use std::fmt::Write as _;

use super::condition::Condition;

/// 条件组 —— 对齐 Hutool `ConditionGroup`。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ConditionGroup {
    conditions: Vec<Condition>,
}

impl ConditionGroup {
    /// 追加条件。
    pub fn add_conditions(&mut self, conditions: impl IntoIterator<Item = Condition>) {
        self.conditions.extend(conditions);
    }

    /// 生成带括号的 SQL 片段。
    pub fn to_sql(&self, param_values: &mut Vec<Value>) -> String {
        if self.conditions.is_empty() {
            return String::new();
        }
        format!(
            "({})",
            crate::sql::condition_builder::ConditionBuilder::of(&self.conditions)
                .build(param_values)
        )
    }
}

impl std::fmt::Display for ConditionGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_sql(&mut Vec::new()))
    }
}
