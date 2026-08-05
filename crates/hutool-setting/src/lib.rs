//! 对齐: `cn.hutool.setting` (配置模块)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/
//! 中文说明: Hutool 兼容的有序配置工具集，支持分组键值对、属性文件、YAML 解析等。
//!
//! Hutool-aligned, ordered configuration utilities with explicit ownership.

#![forbid(unsafe_code)]
#![allow(
    clippy::assigning_clones,
    clippy::missing_panics_doc,
    clippy::struct_field_names
)]

mod global_profile;
mod grouped_map;
mod grouped_set;
pub mod prelude;
mod profile;
mod props;
mod props_util;
mod setting;
mod setting_loader;
mod setting_util;
mod yaml;

pub use config::{Config, ConfigError, Environment, File, FileFormat};
pub use global_profile::GlobalProfile;
pub use grouped_map::GroupedMap;
pub use grouped_set::GroupedSet;
pub use profile::Profile;
pub use props::Props;
pub use props_util::PropsUtil;
pub use setting::{AutoLoadHandle, Setting};
pub use setting_loader::SettingLoader;
pub use setting_util::SettingUtil;
pub use yaml::YamlUtil;

use encoding_rs::{Encoding, UTF_8};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};

/// Errors produced by Hutool-compatible configuration operations.
#[derive(Debug, thiserror::Error)]
pub enum SettingError {
    /// A path, group, key, encoding, or expression was invalid.
    #[error("invalid setting: {0}")]
    Invalid(String),
    /// A filesystem operation failed.
    #[error("setting I/O failed: {0}")]
    Io(#[from] std::io::Error),
    /// Decoding failed for the configured character set.
    #[error("setting input is not valid {0}")]
    Decode(&'static str),
    /// A dynamic configuration operation failed.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// YAML parsing or serialization failed.
    #[error(transparent)]
    Yaml(#[from] serde_yaml_ng::Error),
    /// A file-watching operation failed.
    #[error("setting watcher failed: {0}")]
    Watch(#[from] notify::Error),
}

/// Hutool's runtime exception analogue.
pub type SettingRuntimeException = SettingError;

/// One configuration file and whether it must exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSource {
    /// File path; its extension selects TOML, JSON, YAML, and other formats.
    pub path: PathBuf,
    /// Whether loading fails when the file does not exist.
    pub required: bool,
}

/// Declarative settings sources, applied in the order files then environment.
#[derive(Debug, Clone, Default)]
pub struct SettingsLoader {
    files: Vec<FileSource>,
    environment_prefix: Option<String>,
    environment_separator: String,
}

impl SettingsLoader {
    /// Creates an empty loader.
    #[must_use]
    pub fn new() -> Self {
        Self {
            environment_separator: "__".to_owned(),
            ..Self::default()
        }
    }

    /// Adds a required configuration file.
    #[must_use]
    pub fn required_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.files.push(FileSource {
            path: path.into(),
            required: true,
        });
        self
    }

    /// Adds an optional configuration file.
    #[must_use]
    pub fn optional_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.files.push(FileSource {
            path: path.into(),
            required: false,
        });
        self
    }

    /// Adds environment variables with the given prefix.
    #[must_use]
    pub fn environment(mut self, prefix: impl Into<String>) -> Self {
        self.environment_prefix = Some(prefix.into());
        self
    }

    /// Builds the merged dynamic configuration.
    pub fn build(&self) -> Result<Config, ConfigError> {
        let mut builder = Config::builder();
        for source in &self.files {
            builder = builder.add_source(File::from(source.path.clone()).required(source.required));
        }
        if let Some(prefix) = &self.environment_prefix {
            builder = builder.add_source(
                Environment::with_prefix(prefix)
                    .prefix_separator("_")
                    .separator(&self.environment_separator),
            );
        }
        builder.build()
    }

    /// Builds and deserializes the merged settings into a typed value.
    pub fn load<T: DeserializeOwned>(&self) -> Result<T, ConfigError> {
        self.build()?.try_deserialize()
    }
}

pub(crate) fn checked_path(path: &Path) -> Result<PathBuf, SettingError> {
    if path.as_os_str().is_empty() {
        return Err(SettingError::Invalid("path must not be blank".into()));
    }
    Ok(path.to_path_buf())
}

pub(crate) fn read_text(path: &Path, encoding: &'static Encoding) -> Result<String, SettingError> {
    let bytes = std::fs::read(path)?;
    let (text, _, malformed) = encoding.decode(&bytes);
    if malformed {
        return Err(SettingError::Decode(encoding.name()));
    }
    Ok(text.into_owned())
}

/// UTF-8 charset used by Hutool `Setting` by default (exposed for parity tests).
pub const DEFAULT_ENCODING: &Encoding = UTF_8;

/// Builds a nested [`Config`] from flat property keys.
///
/// Supports Hutool-style dotted paths (`credentials.username`) and array
/// indices (`hobby[0]`) so `to_bean` can deserialize nested beans. Bracket
/// subscripts must be preserved — config treats `.0` as a map key, not an index.
pub(crate) fn config_from_string_map(
    values: &std::collections::HashMap<String, String>,
) -> Result<Config, ConfigError> {
    let mut builder = Config::builder();
    for (key, value) in values {
        builder = builder.set_override(normalize_property_key(key), value.clone())?;
    }
    builder.build()
}

/// Normalizes Hutool property keys into config-crate path expressions.
///
/// Leaves `hobby[0]` / `credentials.username` intact so array indices stay
/// [`config`] Index postfixes rather than string map keys.
pub(crate) fn normalize_property_key(key: &str) -> String {
    key.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct AppSettings {
        name: String,
        workers: u16,
    }

    #[test]
    fn loads_a_typed_json_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/app.json");
        let loaded: AppSettings = SettingsLoader::new().required_file(path).load().unwrap();
        assert_eq!(
            loaded,
            AppSettings {
                name: "hutool".into(),
                workers: 4
            }
        );
        let optional = SettingsLoader::new()
            .optional_file("/definitely/missing/config.json")
            .environment("HITOOL_SETTING_TEST_NEVER")
            .build()
            .unwrap();
        assert!(
            optional
                .try_deserialize::<std::collections::HashMap<String, String>>()
                .unwrap()
                .is_empty()
        );
        let directory = tempfile::tempdir().unwrap();
        let invalid = directory.path().join("invalid.txt");
        std::fs::write(&invalid, [0xff]).unwrap();
        let decode_error = read_text(&invalid, DEFAULT_ENCODING).unwrap_err();
        assert!(decode_error.to_string().contains("UTF-8"));
        assert!(read_text(&directory.path().join("missing"), DEFAULT_ENCODING).is_err());
        assert!(
            SettingsLoader::new()
                .required_file(directory.path().join("missing.json"))
                .load::<AppSettings>()
                .is_err()
        );
    }
}
