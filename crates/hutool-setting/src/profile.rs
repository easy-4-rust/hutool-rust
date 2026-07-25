//! 对齐: `cn.hutool.setting.GroupedProfile` (配置档案)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedProfile.java
//! 中文说明: 显式持有缓存的配置档案，按 profile 名称分目录加载和缓存 Setting 文件。

use crate::{DEFAULT_ENCODING, Setting, SettingError};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use std::path::{Path, PathBuf};

/// 显式持有缓存的配置档案，对应 Hutool 的 `GroupedProfile`。
///
/// 对齐 Java 类: `cn.hutool.setting.GroupedProfile`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/GroupedProfile.java
///
/// 按 profile 名称分目录加载和缓存 Setting 文件。
/// Explicit, cache-owning configuration profile.
#[derive(Debug, Clone)]
pub struct Profile {
    root: PathBuf,
    profile: String,
    charset: &'static Encoding,
    use_variable: bool,
    cache: IndexMap<String, Setting>,
}
impl Default for Profile {
    fn default() -> Self {
        Self::new("default")
    }
}
impl Profile {
    /// Creates a profile rooted at the current directory.
    #[must_use]
    pub fn new(profile: impl Into<String>) -> Self {
        Self {
            root: PathBuf::from("."),
            profile: profile.into(),
            charset: DEFAULT_ENCODING,
            use_variable: false,
            cache: IndexMap::new(),
        }
    }
    /// Creates a profile with all options explicit.
    #[must_use]
    pub fn with_options(
        root: impl Into<PathBuf>,
        profile: impl Into<String>,
        charset: &'static Encoding,
        use_variable: bool,
    ) -> Self {
        Self {
            root: root.into(),
            profile: profile.into(),
            charset,
            use_variable,
            cache: IndexMap::new(),
        }
    }
    /// Loads and caches a setting under `<root>/<profile>`.
    pub fn get_setting(&mut self, name: impl AsRef<Path>) -> Result<&Setting, SettingError> {
        let name = name.as_ref();
        if name.as_os_str().is_empty() {
            return Err(SettingError::Invalid(
                "setting name must not be blank".into(),
            ));
        }
        let mut relative = PathBuf::from(&self.profile);
        relative.push(name);
        if relative.extension().is_none() {
            relative.set_extension("setting");
        }
        let key = relative.to_string_lossy().into_owned();
        if !self.cache.contains_key(&key) {
            let setting = Setting::from_path_with_options(
                self.root.join(&relative),
                self.charset,
                self.use_variable,
            )?;
            self.cache.insert(key.clone(), setting);
        }
        Ok(self.cache.get(&key).expect("profile entry inserted"))
    }
    /// Changes the active profile; cached documents remain available.
    pub fn set_profile(&mut self, profile: impl Into<String>) -> &mut Self {
        self.profile = profile.into();
        self
    }
    /// Changes the encoding for future loads.
    pub fn set_charset(&mut self, charset: &'static Encoding) -> &mut Self {
        self.charset = charset;
        self
    }
    /// Changes variable expansion for future loads.
    pub fn set_use_var(&mut self, use_variable: bool) -> &mut Self {
        self.use_variable = use_variable;
        self
    }
    /// Clears cached settings.
    pub fn clear(&mut self) -> &mut Self {
        self.cache.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn owned_profile_resolves_extensions_and_cache() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir(directory.path().join("dev")).unwrap();
        std::fs::write(directory.path().join("dev/db.setting"), "host=localhost\n").unwrap();
        let mut profile = Profile::with_options(directory.path(), "dev", DEFAULT_ENCODING, false);
        assert_eq!(
            profile.get_setting("db").unwrap().get("host"),
            Some("localhost".into())
        );
        assert_eq!(
            profile.get_setting("db.setting").unwrap().get("host"),
            Some("localhost".into())
        );
        assert!(profile.get_setting("").is_err());
        profile
            .set_charset(DEFAULT_ENCODING)
            .set_use_var(true)
            .set_profile("missing");
        assert!(profile.get_setting("db").is_err());
        profile.clear().set_profile("dev");
        assert!(profile.get_setting("db").is_ok());
        assert_eq!(
            Profile::default()
                .clear()
                .set_profile("x")
                .set_use_var(false)
                .set_charset(DEFAULT_ENCODING)
                .profile,
            "x"
        );
    }
}
