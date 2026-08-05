//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

use indexmap::IndexMap;

use crate::{CoreError, Result};

use super::create_map_kind::CreateMapKind;
use super::empty_map_kind::EmptyMapKind;
use super::linked_or_hash_map::LinkedOrHashMap;
use super::map_builder_gate::MapBuilderGate;
use super::nested_map_value::NestedMapValue;

/// 对齐 Java: `cn.hutool.core.map.MapUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct MapUtil;

impl MapUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `MapUtil.isEmpty(Map)` —— 对非空引用判空。
    pub fn is_empty<K, V>(map: &HashMap<K, V>) -> bool {
        map.is_empty()
    }

    /// 对齐 Java: `MapUtil.isEmpty(Map)` —— `None` 视为空（Java `null`）。
    pub fn is_empty_opt<K, V>(map: Option<&HashMap<K, V>>) -> bool {
        map.map(|m| m.is_empty()).unwrap_or(true)
    }

    /// 对齐 Java: `MapUtil.isNotEmpty(Map)`
    pub fn is_not_empty<K, V>(map: &HashMap<K, V>) -> bool {
        !map.is_empty()
    }

    /// 对齐 Java: `MapUtil.isNotEmpty(Map)` —— `None` 视为空。
    pub fn is_not_empty_opt<K, V>(map: Option<&HashMap<K, V>>) -> bool {
        !Self::is_empty_opt(map)
    }

    /// 对齐 Java: `MapUtil.emptyIfNull(Map)` —— `None` 返回空 HashMap。
    pub fn empty_if_null<K, V>(map: Option<HashMap<K, V>>) -> HashMap<K, V> {
        map.unwrap_or_default()
    }

    /// 对齐 Java: `MapUtil.defaultIfEmpty(Map, Map)`
    pub fn default_if_empty<K, V>(map: HashMap<K, V>, default_map: HashMap<K, V>) -> HashMap<K, V> {
        if map.is_empty() { default_map } else { map }
    }

    // ── 创建操作 ──

    /// 对齐 Java: `MapUtil.newHashMap()`
    pub fn new_hash_map<K, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    /// 对齐 Java: `MapUtil.newHashMap(int size)` —— 按 0.75 负载因子换算容量。
    pub fn new_hash_map_sized<K, V>(size: usize) -> HashMap<K, V> {
        HashMap::with_capacity(Self::initial_capacity(size))
    }

    /// 对齐 Java: `MapUtil.newHashMap(boolean isLinked)`
    pub fn new_hash_map_linked<K, V>(is_linked: bool) -> LinkedOrHashMap<K, V>
    where
        K: Eq + Hash,
    {
        Self::new_hash_map_sized_linked(DEFAULT_INITIAL_CAPACITY, is_linked)
    }

    /// 对齐 Java: `MapUtil.newHashMap(int size, boolean isLinked)`
    pub fn new_hash_map_sized_linked<K, V>(size: usize, is_linked: bool) -> LinkedOrHashMap<K, V>
    where
        K: Eq + Hash,
    {
        let cap = Self::initial_capacity(size);
        if is_linked {
            LinkedOrHashMap::Linked(IndexMap::with_capacity(cap))
        } else {
            LinkedOrHashMap::Hash(HashMap::with_capacity(cap))
        }
    }

    /// 对齐 Java: `MapUtil.newTreeMap(Comparator)` —— Rust 用 `BTreeMap`（键需 `Ord`）。
    pub fn new_tree_map<K: Ord, V>() -> BTreeMap<K, V> {
        BTreeMap::new()
    }

    /// 对齐 Java: `MapUtil.newTreeMap(Map, Comparator)`
    pub fn new_tree_map_from<K, V>(map: &HashMap<K, V>) -> BTreeMap<K, V>
    where
        K: Ord + Clone + Eq + Hash,
        V: Clone,
    {
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// 对齐 Java: `MapUtil.newIdentityMap(int size)` —— Rust 无 IdentityHashMap，退化为按容量预分配的 HashMap。
    pub fn new_identity_map<K, V>(size: usize) -> HashMap<K, V> {
        HashMap::with_capacity(Self::initial_capacity(size))
    }

    /// 对齐 Java: `MapUtil.createMap(Class)` —— 用 [`CreateMapKind`] 代替反射 `Class`。
    pub fn create_map<K, V>(kind: CreateMapKind) -> LinkedOrHashMap<K, V>
    where
        K: Eq + Hash,
    {
        match kind {
            CreateMapKind::Hash | CreateMapKind::Identity | CreateMapKind::Concurrent => {
                LinkedOrHashMap::Hash(HashMap::new())
            }
            CreateMapKind::Linked => LinkedOrHashMap::Linked(IndexMap::new()),
            CreateMapKind::Tree => LinkedOrHashMap::Hash(HashMap::new()),
        }
    }

    /// 对齐 Java: `MapUtil.createMap` —— TreeMap 分支返回 BTreeMap。
    pub fn create_tree_map<K: Ord, V>() -> BTreeMap<K, V> {
        BTreeMap::new()
    }

    /// 对齐 Java: `MapUtil.newConcurrentHashMap()` —— Rust 用 `parking_lot::Mutex<HashMap>` 包装见 `SafeConcurrentHashMap`；
    /// 此处返回普通 HashMap 作为无锁创建入口（与单线程场景一致）。
    pub fn new_concurrent_hash_map<K, V>() -> HashMap<K, V> {
        HashMap::with_capacity(DEFAULT_INITIAL_CAPACITY)
    }

    /// 对齐 Java: `MapUtil.newConcurrentHashMap(int size)`
    pub fn new_concurrent_hash_map_sized<K, V>(size: usize) -> HashMap<K, V> {
        let init = if size == 0 {
            DEFAULT_INITIAL_CAPACITY
        } else {
            size
        };
        HashMap::with_capacity(init)
    }

    /// 对齐 Java: `MapUtil.newConcurrentHashMap(Map)`
    pub fn new_concurrent_hash_map_from<K, V>(map: HashMap<K, V>) -> HashMap<K, V> {
        map
    }

    /// 对齐 Java: `MapUtil.of(K, V)`
    pub fn of_pair<K: Eq + Hash, V>(key: K, value: V) -> HashMap<K, V> {
        let mut map = HashMap::with_capacity(1);
        map.insert(key, value);
        map
    }

    /// 对齐 Java: `MapUtil.of(K, V, boolean isOrder)`
    pub fn of_pair_ordered<K: Eq + Hash, V>(
        key: K,
        value: V,
        is_order: bool,
    ) -> LinkedOrHashMap<K, V> {
        if is_order {
            let mut map = IndexMap::with_capacity(1);
            map.insert(key, value);
            LinkedOrHashMap::Linked(map)
        } else {
            LinkedOrHashMap::Hash(Self::of_pair(key, value))
        }
    }

    /// 对齐 Java: `MapUtil.of(Object[]...)` / `ofEntries`
    pub fn of<K, V>(pairs: &[(K, V)]) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        pairs.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// 对齐 Java: `MapUtil.ofEntries(Entry...)`
    pub fn of_entries<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        entries.into_iter().collect()
    }

    /// 对齐 Java: `MapUtil.entry(K, V)` —— 不可变条目。
    pub fn entry<K, V>(key: K, value: V) -> (K, V) {
        (key, value)
    }

    /// 对齐 Java: `MapUtil.entry(K, V, boolean isImmutable)` —— Rust 元组本身可按需可变。
    pub fn entry_mutable<K, V>(key: K, value: V, _is_immutable: bool) -> (K, V) {
        (key, value)
    }

    // ── 获取操作 ──

    /// 对齐 Java: `MapUtil.getStr(Map, Object)`
    pub fn get_str<'a, K: Eq + Hash>(map: &'a HashMap<K, String>, key: &K) -> Option<&'a str> {
        map.get(key).map(|s| s.as_str())
    }

    /// 对齐 Java: `MapUtil.getStr(Map, Object, String)`
    pub fn get_str_or<'a, K: Eq + Hash>(
        map: &'a HashMap<K, String>,
        key: &K,
        default: &'a str,
    ) -> &'a str {
        map.get(key).map(|s| s.as_str()).unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getInt(Map, Object)` —— 值本身为整数。
    pub fn get_int<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K) -> Option<i64> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getInt(Map, Object, Integer)`
    pub fn get_int_or<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K, default: i64) -> i64 {
        map.get(key).copied().unwrap_or(default)
    }

    /// 从字符串值解析整数 —— 对齐 `getInt` + Convert 路径。
    pub fn get_int_parsed<K: Eq + Hash>(map: &HashMap<K, String>, key: &K) -> Option<i64> {
        map.get(key).and_then(|s| s.parse().ok())
    }

    /// 对齐 Java: `MapUtil.getLong(Map, Object)`
    pub fn get_long<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K) -> Option<i64> {
        Self::get_int(map, key)
    }

    /// 对齐 Java: `MapUtil.getLong(Map, Object, Long)`
    pub fn get_long_or<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K, default: i64) -> i64 {
        Self::get_int_or(map, key, default)
    }

    /// 对齐 Java: `MapUtil.getDouble(Map, Object)`
    pub fn get_double<K: Eq + Hash>(map: &HashMap<K, f64>, key: &K) -> Option<f64> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getDouble(Map, Object, Double)`
    pub fn get_double_or<K: Eq + Hash>(map: &HashMap<K, f64>, key: &K, default: f64) -> f64 {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getFloat(Map, Object)`
    pub fn get_float<K: Eq + Hash>(map: &HashMap<K, f32>, key: &K) -> Option<f32> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getFloat(Map, Object, Float)`
    pub fn get_float_or<K: Eq + Hash>(map: &HashMap<K, f32>, key: &K, default: f32) -> f32 {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getShort(Map, Object)`
    pub fn get_short<K: Eq + Hash>(map: &HashMap<K, i16>, key: &K) -> Option<i16> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getShort(Map, Object, Short)`
    pub fn get_short_or<K: Eq + Hash>(map: &HashMap<K, i16>, key: &K, default: i16) -> i16 {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getBool(Map, Object)`
    pub fn get_bool<K: Eq + Hash>(map: &HashMap<K, bool>, key: &K) -> Option<bool> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getBool(Map, Object, Boolean)`
    pub fn get_bool_or<K: Eq + Hash>(map: &HashMap<K, bool>, key: &K, default: bool) -> bool {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getChar(Map, Object)`
    pub fn get_char<K: Eq + Hash>(map: &HashMap<K, char>, key: &K) -> Option<char> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getChar(Map, Object, Character)`
    pub fn get_char_or<K: Eq + Hash>(map: &HashMap<K, char>, key: &K, default: char) -> char {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.get(Map, Object, Class)` —— Rust 用闭包转换代替反射。
    pub fn get_as<K, V, T, F>(map: &HashMap<K, V>, key: &K, convert: F) -> Option<T>
    where
        K: Eq + Hash,
        F: FnOnce(&V) -> Option<T>,
    {
        map.get(key).and_then(convert)
    }

    /// 对齐 Java: `MapUtil.get(Map, Object, Class, T)` / `getQuietly`
    pub fn get_as_or<K, V, T, F>(map: &HashMap<K, V>, key: &K, convert: F, default: T) -> T
    where
        K: Eq + Hash,
        F: FnOnce(&V) -> Option<T>,
    {
        Self::get_as(map, key, convert).unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.getQuietly` —— 转换失败返回默认值。
    pub fn get_quietly<K, V, T, F>(map: &HashMap<K, V>, key: &K, convert: F, default: T) -> T
    where
        K: Eq + Hash,
        F: FnOnce(&V) -> Option<T>,
    {
        Self::get_as_or(map, key, convert, default)
    }

    /// 对齐 Java: `MapUtil.getAny(Map, K...)`
    pub fn get_any<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let mut out = HashMap::new();
        for k in keys {
            if let Some(v) = map.get(k) {
                out.insert(k.clone(), v.clone());
            }
        }
        out
    }

    // ── 设置 / 删除 ──

    /// 对齐 Java: `MapUtil.putAll(Map, Map)`（非 Hutool 公开静态，保留既有辅助）
    pub fn put_all<K, V>(target: &mut HashMap<K, V>, source: HashMap<K, V>)
    where
        K: Eq + Hash,
    {
        target.extend(source);
    }

    /// 对齐 Java: `MapUtil.removeAny(Map, K...)`
    pub fn remove_any<K, V>(map: &mut HashMap<K, V>, keys: &[K])
    where
        K: Eq + Hash,
    {
        for k in keys {
            map.remove(k);
        }
    }

    /// 对齐 Java: `MapUtil.renameKey(Map, K, K)`
    pub fn rename_key<K, V>(map: &mut HashMap<K, V>, old_key: K, new_key: K) -> Result<()>
    where
        K: Eq + Hash + Display,
    {
        if !map.contains_key(&old_key) {
            return Ok(());
        }
        if map.contains_key(&new_key) {
            return Err(CoreError::InvalidArgument {
                name: "newKey",
                reason: "The key exist !",
            });
        }
        if let Some(v) = map.remove(&old_key) {
            map.insert(new_key, v);
        }
        Ok(())
    }

    /// 对齐 Java: `MapUtil.removeNullValue(Map)` —— 值用 `Option` 表达 Java `null`。
    pub fn remove_null_value<K, V>(map: &mut HashMap<K, Option<V>>)
    where
        K: Eq + Hash,
    {
        map.retain(|_, v| v.is_some());
    }

    /// 对齐 Java: `MapUtil.removeByValue(Map, V)`
    pub fn remove_by_value<K, V>(map: &mut HashMap<K, V>, value: &V)
    where
        K: Eq + Hash,
        V: PartialEq,
    {
        map.retain(|_, v| v != value);
    }

    /// 对齐 Java: `MapUtil.removeIf(Map, Predicate)`
    pub fn remove_if<K, V, F>(map: &mut HashMap<K, V>, mut predicate: F)
    where
        K: Eq + Hash,
        F: FnMut(&K, &V) -> bool,
    {
        map.retain(|k, v| !predicate(k, v));
    }

    /// 对齐 Java: `MapUtil.clear(Map...)`
    pub fn clear<K, V>(maps: &mut [&mut HashMap<K, V>]) {
        for m in maps {
            m.clear();
        }
    }

    // ── 转换 / 过滤 ──

    /// 对齐 Java: `MapUtil.join(Map, separator, keyValueSeparator)`
    pub fn join<K: Display, V: Display>(
        map: &HashMap<K, V>,
        entry_delimiter: &str,
        kv_delimiter: &str,
    ) -> String {
        Self::join_full(map, entry_delimiter, kv_delimiter, false, &[])
    }

    /// 对齐 Java: `MapUtil.joinIgnoreNull`
    pub fn join_ignore_null<K: Display, V: Display>(
        map: &HashMap<K, Option<V>>,
        entry_delimiter: &str,
        kv_delimiter: &str,
    ) -> String {
        let mut parts = Vec::new();
        for (k, v) in map {
            if let Some(val) = v {
                parts.push(format!("{}{}{}", k, kv_delimiter, val));
            }
        }
        parts.join(entry_delimiter)
    }

    /// 对齐 Java: `MapUtil.join(..., boolean isIgnoreNull, String... otherParams)`
    pub fn join_full<K: Display, V: Display>(
        map: &HashMap<K, V>,
        separator: &str,
        key_value_separator: &str,
        _is_ignore_null: bool,
        other_params: &[&str],
    ) -> String {
        let mut s = map
            .iter()
            .map(|(k, v)| format!("{}{}{}", k, key_value_separator, v))
            .collect::<Vec<_>>()
            .join(separator);
        for p in other_params {
            s.push_str(p);
        }
        s
    }

    /// 对齐 Java: `MapUtil.sortJoin`
    pub fn sort_join<K, V>(
        map: &HashMap<K, V>,
        separator: &str,
        key_value_separator: &str,
        other_params: &[&str],
    ) -> String
    where
        K: Display + Ord + Clone + Eq + Hash,
        V: Display + Clone,
    {
        let sorted = Self::sort(map);
        let mut s = sorted
            .iter()
            .map(|(k, v)| format!("{}{}{}", k, key_value_separator, v))
            .collect::<Vec<_>>()
            .join(separator);
        for p in other_params {
            s.push_str(p);
        }
        s
    }

    /// 对齐 Java: `MapUtil.filter(Map, Filter)`
    pub fn filter<K, V, F>(map: &HashMap<K, V>, predicate: F) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
        F: Fn(&K, &V) -> bool,
    {
        map.iter()
            .filter(|(k, v)| predicate(k, v))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// 对齐 Java: `MapUtil.filter(Map, K...)` —— 仅保留指定键。
    pub fn filter_keys<K, V>(map: &HashMap<K, V>, keys: &[K]) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        Self::get_any(map, keys)
    }

    /// 对齐 Java: `MapUtil.edit(Map, Editor)` —— 闭包返回 `None` 表示丢弃。
    pub fn edit<K, V, F>(map: &HashMap<K, V>, mut editor: F) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
        F: FnMut(K, V) -> Option<(K, V)>,
    {
        let mut out = HashMap::with_capacity(map.len());
        for (k, v) in map {
            if let Some((nk, nv)) = editor(k.clone(), v.clone()) {
                out.insert(nk, nv);
            }
        }
        out
    }

    /// 对齐 Java: `MapUtil.map(Map, BiFunction)`
    pub fn map_values<K, V, R, F>(map: &HashMap<K, V>, bi: F) -> HashMap<K, R>
    where
        K: Eq + Hash + Clone,
        F: Fn(&K, &V) -> R,
    {
        map.iter().map(|(k, v)| (k.clone(), bi(k, v))).collect()
    }

    /// 对齐 Java: `MapUtil.reverse(Map)` —— 同类型键值互换。
    pub fn reverse<T>(map: &HashMap<T, T>) -> HashMap<T, T>
    where
        T: Eq + Hash + Clone,
    {
        Self::inverse(map)
    }

    /// 对齐 Java: `MapUtil.inverse(Map)`
    pub fn inverse<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
    where
        K: Eq + Hash + Clone,
        V: Eq + Hash + Clone,
    {
        map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
    }

    /// 对齐 Java: `MapUtil.sort(Map)` —— 按键排序为 BTreeMap。
    pub fn sort<K, V>(map: &HashMap<K, V>) -> BTreeMap<K, V>
    where
        K: Ord + Clone + Eq + Hash,
        V: Clone,
    {
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// 对齐 Java: `MapUtil.sort(Map, Comparator)` —— 自定义键序；Rust 用闭包比较后落入 IndexMap。
    pub fn sort_by<K, V, F>(map: &HashMap<K, V>, mut cmp: F) -> IndexMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
        F: FnMut(&K, &K) -> std::cmp::Ordering,
    {
        let mut entries: Vec<(K, V)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        entries.sort_by(|(a, _), (b, _)| cmp(a, b));
        entries.into_iter().collect()
    }

    /// 对齐 Java: `MapUtil.sortByValue(Map, boolean isDesc)` —— 返回保持值序的 IndexMap。
    pub fn sort_by_value<K, V>(map: &HashMap<K, V>, is_desc: bool) -> IndexMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone + Ord,
    {
        let mut entries: Vec<(K, V)> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        entries.sort_by(|(_, a), (_, b)| if is_desc { b.cmp(a) } else { a.cmp(b) });
        entries.into_iter().collect()
    }

    /// 对齐 Java: `MapUtil.toListMap` —— 多行 Map 合并为 `K -> Vec<V>`。
    pub fn to_list_map<K, V>(map_list: &[HashMap<K, V>]) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let mut out: HashMap<K, Vec<V>> = HashMap::new();
        for m in map_list {
            for (k, v) in m {
                out.entry(k.clone()).or_default().push(v.clone());
            }
        }
        out
    }

    /// 对齐 Java: `MapUtil.toMapList`
    pub fn to_map_list<K, V>(list_map: &HashMap<K, Vec<V>>) -> Vec<HashMap<K, V>>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let max_len = list_map.values().map(|v| v.len()).max().unwrap_or(0);
        let mut result = Vec::with_capacity(max_len);
        for i in 0..max_len {
            let mut row = HashMap::new();
            for (k, values) in list_map {
                if let Some(v) = values.get(i) {
                    row.insert(k.clone(), v.clone());
                }
            }
            result.push(row);
        }
        result
    }

    /// 对齐 Java: `MapUtil.grouping(Iterable<Entry>)`
    pub fn grouping<K, V>(entries: impl IntoIterator<Item = (K, V)>) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
    {
        let mut out: HashMap<K, Vec<V>> = HashMap::new();
        for (k, v) in entries {
            out.entry(k).or_default().push(v);
        }
        out
    }

    /// 对齐 Java: `MapUtil.toObjectArray(Map)`
    pub fn to_object_array<K: Clone, V: Clone>(map: &BTreeMap<K, V>) -> Vec<(K, V)> {
        map.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// 对齐 Java: `MapUtil.toCamelCaseMap` —— 键转驼峰（仅 `&str`/`String` 键）。
    pub fn to_camel_case_map<V: Clone>(map: &HashMap<String, V>) -> HashMap<String, V> {
        map.iter()
            .map(|(k, v)| (simple_to_camel_case(k), v.clone()))
            .collect()
    }

    // ── 合并 / 键值 ──

    /// 对齐 Java: `MapUtil.merge(Map, Map)`（既有辅助）
    pub fn merge<K, V>(mut left: HashMap<K, V>, right: HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + Hash,
    {
        left.extend(right);
        left
    }

    /// 对齐 Java: `MapUtil.keys`（既有辅助）
    pub fn keys<K: Clone, V>(map: &HashMap<K, V>) -> Vec<K> {
        map.keys().cloned().collect()
    }

    /// 对齐 Java: `MapUtil.values`（既有辅助）
    pub fn values<K, V: Clone>(map: &HashMap<K, V>) -> Vec<V> {
        map.values().cloned().collect()
    }

    /// 对齐 Java: `MapUtil.valuesOfKeys`
    pub fn values_of_keys<K, V>(
        map: &HashMap<K, V>,
        keys: impl IntoIterator<Item = K>,
    ) -> Vec<Option<V>>
    where
        K: Eq + Hash,
        V: Clone,
    {
        keys.into_iter().map(|k| map.get(&k).cloned()).collect()
    }

    /// 对齐 Java: `MapUtil.computeIfAbsent` / `computeIfAbsentForJdk8`
    pub fn compute_if_absent<K, V, F>(map: &mut HashMap<K, V>, key: K, mapping: F) -> &V
    where
        K: Eq + Hash,
        F: FnOnce(&K) -> V,
    {
        map.entry(key).or_insert_with_key(mapping)
    }

    /// 对齐 Java: `MapUtil.partition(Map, int)`
    pub fn partition<K, V>(map: Option<&HashMap<K, V>>, size: i32) -> Result<Vec<HashMap<K, V>>>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let map = map.ok_or(CoreError::InvalidArgument {
            name: "map",
            reason: "must not be null",
        })?;
        if size <= 0 {
            return Err(CoreError::InvalidArgument {
                name: "size",
                reason: "must be greater than 0",
            });
        }
        let size = size as usize;
        let mut list = Vec::new();
        let mut iter = map.iter();
        loop {
            let mut sub = HashMap::new();
            for _ in 0..size {
                match iter.next() {
                    Some((k, v)) => {
                        sub.insert(k.clone(), v.clone());
                    }
                    None => break,
                }
            }
            if sub.is_empty() {
                break;
            }
            list.push(sub);
        }
        Ok(list)
    }

    /// 对齐 Java: `MapUtil.empty()`
    pub fn empty_map<K, V>() -> HashMap<K, V> {
        HashMap::new()
    }

    /// 对齐 Java: `MapUtil.empty(Class)`
    pub fn empty<K, V>(kind: EmptyMapKind) -> Result<HashMap<K, V>> {
        match kind {
            EmptyMapKind::Map | EmptyMapKind::SortedMap | EmptyMapKind::NavigableMap => {
                Ok(HashMap::new())
            }
            EmptyMapKind::TreeMap => Err(CoreError::InvalidArgument {
                name: "mapClass",
                reason: "TreeMap is not support to get empty",
            }),
        }
    }

    /// 对齐 Java: `MapUtil.builder()` —— 返回可链式 put 的轻量 builder（map 子包由其他 agent 演进）。
    pub fn builder<K: Eq + Hash, V>() -> MapBuilderGate<K, V> {
        MapBuilderGate::create()
    }

    /// 对齐 Java: `MapUtil.builder(Map)`
    pub fn builder_of<K: Eq + Hash, V>(map: HashMap<K, V>) -> MapBuilderGate<K, V> {
        MapBuilderGate::create_from(map)
    }

    /// 对齐 Java: `MapUtil.builder(K, V)`
    pub fn builder_pair<K: Eq + Hash, V>(k: K, v: V) -> MapBuilderGate<K, V> {
        MapBuilderGate::create().put(k, v)
    }

    /// 对齐 Java: `MapUtil.wrap(Map)` —— 当前返回原 map（MapWrapper 桩由 map agent 补齐）。
    pub fn wrap<K: Eq + Hash, V>(map: HashMap<K, V>) -> HashMap<K, V> {
        map
    }

    /// 对齐 Java: `MapUtil.unmodifiable(Map)` —— 返回只读视图（克隆快照）。
    pub fn unmodifiable<K, V>(map: &HashMap<K, V>) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        map.clone()
    }

    /// 对齐 Java: `MapUtil.createProxy(Map)` —— 无动态代理，返回原 map（Bean 代理见 `map::MapProxy` planned）。
    pub fn create_proxy<K: Eq + Hash, V>(map: HashMap<K, V>) -> HashMap<K, V> {
        map
    }

    /// 对齐 Java: `MapUtil.getDate(Map, Object)` —— 值已是 epoch 毫秒。
    pub fn get_date_millis<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K) -> Option<i64> {
        map.get(key).copied()
    }

    /// 对齐 Java: `MapUtil.getDate(Map, Object, Date)` —— 默认 epoch 毫秒。
    pub fn get_date_millis_or<K: Eq + Hash>(map: &HashMap<K, i64>, key: &K, default: i64) -> i64 {
        map.get(key).copied().unwrap_or(default)
    }

    /// 对齐 Java: `MapUtil.flatten(Map)` —— 展开一层嵌套 Map。
    pub fn flatten<K, V>(map: &HashMap<K, NestedMapValue<K, V>>) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        let mut flat = HashMap::new();
        Self::flatten_into(map, &mut flat);
        flat
    }

    /// 对齐 Java: `MapUtil.flatten(Map, Map)` —— 写入给定 flatMap。
    pub fn flatten_into<K, V>(map: &HashMap<K, NestedMapValue<K, V>>, flat_map: &mut HashMap<K, V>)
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        for (k, v) in map {
            match v {
                NestedMapValue::Nested(child) => Self::flatten_into(child, flat_map),
                NestedMapValue::Leaf(leaf) => {
                    flat_map.insert(k.clone(), leaf.clone());
                }
            }
        }
    }

    /// 按 Hutool 公式换算初始容量：`size / 0.75 + 1`
    fn initial_capacity(size: usize) -> usize {
        (size as f32 / DEFAULT_LOAD_FACTOR) as usize + 1
    }
}

use super::{DEFAULT_INITIAL_CAPACITY, DEFAULT_LOAD_FACTOR, simple_to_camel_case};
