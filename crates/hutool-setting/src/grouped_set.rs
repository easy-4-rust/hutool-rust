//! 对齐: `cn.hutool.setting.GroupedSet` (分组有序集合)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedSet.java
//! 中文说明: 从 Hutool 分组集合语法加载的插入有序分组字符串集合。

use crate::{DEFAULT_ENCODING, SettingError, checked_path, read_text};
use encoding_rs::Encoding;
use indexmap::IndexSet;
use std::path::{Path, PathBuf};

/// 插入有序的分组字符串集合，对应 Hutool 的 `GroupedSet`。
///
/// 对齐 Java 类: `cn.hutool.setting.GroupedSet`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedSet.java
///
/// Insertion-ordered grouped string sets loaded from Hutool's grouped-set syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupedSet {
    groups: indexmap::IndexMap<String, IndexSet<String>>,
    path: Option<PathBuf>,
    encoding: &'static Encoding,
}

impl Default for GroupedSet {
    fn default() -> Self {
        Self {
            groups: indexmap::IndexMap::new(),
            path: None,
            encoding: DEFAULT_ENCODING,
        }
    }
}

impl GroupedSet {
    /// Creates an empty grouped set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            encoding: DEFAULT_ENCODING,
            ..Self::default()
        }
    }
    /// Loads a path using UTF-8.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SettingError> {
        Self::from_path_with_encoding(path, DEFAULT_ENCODING)
    }
    /// Loads a path using an explicit encoding.
    pub fn from_path_with_encoding(
        path: impl AsRef<Path>,
        encoding: &'static Encoding,
    ) -> Result<Self, SettingError> {
        Self::from_path_ref(path.as_ref(), encoding)
    }
    fn from_path_ref(path: &Path, encoding: &'static Encoding) -> Result<Self, SettingError> {
        let path = checked_path(path)?;
        let mut value = Self {
            path: Some(path),
            encoding,
            ..Self::default()
        };
        value.reload()?;
        Ok(value)
    }
    /// Replaces content from text.
    pub fn load_text(&mut self, text: &str) {
        self.groups.clear();
        let mut group = String::new();
        for raw in text.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                group = line[1..line.len() - 1].trim().to_owned();
            } else {
                self.groups
                    .entry(group.clone())
                    .or_default()
                    .insert(line.to_owned());
            }
        }
    }
    /// Reloads the original path.
    pub fn reload(&mut self) -> Result<(), SettingError> {
        let path = self
            .path
            .as_deref()
            .ok_or_else(|| SettingError::Invalid("grouped set has no path".into()))?;
        let text = read_text(path, self.encoding)?;
        self.load_text(&text);
        Ok(())
    }
    /// Returns the original path.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
    /// Returns group names.
    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.groups.keys().map(String::as_str)
    }
    /// Returns values in a group.
    #[must_use]
    pub fn values(&self, group: &str) -> Option<&IndexSet<String>> {
        self.groups.get(group)
    }
    /// Tests that every requested value exists.
    #[must_use]
    pub fn contains(&self, group: &str, values: &[&str]) -> bool {
        self.groups
            .get(group)
            .is_some_and(|set| values.iter().all(|v| set.contains(*v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grouped_set_loads_comments_groups_and_reload() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("roles.grouped");
        std::fs::write(
            &path,
            "# comment\nroot\n[admin]\nread\nwrite\n;ignored\n[user]\nread\n",
        )
        .unwrap();
        let mut set = GroupedSet::from_path(&path).unwrap();
        assert_eq!(set.path(), Some(path.as_path()));
        assert_eq!(set.groups().collect::<Vec<_>>(), ["", "admin", "user"]);
        assert!(set.contains("admin", &["read", "write"]));
        assert!(!set.contains("admin", &["missing"]));
        assert!(!set.contains("missing", &["read"]));
        assert_eq!(set.values("").unwrap().len(), 1);
        std::fs::write(&path, "[new]\nvalue\n").unwrap();
        set.reload().unwrap();
        assert!(set.contains("new", &["value"]));
        std::fs::remove_file(&path).unwrap();
        assert!(set.reload().is_err());
        let mut detached = GroupedSet::new();
        detached.load_text("\n# only\n[group]\nx\n");
        assert!(detached.reload().is_err());
        assert!(GroupedSet::from_path("").is_err());
        assert!(GroupedSet::from_path(directory.path().join("missing")).is_err());
    }
}
