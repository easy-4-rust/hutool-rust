//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use super::object_util::ObjectUtil;

/// 对齐 Java `ObjectUtil.contains` 支持的包含探测目标。
pub trait ObjectContains<E: ?Sized> {
    /// 判断对象是否包含指定元素。
    fn object_contains(&self, element: &E) -> bool;
}

impl ObjectContains<str> for str {
    fn object_contains(&self, element: &str) -> bool {
        self.contains(element)
    }
}

impl ObjectContains<String> for str {
    fn object_contains(&self, element: &String) -> bool {
        self.contains(element.as_str())
    }
}

impl<T: PartialEq> ObjectContains<T> for [T] {
    fn object_contains(&self, element: &T) -> bool {
        self.iter()
            .any(|item| ObjectUtil::equal(Some(item), Some(element)))
    }
}

impl<T: PartialEq> ObjectContains<T> for Vec<T> {
    fn object_contains(&self, element: &T) -> bool {
        self.as_slice().object_contains(element)
    }
}

impl<V: PartialEq, K: Eq + Hash, S: std::hash::BuildHasher> ObjectContains<V> for HashMap<K, V, S> {
    fn object_contains(&self, element: &V) -> bool {
        self.values()
            .any(|value| ObjectUtil::equal(Some(value), Some(element)))
    }
}
