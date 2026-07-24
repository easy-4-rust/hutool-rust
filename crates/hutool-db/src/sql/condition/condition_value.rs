//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

use crate::sql::logical_operator::LogicalOperator;
use rust_decimal::Decimal;
use serde_json::Value;
use std::fmt::Write as _;

use super::condition::Condition;

/// 条件值包装，用于 Entity 字段存储 Condition 或原始值。
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionValue {
    Raw(Value),
    Condition(Condition),
}

impl ConditionValue {
    /// 若值为 Condition 则返回引用。
    #[must_use]
    pub fn as_condition(&self) -> Option<&Condition> {
        match self {
            Self::Condition(c) => Some(c),
            Self::Raw(_) => None,
        }
    }
}

impl From<Value> for ConditionValue {
    fn from(value: Value) -> Self {
        Self::Raw(value)
    }
}

impl From<Condition> for ConditionValue {
    fn from(value: Condition) -> Self {
        Self::Condition(value)
    }
}
