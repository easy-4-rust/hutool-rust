//! 对齐: `cn.hutool.core.util.ModifierUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ModifierUtil.java

use super::modifier_type::ModifierType;

/// 对齐 Java `Method.getModifiers()` 的位标记集合。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers(u32);

impl Modifiers {
    const PUBLIC: u32 = 1 << 0;
    const PRIVATE: u32 = 1 << 1;
    const PROTECTED: u32 = 1 << 2;
    const STATIC: u32 = 1 << 3;
    const FINAL: u32 = 1 << 4;
    const SYNCHRONIZED: u32 = 1 << 5;
    const VOLATILE: u32 = 1 << 6;
    const TRANSIENT: u32 = 1 << 7;
    const NATIVE: u32 = 1 << 8;
    const ABSTRACT: u32 = 1 << 9;
    const STRICT: u32 = 1 << 10;

    /// 构造修饰符集合。
    #[must_use]
    pub fn new(flags: u32) -> Self {
        Self(flags)
    }

    /// 判断包含指定修饰符。
    #[must_use]
    pub fn has(&self, modifier_type: ModifierType) -> bool {
        self.0 & flag(modifier_type) != 0
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
