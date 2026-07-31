//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

use std::collections::HashMap;

/// 嵌套 Map 值 —— 对齐 `MapUtil.flatten` 的多层级结构。
#[derive(Debug, Clone)]
pub enum NestedMapValue<K, V> {
    /// 叶子值
    Leaf(V),
    /// 子 Map
    Nested(HashMap<K, NestedMapValue<K, V>>),
}
