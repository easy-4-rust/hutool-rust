//! 对齐: `cn.hutool.core.util.ReferenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReferenceUtil.java

use std::marker::PhantomData;
use std::sync::{Arc, Weak as StdWeak};

use super::hit_reference::HitReference;
use super::reference_type::ReferenceType;

/// 对齐 Java: `cn.hutool.core.util.ReferenceUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ReferenceUtil;

impl ReferenceUtil {
    /// 对齐 Java: `ReferenceUtil.create(ReferenceType, T)`
    pub fn create<T>(reference_type: ReferenceType, referent: T) -> HitReference<T>
    where
        T: Clone,
    {
        match reference_type {
            ReferenceType::Weak => {
                let arc = Arc::new(referent);
                HitReference::Weak(std::sync::Arc::downgrade(&arc))
            }
            ReferenceType::Soft => HitReference::Soft(Arc::new(referent)),
            ReferenceType::Phantom => HitReference::Phantom(PhantomData),
        }
    }

    /// 对齐 Java: `ReferenceUtil.get(Reference<T>)`
    #[must_use]
    pub fn get<T: Clone>(reference: &HitReference<T>) -> Option<T> {
        reference.get()
    }
}
