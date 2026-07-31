//! 对齐: `cn.hutool.core.lang.EnumItem`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/EnumItem.java
//!
//! Java 枚举接口映射为 Rust trait：提供 `name` / `text` / `int_val` 与按值查找。

#![allow(dead_code)] // 对齐 Java EnumItem，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.EnumItem<E>`
pub trait EnumItem: Sized + Copy + 'static {
    /// 对齐 Java: `name()`
    fn name(&self) -> &'static str;
    /// 对齐 Java: `text()` — 默认与 name 相同。
    fn text(&self) -> &'static str {
        self.name()
    }
    /// 对齐 Java: `intVal()`
    fn int_val(&self) -> i32;
    /// 对齐 Java: `items()` — 由实现方提供全部枚举常量。
    fn items() -> &'static [Self];
    /// 对齐 Java: `fromInt`
    fn from_int(int_val: Option<i32>) -> Option<Self> {
        let v = int_val?;
        Self::items().iter().copied().find(|e| e.int_val() == v)
    }
    /// 对齐 Java: `fromStr` — 忽略大小写匹配 name。
    fn from_str(str_val: Option<&str>) -> Option<Self> {
        let s = str_val?;
        Self::items()
            .iter()
            .copied()
            .find(|e| e.name().eq_ignore_ascii_case(s))
    }
}

#[cfg(test)]
mod enum_item_idiomatic_parity {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Color {
        Red,
        Blue,
    }

    impl EnumItem for Color {
        fn name(&self) -> &'static str {
            match self {
                Color::Red => "RED",
                Color::Blue => "BLUE",
            }
        }

        fn int_val(&self) -> i32 {
            match self {
                Color::Red => 1,
                Color::Blue => 2,
            }
        }

        fn items() -> &'static [Self] {
            &[Color::Red, Color::Blue]
        }
    }

    #[test]
    fn from_int_and_str() {
        assert_eq!(Color::from_int(Some(1)), Some(Color::Red));
        assert_eq!(Color::from_str(Some("blue")), Some(Color::Blue));
    }
}
