//! 对齐: `cn.hutool.setting.dialect.YamlProps` (YAML 工具)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/dialect/YamlProps.java
//! 中文说明: 安全的 YAML 解析与序列化工具类，基于 Serde 实现，支持文件路径和 Reader/Writer。

use crate::{SettingError, checked_path};
use serde::{Serialize, de::DeserializeOwned};
use serde_yaml_ng::Value;
use std::{
    io::{Read, Write},
    path::Path,
};

/// 安全的 YAML 解析与序列化工具类，对应 Hutool 的 `YamlProps`。
///
/// 对齐 Java 类: `cn.hutool.setting.dialect.YamlProps`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/dialect/YamlProps.java
///
/// Safe YAML parsing and dumping helpers backed by Serde.
pub struct YamlUtil;
impl YamlUtil {
    /// Loads a YAML path into a dynamic value.
    pub fn load_by_path(path: impl AsRef<Path>) -> Result<Value, SettingError> {
        Self::load_path_as(path)
    }
    /// Loads a YAML path into a typed value.
    pub fn load_path_as<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, SettingError> {
        Self::load_path_as_ref(path.as_ref())
    }
    fn load_path_as_ref<T: DeserializeOwned>(path: &Path) -> Result<T, SettingError> {
        let path = checked_path(path)?;
        let file = std::fs::File::open(path)?;
        Self::load(file)
    }
    /// Loads YAML from a reader.
    pub fn load<T: DeserializeOwned, R: Read>(reader: R) -> Result<T, SettingError> {
        Ok(serde_yaml_ng::from_reader(reader)?)
    }
    /// Loads YAML from text.
    pub fn load_str<T: DeserializeOwned>(text: &str) -> Result<T, SettingError> {
        Ok(serde_yaml_ng::from_str(text)?)
    }
    /// Dumps YAML to a writer.
    pub fn dump<T: Serialize, W: Write>(object: &T, writer: W) -> Result<(), SettingError> {
        serde_yaml_ng::to_writer(writer, object)?;
        Ok(())
    }
    /// Dumps YAML to a string.
    pub fn dump_string<T: Serialize>(object: &T) -> Result<String, SettingError> {
        Ok(serde_yaml_ng::to_string(object)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Data {
        name: String,
        count: u8,
    }
    struct ToggleSerialize(std::cell::Cell<bool>);
    impl Serialize for ToggleSerialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.0.get() {
                Data {
                    name: "hutool".into(),
                    count: 2,
                }
                .serialize(serializer)
            } else {
                Err(serde::ser::Error::custom("intentional"))
            }
        }
    }
    #[test]
    fn yaml_loads_and_dumps_dynamic_and_typed_values() {
        let data = Data {
            name: "hutool".into(),
            count: 2,
        };
        let toggle = ToggleSerialize(std::cell::Cell::new(true));
        let text = YamlUtil::dump_string(&toggle).unwrap();
        assert_eq!(YamlUtil::load_str::<Data>(&text).unwrap(), data);
        let mut output = Vec::new();
        YamlUtil::dump(&toggle, &mut output).unwrap();
        assert_eq!(YamlUtil::load::<Data, _>(output.as_slice()).unwrap(), data);
        assert!(YamlUtil::load::<Data, _>(b"[".as_slice()).is_err());
        toggle.0.set(false);
        assert!(YamlUtil::dump_string(&toggle).is_err());
        assert!(YamlUtil::dump(&toggle, &mut Vec::new()).is_err());
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("data.yml");
        std::fs::write(&path, &text).unwrap();
        assert_eq!(YamlUtil::load_path_as::<Data>(&path).unwrap(), data);
        assert!(YamlUtil::load_path_as::<Data>(directory.path().join("missing")).is_err());
        assert!(YamlUtil::load_path_as::<Data>("").is_err());
        assert!(YamlUtil::load_by_path(&path).unwrap().is_mapping());
        assert!(YamlUtil::load_by_path("").is_err());
        assert!(YamlUtil::load_by_path(directory.path().join("missing")).is_err());
        assert!(YamlUtil::load_str::<Data>("[").is_err());
        let invalid_path = directory.path().join("invalid.yml");
        std::fs::write(&invalid_path, "[").unwrap();
        assert!(YamlUtil::load_path_as::<Data>(&invalid_path).is_err());
        assert!(YamlUtil::load_by_path(&invalid_path).is_err());
    }
}
