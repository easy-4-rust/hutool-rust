//! General collection operations aligned with Hutool's `CollUtil` capability model.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    sync::{
        Arc,
        mpsc::{Receiver, RecvError, SendError, SyncSender, sync_channel},
    },
};

use indexmap::{IndexMap, IndexSet};
use parking_lot::Mutex;

use crate::{CoreError, IterUtil, ListUtil, Result};

use super::blocking_queue::BlockingQueue;
use super::collection_kind::CollectionKind;
use super::created_collection::CreatedCollection;

/// Hutool-aligned collection operations with Rust-native static typing.
#[derive(Debug, Clone, Copy, Default)]
pub struct CollUtil;

impl CollUtil {
    /// Returns the supplied slice or an empty slice when it is absent.
    #[must_use]
    pub fn empty_if_none<T>(values: Option<&[T]>) -> &[T] {
        values.unwrap_or_default()
    }

    /// Multiset union: keeps the maximum occurrence count from either input.
    #[must_use]
    pub fn union<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        left.iter()
            .chain(right)
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .max(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Multiset union across any number of collections.
    #[must_use]
    pub fn union_many<T>(collections: &[&[T]]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        collections
            .iter()
            .fold(Vec::new(), |union, values| Self::union(&union, values))
    }

    /// Insertion-ordered distinct union across collections.
    #[must_use]
    pub fn union_distinct<T>(collections: &[&[T]]) -> IndexSet<T>
    where
        T: Clone + Eq + Hash,
    {
        collections
            .iter()
            .flat_map(|values| values.iter().cloned())
            .collect()
    }

    /// Concatenates every collection without deduplication.
    #[must_use]
    pub fn union_all<T: Clone>(collections: &[&[T]]) -> Vec<T> {
        collections
            .iter()
            .flat_map(|values| values.iter().cloned())
            .collect()
    }

    /// Multiset intersection: keeps the minimum occurrence count.
    #[must_use]
    pub fn intersection<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        right
            .iter()
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .min(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Multiset intersection across any number of collections.
    #[must_use]
    pub fn intersection_many<T>(collections: &[&[T]]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let Some((first, rest)) = collections.split_first() else {
            return Vec::new();
        };
        rest.iter().fold(first.to_vec(), |values, next| {
            Self::intersection(&values, next)
        })
    }

    /// Insertion-ordered distinct intersection across collections.
    #[must_use]
    pub fn intersection_distinct<T>(collections: &[&[T]]) -> IndexSet<T>
    where
        T: Clone + Eq + Hash,
    {
        let Some((first, rest)) = collections.split_first() else {
            return IndexSet::new();
        };
        first
            .iter()
            .filter(|value| rest.iter().all(|values| values.contains(value)))
            .cloned()
            .collect()
    }

    /// Symmetric multiset difference: keeps the absolute count difference.
    #[must_use]
    pub fn disjunction<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let left_counts = IterUtil::count_map(left);
        let right_counts = IterUtil::count_map(right);
        left.iter()
            .chain(right)
            .cloned()
            .collect::<IndexSet<_>>()
            .into_iter()
            .flat_map(|value| {
                let count = left_counts
                    .get(&value)
                    .copied()
                    .unwrap_or_default()
                    .abs_diff(right_counts.get(&value).copied().unwrap_or_default());
                std::iter::repeat_n(value, count)
            })
            .collect()
    }

    /// Removes every left value that occurs at least once in `right`.
    #[must_use]
    pub fn subtract<T>(left: &[T], right: &[T]) -> Vec<T>
    where
        T: Clone + Eq + Hash,
    {
        let removed: HashSet<_> = right.iter().collect();
        left.iter()
            .filter(|value| !removed.contains(value))
            .cloned()
            .collect()
    }

    /// Returns whether a value is present.
    #[must_use]
    pub fn contains<T: PartialEq>(values: &[T], value: &T) -> bool {
        values.contains(value)
    }

    /// Returns whether any value matches a predicate.
    #[must_use]
    pub fn contains_by<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        values.iter().any(matcher)
    }

    /// Returns whether the two slices share at least one value.
    #[must_use]
    pub fn contains_any<T: PartialEq>(left: &[T], right: &[T]) -> bool {
        left.iter().any(|value| right.contains(value))
    }

