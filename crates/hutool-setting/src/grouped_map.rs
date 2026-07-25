//! 对齐: `cn.hutool.setting.GroupedMap` (分组有序映射)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedMap.java
//! 中文说明: 按分组分区的插入有序映射，是 Setting 内部的核心数据结构。

use indexmap::IndexMap;

/// 按分组分区的插入有序键值对映射，对应 Hutool 的 `GroupedMap`。
///
/// 对齐 Java 类: `cn.hutool.setting.GroupedMap`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedMap.java
///
/// Insertion-ordered values partitioned by group.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroupedMap(IndexMap<String, IndexMap<String, String>>);

impl GroupedMap {
    /// Creates an empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Returns a value.
    #[must_use]
    pub fn get(&self, group: &str, key: &str) -> Option<&str> {
        self.0
            .get(group)
            .and_then(|m| m.get(key))
            .map(String::as_str)
    }
    /// Returns one group.
    #[must_use]
    pub fn group(&self, group: &str) -> Option<&IndexMap<String, String>> {
        self.0.get(group)
    }
    /// Counts values across groups.
    #[must_use]
    pub fn size(&self) -> usize {
        self.0.values().map(IndexMap::len).sum()
    }
    /// Inserts a value and returns the previous value.
    pub fn put(
        &mut self,
        group: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.0
            .entry(group.into())
            .or_default()
            .insert(key.into(), value.into())
    }
    /// Extends one group.
    pub fn put_all<I, K, V>(&mut self, group: impl Into<String>, values: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.0
            .entry(group.into())
            .or_default()
            .extend(values.into_iter().map(|(k, v)| (k.into(), v.into())));
        self
    }
    /// Removes a value.
    pub fn remove(&mut self, group: &str, key: &str) -> Option<String> {
        self.0.get_mut(group).and_then(|m| m.shift_remove(key))
    }
    /// Tests whether a group is empty or absent.
    #[must_use]
    pub fn is_group_empty(&self, group: &str) -> bool {
        self.0.get(group).is_none_or(IndexMap::is_empty)
    }
    /// Tests whether all groups are empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.values().all(IndexMap::is_empty)
    }
    /// Tests a key.
    #[must_use]
    pub fn contains_key(&self, group: &str, key: &str) -> bool {
        self.0.get(group).is_some_and(|m| m.contains_key(key))
    }
    /// Tests a value.
    #[must_use]
    pub fn contains_value(&self, group: &str, value: &str) -> bool {
        self.0
            .get(group)
            .is_some_and(|m| m.values().any(|v| v == value))
    }
    /// Clears one group.
    pub fn clear_group(&mut self, group: &str) -> &mut Self {
        if let Some(values) = self.0.get_mut(group) {
            values.clear();
        }
        self
    }
    /// Clears all groups.
    pub fn clear(&mut self) {
        self.0.clear();
    }
    /// Returns group names.
    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.0.keys().map(String::as_str)
    }
    /// Returns a group's keys.
    pub fn keys(&self, group: &str) -> impl Iterator<Item = &str> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.keys().map(String::as_str))
    }
    /// Returns a group's values.
    pub fn values(&self, group: &str) -> impl Iterator<Item = &str> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.values().map(String::as_str))
    }
    /// Returns a group's entries.
    pub fn entries(&self, group: &str) -> impl Iterator<Item = (&str, &str)> {
        self.0
            .get(group)
            .into_iter()
            .flat_map(|m| m.iter().map(|(k, v)| (k.as_str(), v.as_str())))
    }
    /// Returns all groups.
    #[must_use]
    pub const fn as_map(&self) -> &IndexMap<String, IndexMap<String, String>> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouped_map_covers_ordered_map_semantics() {
        let mut map = GroupedMap::new();
        assert!(map.is_empty());
        assert!(map.is_group_empty("missing"));
        assert_eq!(map.put("db", "host", "localhost"), None);
        assert_eq!(map.put("db", "host", "db.local"), Some("localhost".into()));
        map.put_all("db", [("port", "5432"), ("user", "sa")]);
        map.put("", "root", "yes");
        assert_eq!(map.size(), 4);
        assert_eq!(map.get("db", "host"), Some("db.local"));
        assert_eq!(map.group("db").unwrap().len(), 3);
        assert!(map.contains_key("db", "port"));
        assert!(map.contains_value("db", "sa"));
        assert_eq!(map.groups().collect::<Vec<_>>(), ["db", ""]);
        assert_eq!(map.keys("db").collect::<Vec<_>>(), ["host", "port", "user"]);
        assert_eq!(map.values("db").count(), 3);
        assert_eq!(map.entries("db").count(), 3);
        assert_eq!(map.as_map().len(), 2);
        assert_eq!(map.remove("db", "port"), Some("5432".into()));
        assert_eq!(map.remove("missing", "x"), None);
        map.clear_group("missing").clear_group("db");
        assert!(map.is_group_empty("db"));
        map.clear();
        assert!(map.is_empty());
    }
}
