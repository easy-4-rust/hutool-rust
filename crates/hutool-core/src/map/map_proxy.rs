//! 对齐: `cn.hutool.core.map.MapProxy`
//!
//! JVM 动态代理无安全 1:1 映射；保留类型占位并标记 planned。

#![allow(dead_code)] // 对齐 Java MapProxy，暂未接线，预留

use std::collections::HashMap;
use std::hash::Hash;

use crate::{CoreError, Result};

/// 对齐 Java 类: `cn.hutool.core.map.MapProxy`（planned：无反射代理）。
#[derive(Debug, Clone)]
pub struct MapProxy<K, V> {
    raw: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MapProxy<K, V> {
    /// 对齐 Java: `MapProxy.create(Map)` —— 仅包装，不提供 Bean 式 getter 代理。
    pub fn create(map: HashMap<K, V>) -> Self {
        Self { raw: map }
    }

    /// 底层 map。
    pub fn raw(&self) -> &HashMap<K, V> {
        &self.raw
    }

    /// Bean 风格属性访问 —— planned。
    pub fn get_property(&self, _name: &str) -> Result<()> {
        Err(CoreError::PendingEngine(
            "MapProxy reflective property access (Java Proxy)",
        ))
    }
}