    /// Returns whether every distinct right value appears in the left slice.
    #[must_use]
    pub fn contains_all<T: PartialEq>(left: &[T], right: &[T]) -> bool {
        right.iter().all(|value| left.contains(value))
    }

    /// Counts occurrences of values.
    #[must_use]
    pub fn count_map<T>(values: impl IntoIterator<Item = T>) -> HashMap<T, usize>
    where
        T: Eq + Hash,
    {
        IterUtil::count_map(values)
    }

    /// Joins displayed values.
    #[must_use]
    pub fn join<T: Display>(values: impl IntoIterator<Item = T>, delimiter: &str) -> String {
        IterUtil::join(values, delimiter)
    }

    /// Joins projected values.
    #[must_use]
    pub fn join_by<T>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        display: impl FnMut(T) -> String,
    ) -> String {
        IterUtil::join_by(values, delimiter, display)
    }

    /// Joins values with a per-element prefix and suffix.
    #[must_use]
    pub fn join_wrapped<T: Display>(
        values: impl IntoIterator<Item = T>,
        delimiter: &str,
        prefix: &str,
        suffix: &str,
    ) -> String {
        IterUtil::join_wrapped(values, delimiter, prefix, suffix)
    }

    /// Removes up to `part_size` values from the front of a deque.
    #[must_use]
    pub fn pop_part<T>(values: &mut VecDeque<T>, part_size: usize) -> Vec<T> {
        (0..part_size.min(values.len()))
            .filter_map(|_| values.pop_front())
            .collect()
    }

    /// Hutool-compatible `anyMatch`: empty input returns `false`.
    #[must_use]
    pub fn any_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        values.iter().any(matcher)
    }

    /// Hutool-compatible `allMatch`: empty input returns `false`.
    #[must_use]
    pub fn all_match<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> bool {
        !values.is_empty() && values.iter().all(matcher)
    }

    /// Collects values into a hash set.
    #[must_use]
    pub fn new_hash_set<T>(values: impl IntoIterator<Item = T>) -> HashSet<T>
    where
        T: Eq + Hash,
    {
        values.into_iter().collect()
    }

    /// Collects values into an insertion-ordered set.
    #[must_use]
    pub fn new_linked_hash_set<T>(values: impl IntoIterator<Item = T>) -> IndexSet<T>
    where
        T: Eq + Hash,
    {
        values.into_iter().collect()
    }

    /// Collects values into a vector.
    #[must_use]
    pub fn new_array_list<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Collects values into a double-ended queue.
    #[must_use]
    pub fn new_linked_list<T>(values: impl IntoIterator<Item = T>) -> VecDeque<T> {
        values.into_iter().collect()
    }

    /// Creates an atomically shared copy-on-write list.
    #[must_use]
    pub fn new_copy_on_write_array_list<T>(values: impl IntoIterator<Item = T>) -> Arc<Vec<T>> {
        Arc::new(values.into_iter().collect())
    }

    /// Obtains the mutable copy-on-write view, cloning only when the list is shared.
    pub fn copy_on_write_mut<T: Clone>(values: &mut Arc<Vec<T>>) -> &mut Vec<T> {
        Arc::make_mut(values)
    }

    /// Creates a bounded blocking queue.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when capacity is zero.
    pub fn new_blocking_queue<T>(capacity: usize) -> Result<BlockingQueue<T>> {
        if capacity == 0 {
            return Err(CoreError::InvalidArgument {
                name: "capacity",
                reason: "must be greater than zero",
            });
        }
        let (sender, receiver) = sync_channel(capacity);
        Ok(BlockingQueue {
            sender,
            receiver: Mutex::new(receiver),
        })
    }

    /// Creates an empty concrete collection without runtime reflection.
    #[must_use]
    pub fn create<T>(kind: CollectionKind) -> CreatedCollection<T> {
        match kind {
            CollectionKind::List => CreatedCollection::List(Vec::new()),
            CollectionKind::Deque => CreatedCollection::Deque(VecDeque::new()),
            CollectionKind::Set => CreatedCollection::Set(HashSet::new()),
            CollectionKind::OrderedSet => CreatedCollection::OrderedSet(IndexSet::new()),
            CollectionKind::SortedSet => CreatedCollection::SortedSet(BTreeSet::new()),
        }
    }

    /// Stable equality-based deduplication.
    #[must_use]
    pub fn distinct<T>(values: impl IntoIterator<Item = T>) -> Vec<T>
    where
        T: Eq + Hash,
    {
        values
            .into_iter()
            .collect::<IndexSet<_>>()
            .into_iter()
            .collect()
    }

    /// Stable deduplication by a derived key, optionally replacing earlier values.
    #[must_use]
    pub fn distinct_by<T, K>(
        values: impl IntoIterator<Item = T>,
        mut key_of: impl FnMut(&T) -> K,
        replace: bool,
    ) -> Vec<T>
    where
        K: Eq + Hash,
    {
        let mut unique = IndexMap::new();
        for value in values {
            let key = key_of(&value);
            if replace || !unique.contains_key(&key) {
                unique.insert(key, value);
            }
        }
        unique.into_values().collect()
    }

    /// Returns an owned Hutool-style signed sub-list.
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>> {
        ListUtil::sub(values, start, end, step)
    }

    /// Splits values into owned fixed-size segments.
    pub fn split<T: Clone>(values: &[T], size: usize) -> Result<Vec<Vec<T>>> {
        if size == 0 {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "must be greater than zero",
            });
        }
        Ok(values.chunks(size).map(<[T]>::to_vec).collect())
    }

    /// Edits values and drops rejected results.
    #[must_use]
    pub fn edit<T, U>(
        values: impl IntoIterator<Item = T>,
        editor: impl FnMut(T) -> Option<U>,
    ) -> Vec<U> {
        IterUtil::edit(values, editor)
    }

    /// Filters values into a new vector.
    #[must_use]
    pub fn filter_new<T>(
        values: impl IntoIterator<Item = T>,
        predicate: impl FnMut(&T) -> bool,
    ) -> Vec<T> {
        IterUtil::filter_to_list(values, predicate)
    }

    /// Retains accepted values in place.
    pub fn filter<T>(values: &mut Vec<T>, predicate: impl FnMut(&T) -> bool) {
        values.retain(predicate);
    }

    /// Removes all values included in `removed`.
    pub fn remove_any<T>(values: &mut Vec<T>, removed: &[T])
    where
        T: Eq + Hash,
    {
        let removed: HashSet<_> = removed.iter().collect();
        values.retain(|value| !removed.contains(value));
    }

    /// Removes absent optional values.
    pub fn remove_none<T>(values: &mut Vec<Option<T>>) {
        values.retain(Option::is_some);
    }

    /// Removes empty strings.
    pub fn remove_empty(values: &mut Vec<String>) {
        values.retain(|value| !value.is_empty());
    }

    /// Removes blank strings.
    pub fn remove_blank(values: &mut Vec<String>) {
        values.retain(|value| !value.trim().is_empty());
    }

    /// Moves matching elements out of a vector and returns them.
    #[must_use]
    pub fn remove_with_add_if<T>(
        values: &mut Vec<T>,
        mut predicate: impl FnMut(&T) -> bool,
    ) -> Vec<T> {
        let mut retained = Vec::with_capacity(values.len());
        let mut removed = Vec::new();
        for value in values.drain(..) {
            if predicate(&value) {
                removed.push(value);
            } else {
                retained.push(value);
            }
        }
        *values = retained;
        removed
    }
}

