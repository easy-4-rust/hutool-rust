//! 结果集处理器 —— 对齐 Hutool `cn.hutool.db.handler.*`。

use crate::entity::Entity;
use crate::page_result::PageResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::BTreeSet;

mod rs_handler;
mod handle_helper;
mod entity_handler;
mod entity_list_handler;
mod entity_set_handler;
mod page_result_handler;
mod bean_handler;
mod bean_list_handler;
mod number_handler;
mod string_handler;
mod value_list_handler;

pub use rs_handler::RsHandler;
pub use handle_helper::HandleHelper;
pub use entity_handler::EntityHandler;
pub use entity_list_handler::EntityListHandler;
pub use entity_set_handler::EntitySetHandler;
pub use page_result_handler::PageResultHandler;
pub use bean_handler::BeanHandler;
pub use bean_list_handler::BeanListHandler;
pub use number_handler::NumberHandler;
pub use string_handler::StringHandler;
pub use value_list_handler::ValueListHandler;
