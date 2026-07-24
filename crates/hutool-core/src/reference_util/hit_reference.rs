//! 对齐: `cn.hutool.core.util.ReferenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReferenceUtil.java

use std::marker::PhantomData;
use std::sync::{Arc, Weak as StdWeak};

/// 对齐 Java `Reference<T>` 的统一封装。
#[derive(Debug)]
pub enum HitReference<T> {
    /// 弱引用。
    Weak(StdWeak<T>),
    /// 软引用（Rust 用 Arc 持有，语义上 get 可返回值）。
    Soft(Arc<T>),
    /// 虚引用（get 恒为 None）。
    Phantom(PhantomData<T>),
}

impl<T> HitReference<T> {
    /// 对齐 Java: `Reference.get()`
    #[must_use]
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Weak(reference) => reference.upgrade().map(|value| (*value).clone()),
            Self::Soft(value) => Some((**value).clone()),
            Self::Phantom(_) => None,
        }
    }
}
