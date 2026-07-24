//! 列/索引元数据 —— 对齐 Hutool `cn.hutool.db.meta.Column` / `IndexInfo`。

use std::fmt;

mod table_type;
mod jdbc_type;
mod column;
mod column_index_info;
mod index_info;

pub use table_type::TableType;
pub use jdbc_type::JdbcType;
pub use column::Column;
pub use column_index_info::ColumnIndexInfo;
pub use index_info::IndexInfo;
