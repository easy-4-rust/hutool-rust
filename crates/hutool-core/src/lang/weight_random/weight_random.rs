//! 对齐: `cn.hutool.core.lang.WeightRandom` / 内部类 `WeightObj`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/WeightRandom.java

use rand::Rng;
use std::hash::{Hash, Hasher};

use super::weight_obj::WeightObj;

/// 对齐 Java: `WeightRandom<T>`
pub struct WeightRandom<T> {
    items: Vec<WeightObj<T>>,
}

impl<T: Clone> WeightRandom<T> {
    /// 对齐 `WeightRandom.create()` / 无参构造
    pub fn create() -> Self {
        Self { items: Vec::new() }
    }

    /// 对齐 `WeightRandom(WeightObj)`
    pub fn from_obj(weight_obj: WeightObj<T>) -> Self {
        let mut w = Self::create();
        w.add_obj(weight_obj);
        w
    }

    /// 对齐 `WeightRandom(Iterable)` / 数组构造
    pub fn from_iter(weight_objs: impl IntoIterator<Item = WeightObj<T>>) -> Self {
        let mut w = Self::create();
        for o in weight_objs {
            w.add_obj(o);
        }
        w
    }

    /// 对齐 `add(T, double)`
    pub fn add(&mut self, item: T, weight: f64) -> &mut Self {
        self.add_obj(WeightObj::new(item, weight))
    }

    /// 对齐 `add(WeightObj)`
    pub fn add_obj(&mut self, weight_obj: WeightObj<T>) -> &mut Self {
        self.items.push(weight_obj);
        self
    }

    /// 对齐 `clear`
    pub fn clear(&mut self) -> &mut Self {
        self.items.clear();
        self
    }

    /// 对齐 `next()`
    pub fn next(&self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let total: f64 = self.items.iter().map(|w| w.weight).sum();
        if total <= 0.0 {
            return self.items.last().map(|w| w.obj.clone());
        }
        let mut r = rand::thread_rng().gen_range(0.0..total);
        for w in &self.items {
            if r < w.weight {
                return Some(w.obj.clone());
            }
            r -= w.weight;
        }
        self.items.last().map(|w| w.obj.clone())
    }

    /// 当前权重项数量（测试辅助）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Clone> Default for WeightRandom<T> {
    fn default() -> Self {
        Self::create()
    }
}
