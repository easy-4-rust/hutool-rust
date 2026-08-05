//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

mod create_map_kind;
mod empty_map_kind;
mod linked_or_hash_map;
mod map_builder_gate;
mod map_util;
mod nested_map_value;

pub use create_map_kind::CreateMapKind;
pub use empty_map_kind::EmptyMapKind;
pub use linked_or_hash_map::LinkedOrHashMap;
pub use map_util::MapUtil;
pub use nested_map_value::NestedMapValue;

pub const DEFAULT_INITIAL_CAPACITY: usize = 16;

pub const DEFAULT_LOAD_FACTOR: f32 = 0.75;

fn simple_to_camel_case(name: &str) -> String {
    if !name.contains('_') {
        return name.to_string();
    }
    let mut sb = String::with_capacity(name.len());
    let mut upper = false;
    for c in name.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            for u in c.to_uppercase() {
                sb.push(u);
            }
            upper = false;
        } else {
            for l in c.to_lowercase() {
                sb.push(l);
            }
        }
    }
    sb
}
