//! 对齐: `cn.hutool.core.util.ReferenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReferenceUtil.java

use std::marker::PhantomData;
use std::sync::{Arc, Weak as StdWeak};

mod reference_type;
mod hit_reference;
mod reference_util;

pub use reference_type::ReferenceType;
pub use hit_reference::HitReference;
pub use reference_util::ReferenceUtil;
