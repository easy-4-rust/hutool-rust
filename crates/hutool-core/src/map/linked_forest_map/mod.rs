//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

use std::collections::HashMap;
use std::hash::Hash;

mod tree_entry;
mod forest_map;
mod linked_forest_map;

pub use tree_entry::TreeEntry;
pub use forest_map::ForestMap;
pub use linked_forest_map::LinkedForestMap;
