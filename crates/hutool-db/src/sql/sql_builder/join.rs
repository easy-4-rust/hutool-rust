//! SQL 构建器 —— 对齐 Hutool `cn.hutool.db.sql.SqlBuilder`。

/// JOIN 类型 —— 对齐 Hutool `SqlBuilder.Join`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Join {
    /// INNER JOIN。
    Inner,
    /// LEFT JOIN。
    Left,
    /// RIGHT JOIN。
    Right,
    /// FULL JOIN。
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
