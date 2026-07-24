//! 对齐: `cn.hutool.core.map.multi.RowKeyTable` / `Table` / `AbsTable`
//! 来源: hutool-core/.../multi/RowKeyTable.java

use std::collections::HashMap;
use std::hash::Hash;

use super::table::Table;

/// 对齐 Java 接口: `Table.Cell`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCell<R, C, V> {
    /// 行键
    pub row_key: R,
    /// 列键
    pub column_key: C,
    /// 值
    pub value: V,
}
