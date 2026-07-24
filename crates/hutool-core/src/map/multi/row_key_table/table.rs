//! 对齐: `cn.hutool.core.map.multi.RowKeyTable` / `Table` / `AbsTable`
//! 来源: hutool-core/.../multi/RowKeyTable.java

use std::collections::HashMap;
use std::hash::Hash;

use super::row_key_table::RowKeyTable;

/// 对齐 Java: `Table` 接口的实现类型。
pub type Table<R, C, V> = RowKeyTable<R, C, V>;
