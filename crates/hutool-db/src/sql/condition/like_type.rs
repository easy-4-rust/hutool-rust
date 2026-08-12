//! 条件对象 —— 对齐 Hutool `cn.hutool.db.sql.Condition`。

/// LIKE 匹配方式 —— 对齐 Hutool `Condition.LikeType`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LikeType {
    /// 以指定值开头。
    StartWith,
    /// 以指定值结尾。
    EndWith,
    /// 包含指定值。
    Contains,
}
