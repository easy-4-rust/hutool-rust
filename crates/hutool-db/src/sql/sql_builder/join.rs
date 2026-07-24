//! SQL 构建器 —— 对齐 Hutool `cn.hutool.db.sql.SqlBuilder`。

use crate::entity::Entity;
use crate::sql::condition::Condition;
use crate::sql::condition_builder::ConditionBuilder;
use crate::sql::formatter;
use crate::sql::order::Order;
use crate::wrapper::Wrapper;
use serde_json::Value;

use super::sql_builder::SqlBuilder;

/// JOIN 类型 —— 对齐 Hutool `SqlBuilder.Join`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Join {
    Inner,
    Left,
    Right,
    Full,
}

impl std::fmt::Display for Join {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inner => write!(f, "INNER"),
            Self::Left => write!(f, "LEFT"),
            Self::Right => write!(f, "RIGHT"),
            Self::Full => write!(f, "FULL"),
        }
    }
}
