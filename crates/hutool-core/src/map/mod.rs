//! `cn.hutool.core.map` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod abs_entry;
pub mod bi_map;
pub mod camel_case_linked_map;
pub mod camel_case_map;
pub mod case_insensitive_linked_map;
pub mod case_insensitive_map;
pub mod case_insensitive_tree_map;
pub mod custom_key_map;
pub mod fixed_linked_hash_map;
pub mod forest_map;
pub mod func_key_map;
pub mod func_map;
pub mod linked_forest_map;
pub mod map_builder;
pub mod map_proxy;
pub mod map_util;
pub mod map_wrapper;
pub mod multi;
pub mod reference;
pub mod reference_concurrent_map;
pub mod safe_concurrent_hash_map;
pub mod table_map;
pub mod tolerant_map;
pub mod trans_map;
pub mod tree_entry;
pub mod weak_concurrent_map;

pub use abs_entry::AbsEntry;
pub use bi_map::BiMap;
pub use camel_case_linked_map::CamelCaseLinkedMap;
pub use camel_case_map::CamelCaseMap;
pub use case_insensitive_linked_map::CaseInsensitiveLinkedMap;
pub use case_insensitive_map::CaseInsensitiveMap;
pub use case_insensitive_tree_map::CaseInsensitiveTreeMap;
pub use custom_key_map::{CustomKeyMap, custom_key_map};
pub use fixed_linked_hash_map::FixedLinkedHashMap;
pub use forest_map::LinkedForestMap;
pub use func_key_map::FuncKeyMap;
pub use func_map::FuncMap;
pub use map_builder::MapBuilder;
pub use map_wrapper::MapWrapper;
pub use multi::{ListValueMap, RowKeyTable, SetValueMap};
pub use safe_concurrent_hash_map::SafeConcurrentHashMap;
pub use table_map::TableMap;
pub use tolerant_map::TolerantMap;
pub use trans_map::TransMap;
