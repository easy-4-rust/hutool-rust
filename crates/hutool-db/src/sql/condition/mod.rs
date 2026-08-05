//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

mod condition;
mod condition_group;
mod condition_value;
mod like_type;

pub use condition::Condition;
pub use condition_group::ConditionGroup;
pub use condition_value::ConditionValue;
pub use like_type::LikeType;
