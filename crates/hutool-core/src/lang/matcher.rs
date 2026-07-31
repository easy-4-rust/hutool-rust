//! 对齐: `cn.hutool.core.lang.Matcher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Matcher.java

#![allow(dead_code)] // 对齐 Java Matcher，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.Matcher<T>`
pub trait Matcher<T> {
    /// 对齐 Java: `Matcher.match(T)`
    fn match_item(&self, value: &T) -> bool;
}

impl<T, F> Matcher<T> for F
where
    F: Fn(&T) -> bool,
{
    fn match_item(&self, value: &T) -> bool {
        self(value)
    }
}

/// 按匹配器过滤切片。
pub fn match_all<'a, T, M: Matcher<T>>(
    items: impl IntoIterator<Item = &'a T>,
    matcher: &M,
) -> Vec<&'a T>
where
    T: 'a,
{
    items
        .into_iter()
        .filter(|v| matcher.match_item(v))
        .collect()
}

#[cfg(test)]
mod matcher_idiomatic_parity {
    use super::*;

    #[test]
    fn matcher_filters_matching() {
        let data = [1, 2, 3, 4];
        let kept = match_all(&data, &(|x: &i32| *x > 2));
        assert_eq!(kept, vec![&3, &4]);
    }
}
