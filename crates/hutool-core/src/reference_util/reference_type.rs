//! 对齐: `cn.hutool.core.util.ReferenceUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReferenceUtil.java

use std::marker::PhantomData;
use std::sync::{Arc, Weak as StdWeak};

use super::reference_util::ReferenceUtil;

/// 对齐 Java: `ReferenceUtil.ReferenceType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceType {
    /// 软引用。
    Soft,
    /// 弱引用。
    Weak,
    /// 虚引用。
    Phantom,
}
