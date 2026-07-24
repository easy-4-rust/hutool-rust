//! 对齐: `cn.hutool.core.util.ModifierUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ModifierUtil.java

use super::modifiers::Modifiers;

/// 对齐 Java `Method` 的轻量描述。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodDescriptor {
    /// 修饰符位标记。
    pub modifiers: Modifiers,
}