impl CollUtil {
    /// Maps values while preserving or dropping explicit absence.
    #[must_use]
    pub fn map_optional<T, U>(
        values: impl IntoIterator<Item = Option<T>>,
        ignore_none: bool,
        mut mapper: impl FnMut(Option<T>) -> Option<U>,
    ) -> Vec<Option<U>> {
        values
            .into_iter()
            .filter_map(|value| {
                if ignore_none && value.is_none() {
                    None
                } else {
                    let mapped_value = mapper(value);
                    (!ignore_none || mapped_value.is_some()).then_some(mapped_value)
                }
            })
            .collect()
    }

    /// Extracts explicit fields using a compile-time accessor instead of reflection.
    #[must_use]
    pub fn field_values<T, V>(
        values: impl IntoIterator<Item = T>,
        accessor: impl FnMut(T) -> V,
    ) -> Vec<V> {
        values.into_iter().map(accessor).collect()
    }

    /// Indexes objects by an explicit field accessor; later keys replace earlier ones.
    #[must_use]
    pub fn field_value_map<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, T>
    where
        K: Eq + Hash,
    {
        IterUtil::field_value_map(values, key_of)
    }

    /// Maps one explicit field to another without runtime reflection.
    #[must_use]
    pub fn field_value_as_map<T, K, V>(
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

    /// Finds the first matching owned value.
    pub fn find_one<T>(
        values: impl IntoIterator<Item = T>,
        mut matcher: impl FnMut(&T) -> bool,
    ) -> Option<T> {
        values.into_iter().find(&mut matcher)
    }

    /// Finds the first value whose projected field equals `expected`.
    pub fn find_one_by<T, V: PartialEq>(
        values: impl IntoIterator<Item = T>,
        mut field_of: impl FnMut(&T) -> V,
        expected: &V,
    ) -> Option<T> {
        values
            .into_iter()
            .find(|value| field_of(value) == *expected)
    }

    /// Counts matching values.
    #[must_use]
    pub fn count<T>(values: impl IntoIterator<Item = T>, matcher: impl FnMut(&T) -> bool) -> usize {
        values.into_iter().filter(matcher).count()
    }

    /// Returns the first matching index.
    #[must_use]
    pub fn index_of<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Option<usize> {
        values.iter().position(matcher)
    }

    /// Returns the last matching index.
    #[must_use]
    pub fn last_index_of<T>(values: &[T], matcher: impl FnMut(&T) -> bool) -> Option<usize> {
        values.iter().rposition(matcher)
    }

    /// Returns every matching index.
    #[must_use]
    pub fn index_of_all<T>(values: &[T], mut matcher: impl FnMut(&T) -> bool) -> Vec<usize> {
        values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| matcher(value).then_some(index))
            .collect()
    }

