//! 对齐: `cn.hutool.core.lang.WeightListRandom`

#![allow(dead_code)] // 对齐 Java WeightListRandom，暂未接线，预留

use crate::lang::weight_random::{WeightObj, WeightRandom};

/// 对齐 Java: `WeightListRandom`（基于 WeightRandom）
pub struct WeightListRandom<T> {
    inner: WeightRandom<T>,
}

impl<T: Clone> WeightListRandom<T> {
    /// 创建
    pub fn new() -> Self {
        Self {
            inner: WeightRandom::create(),
        }
    }

    /// 添加权重项
    pub fn add(&mut self, item: T, weight: f64) {
        self.inner.add(item, weight);
    }

    /// 添加 WeightObj
    pub fn add_obj(&mut self, obj: WeightObj<T>) {
        self.inner.add_obj(obj);
    }

    /// 按权重抽样
    pub fn next(&self) -> Option<T> {
        self.inner.next()
    }

    /// 清空
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T: Clone> Default for WeightListRandom<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod weight_list_random_idiomatic_parity {
    use super::*;

    #[test]
    fn weight_list_random_samples() {
        let mut w = WeightListRandom::new();
        w.add("x", 10.0);
        assert_eq!(w.next(), Some("x"));
        w.clear();
        assert!(w.next().is_none());
    }
}
