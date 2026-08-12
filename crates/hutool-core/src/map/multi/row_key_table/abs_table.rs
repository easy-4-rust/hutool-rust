//! 对齐: `cn.hutool.core.map.multi.RowKeyTable` / `Table` / `AbsTable`
//! 来源: hutool-core/.../multi/RowKeyTable.java

#![allow(dead_code)] // 对齐 Java AbsTable，暂未接线，预留

use super::row_key_table::RowKeyTable;

/// 对齐 Java: `AbsTable` —— 与 RowKeyTable 同构的类型别名。
pub type AbsTable<R, C, V> = RowKeyTable<R, C, V>;
