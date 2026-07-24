//! Stream-style collection transformations aligned with Hutool's `CollStreamUtil`.

/// 对齐: `cn.hutool.core.collection.CollStreamUtil`
/// 集合流工具类

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Hutool-aligned grouping, mapping, and map-merging operations.
///
/// Java's sequential/parallel overloads collapse into iterator-based Rust APIs:
/// callers choose the iterator implementation, while these operations retain
/// Hutool's duplicate-key and filtering semantics.
#[derive(Debug, Clone, Copy, Default)]
pub struct CollStreamUtil;

impl CollStreamUtil {
    /// Indexes values by a derived key. A later duplicate replaces the earlier value.
    #[must_use]
    pub fn to_identity_map<T, K>(
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

    /// Maps source values to key/value pairs. A later duplicate replaces the earlier value.
    #[must_use]
    pub fn to_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(&T) -> V,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        values
            .into_iter()
            .map(|value| (key_of(&value), value_of(&value)))
            .collect()
    }

    /// Groups owned source values by one derived key.
    #[must_use]
    pub fn group_by_key<T, K>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, Vec<T>>
    where
        K: Eq + Hash,
    {
        let mut groups = HashMap::<K, Vec<T>>::new();
        for value in values {
            groups.entry(key_of(&value)).or_default().push(value);
        }
        groups
    }

    /// Groups owned values into two nested key levels.
    #[must_use]
    pub fn group_by_two_keys<T, K, U>(
        values: impl IntoIterator<Item = T>,
        mut first_key: impl FnMut(&T) -> K,
        mut second_key: impl FnMut(&T) -> U,
    ) -> HashMap<K, HashMap<U, Vec<T>>>
    where
        K: Eq + Hash,
        U: Eq + Hash,
    {
        let mut groups = HashMap::<K, HashMap<U, Vec<T>>>::new();
        for value in values {
            groups
                .entry(first_key(&value))
                .or_default()
                .entry(second_key(&value))
                .or_default()
                .push(value);
        }
        groups
    }

    /// Groups values into a two-level map, preserving the first duplicate leaf value.
    #[must_use]
    pub fn group_to_two_level_map<T, K, U>(
        values: impl IntoIterator<Item = T>,
        mut first_key: impl FnMut(&T) -> K,
        mut second_key: impl FnMut(&T) -> U,
    ) -> HashMap<K, HashMap<U, T>>
    where
        K: Eq + Hash,
        U: Eq + Hash,
    {
        let mut groups = HashMap::<K, HashMap<U, T>>::new();
        for value in values {
            groups
                .entry(first_key(&value))
                .or_default()
                .entry(second_key(&value))
                .or_insert(value);
        }
        groups
    }

