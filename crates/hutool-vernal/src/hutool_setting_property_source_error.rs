//! Hutool Setting 转换为 Vernal 属性来源时的结构化错误。

use hutool_setting::SettingError;
use vernal_context::EnvironmentError;

/// Hutool 配置装载或不可变属性快照构建失败。
///
/// 顶层错误文本只说明失败阶段，不拼接文件内容、属性值或密钥；调用方仍可通过
/// 标准 [`std::error::Error::source`] 错误链在受控的服务端诊断中查看原始原因。
#[derive(Debug, thiserror::Error)]
pub enum HutoolSettingPropertySourceError {
    /// `Profile` 无法读取或解析目标 `.setting` 文档。
    #[error("failed to load the Hutool setting document")]
    Load(#[source] SettingError),

    /// 文档中的分组和键无法组成合法且无冲突的 Vernal 属性键。
    #[error("failed to build the Vernal property source")]
    Environment(#[source] EnvironmentError),
}

impl From<SettingError> for HutoolSettingPropertySourceError {
    fn from(error: SettingError) -> Self {
        Self::Load(error)
    }
}

impl From<EnvironmentError> for HutoolSettingPropertySourceError {
    fn from(error: EnvironmentError) -> Self {
        Self::Environment(error)
    }
}
