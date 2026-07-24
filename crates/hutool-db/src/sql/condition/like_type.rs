//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

use crate::sql::logical_operator::LogicalOperator;
use rust_decimal::Decimal;
use serde_json::Value;
use std::fmt::Write as _;

use super::condition::Condition;

/// LIKE 匹配方式 —— 对齐 Hutool `Condition.LikeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LikeType {
    StartWith,
    EndWith,
    Contains,
}