    /// Groups mapped values by a key derived from each source value.
    #[must_use]
    pub fn group_key_value<T, K, V>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut value_of: impl FnMut(&T) -> V,
    ) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut groups = HashMap::<K, Vec<V>>::new();
        for value in values {
            groups
                .entry(key_of(&value))
                .or_default()
                .push(value_of(&value));
        }
        groups
    }

    /// Groups values and folds each group into a caller-defined accumulator.
    #[must_use]
    pub fn group_fold<T, K, A>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        mut initial: impl FnMut() -> A,
        mut fold: impl FnMut(&mut A, T),
    ) -> HashMap<K, A>
    where
        K: Eq + Hash,
    {
        let mut groups = HashMap::<K, A>::new();
        for value in values {
            fold(
                groups.entry(key_of(&value)).or_insert_with(&mut initial),
                value,
            );
        }
        groups
    }

    /// Maps present results into a list, dropping absent (`None`) results.
    #[must_use]
    pub fn filter_map_to_list<T, U>(
        values: impl IntoIterator<Item = T>,
        mapper: impl FnMut(T) -> Option<U>,
    ) -> Vec<U> {
        values.into_iter().filter_map(mapper).collect()
    }

    /// Maps present results into a set, dropping absent results and duplicates.
    #[must_use]
    pub fn filter_map_to_set<T, U>(
        values: impl IntoIterator<Item = T>,
        mapper: impl FnMut(T) -> Option<U>,
    ) -> HashSet<U>
    where
        U: Eq + Hash,
    {
        values.into_iter().filter_map(mapper).collect()
    }

    /// Merges the union of two maps' keys.
    ///
    /// The merge function sees an absent side as `None`; returning `None`
    /// removes that key from the result, matching Hutool's null filtering.
    #[must_use]
    pub fn merge_maps<K, L, R, V>(
        left: &HashMap<K, L>,
        right: &HashMap<K, R>,
        mut merge: impl FnMut(&K, Option<&L>, Option<&R>) -> Option<V>,
    ) -> HashMap<K, V>
    where
        K: Clone + Eq + Hash,
    {
        left.keys()
            .chain(right.keys())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter_map(|key| {
                merge(&key, left.get(&key), right.get(&key)).map(|value| (key, value))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Item {
        id: u8,
        group: char,
        bucket: bool,
        value: i32,
    }

    fn items() -> Vec<Item> {
        vec![
            Item {
                id: 1,
                group: 'a',
                bucket: true,
                value: 10,
            },
            Item {
                id: 2,
                group: 'a',
                bucket: false,
                value: 20,
            },
            Item {
                id: 1,
                group: 'b',
                bucket: true,
                value: 30,
            },
        ]
    }

    #[test]
    fn coll_stream_util_matches_hutool_grouping_mapping_and_merge() {
        assert!(CollStreamUtil::to_identity_map(Vec::<Item>::new(), |item| item.id).is_empty());

        let indexed = CollStreamUtil::to_identity_map(items(), |item| item.id);
        assert_eq!(indexed.len(), 2);
        assert_eq!(indexed[&1].value, 30);

        let mapped = CollStreamUtil::to_map(items(), |item| item.id, |item| item.value);
        assert_eq!(mapped, HashMap::from([(1, 30), (2, 20)]));

        let grouped = CollStreamUtil::group_by_key(items(), |item| item.group);
        assert_eq!(
            grouped[&'a'].iter().map(|item| item.id).collect::<Vec<_>>(),
            [1, 2]
        );
        assert_eq!(grouped[&'b'][0].value, 30);

        let nested =
            CollStreamUtil::group_by_two_keys(items(), |item| item.group, |item| item.bucket);
        assert_eq!(nested[&'a'][&true][0].value, 10);
        assert_eq!(nested[&'a'][&false][0].value, 20);

        let first_wins = CollStreamUtil::group_to_two_level_map(items(), |_| "all", |item| item.id);
        assert_eq!(first_wins["all"][&1].value, 10);

        let key_values =
            CollStreamUtil::group_key_value(items(), |item| item.group, |item| item.value);
        assert_eq!(key_values[&'a'], [10, 20]);

        let sums = CollStreamUtil::group_fold(
            items(),
            |item| item.group,
            || 0,
            |sum, item| *sum += item.value,
        );
        assert_eq!(sums, HashMap::from([('a', 30), ('b', 30)]));

        let list =
            CollStreamUtil::filter_map_to_list(0..5, |value| (value % 2 == 0).then_some(value));
        assert_eq!(list, [0, 2, 4]);
        let set =
            CollStreamUtil::filter_map_to_set([1, 1, 2, 3], |value| (value < 3).then_some(value));
        assert_eq!(set, HashSet::from([1, 2]));

        let left = HashMap::from([("both", 2), ("left", 4), ("drop", 9)]);
        let right = HashMap::from([("both", 3), ("right", 5)]);
        let merged = CollStreamUtil::merge_maps(&left, &right, |key, left, right| {
            (key != &"drop")
                .then(|| left.copied().unwrap_or_default() + right.copied().unwrap_or_default())
        });
        assert_eq!(
            merged,
            HashMap::from([("both", 5), ("left", 4), ("right", 5)])
        );
    }
}