    /// Returns whether an optional slice is absent or empty.
    #[must_use]
    pub fn is_empty<T>(values: Option<&[T]>) -> bool {
        values.is_none_or(<[T]>::is_empty)
    }

    /// Returns whether an optional slice is present and non-empty.
    #[must_use]
    pub fn is_not_empty<T>(values: Option<&[T]>) -> bool {
        !Self::is_empty(values)
    }

    /// Returns `values` unless empty, otherwise evaluates `default`.
    #[must_use]
    pub fn default_if_empty<T>(values: Vec<T>, default: impl FnOnce() -> Vec<T>) -> Vec<T> {
        if values.is_empty() { default() } else { values }
    }

    /// Returns whether any optional element is absent.
    #[must_use]
    pub fn has_none<T>(values: impl IntoIterator<Item = Option<T>>) -> bool {
        values.into_iter().any(|value| value.is_none())
    }

    /// Zips keys and values to the shorter length; later duplicate keys replace earlier ones.
    #[must_use]
    pub fn zip<K, V>(
        keys: impl IntoIterator<Item = K>,
        values: impl IntoIterator<Item = V>,
    ) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        keys.into_iter().zip(values).collect()
    }

    /// Splits delimited key/value strings and zips them.
    #[must_use]
    pub fn zip_strings(keys: &str, values: &str, delimiter: &str) -> HashMap<String, String> {
        Self::zip(
            keys.split(delimiter).map(str::to_owned),
            values.split(delimiter).map(str::to_owned),
        )
    }

    /// Collects entries to a map.
    #[must_use]
    pub fn entries_to_map<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        entries.into_iter().collect()
    }

    /// Creates a sorted set.
    #[must_use]
    pub fn to_tree_set<T: Ord>(values: impl IntoIterator<Item = T>) -> BTreeSet<T> {
        values.into_iter().collect()
    }

    /// Converts an iterable to an owned vector; iterator/enumeration adapters are native iterators.
    #[must_use]
    pub fn to_collection<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Converts row maps to a column map of lists.
    #[must_use]
    pub fn to_list_map<K, V>(rows: impl IntoIterator<Item = HashMap<K, V>>) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut columns = HashMap::<K, Vec<V>>::new();
        for row in rows {
            for (key, value) in row {
                columns.entry(key).or_default().push(value);
            }
        }
        columns
    }

    /// Converts a column map of lists to row maps, omitting exhausted columns.
    #[must_use]
    pub fn to_map_list<K, V>(columns: HashMap<K, Vec<V>>) -> Vec<HashMap<K, V>>
    where
        K: Clone + Eq + Hash,
    {
        let row_count = columns.values().map(Vec::len).max().unwrap_or_default();
        let mut rows: Vec<_> = std::iter::repeat_with(HashMap::new)
            .take(row_count)
            .collect();
        for (key, values) in columns {
            for (index, value) in values.into_iter().enumerate() {
                rows[index].insert(key.clone(), value);
            }
        }
        rows
    }

    /// Maps objects to derived keys and preserves each object as the value.
    #[must_use]
    pub fn to_identity_map<T, K>(
        values: impl IntoIterator<Item = T>,
        key_of: impl FnMut(&T) -> K,
    ) -> HashMap<K, T>
    where
        K: Eq + Hash,
    {
        Self::field_value_map(values, key_of)
    }

    /// Maps objects to derived key/value pairs.
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

    /// Adds a non-absent value only when it is not already present.
    pub fn add_if_absent<T: PartialEq>(values: &mut Vec<T>, value: Option<T>) -> bool {
        let Some(value) = value else { return false };
        if values.contains(&value) {
            false
        } else {
            values.push(value);
            true
        }
    }

    /// Extends a collection from any iterator.
    pub fn add_all<T>(target: &mut Vec<T>, values: impl IntoIterator<Item = T>) {
        target.extend(values);
    }

    /// Parses Hutool's comma-list string form and appends converted values.
    pub fn add_all_from_str<T, E>(
        target: &mut Vec<T>,
        values: &str,
        mut convert: impl FnMut(&str) -> std::result::Result<T, E>,
    ) -> std::result::Result<(), E> {
        let values = values
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
            .unwrap_or(values);
        target.extend(
            values
                .split(',')
                .map(str::trim)
                .map(&mut convert)
                .collect::<std::result::Result<Vec<_>, _>>()?,
        );
        Ok(())
    }

    /// Appends only values not already present in the target.
    pub fn add_all_if_not_contains<T: Clone + PartialEq>(target: &mut Vec<T>, values: &[T]) {
        for value in values {
            if !target.contains(value) {
                target.push(value.clone());
            }
        }
    }

    /// Gets a value using a signed index; negative positions count from the end.
    #[must_use]
    pub fn get<T>(values: &[T], index: isize) -> Option<&T> {
        let index = if index < 0 {
            values.len().checked_add_signed(index)?
        } else {
            index.unsigned_abs()
        };
        values.get(index)
    }

    /// Gets several signed indices, returning `None` for out-of-range positions.
    #[must_use]
    pub fn get_any<'a, T>(values: &'a [T], indexes: &[isize]) -> Vec<Option<&'a T>> {
        indexes
            .iter()
            .map(|index| Self::get(values, *index))
            .collect()
    }

    /// Gets the first value.
    #[must_use]
    pub fn first<T>(values: &[T]) -> Option<&T> {
        values.first()
    }

    /// Gets the last value.
    #[must_use]
    pub fn last<T>(values: &[T]) -> Option<&T> {
        values.last()
    }

    /// Returns the static element type for non-empty slices.
    #[must_use]
    pub fn element_type<T>(values: &[T]) -> Option<&'static str> {
        IterUtil::element_type(values)
    }

    /// Looks up keys in order, retaining missing entries explicitly as `None`.
    #[must_use]
    pub fn values_of_keys<'a, K, V>(
        map: &'a HashMap<K, V>,
        keys: impl IntoIterator<Item = &'a K>,
    ) -> Vec<Option<&'a V>>
    where
        K: Eq + Hash + 'a,
    {
        keys.into_iter().map(|key| map.get(key)).collect()
    }
}

