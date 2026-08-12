//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;

/// 对齐 Hutool `RsHandler`：将行集合折叠为自定义结果。
pub trait RsHandler<T> {
    /// 对齐 Java: `handle(ResultSet)`。
    fn handle(&self, rows: &[Entity]) -> T;
}
