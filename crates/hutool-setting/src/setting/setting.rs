use super::WatchOwner;
use crate::{DEFAULT_ENCODING, GroupedMap, Props, SettingError, SettingLoader, checked_path};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use super::auto_load_handle::AutoLoadHandle;

/// Hutool 兼容的分组配置文档。
///
/// 对齐 Java 类: `cn.hutool.setting.Setting`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/Setting.java
///
/// 支持分组的键值对配置，支持变量替换、自动重载等特性。
/// Rust 版本使用 `Arc<RwLock<GroupedMap>>` 实现线程安全。
#[derive(Debug, Clone)]
pub struct Setting {
    data: Arc<RwLock<GroupedMap>>,
    path: Option<PathBuf>,
    charset: &'static Encoding,
    use_variable: bool,
    variable_prefix: String,
    variable_suffix: String,
}

impl Default for Setting {
    fn default() -> Self {
        Self::new()
    }
}

impl Setting {
    /// 创建空的 Setting。
    ///
    /// 对齐 Java: `Setting::new()`
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(GroupedMap::new())),
            path: None,
            charset: DEFAULT_ENCODING,
            use_variable: false,
            variable_prefix: "${".into(),
            variable_suffix: "}".into(),
        }
    }
    /// 别名，对齐 Hutool 命名规范。
    ///
    /// 对齐 Java: `Setting::create()`
    #[must_use]
    pub fn create() -> Self {
        Self::new()
    }
    /// 加载 UTF-8 编码的配置文件。
    ///
    /// 对齐 Java: `Setting::fromPath(String)`
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SettingError> {
        Self::from_path_with_options(path, DEFAULT_ENCODING, false)
    }
    /// Loads a path with explicit options.
    pub fn from_path_with_options(
        path: impl AsRef<Path>,
        charset: &'static Encoding,
        use_variable: bool,
    ) -> Result<Self, SettingError> {
        Self::from_path_ref(path.as_ref(), charset, use_variable)
    }
    fn from_path_ref(
        path: &Path,
        charset: &'static Encoding,
        use_variable: bool,
    ) -> Result<Self, SettingError> {
        let path = checked_path(path)?;
        let mut value = Self {
            path: Some(path),
            charset,
            use_variable,
            ..Self::new()
        };
        value.load()?;
        Ok(value)
    }
    fn loader(&self) -> SettingLoader {
        let mut loader = SettingLoader::new(self.charset, self.use_variable);
        loader
            .set_var_regex(self.variable_prefix.clone(), self.variable_suffix.clone())
            .expect("stored delimiters are valid");
        loader
    }
    /// 原子重载配置文件。
    ///
    /// 对齐 Java: `Setting::load()`
    pub fn load(&mut self) -> Result<bool, SettingError> {
        let path = self
            .path
            .as_deref()
            .ok_or_else(|| SettingError::Invalid("setting has no path".into()))?;
        let bytes = std::fs::read(path)?;
        let mut parsed = GroupedMap::new();
        let mut reader = bytes.as_slice();
        self.loader().load(&mut reader, &mut parsed)?;
        *self.data.write().expect("setting poisoned") = parsed;
        Ok(true)
    }
    /// 启动自动重载并返回其句柄。
    ///
    /// 对齐 Java: `Setting::autoLoad(Function<String, Boolean>)`
    pub fn auto_load<F>(&self, callback: F) -> Result<AutoLoadHandle, SettingError>
    where
        F: Fn(bool) + Send + Sync + 'static,
    {
        self.auto_load_arc(Arc::new(callback))
    }

    fn auto_load_arc(
        &self,
        callback: Arc<dyn Fn(bool) + Send + Sync>,
    ) -> Result<AutoLoadHandle, SettingError> {
        self.auto_load_arc_with(callback, create_recommended_watcher)
    }

    fn auto_load_arc_with(
        &self,
        callback: Arc<dyn Fn(bool) + Send + Sync>,
        factory: fn(ReloadHandler) -> Result<Box<dyn WatchOwner>, notify::Error>,
    ) -> Result<AutoLoadHandle, SettingError> {
        let path = self
            .path
            .clone()
            .ok_or_else(|| SettingError::Invalid("setting has no path".into()))?;
        let data = Arc::clone(&self.data);
        let loader = self.loader();
        let handler = ReloadHandler {
            path: path.clone(),
            loader,
            data,
            callback,
        };
        let mut watcher = factory(handler)?;
        watcher.watch_path(&path)?;
        Ok(AutoLoadHandle { _watcher: watcher })
    }
    /// 返回配置文件路径。
    ///
    /// 对齐 Java: `Setting::getPath()`
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
    /// 返回配置条目数量。
    ///
    /// 对齐 Java: `Setting::size()`
    #[must_use]
    pub fn size(&self) -> usize {
        self.data.read().expect("setting poisoned").size()
    }
    /// 按分组获取配置值。
    ///
    /// 对齐 Java: `Setting::getByGroup(String, String)`
    #[must_use]
    pub fn get_by_group(&self, key: &str, group: &str) -> Option<String> {
        self.data
            .read()
            .expect("setting poisoned")
            .get(group, key)
            .map(str::to_owned)
    }
    /// 获取默认分组的配置值。
    ///
    /// 对齐 Java: `Setting::get(String)`
    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        self.get_by_group(key, "")
    }
    /// Gets a value or default.
    #[must_use]
    pub fn get_or(&self, key: &str, group: &str, default: impl Into<String>) -> String {
        self.get_by_group(key, group)
            .unwrap_or_else(|| default.into())
    }
    /// Gets a non-empty value or default.
    #[must_use]
    pub fn get_not_empty_or(&self, key: &str, group: &str, default: impl Into<String>) -> String {
        self.get_by_group(key, group)
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| default.into())
    }
    /// Parses a typed value.
    pub fn get_parse<T: std::str::FromStr>(&self, key: &str, group: &str) -> Option<T> {
        self.get_by_group(key, group)?.parse().ok()
    }
    /// Splits a value.
    #[must_use]
    pub fn get_strings(&self, key: &str, group: &str, delimiter: &str) -> Option<Vec<String>> {
        let value = self.get_by_group(key, group)?;
        if value.trim().is_empty() {
            None
        } else {
            Some(
                value
                    .split(delimiter)
                    .map(|v| v.trim().to_owned())
                    .collect(),
            )
        }
    }
    /// Gets and removes the first matching key from the default group.
    pub fn get_and_remove(&self, keys: &[&str]) -> Option<String> {
        let mut data = self.data.write().expect("setting poisoned");
        for key in keys {
            if let Some(value) = data.remove("", key) {
                return Some(value);
            }
        }
        None
    }
    /// Returns a group snapshot.
    #[must_use]
    pub fn get_map(&self, group: &str) -> IndexMap<String, String> {
        self.data
            .read()
            .expect("setting poisoned")
            .group(group)
            .cloned()
            .unwrap_or_default()
    }
    /// Returns a subgroup as a setting.
    #[must_use]
    pub fn get_setting(&self, group: &str) -> Self {
        let result = Self::new();
        *result.data.write().expect("setting poisoned") = GroupedMap::new();
        result
            .data
            .write()
            .expect("setting poisoned")
            .put_all("", self.get_map(group));
        result
    }
    /// Returns a group as properties.
    #[must_use]
    pub fn get_props(&self, group: &str) -> Props {
        Props::from_map(self.get_map(group))
    }
    /// Serializes a group into a map.
    #[must_use]
    pub fn to_properties(&self, group: &str) -> IndexMap<String, String> {
        self.get_map(group)
    }
    /// 存储整个配置文档到文件。
    ///
    /// 对齐 Java: `Setting::store(String)`
    pub fn store(&self, path: impl AsRef<Path>) -> Result<(), SettingError> {
        self.store_path(path.as_ref())
    }
    fn store_path(&self, path: &Path) -> Result<(), SettingError> {
        let mut file = std::fs::File::create(path)?;
        self.loader()
            .store(&self.data.read().expect("setting poisoned"), &mut file)
    }
    /// Returns a data snapshot.
    #[must_use]
    pub fn grouped_map(&self) -> GroupedMap {
        self.data.read().expect("setting poisoned").clone()
    }
    /// Returns group names.
    #[must_use]
    pub fn groups(&self) -> Vec<String> {
        self.data
            .read()
            .expect("setting poisoned")
            .groups()
            .map(str::to_owned)
            .collect()
    }
    /// Changes variable delimiters.
    pub fn set_var_regex(
        &mut self,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
    ) -> Result<&mut Self, SettingError> {
        let mut loader = self.loader();
        loader.set_var_regex(prefix, suffix)?;
        self.variable_prefix = loader.variable_prefix;
        self.variable_suffix = loader.variable_suffix;
        Ok(self)
    }
    /// Changes decoding for subsequent loads.
    pub fn set_charset(&mut self, charset: &'static Encoding) -> &mut Self {
        self.charset = charset;
        self
    }
    /// Tests emptiness.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.read().expect("setting poisoned").is_empty()
    }
    /// Tests a group key.
    #[must_use]
    pub fn contains_key(&self, group: &str, key: &str) -> bool {
        self.data
            .read()
            .expect("setting poisoned")
            .contains_key(group, key)
    }
    /// Tests a group value.
    #[must_use]
    pub fn contains_value(&self, group: &str, value: &str) -> bool {
        self.data
            .read()
            .expect("setting poisoned")
            .contains_value(group, value)
    }
    /// Inserts a group value.
    pub fn put_by_group(
        &self,
        key: impl Into<String>,
        group: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<String> {
        self.data
            .write()
            .expect("setting poisoned")
            .put(group, key, value)
    }
    /// 插入默认分组的配置值。
    ///
    /// 对齐 Java: `Setting::set(String, String)`
    pub fn set(&self, key: impl Into<String>, value: impl Into<String>) -> &Self {
        self.put_by_group(key, "", value);
        self
    }
    /// Extends a group.
    pub fn put_all<I, K, V>(&self, group: impl Into<String>, values: I) -> &Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.data
            .write()
            .expect("setting poisoned")
            .put_all(group, values);
        self
    }
    /// Merges another setting.
    pub fn add_setting(&self, other: &Self) -> &Self {
        for group in other.groups() {
            self.put_all(group.clone(), other.get_map(&group));
        }
        self
    }
    /// Removes a group key.
    pub fn remove(&self, group: &str, key: &str) -> Option<String> {
        self.data
            .write()
            .expect("setting poisoned")
            .remove(group, key)
    }
    /// Clears all values.
    pub fn clear(&self) {
        self.data.write().expect("setting poisoned").clear();
    }
    /// Deserializes a group into a typed object.
    pub fn to_bean<T: DeserializeOwned>(&self, group: &str) -> Result<T, SettingError> {
        self.to_bean_with(group, crate::config_from_string_map)
    }
    fn to_bean_with<T: DeserializeOwned>(
        &self,
        group: &str,
        converter: fn(
            &std::collections::HashMap<String, String>,
        ) -> Result<config::Config, config::ConfigError>,
    ) -> Result<T, SettingError> {
        let values: std::collections::HashMap<String, String> =
            self.get_map(group).into_iter().collect();
        Ok(converter(&values)?.try_deserialize()?)
    }
}

impl notify::EventHandler for ReloadHandler {
    fn handle_event(&mut self, event: Result<notify::Event, notify::Error>) {
        reload_event(
            event,
            &self.path,
            &self.loader,
            &self.data,
            self.callback.as_ref(),
        );
    }
}

use super::{ReloadHandler, create_recommended_watcher, reload_event};
