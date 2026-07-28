//! 对齐: `cn.hutool.core.util.ArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ArrayUtil.java
//!
//! Rust 版本提供数组操作的 idiomatic 实现。

use std::collections::HashMap;

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ArrayUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ArrayUtil;

impl ArrayUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `ArrayUtil.isEmpty(T[])`
    pub fn is_empty<T>(array: &[T]) -> bool {
        array.is_empty()
    }

    /// 对齐 Java: `ArrayUtil.isNotEmpty(T[])`
    pub fn is_not_empty<T>(array: &[T]) -> bool {
        !array.is_empty()
    }

    // ── 默认值 ──

    /// 对齐 Java: `ArrayUtil.defaultIfEmpty(T[], T[])`
    pub fn default_if_empty<T: Clone>(array: &[T], default: &[T]) -> Vec<T> {
        if array.is_empty() {
            default.to_vec()
        } else {
            array.to_vec()
        }
    }

    // ── 查找操作 ──

    /// 对齐 Java: `ArrayUtil.contains(T[], T)`
    pub fn contains<T: PartialEq>(array: &[T], value: &T) -> bool {
        array.contains(value)
    }

    /// 对齐 Java: `ArrayUtil.indexOf(T[], T)`
    pub fn index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().position(|x| x == value)
    }

    /// 对齐 Java: `ArrayUtil.lastIndexOf(T[], T)`
    pub fn last_index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().rposition(|x| x == value)
    }

    // ── 转换操作 ──

    /// 对齐 Java: `ArrayUtil.toArray(T...)`
    pub fn to_vec<T>(values: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        values.to_vec()
    }

    /// 对齐 Java: `ArrayUtil.newArray(Class, int)`
    pub fn new_array<T: Clone + Default>(size: usize) -> Vec<T> {
        vec![T::default(); size]
    }

    // ── 操作 ──

    /// 对齐 Java: `ArrayUtil.append(T[], T...)`
    pub fn append<T: Clone>(array: &[T], elements: &[T]) -> Vec<T> {
        let mut result = array.to_vec();
        result.extend_from_slice(elements);
        result
    }

    /// 对齐 Java: `ArrayUtil.insert(T[], int, T...)`
    pub fn insert<T: Clone>(array: &[T], index: usize, elements: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(array.len() + elements.len());
        result.extend_from_slice(&array[..index]);
        result.extend_from_slice(elements);
        result.extend_from_slice(&array[index..]);
        result
    }

    /// 对齐 Java: `ArrayUtil.remove(T[], int)`
    pub fn remove<T: Clone>(array: &[T], index: usize) -> Vec<T> {
        let mut result = array.to_vec();
        if index < result.len() {
            result.remove(index);
        }
        result
    }

    /// 对齐 Java: `ArrayUtil.removeEle(T[], T)`
    pub fn remove_element<T: Clone + PartialEq>(array: &[T], element: &T) -> Vec<T> {
        array.iter().filter(|x| *x != element).cloned().collect()
    }

    // ── 排序操作 ──

    /// 对齐 Java: `ArrayUtil.reverse(T[])`
    pub fn reverse<T>(array: &mut [T]) {
        array.reverse();
    }

    /// 对齐 Java: `ArrayUtil.sort(T[])`
    pub fn sort<T: Ord>(array: &mut [T]) {
        array.sort();
    }

    /// 对齐 Java: `ArrayUtil.sort(T[], Comparator)`
    pub fn sort_by<T>(array: &mut [T], compare: impl FnMut(&T, &T) -> std::cmp::Ordering) {
        array.sort_by(compare);
    }

    // ── 合并操作 ──

    /// 对齐 Java: `ArrayUtil.addAll(T[]...)`
    pub fn add_all<T: Clone>(arrays: &[&[T]]) -> Vec<T> {
        let total_len: usize = arrays.iter().map(|a| a.len()).sum();
        let mut result = Vec::with_capacity(total_len);
        for array in arrays {
            result.extend_from_slice(array);
        }
        result
    }

    // ── 截取操作 ──

    /// 对齐 Java: `ArrayUtil.sub(T[], int, int)`
    pub fn sub<T: Clone>(array: &[T], start: usize, end: usize) -> Vec<T> {
        let start = start.min(array.len());
        let end = end.min(array.len());
        if start >= end {
            return Vec::new();
        }
        array[start..end].to_vec()
    }

    /// 对齐 Java: `ArrayUtil.split(T[], int)`
    pub fn split<T: Clone>(array: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 || array.is_empty() {
            return vec![array.to_vec()];
        }
        array.chunks(size).map(|chunk| chunk.to_vec()).collect()
    }

    // ── 填充操作 ──

    /// 对齐 Java: `ArrayUtil.fill(T[], T)`
    pub fn fill<T: Clone>(array: &mut [T], value: &T) {
        array.fill_with(|| value.clone());
    }

    /// 对齐 Java: `ArrayUtil.fillRange(T[], int, int, T)`
    pub fn fill_range<T: Clone>(array: &mut [T], from: usize, to: usize, value: &T) {
        let to = to.min(array.len());
        for item in array.iter_mut().take(to).skip(from) {
            *item = value.clone();
        }
    }

    // ── 转换操作 ──

    /// 对齐 Java: `ArrayUtil.toString(T[])`
    pub fn to_string<T: std::fmt::Display>(array: &[T]) -> String {
        let parts: Vec<String> = array.iter().map(|x| x.to_string()).collect();
        format!("[{}]", parts.join(", "))
    }

    /// 对齐 Java: `ArrayUtil.join(CharSequence, T[])`
    pub fn join<T: std::fmt::Display>(array: &[T], delimiter: &str) -> String {
        array
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(delimiter)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `ArrayUtil.equals(T[], T[])`
    pub fn equals<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a == b
    }

    /// 对齐 Java: `ArrayUtil.isEquals(T[], T[])`
    pub fn is_equals<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a == b
    }

    /// 对齐 Java: `ArrayUtil.containsAny(T[], T...)`
    pub fn contains_any<T: PartialEq>(array: &[T], values: &[T]) -> bool {
        values.iter().any(|v| array.contains(v))
    }

    /// 对齐 Java: `ArrayUtil.containsAll(T[], T...)`
    pub fn contains_all<T: PartialEq>(array: &[T], values: &[T]) -> bool {
        values.iter().all(|v| array.contains(v))
    }

    /// 对齐 Java: `ArrayUtil.isAllNotNull(T[])`
    pub fn is_all_not_null<T>(array: &[Option<T>]) -> bool {
        array.iter().all(|v| v.is_some())
    }

    /// 对齐 Java: `ArrayUtil.range(int, int)`
    pub fn range(start: i32, end: i32) -> Result<Vec<i32>> {
        let len = end
            .checked_sub(start)
            .ok_or_else(|| CoreError::InvalidArgument {
                name: "end",
                reason: "range end underflowed relative to start",
            })?;
        if len < 0 {
            return Err(CoreError::InvalidArgument {
                name: "end",
                reason: "range size must not be negative",
            });
        }
        Ok((start..end).collect())
    }

    /// 对齐 Java: `ArrayUtil.zip(K[], V[], boolean)`
    pub fn zip<K, V>(keys: &[K], values: &[V], ordered: bool) -> HashMap<K, V>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Clone,
    {
        if ordered {
            let mut map = indexmap::IndexMap::new();
            for (key, value) in keys.iter().zip(values.iter()) {
                map.insert(key.clone(), value.clone());
            }
            map.into_iter().collect()
        } else {
            keys.iter()
                .zip(values.iter())
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        }
    }

    /// 对齐 Java: `ArrayUtil.cast(Class, Object[])` — Rust 侧为同类型克隆
    pub fn cast<T: Clone>(values: &[T]) -> Vec<T> {
        values.to_vec()
    }

    /// 对齐 Java: `ArrayUtil.getArrayType(Class)` — 返回组件类型名加 `[]`
    pub fn get_array_type(component_type: &str) -> String {
        format!("[{component_type}]")
    }

    /// 对齐 Java: `ArrayUtil.distinct(T[])`
    pub fn distinct<T: Eq + std::hash::Hash + Clone>(array: &[T]) -> Vec<T> {
        let mut seen = std::collections::HashSet::new();
        array
            .iter()
            .filter(|item| seen.insert(*item))
            .cloned()
            .collect()
    }

    /// 对齐 Java: `ArrayUtil.distinct(T[], Function, boolean)`
    pub fn distinct_by<T, K, F>(array: &[T], key_fn: F, override_last: bool) -> Vec<T>
    where
        T: Clone,
        K: Eq + std::hash::Hash + Clone,
        F: Fn(&T) -> K,
    {
        if override_last {
            let mut order = Vec::new();
            let mut map: HashMap<K, T> = HashMap::new();
            for item in array {
                let key = key_fn(item);
                if !map.contains_key(&key) {
                    order.push(key.clone());
                }
                map.insert(key, item.clone());
            }
            order.into_iter().filter_map(|k| map.remove(&k)).collect()
        } else {
            let mut seen = std::collections::HashSet::new();
            array
                .iter()
                .filter(|item| seen.insert(key_fn(item)))
                .cloned()
                .collect()
        }
    }

    /// 对齐 Java: `ArrayUtil.indexOfSub(T[], T[])`
    pub fn index_of_sub<T: PartialEq>(array: &[T], sub: &[T]) -> isize {
        if sub.is_empty() {
            return -1;
        }
        if array.len() < sub.len() {
            return -1;
        }
        array
            .windows(sub.len())
            .position(|window| window == sub)
            .map(|idx| idx as isize)
            .unwrap_or(-1)
    }

    /// 对齐 Java: `ArrayUtil.lastIndexOfSub(T[], T[])`
    pub fn last_index_of_sub<T: PartialEq>(array: &[T], sub: &[T]) -> isize {
        if sub.is_empty() {
            return -1;
        }
        if array.len() < sub.len() {
            return -1;
        }
        array
            .windows(sub.len())
            .rposition(|window| window == sub)
            .map(|idx| idx as isize)
            .unwrap_or(-1)
    }

    /// 对齐 Java: `ArrayUtil.removeEmpty(T[])` — `None` 视为 null 并移除
    pub fn remove_empty(array: &[Option<&str>]) -> Vec<String> {
        array
            .iter()
            .filter_map(|item| item.map(|s| s.to_string()))
            .filter(|item| !item.is_empty())
            .collect()
    }

    /// 对齐 Java: `ArrayUtil.removeBlank(T[])` — `None` 视为 null 并移除
    pub fn remove_blank(array: &[Option<&str>]) -> Vec<String> {
        array
            .iter()
            .filter_map(|item| item.map(|s| s.to_string()))
            .filter(|item| !item.trim().is_empty())
            .collect()
    }

    /// 对齐 Java: `ArrayUtil.nullToEmpty(String[])`
    pub fn null_to_empty(array: &[Option<&str>]) -> Vec<String> {
        array
            .iter()
            .map(|item| item.unwrap_or("").to_string())
            .collect()
    }

    /// 对齐 Java: `ArrayUtil.wrap(Object)` — Rust 将单值包装为单元素数组
    pub fn wrap<T>(value: T) -> Vec<T> {
        vec![value]
    }

    /// 对齐 Java: `ArrayUtil.get(Object, int)` — 支持下标为负
    pub fn get<T: Clone>(array: &[T], index: isize) -> Option<T> {
        let len = array.len() as isize;
        let resolved = if index < 0 { len + index } else { index };
        usize::try_from(resolved)
            .ok()
            .and_then(|idx| array.get(idx))
            .cloned()
    }

    /// 对齐 Java: `ArrayUtil.getAny(Object, int...)`
    pub fn get_any<T: Clone>(array: &[T], indexes: &[isize]) -> Vec<T> {
        indexes
            .iter()
            .filter_map(|&index| Self::get(array, index))
            .collect()
    }

    /// 对齐 Java: `ArrayUtil.replace(T[], int, T...)`
    pub fn replace<T: Clone>(buffer: &[T], index: isize, values: &[T]) -> Vec<T> {
        if values.is_empty() {
            return buffer.to_vec();
        }
        if buffer.is_empty() {
            return values.to_vec();
        }
        if index < 0 {
            return Self::insert(buffer, 0, values);
        }
        let index = index as usize;
        if index >= buffer.len() {
            return Self::append(buffer, values);
        }
        if buffer.len() >= values.len() + index {
            let mut result = buffer.to_vec();
            result[index..index + values.len()].clone_from_slice(values);
            return result;
        }
        let mut result = Vec::with_capacity(index + values.len());
        result.extend_from_slice(&buffer[..index]);
        result.extend_from_slice(values);
        result
    }

    /// 对齐 Java: `ArrayUtil.setOrAppend(T[], int, T)`
    pub fn set_or_append<T: Clone>(array: &[T], index: usize, value: T) -> Vec<T> {
        if index < array.len() {
            let mut result = array.to_vec();
            result[index] = value;
            result
        } else {
            let mut result = array.to_vec();
            result.push(value);
            result
        }
    }

    /// 对齐 Java: `ArrayUtil.length`
    pub fn length<T>(array: &[T]) -> usize {
        array.len()
    }

    /// 对齐 Java: `ArrayUtil.min`
    pub fn min<T: Ord + Clone>(array: &[T]) -> Option<T> {
        array.iter().min().cloned()
    }

    /// 对齐 Java: `ArrayUtil.max`
    pub fn max<T: Ord + Clone>(array: &[T]) -> Option<T> {
        array.iter().max().cloned()
    }

    /// 对齐 Java: `ArrayUtil.swap`
    pub fn swap<T>(array: &mut [T], i: usize, j: usize) {
        if i < array.len() && j < array.len() {
            array.swap(i, j);
        }
    }

    /// 对齐 Java: `ArrayUtil.shuffle`
    pub fn shuffle<T>(array: &mut [T]) {
        use rand::seq::SliceRandom;
        array.shuffle(&mut rand::rng());
    }

    /// 对齐 Java: `ArrayUtil.hasNull`
    pub fn has_null<T>(array: &[Option<T>]) -> bool {
        array.iter().any(Option::is_none)
    }

    /// 对齐 Java: `ArrayUtil.isAllNull`
    pub fn is_all_null<T>(array: &[Option<T>]) -> bool {
        array.iter().all(Option::is_none)
    }

    /// 对齐 Java: `ArrayUtil.firstNonNull`
    pub fn first_non_null<T: Clone>(array: &[Option<T>]) -> Option<T> {
        array.iter().find_map(|v| v.clone())
    }

    /// 对齐 Java: `ArrayUtil.resize`
    pub fn resize<T: Clone + Default>(array: &[T], new_size: usize) -> Vec<T> {
        let mut out = array.to_vec();
        out.resize(new_size, T::default());
        out
    }

    /// 对齐 Java: `ArrayUtil.isSorted` / ASC
    pub fn is_sorted_asc<T: Ord>(array: &[T]) -> bool {
        array.windows(2).all(|w| w[0] <= w[1])
    }

    /// 对齐 Java: `ArrayUtil.isSortedDESC`
    pub fn is_sorted_desc<T: Ord>(array: &[T]) -> bool {
        array.windows(2).all(|w| w[0] >= w[1])
    }

    /// 对齐 Java: `ArrayUtil.filter`
    pub fn filter<T: Clone, F: FnMut(&T) -> bool>(array: &[T], mut pred: F) -> Vec<T> {
        array.iter().filter(|v| pred(v)).cloned().collect()
    }

    /// 对齐 Java: `ArrayUtil.map`
    pub fn map<T, U, F: FnMut(&T) -> U>(array: &[T], mapper: F) -> Vec<U> {
        array.iter().map(mapper).collect()
    }

    /// 对齐 Java: `ArrayUtil.removeNull`
    pub fn remove_null<T: Clone>(array: &[Option<T>]) -> Vec<T> {
        array.iter().filter_map(|v| v.clone()).collect()
    }

    /// 对齐 Java: `ArrayUtil.emptyCount`
    pub fn empty_count(array: &[Option<&str>]) -> i32 {
        array
            .iter()
            .filter(|v| v.map_or(true, |s| s.is_empty()))
            .count() as i32
    }

    /// 对齐 Java: `ArrayUtil.hasEmpty`
    pub fn has_empty(array: &[Option<&str>]) -> bool {
        array.iter().any(|v| v.map_or(true, |s| s.is_empty()))
    }

    /// 对齐 Java: `ArrayUtil.isAllEmpty`
    pub fn is_all_empty(array: &[Option<&str>]) -> bool {
        array.iter().all(|v| v.map_or(true, |s| s.is_empty()))
    }

    /// 对齐 Java: `ArrayUtil.isAllNotEmpty`
    pub fn is_all_not_empty(array: &[Option<&str>]) -> bool {
        array.iter().all(|v| v.map_or(false, |s| !s.is_empty()))
    }

    /// 对齐 Java: `ArrayUtil.matchIndex`
    pub fn match_index<T, F: FnMut(&T) -> bool>(array: &[T], mut pred: F) -> Option<usize> {
        array.iter().position(|v| pred(v))
    }

    /// 对齐 Java: `ArrayUtil.firstMatch`
    pub fn first_match<T: Clone, F: FnMut(&T) -> bool>(array: &[T], mut pred: F) -> Option<T> {
        array.iter().find(|v| pred(v)).cloned()
    }

    /// 对齐 Java: `ArrayUtil.edit` — editor 返回 `None` 表示删除该元素。
    pub fn edit<T: Clone, F: FnMut(T) -> Option<T>>(array: &[T], mut editor: F) -> Vec<T> {
        array.iter().cloned().filter_map(|v| editor(v)).collect()
    }

    /// 对齐 Java: `ArrayUtil.copy(Object, Object, int)` — 有界切片拷贝。
    pub fn copy<T: Clone>(src: &[T], dest: &mut [T], length: usize) {
        let n = length.min(src.len()).min(dest.len());
        dest[..n].clone_from_slice(&src[..n]);
    }

    /// 对齐 Java: `ArrayUtil.copy(Object, int, Object, int, int)`
    pub fn copy_range<T: Clone>(
        src: &[T],
        src_pos: usize,
        dest: &mut [T],
        dest_pos: usize,
        length: usize,
    ) {
        if src_pos >= src.len() || dest_pos >= dest.len() || length == 0 {
            return;
        }
        let n = length
            .min(src.len().saturating_sub(src_pos))
            .min(dest.len().saturating_sub(dest_pos));
        dest[dest_pos..dest_pos + n].clone_from_slice(&src[src_pos..src_pos + n]);
    }

    /// 对齐 Java: `ArrayUtil.toArray` / varargs 包装。
    pub fn to_array<T: Clone>(values: &[T]) -> Vec<T> {
        Self::to_vec(values)
    }

    /// 对齐 Java: `ArrayUtil.isSub`
    pub fn is_sub<T: PartialEq>(array: &[T], sub: &[T]) -> bool {
        Self::index_of_sub(array, sub) >= 0
    }

    /// 对齐 Java: `ArrayUtil.isArray` — Rust 侧切片恒为 true。
    pub fn is_array<T>(_array: &[T]) -> bool {
        true
    }

    /// 对齐 Java: `ArrayUtil.clone`
    pub fn clone_array<T: Clone>(array: &[T]) -> Vec<T> {
        array.to_vec()
    }
}
