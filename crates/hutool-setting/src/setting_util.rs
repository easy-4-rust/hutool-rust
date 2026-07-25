//! 对齐: `cn.hutool.setting.SettingUtil` (静态工具)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/SettingUtil.java
//! 中文说明: 基于路径的 Setting 便捷加载工具，支持自动补全扩展名和查找首个存在的文件。

use crate::{Setting, SettingError};
use std::path::{Path, PathBuf};

/// 基于路径的 Setting 便捷加载工具，对应 Hutool 的 `SettingUtil`。
///
/// 对齐 Java 类: `cn.hutool.setting.SettingUtil`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/SettingUtil.java
///
/// Path-based convenience operations.
pub struct SettingUtil;
impl SettingUtil {
    /// Loads a setting, appending `.setting` when absent.
    pub fn get(name: impl AsRef<Path>) -> Result<Setting, SettingError> {
        Setting::from_path(super::setting::fix_extension(name.as_ref(), "setting"))
    }
    /// Loads the first existing name.
    pub fn get_first_found<I, P>(names: I) -> Result<Option<Setting>, SettingError>
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
    fn get_first_found_paths(names: &[PathBuf]) -> Result<Option<Setting>, SettingError> {
        for name in names {
            let path = super::setting::fix_extension(name, "setting");
            if path.is_file() {
                return Setting::from_path(path).map(Some);
            }
        }
        Ok(None)
    }
}
