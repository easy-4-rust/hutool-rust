//! 对齐: `cn.hutool.core.lang.intern.InternUtil`

#![allow(dead_code)] // 对齐 Java InternUtil，暂未接线，预留

use super::weak_interner::WeakInterner;

/// 对齐 Java: `InternUtil`
pub struct InternUtil;

impl InternUtil {
    /// 对齐 `createWeakInterner`
    pub fn create_weak_interner() -> WeakInterner {
        WeakInterner::new()
    }
}
