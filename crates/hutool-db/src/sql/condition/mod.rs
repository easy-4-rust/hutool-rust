//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

use crate::sql::logical_operator::LogicalOperator;
use rust_decimal::Decimal;
use serde_json::Value;
use std::fmt::Write as _;

mod like_type;
mod condition_value;
mod condition;
mod condition_group;

pub use like_type::LikeType;
pub use condition_value::ConditionValue;
pub use condition::Condition;
pub use condition_group::ConditionGroup;
