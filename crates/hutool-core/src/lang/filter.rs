//! 对齐: `cn.hutool.core.lang.Filter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Filter.java

#![allow(dead_code)] // 对齐 Java Filter，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.Filter`
pub trait Filter<T> {
    /// 对齐 Java: `Filter.accept(T)`
    fn accept(&self, value: &T) -> bool;
}

impl<T, F> Filter<T> for F
where
    F: Fn(&T) -> bool,
{
    fn accept(&self, value: &T) -> bool {
        self(value)
    }
}

/// 按过滤器保留元素。
pub fn filter_all<'a, T, F: Filter<T>>(
    items: impl IntoIterator<Item = &'a T>,
    filter: &F,
) -> Vec<&'a T>
where
    T: 'a,
{
    items.into_iter().filter(|v| filter.accept(v)).collect()
}

#[cfg(test)]
mod filter_idiomatic_parity {
    use super::*;

    #[test]
    fn filter_accepts_matching() {
        let data = [1, 2, 3, 4];
        let kept = filter_all(&data, &(|x: &i32| *x % 2 == 0));
        assert_eq!(kept, vec![&2, &4]);
    }
}
