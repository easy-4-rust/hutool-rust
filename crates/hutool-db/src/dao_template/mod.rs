//! DaoTemplate facade，对齐 hutool 的 `cn.hutool.db.DaoTemplate`。
//!
//! 提供 CRUD 模板：add/del/update/get/find/page/count/exist 等 25 个方法。
//! 具体数据库操作依赖 Db（基于 SQLx），属于 unsafe-to-copy 的 JDBC 部分。

use crate::entity::Entity;
use crate::page_result::PageResult;
use crate::hutool_page::HutoolPage;
use crate::DbResult;

mod dao_template;
mod dao_operations;

pub use dao_template::DaoTemplate;
pub use dao_operations::DaoOperations;
