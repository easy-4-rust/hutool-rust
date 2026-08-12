//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

mod forest_map;
mod linked_forest_map;
mod tree_entry;

pub use forest_map::ForestMap;
pub use linked_forest_map::LinkedForestMap;
pub use tree_entry::TreeEntry;