impl CollUtil {
    /// Sorts all supplied collections and returns one zero-based page.
    pub fn sort_page_all<T: Clone>(
        page_no: usize,
        page_size: usize,
        collections: &[&[T]],
        compare: impl FnMut(&T, &T) -> Ordering,
    ) -> Result<Vec<T>> {
        let mut values = Self::union_all(collections);
        values.sort_by(compare);
        Ok(ListUtil::page(&values, page_no, page_size)?.to_vec())
    }

    /// Returns one zero-based owned page.
    pub fn page<T: Clone>(values: &[T], page_no: usize, page_size: usize) -> Result<Vec<T>> {
        Ok(ListUtil::page(values, page_no, page_size)?.to_vec())
    }

    /// Returns a sorted clone without modifying the input.
    #[must_use]
    pub fn sort<T: Clone>(values: &[T], mut compare: impl FnMut(&T, &T) -> Ordering) -> Vec<T> {
        let mut sorted = values.to_vec();
        sorted.sort_by(&mut compare);
        sorted
    }

    /// Sorts a list in place.
    pub fn sort_in_place<T>(values: &mut [T], compare: impl FnMut(&T, &T) -> Ordering) {
        values.sort_by(compare);
    }

    /// Sorts by an explicit property accessor instead of Java reflection.
    #[must_use]
    pub fn sort_by_property<T: Clone, K: Ord>(
        values: &[T],
        mut property: impl FnMut(&T) -> K,
    ) -> Vec<T> {
        Self::sort(values, |left, right| property(left).cmp(&property(right)))
    }

