//! 对齐: `cn.hutool.core.lang.WeightRandom` / 内部类 `WeightObj`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/WeightRandom.java

use rand::Rng;
use std::hash::{Hash, Hasher};

use super::weight_random::WeightRandom;

/// 对齐 Java: `WeightRandom.WeightObj<T>`
#[derive(Debug, Clone)]
pub struct WeightObj<T> {
    pub(crate) obj: T,
    pub(crate) weight: f64,
}

impl<T> WeightObj<T> {
    /// 对齐 Java: `WeightObj(T, double)`
    pub fn new(obj: T, weight: f64) -> Self {
        Self { obj, weight }
    }

    /// 对齐 Java: `getObj`
    pub fn get_obj(&self) -> &T {
        &self.obj
    }

    /// 对齐 Java: `setObj`
    pub fn set_obj(&mut self, obj: T) {
        self.obj = obj;
    }

    /// 对齐 Java: `getWeight`
    #[must_use]
    pub fn get_weight(&self) -> f64 {
        self.weight
    }
}

impl<T: PartialEq> PartialEq for WeightObj<T> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
            && self.weight.to_bits() == other.weight.to_bits()
    }
}

impl<T: Eq> Eq for WeightObj<T> {}

impl<T: Hash> Hash for WeightObj<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
        self.weight.to_bits().hash(state);
    }
}
