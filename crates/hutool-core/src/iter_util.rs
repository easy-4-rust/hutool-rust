//! Iterator operations aligned with Hutool's `IterUtil` behavior families.

/// 对齐: `cn.hutool.core.collection.IterUtil`
/// 迭代器工具类

use std::{any::type_name, collections::HashMap, fmt::Display, hash::Hash};

use crate::{FilterIter, TransIter};

/// Hutool-aligned iterator operations using Rust's static iterator protocol.
#[derive(Debug, Clone, Copy, Default)]
pub struct IterUtil;

impl IterUtil {
    /// Converts an iterable into its iterator.
    pub fn get_iter<I: IntoIterator>(values: I) -> I::IntoIter {
        values.into_iter()
    }

    /// Returns whether a slice has no values.
    #[must_use]
    pub const fn is_empty<T>(values: &[T]) -> bool {
        values.is_empty()
    }

    /// Returns whether a slice has at least one value.
    #[must_use]
    pub const fn is_not_empty<T>(values: &[T]) -> bool {
        !values.is_empty()
    }

    /// Returns whether at least one explicit optional value is absent.
    #[must_use]
    pub fn has_none<T>(values: impl IntoIterator<Item = Option<T>>) -> bool {
        values.into_iter().any(|value| value.is_none())
    }

    /// Returns whether every explicit optional value is absent.
    #[must_use]
    pub fn is_all_none<T>(values: impl IntoIterator<Item = Option<T>>) -> bool {
        values.into_iter().all(|value| value.is_none())
    }

    /// Counts occurrences of every value.
    #[must_use]
    pub fn count_map<T>(values: impl IntoIterator<Item = T>) -> HashMap<T, usize>
    where
        T: Eq + Hash,
    {
        let mut counts = HashMap::new();
        for value in values {
            *counts.entry(value).or_insert(0) += 1;
        }
        counts
    }

    /// Builds a key-to-value map using explicit accessors instead of reflection.
    #[must_use]
    pub fn field_value_map<T, K>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, T>
    where
        K: Eq + Hash,
    {
        values
            .into_iter()
            .map(|value| (key_of(&value), value))
            .collect()
    }

    /// Extracts one projected field from each value.
    #[must_use]
    pub fn field_value_list<T, V>(
        values: impl IntoIterator<Item = T>,
        field_of: impl FnMut(T) -> V,
    ) -> Vec<V> {
        values.into_iter().map(field_of).collect()
    }

    /// Joins displayed values with a delimiter.
    #[must_use]
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String {
        Self::join_by(values, delimiter, |value| value.to_string())
    }

