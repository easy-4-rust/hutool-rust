//! 对齐: `cn.hutool.core.lang.Editor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Editor.java
//!
//! Java 函数式接口在 Rust 中映射为 [`FnMut`] trait alias 语义：`edit` 返回 `None` 表示丢弃。

#![allow(dead_code)] // 对齐 Java Editor，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.Editor`
pub trait Editor<T> {
    /// 对齐 Java: `Editor.edit(T)` — 返回 `None` 表示过滤掉该元素。
    fn edit(&mut self, value: T) -> Option<T>;
}

impl<T, F> Editor<T> for F
where
    F: FnMut(T) -> Option<T>,
{
    fn edit(&mut self, value: T) -> Option<T> {
        self(value)
    }
}

/// 对切片应用编辑器，丢弃返回 `None` 的项。
pub fn edit_all<T, E: Editor<T>>(items: impl IntoIterator<Item = T>, mut editor: E) -> Vec<T> {
    items.into_iter().filter_map(|v| editor.edit(v)).collect()
}

#[cfg(test)]
mod editor_idiomatic_parity {
    use super::*;

    #[test]
    fn editor_maps_and_filters() {
        let out = edit_all(
            vec![1, 2, 3],
            |x| if x % 2 == 0 { None } else { Some(x * 10) },
        );
        assert_eq!(out, vec![10, 30]);
    }
}
