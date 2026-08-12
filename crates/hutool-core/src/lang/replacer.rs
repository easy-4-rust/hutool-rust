//! 对齐: `cn.hutool.core.lang.Replacer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Replacer.java

#![allow(dead_code)] // 对齐 Java Replacer，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.Replacer<T>`
pub trait Replacer<T> {
    /// 对齐 Java: `Replacer.replace(T)`
    fn replace(&mut self, value: T) -> T;
}

impl<T, F> Replacer<T> for F
where
    F: FnMut(T) -> T,
{
    fn replace(&mut self, value: T) -> T {
        self(value)
    }
}

/// 对迭代器逐项替换。
pub fn replace_all<T, R: Replacer<T>>(
    items: impl IntoIterator<Item = T>,
    mut replacer: R,
) -> Vec<T> {
    items.into_iter().map(|v| replacer.replace(v)).collect()
}

#[cfg(test)]
mod replacer_idiomatic_parity {
    use super::*;

    #[test]
    fn replacer_maps_values() {
        let out = replace_all(vec![1, 2, 3], |x| x * 2);
        assert_eq!(out, vec![2, 4, 6]);
    }
}