    /// Joins projected values with a delimiter.
    #[must_use]
    pub fn join_by<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        mut display: impl FnMut(T) -> String,
    ) -> String {
        values
            .into_iter()
            .map(&mut display)
            .collect::<Vec<_>>()
            .join(delimiter)
    }

    /// Joins values while wrapping every element with a prefix and suffix.
    #[must_use]
    pub fn join_wrapped<T: Display>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
    ) -> String {
        Self::join_by(values, delimiter, |value| {
            format!("{prefix}{value}{suffix}")
        })
    }

    /// Collects entry pairs into a map; later duplicate keys replace earlier values.
    #[must_use]
    pub fn entries_to_map<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        entries.into_iter().collect()
    }

    /// Maps keys to positional optional values, padding missing values with `None`.
    #[must_use]
    pub fn keys_with_values<K, V>(
        keys: impl IntoIterator<Item = K>,
        values: impl IntoIterator<Item = V>,
    ) -> HashMap<K, Option<V>>
    where
        K: Eq + Hash,
    {
        let mut values = values.into_iter();
        keys.into_iter().map(|key| (key, values.next())).collect()
    }

    /// Groups projected values by a generated key.
    #[must_use]
    pub fn to_list_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(T) -> V,
    ) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut grouped = HashMap::<K, Vec<V>>::new();
        for value in values {
            grouped
                .entry(key_of(&value))
                .or_default()
                .push(value_of(value));
        }
        grouped
    }

    /// Maps values to generated keys and projected values.
    #[must_use]
    pub fn to_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(T) -> V,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        values
            .into_iter()
            .map(|value| (key_of(&value), value_of(value)))
            .collect()
    }

    /// Collects an iterator into a vector.
    #[must_use]
    pub fn to_list<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Consumes values until `index` and returns that item.
    pub fn get<T>(values: impl IntoIterator<Item = T>, index: usize) -> Option<T> {
        values.into_iter().nth(index)
    }

    /// Returns the first value.
    pub fn first<T>(values: impl IntoIterator<Item = T>) -> Option<T> {
        values.into_iter().next()
    }

    /// Returns the first present optional value.
    pub fn first_some<T>(values: impl IntoIterator<Item = Option<T>>) -> Option<T> {
        values.into_iter().flatten().next()
    }

    /// Returns the first value accepted by `matcher`.
    pub fn first_match<T>(
        values: impl IntoIterator<Item = T>,
        mut matcher: impl FnMut(&T) -> bool,
    ) -> Option<T> {
        values.into_iter().find(&mut matcher)
    }

    /// Returns the static element type when a slice is non-empty.
    #[must_use]
    pub fn element_type<T>(values: &[T]) -> Option<&'static str> {
        (!values.is_empty()).then_some(type_name::<T>())
    }

    /// Projects and filters values in one pass.
    #[must_use]
    pub fn edit<T, U>(
        values: impl IntoIterator<Item = T>,
        editor: impl FnMut(T) -> Option<U>,
    ) -> Vec<U> {
        values.into_iter().filter_map(editor).collect()
    }

    /// Retains accepted values in place.
    pub fn filter<T>(values: &mut Vec<T>, filter: impl FnMut(&T) -> bool) {
        values.retain(filter);
    }

    /// Collects accepted values into a new list.
    #[must_use]
    pub fn filter_to_list<T>(
        values: impl IntoIterator<Item = T>,
        filter: impl FnMut(&T) -> bool,
    ) -> Vec<T> {
        values.into_iter().filter(filter).collect()
    }

    /// Creates a lazy filtering iterator with lookahead.
    #[must_use]
    pub fn filtered<I, P>(values: I, filter: Option<P>) -> FilterIter<I, P>
    where
        I: Iterator,
        P: FnMut(&I::Item) -> bool,
    {
        FilterIter::new(values, filter)
    }

    /// Creates a lazy transforming iterator.
    #[must_use]
    pub fn trans<I, F>(values: I, transform: F) -> TransIter<I, F>
    where
        I: Iterator,
    {
        TransIter::new(values, transform)
    }

    /// Returns an empty iterator.
    pub fn empty<T>() -> std::iter::Empty<T> {
        std::iter::empty()
    }

    /// Consumes and counts all values.
    pub fn size(values: impl IntoIterator) -> usize {
        values.into_iter().count()
    }

    /// Compares iteration length, element equality, and order.
    #[must_use]
    pub fn is_equal_list<T: PartialEq>(
        left: impl IntoIterator<Item = T>,
        right: impl IntoIterator<Item = T>,
    ) -> bool {
        left.into_iter().eq(right)
    }

    /// Clears a mutable vector.
    pub fn clear<T>(values: &mut Vec<T>) {
        values.clear();
    }

    /// Consumes every value with a caller-provided action.
    pub fn for_each<T>(values: impl IntoIterator<Item = T>, action: impl FnMut(T)) {
        values.into_iter().for_each(action);
    }

    /// Formats values using Hutool's default list representation.
    #[must_use]
    pub fn to_string<T: Display>(values: impl IntoIterator<Item = T>) -> String {
        Self::to_string_by(values, |value| value.to_string(), ", ", "[", "]")
    }

    /// Formats projected values with configurable delimiters.
    #[must_use]
    pub fn to_string_by<T>(
        values: impl IntoIterator<Item = T>,
        display: impl FnMut(T) -> String,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
    ) -> String {
        format!(
            "{prefix}{}{suffix}",
            Self::join_by(values, delimiter, display)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Record {
        id: u32,
        group: &'static str,
        value: i32,
    }

    #[test]
    fn iter_util_matches_hutool_consumption_mapping_and_formatting() {
        assert_eq!(IterUtil::get_iter([1, 2]).collect::<Vec<_>>(), [1, 2]);
        assert!(IterUtil::is_empty::<i32>(&[]));
        assert!(IterUtil::is_not_empty(&[1]));
        assert!(IterUtil::has_none([Some(1), None]));
        assert!(!IterUtil::has_none([Some(1), Some(2)]));
        assert!(IterUtil::is_all_none::<i32>([None, None]));
        assert!(!IterUtil::is_all_none([None, Some(1)]));

        let counts = IterUtil::count_map(["a", "b", "a"]);
        assert_eq!(counts.get("a"), Some(&2));

        let records = || {
            vec![
                Record {
                    id: 1,
                    group: "a",
                    value: 10,
                },
                Record {
                    id: 2,
                    group: "a",
                    value: 20,
                },
                Record {
                    id: 1,
                    group: "b",
                    value: 30,
                },
            ]
        };
        let keyed = IterUtil::field_value_map(records(), |record| record.id);
        assert_eq!(keyed.get(&1).unwrap().value, 30);
        assert_eq!(
            IterUtil::field_value_list(records(), |record| record.value),
            [10, 20, 30]
        );
        assert_eq!(IterUtil::join([1, 2, 3], "/"), "1/2/3");
        assert_eq!(
            IterUtil::join_by([1, 2], ",", |value| format!("x{value}")),
            "x1,x2"
        );
        assert_eq!(IterUtil::join_wrapped([1, 2], ",", "<", ">"), "<1>,<2>");

        assert_eq!(IterUtil::entries_to_map([("a", 1)]).get("a"), Some(&1));
        let paired = IterUtil::keys_with_values(["a", "b", "c"], [1, 2]);
        assert_eq!(paired.get("c"), Some(&None));
        let grouped =
            IterUtil::to_list_map(records(), |record| record.group, |record| record.value);
        assert_eq!(grouped.get("a"), Some(&vec![10, 20]));
        let mapped = IterUtil::to_map(records(), |record| record.id, |record| record.value);
        assert_eq!(mapped.get(&1), Some(&30));

        assert_eq!(IterUtil::to_list(1..=3), [1, 2, 3]);
        assert_eq!(IterUtil::get(5..10, 2), Some(7));
        assert_eq!(IterUtil::get(0..1, 9), None);
        assert_eq!(IterUtil::first([4, 5]), Some(4));
        assert_eq!(IterUtil::first::<i32>([]), None);
        assert_eq!(IterUtil::first_some([None, Some(8), Some(9)]), Some(8));
        assert_eq!(IterUtil::first_match(1..5, |value| value % 2 == 0), Some(2));
        assert_eq!(IterUtil::first_match(1..3, |value| *value > 9), None);
        assert!(IterUtil::element_type(&[1]).unwrap().ends_with("i32"));
        assert_eq!(IterUtil::element_type::<i32>(&[]), None);

        assert_eq!(
            IterUtil::edit(0..5, |value| (value % 2 == 0).then_some(value * 2)),
            [0, 4, 8]
        );
        let mut filtered = vec![1, 2, 3, 4];
        IterUtil::filter(&mut filtered, |value| value % 2 == 0);
        assert_eq!(filtered, [2, 4]);
        assert_eq!(
            IterUtil::filter_to_list(1..5, |value| value % 2 == 1),
            [1, 3]
        );
        assert_eq!(
            IterUtil::filtered(1..=4, Some(|value: &i32| value % 2 == 0)).collect::<Vec<_>>(),
            [2, 4]
        );
        assert_eq!(
            IterUtil::trans(1..=2, |value| value * 3).collect::<Vec<_>>(),
            [3, 6]
        );
        assert_eq!(IterUtil::empty::<i32>().next(), None);
        assert_eq!(IterUtil::size(1..=4), 4);
        assert!(IterUtil::is_equal_list([1, 2], [1, 2]));
        assert!(!IterUtil::is_equal_list([1, 2], [2, 1]));
        assert!(!IterUtil::is_equal_list([1], [1, 2]));

        let mut clearable = vec![1, 2];
        IterUtil::clear(&mut clearable);
        assert!(clearable.is_empty());
        let mut total = 0;
        IterUtil::for_each(1..=3, |value| total += value);
        assert_eq!(total, 6);
        assert_eq!(IterUtil::to_string([1, 2]), "[1, 2]");
        assert_eq!(
            IterUtil::to_string_by([1, 2], |value| format!("x{value}"), "|", "<", ">"),
            "<x1|x2>"
        );
        assert!(format!("{IterUtil:?}").contains("IterUtil"));
    }
}
