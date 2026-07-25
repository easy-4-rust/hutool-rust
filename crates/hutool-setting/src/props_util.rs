//! 对齐: `cn.hutool.setting.PropsUtil`
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/PropsUtil.java
//! 中文说明: 属性文件加载工具类，提供按名称加载、查找首个存在的文件、获取系统属性等便捷方法。

use crate::SettingError;
use std::path::{Path, PathBuf};

use super::props::Props;

/// 属性文件加载工具类，对应 Hutool 的 `PropsUtil`。
///
/// 对齐 Java 类: `cn.hutool.setting.PropsUtil`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/PropsUtil.java
///
/// Properties lookup helpers.
pub struct PropsUtil;
impl PropsUtil {
    /// Loads a properties file, appending `.properties` when absent.
    pub fn get(name: impl AsRef<Path>) -> Result<Props, SettingError> {
        Props::from_path(super::setting::fix_extension(name.as_ref(), "properties"))
    }
    /// Loads the first existing file.
    pub fn get_first_found<I, P>(names: I) -> Result<Option<Props>, SettingError>
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        let names: Vec<PathBuf> = names
            .into_iter()
            .map(|name| name.as_ref().to_path_buf())
            .collect();
        Self::get_first_found_paths(&names)
    }
    fn get_first_found_paths(names: &[PathBuf]) -> Result<Option<Props>, SettingError> {
        for name in names {
            let path = super::setting::fix_extension(name, "properties");
            if path.is_file() {
                return Props::from_path(path).map(Some);
            }
        }
        Ok(None)
    }
    /// Captures environment variables as explicit properties.
    #[must_use]
    pub fn get_system_props() -> Props {
        Props::from_map(std::env::vars().collect())
    }
}
