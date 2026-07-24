//! 对齐: `cn.hutool.core.util.ModifierUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ModifierUtil.java

use super::modifier_util::ModifierUtil;

/// 对齐 Java: `ModifierUtil.ModifierType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierType {
    /// public
    Public,
    /// private
    Private,
    /// protected
    Protected,
    /// static
    Static,
    /// final
    Final,
    /// synchronized
    Synchronized,
    /// volatile
    Volatile,
    /// transient
    Transient,
    /// native
    Native,
    /// abstract
    Abstract,
    /// strictfp
    Strict,
}
