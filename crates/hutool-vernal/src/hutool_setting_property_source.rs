//! Hutool Setting 到 Vernal Environment 的不可变属性来源适配器。

use std::{fmt, path::Path};

use hutool_setting::{Profile, Setting};
use vernal_context::{EnvironmentError, MapPropertySource, PropertySource};

use crate::HutoolSettingPropertySourceError;

/// 把一份 Hutool [`Setting`] 快照投影为 Vernal [`PropertySource`]。
///
/// 默认分组中的键保持原名；命名分组使用 `<group>.<key>` 形成 Vernal 属性键。
/// 例如 `[database]` 下的 `host=localhost` 会成为 `database.host`。转换发生在
/// 应用装配阶段，后续 Hutool 自动重载不会悄悄改变已经冻结的
/// `ApplicationEnvironment`，需要新配置时应显式重建应用上下文。
///
/// 转换委托给 [`MapPropertySource`] 完整校验。若默认分组已经声明
/// `database.host`，同时 `[database]` 又声明 `host`，最终键发生冲突并按
/// fail-closed 返回错误，不使用隐式覆盖顺序。
pub struct HutoolSettingPropertySource {
    source: MapPropertySource,
}

impl HutoolSettingPropertySource {
    /// 从已经加载完成的 Setting 创建不可变属性来源。
    ///
    /// # Errors
    ///
    /// 来源名称非法、分组/键无法组成合法属性键，或扁平化后存在重复键时返回
    /// [`EnvironmentError`]；失败不会保留部分属性。
    pub fn new(
        source_name: impl Into<String>,
        setting: &Setting,
    ) -> Result<Self, EnvironmentError> {
        let grouped = setting.grouped_map();
        let mut properties = Vec::with_capacity(grouped.size());

        // 空分组是 Hutool 的默认分组，键可原样进入 Vernal；命名分组通过点号形成
        // 常见的分层属性名。这里只复制配置快照，不保存可变 Profile 或文件句柄。
        for group in grouped.groups() {
            for (key, value) in grouped.entries(group) {
                let property_key = if group.is_empty() {
                    key.to_owned()
                } else {
                    format!("{group}.{key}")
                };
                properties.push((property_key, value.to_owned()));
            }
        }

        Ok(Self {
            source: MapPropertySource::new(source_name, properties)?,
        })
    }

    /// 通过 Hutool Profile 加载指定文档并创建不可变属性来源。
    ///
    /// `setting_name` 没有扩展名时，Hutool 会在
    /// `<profile-root>/<active-profile>` 下补充 `.setting`。Profile 自己负责字符集、
    /// 变量展开和缓存；本桥只负责确定性的属性键投影。
    ///
    /// # Errors
    ///
    /// 文档加载失败，或属性来源校验失败时返回
    /// [`HutoolSettingPropertySourceError`]。
    pub fn from_profile(
        source_name: impl Into<String>,
        profile: &mut Profile,
        setting_name: impl AsRef<Path>,
    ) -> Result<Self, HutoolSettingPropertySourceError> {
        let setting = profile.get_setting(setting_name)?;
        Self::new(source_name, setting).map_err(Into::into)
    }

    /// 返回快照中的属性数量。
    #[must_use]
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// 判断快照是否不含任何属性。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }
}

impl PropertySource for HutoolSettingPropertySource {
    fn name(&self) -> &str {
        self.source.name()
    }

    fn get(&self, key: &str) -> Result<Option<String>, EnvironmentError> {
        self.source.get(key)
    }
}

impl fmt::Debug for HutoolSettingPropertySource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HutoolSettingPropertySource")
            .field("name", &self.source.name())
            .field("property_count", &self.source.len())
            .finish()
    }
}
