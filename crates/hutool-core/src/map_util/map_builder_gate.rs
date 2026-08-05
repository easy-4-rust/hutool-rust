//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

use std::collections::HashMap;
use std::hash::Hash;

/// 轻量 Map builder 门面（避免依赖尚未接线的 `crate::map` 子包）。
#[derive(Debug, Clone)]
pub struct MapBuilderGate<K, V> {
    map: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MapBuilderGate<K, V> {
    /// 创建空 builder。
    pub fn create() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    /// 从已有 map 创建。
    pub fn create_from(map: HashMap<K, V>) -> Self {
        Self { map }
    }
    /// 放入键值对。
    pub fn put(mut self, k: K, v: V) -> Self {
        self.map.insert(k, v);
        self
    }
    /// 构建 HashMap。
    pub fn build(self) -> HashMap<K, V> {
        self.map
    }
}
