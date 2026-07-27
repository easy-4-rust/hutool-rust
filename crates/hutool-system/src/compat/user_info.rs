//! 对齐: `cn.hutool.system.UserInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/UserInfo.java`
//! 中文说明: 采集当前用户名称、主目录、工作目录、临时目录以及语言地区信息，对应 Hutool 的用户信息对象。

use std::{env, ffi::OsString, io, path::PathBuf};

use super::SystemPropsKeys;

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
            first_env(env::var(SystemPropsKeys::USER_NAME), env::var("USERNAME")),
            optional_path(env::var_os(SystemPropsKeys::USER_HOME)),
            result_path(env::current_dir()),
            env::temp_dir(),
            &locale,
        )
    }
}

pub(crate) fn first_env(
    primary: Result<String, env::VarError>,
    secondary: Result<String, env::VarError>,
) -> Option<String> {
    primary.or(secondary).ok()
}

pub(crate) fn option_or_default(value: Option<String>) -> String {
    value.unwrap_or_default()
}

pub(crate) fn optional_path(value: Option<OsString>) -> Option<PathBuf> {
    value.map(PathBuf::from)
}

pub(crate) fn result_path(value: io::Result<PathBuf>) -> Option<PathBuf> {
    value.ok()
}

pub(crate) fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}
