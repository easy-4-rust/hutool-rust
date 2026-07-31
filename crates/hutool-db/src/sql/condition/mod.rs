//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

mod like_type;
mod condition_value;
mod condition;
mod condition_group;

pub use like_type::LikeType;
pub use condition_value::ConditionValue;
pub use condition::Condition;
pub use condition_group::ConditionGroup;
