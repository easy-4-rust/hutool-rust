//! 对齐: `cn.hutool.core.lang.intern.InternUtil`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

mod weak_interner;
mod intern_util;

pub use weak_interner::WeakInterner;
pub use intern_util::InternUtil;
