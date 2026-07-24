//! 对齐: `cn.hutool.core.lang.intern.InternUtil`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use super::weak_interner::WeakInterner;

/// 对齐 Java: `InternUtil`
pub struct InternUtil;

impl InternUtil {
    /// 对齐 `createWeakInterner`
    pub fn create_weak_interner() -> WeakInterner {
        WeakInterner::new()
    }
}
