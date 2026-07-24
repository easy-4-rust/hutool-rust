//! 对齐: `cn.hutool.core.util.ModifierUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ModifierUtil.java

use super::method_descriptor::MethodDescriptor;
use super::modifier_type::ModifierType;
use super::modifiers::Modifiers;

/// 对齐 Java: `cn.hutool.core.util.ModifierUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierUtil;

impl ModifierUtil {
    /// 对齐 Java: `ModifierUtil.hasModifier(Member, ModifierType...)`
    #[must_use]
    pub fn has_modifier(method: &MethodDescriptor, modifier_types: &[ModifierType]) -> bool {
        modifier_types
            .iter()
            .any(|modifier| method.modifiers.has(*modifier))
    }

    /// 对齐 Java: `ModifierUtil.hasAllModifiers(Member, ModifierType...)`
    #[must_use]
    pub fn has_all_modifiers(method: &MethodDescriptor, modifier_types: &[ModifierType]) -> bool {
        modifier_types
            .iter()
            .all(|modifier| method.modifiers.has(*modifier))
    }
}

const fn flag(modifier_type: ModifierType) -> u32 {
    match modifier_type {
        ModifierType::Public => Modifiers::PUBLIC,
        ModifierType::Private => Modifiers::PRIVATE,
        ModifierType::Protected => Modifiers::PROTECTED,
        ModifierType::Static => Modifiers::STATIC,
        ModifierType::Final => Modifiers::FINAL,
        ModifierType::Synchronized => Modifiers::SYNCHRONIZED,
        ModifierType::Volatile => Modifiers::VOLATILE,
        ModifierType::Transient => Modifiers::TRANSIENT,
        ModifierType::Native => Modifiers::NATIVE,
        ModifierType::Abstract => Modifiers::ABSTRACT,
        ModifierType::Strict => Modifiers::STRICT,
    }
}