    /// Sorts strings by a caller-selected collation key (for example a pinyin crate).
    #[must_use]
    pub fn sort_strings_by_key<K: Ord>(
        values: &[String],
        mut collation_key: impl FnMut(&str) -> K,
    ) -> Vec<String> {
        Self::sort(values, |left, right| {
            collation_key(left).cmp(&collation_key(right))
        })
    }

    /// Converts a map to a key-ordered map.
    #[must_use]
    pub fn sort_map<K: Ord, V>(values: impl IntoIterator<Item = (K, V)>) -> BTreeMap<K, V> {
        values.into_iter().collect()
    }

    /// Sorts entries with an arbitrary comparator and preserves that order.
    #[must_use]
    pub fn sort_entries<K, V>(
        entries: impl IntoIterator<Item = (K, V)>,
        mut compare: impl FnMut(&(K, V), &(K, V)) -> Ordering,
    ) -> IndexMap<K, V>
    where
        K: Eq + Hash,
    {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_by(&mut compare);
        entries.into_iter().collect()
    }

    /// Sorts entries by their values.
    #[must_use]
    pub fn sort_entries_by_value<K, V: Ord>(
        entries: impl IntoIterator<Item = (K, V)>,
    ) -> Vec<(K, V)> {
        let mut entries: Vec<_> = entries.into_iter().collect();
        entries.sort_by(|left, right| left.1.cmp(&right.1));
        entries
    }

    /// Visits values with a stable zero-based index.
    pub fn for_each<T>(values: impl IntoIterator<Item = T>, mut consumer: impl FnMut(T, usize)) {
        values
            .into_iter()
            .enumerate()
            .for_each(|(index, value)| consumer(value, index));
    }

    /// Visits map entries with a stable zero-based iteration index.
    pub fn for_each_map<K, V>(
        values: impl IntoIterator<Item = (K, V)>,
        mut consumer: impl FnMut(K, V, usize),
    ) {
        values
            .into_iter()
            .enumerate()
            .for_each(|(index, (key, value))| consumer(key, value, index));
    }

    /// Places values in sparse groups selected by a non-negative index.
    pub fn group<T>(
        values: impl IntoIterator<Item = T>,
        mut group_index: impl FnMut(&T) -> usize,
    ) -> Vec<Vec<T>> {
        let mut groups = Vec::<Vec<T>>::new();
        for value in values {
            let index = group_index(&value);
            if groups.len() <= index {
                groups.resize_with(index + 1, Vec::new);
            }
            groups[index].push(value);
        }
        groups
    }

