//! 对齐: `cn.hutool.core.lang.WeightRandom` / 内部类 `WeightObj`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/WeightRandom.java

use rand::Rng;
use std::hash::{Hash, Hasher};

mod weight_obj;
mod weight_random;

pub use weight_obj::WeightObj;
pub use weight_random::WeightRandom;
