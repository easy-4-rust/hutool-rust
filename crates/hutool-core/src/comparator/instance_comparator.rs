//! 对齐: `cn.hutool.core.comparator.InstanceComparator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/InstanceComparator.java
//!
//! Java 按 Class 顺序；Rust 用 `TypeId` 顺序表。

use std::any::{Any, TypeId};
use std::cmp::Ordering;

/// 对齐 Java 类: `cn.hutool.core.comparator.InstanceComparator`
#[derive(Debug, Clone)]
pub struct InstanceComparator {
    at_end_if_miss: bool,
    order: Vec<TypeId>,
}

impl InstanceComparator {
    /// 对齐 Java: `InstanceComparator(Class<?>...)`
    #[must_use]
    pub fn new(instance_order: impl IntoIterator<Item = TypeId>) -> Self {
        Self::with_miss(false, instance_order)
    }

    /// 对齐 Java: `InstanceComparator(boolean atEndIfMiss, Class<?>...)`
    #[must_use]
    pub fn with_miss(
        at_end_if_miss: bool,
        instance_order: impl IntoIterator<Item = TypeId>,
    ) -> Self {
        Self {
            at_end_if_miss,
            order: instance_order.into_iter().collect(),
        }
    }

    /// 对齐 Java: `compare(T o1, T o2)` —— 对 `&dyn Any` 比较。
    #[must_use]
    pub fn compare_any(&self, o1: &dyn Any, o2: &dyn Any) -> i32 {
        let i1 = self.get_order(o1.type_id());
        let i2 = self.get_order(o2.type_id());
        match i1.cmp(&i2) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `compare` —— 对同类型引用按 TypeId 表比较（测试用）。
    #[must_use]
    pub fn compare_type_ids(&self, t1: TypeId, t2: TypeId) -> i32 {
        match self.get_order(t1).cmp(&self.get_order(t2)) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    fn get_order(&self, tid: TypeId) -> i32 {
        match self.order.iter().position(|t| *t == tid) {
            Some(i) => i as i32,
            None => {
                if self.at_end_if_miss {
                    self.order.len() as i32
                } else {
                    -1
                }
            }
        }
    }
}
