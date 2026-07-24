//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

use std::collections::HashMap;
use std::hash::Hash;

mod coll_value_map_ops;
mod list_value_map;
mod set_value_map;
mod collection_value_map;

pub use coll_value_map_ops::CollValueMapOps;
pub use list_value_map::ListValueMap;
pub use set_value_map::SetValueMap;
pub use collection_value_map::CollectionValueMap;