    /// Groups values by first-seen explicit field values instead of reflection.
    #[must_use]
    pub fn group_by_field<T, K>(
        values: impl IntoIterator<Item = T>,
        mut field_of: impl FnMut(&T) -> K,
    ) -> Vec<Vec<T>>
    where
        K: Eq + Hash,
    {
        let mut indexes = HashMap::<K, usize>::new();
        let mut groups = Vec::<Vec<T>>::new();
        for value in values {
            let next_index = indexes.len();
            let index = *indexes.entry(field_of(&value)).or_insert(next_index);
            if groups.len() == index {
                groups.push(Vec::new());
            }
            groups[index].push(value);
        }
        groups
    }

    /// Reverses values in place.
    pub fn reverse<T>(values: &mut [T]) {
        values.reverse();
    }

    /// Returns a reversed clone.
    #[must_use]
    pub fn reverse_new<T: Clone>(values: &[T]) -> Vec<T> {
        values.iter().rev().cloned().collect()
    }

    /// Replaces an existing index or appends when it is out of range.
    pub fn set_or_append<T>(values: &mut Vec<T>, index: usize, value: T) {
        ListUtil::set_or_append(values, index, value);
    }

    /// Returns the union of keys from a collection of maps.
    #[must_use]
    pub fn key_set<K, V>(maps: &[HashMap<K, V>]) -> HashSet<K>
    where
        K: Clone + Eq + Hash,
    {
        maps.iter().flat_map(|map| map.keys().cloned()).collect()
    }

    /// Returns every map value, preserving map iteration order.
    #[must_use]
    pub fn values<K, V: Clone>(maps: &[HashMap<K, V>]) -> Vec<V> {
        maps.iter().flat_map(|map| map.values().cloned()).collect()
    }

    /// Returns the maximum value, or `None` when empty.
    #[must_use]
    pub fn max<T: Ord>(values: &[T]) -> Option<&T> {
        values.iter().max()
    }

    /// Returns the minimum value, or `None` when empty.
    #[must_use]
    pub fn min<T: Ord>(values: &[T]) -> Option<&T> {
        values.iter().min()
    }

    /// Returns a read-only slice view.
    #[must_use]
    pub const fn unmodifiable<T>(values: &[T]) -> &[T] {
        values
    }

    /// Clears every supplied vector.
    pub fn clear<T>(collections: &mut [&mut Vec<T>]) {
        for values in collections.iter_mut() {
            values.clear();
        }
    }

    /// Prepends clones until a minimum length is reached.
    pub fn pad_left<T: Clone>(values: &mut Vec<T>, minimum_len: usize, padding: T) {
        if values.len() < minimum_len {
            let mut prefix = vec![padding; minimum_len - values.len()];
            prefix.append(values);
            *values = prefix;
        }
    }

    /// Appends clones until a minimum length is reached.
    pub fn pad_right<T: Clone>(values: &mut Vec<T>, minimum_len: usize, padding: T) {
        values.resize(minimum_len.max(values.len()), padding);
    }

    /// Lazily transforms a collection using Rust's native iterator view.
    pub fn trans<I, F, U>(values: I, transform: F) -> std::iter::Map<I::IntoIter, F>
    where
        I: IntoIterator,
        F: FnMut(I::Item) -> U,
    {
        values.into_iter().map(transform)
    }

    /// Applies mapped values to elements whose generated key exists in the map.
    pub fn set_value_by_map<T, K, V>(
        values: impl IntoIterator<Item = T>,
        map: &HashMap<K, V>,
        mut key_of: impl FnMut(&T) -> K,
        mut apply: impl FnMut(T, &V),
    ) where
        K: Eq + Hash,
    {
        for value in values {
            if let Some(mapped) = map.get(&key_of(&value)) {
                apply(value, mapped);
            }
        }
    }

    /// Counts values in any iterator. Runtime object inspection is unnecessary in Rust.
    #[must_use]
    pub fn size<T>(values: impl IntoIterator<Item = T>) -> usize {
        values.into_iter().count()
    }

    /// Returns whether two iterables have equal values in equal order.
    #[must_use]
    pub fn is_equal_list<T: PartialEq>(
        left: impl IntoIterator<Item = T>,
        right: impl IntoIterator<Item = T>,
    ) -> bool {
        left.into_iter().eq(right)
    }
}
