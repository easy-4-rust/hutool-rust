//! 对齐: `cn.hutool.system.UserInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/UserInfo.java`
//! 中文说明: 采集当前用户名称、主目录、工作目录、临时目录以及语言地区信息，对应 Hutool 的用户信息对象。

use std::{env, ffi::OsString, io, path::PathBuf};

use crate::system_props::SystemPropsKeys;

/// 对齐: `cn.hutool.system.UserInfo`
/// 中文说明: 当前用户和区域设置属性，包含用户名、主目录、工作目录、临时目录及语言国家信息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    /// 中文说明: 用户名。
    pub name: Option<String>,
    /// 中文说明: 用户主目录。
    pub home_dir: Option<PathBuf>,
    /// 中文说明: 当前工作目录。
    pub current_dir: Option<PathBuf>,
    /// 中文说明: 临时目录。
    pub temp_dir: PathBuf,
    /// 中文说明: ISO 风格的语言部分。
    pub language: Option<String>,
    /// 中文说明: ISO 风格的国家部分。
    pub country: Option<String>,
}

impl UserInfo {
    /// 中文说明: 从显式便携式输入创建用户信息。
    /// 对齐 Java 方法: `UserInfo` 构造逻辑
    #[must_use]
    pub fn from_parts(
        name: Option<String>,
        home_dir: Option<PathBuf>,
        current_dir: Option<PathBuf>,
        temp_dir: PathBuf,
        locale: &str,
    ) -> Self {
        let locale = locale.split('.').next().unwrap_or_default();
        let (language, country) = locale.split_once('_').map_or_else(
            || (non_empty(locale), None),
            |(language, country)| (non_empty(language), non_empty(country)),
        );
        Self {
            name,
            home_dir,
            current_dir,
            temp_dir,
            language,
            country,
        }
    }

    /// 中文说明: 采集用户、路径和区域设置属性。
    /// 对齐 Java 方法: `SystemUtil.getUserInfo`
    #[must_use]
    pub fn collect() -> Self {
        let locale = option_or_default(first_env(env::var("LC_ALL"), env::var("LANG")));
        Self::from_parts(
            first_env(
                env::var(SystemPropsKeys::USER_NAME),
                env::var("USERNAME"),
            ),
            optional_path(env::var_os(SystemPropsKeys::USER_HOME)),
            result_path(env::current_dir()),
            env::temp_dir(),
            &locale,
        )
    }
}

/// 私有辅助函数: 返回首个成功读取的环境变量。
fn first_env(
    primary: Result<String, env::VarError>,
    secondary: Result<String, env::VarError>,
) -> Option<String> {
    primary.or(secondary).ok()
}

/// 私有辅助函数: 将 `Option<String>` 转换为默认值填充的字符串。
fn option_or_default(value: Option<String>) -> String {
    value.unwrap_or_default()
}

/// 私有辅助函数: 将 `Option<OsString>` 转换为 `Option<PathBuf>`。
fn optional_path(value: Option<OsString>) -> Option<PathBuf> {
    value.map(PathBuf::from)
}

/// 私有辅助函数: 将 `io::Result<PathBuf>` 转换为 `Option<PathBuf>`。
fn result_path(value: io::Result<PathBuf>) -> Option<PathBuf> {
    value.ok()
}

/// 私有辅助函数: 过滤空字符串后返回 `Option<String>`。
fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_returns_absolute_temp_dir() {
        let user = UserInfo::collect();
        assert!(user.temp_dir.is_absolute());
    }

    #[test]
    fn from_parts_parses_locale_components() {
        let locale = UserInfo::from_parts(None, None, None, PathBuf::from("/tmp"), "zh_CN.UTF-8");
        assert_eq!(locale.language.as_deref(), Some("zh"));
        assert_eq!(locale.country.as_deref(), Some("CN"));

        let language_only = UserInfo::from_parts(None, None, None, PathBuf::from("/tmp"), "en");
        assert_eq!(language_only.language.as_deref(), Some("en"));
        assert_eq!(language_only.country, None);
    }

    #[test]
    fn helpers_cover_locale_and_io_paths() {
        assert_eq!(non_empty(""), None);
        assert_eq!(non_empty("en"), Some("en".into()));
        assert_eq!(option_or_default(None), "");
        assert_eq!(option_or_default(Some("value".into())), "value");
        assert_eq!(optional_path(None), None);
        assert_eq!(
            optional_path(Some(OsString::from("path"))),
            Some(PathBuf::from("path"))
        );
        assert_eq!(result_path(Err(io::Error::other("injected"))), None);
        assert_eq!(
            result_path(Ok(PathBuf::from("path"))),
            Some(PathBuf::from("path"))
        );

        let missing = Err(env::VarError::NotPresent);
        assert_eq!(
            first_env(missing.clone(), Ok("fallback".into())),
            Some("fallback".into())
        );
        assert_eq!(first_env(missing.clone(), missing), None);
    }
}