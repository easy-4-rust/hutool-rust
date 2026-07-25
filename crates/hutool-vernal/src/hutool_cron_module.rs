//! Hutool 定时任务模块到 Vernal Context 的桥接。
//!
//! 将 hutool-cron 的定时调度能力注入 Vernal ApplicationContext。
//! 通过 Vernal 的 `ScheduledTask` trait 统一生命周期管理。
//!
//! # 设计来源
//!
//! 对标 tx_di 的 `tx_di_job` 插件模式：cron 表达式 + 执行器。
//! 通过 Vernal 的 `ApplicationModule` trait 注册定时任务配置。

use std::error::Error;

use vernal_context::{ApplicationModule, ApplicationModuleRegistrar};
use vernal_ioc::ComponentDefinition;

/// Hutool 定时任务条目。
///
/// 描述一个 cron 调度任务的配置。
#[derive(Debug, Clone)]
pub struct HutoolCronEntry {
    /// 任务名称（用于诊断和日志）
    pub name: String,
    /// cron 表达式（标准 5 字段格式）
    pub cron_expression: String,
    /// 是否启用
    pub enabled: bool,
}

/// Hutool 定时任务配置。
///
/// 包含一组 cron 调度条目和全局调度参数。
#[derive(Debug, Clone)]
pub struct HutoolCronConfig {
    /// 调度条目列表
    pub entries: Vec<HutoolCronEntry>,
    /// 是否启用调度器
    pub enabled: bool,
}

impl Default for HutoolCronConfig {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            enabled: true,
        }
    }
}

/// Hutool 定时任务模块。
///
/// 将 hutool-cron 的定时调度能力注入 Vernal ApplicationContext。
/// 注册 `HutoolCronConfig` 作为单例配置组件。
///
/// # 使用方式
///
/// ```rust,ignore
/// use hutool_vernal::{HutoolCronModule, HutoolCronConfig, HutoolCronEntry};
///
/// let module = HutoolCronModule::new()
///     .with_config(HutoolCronConfig {
///         entries: vec![
///             HutoolCronEntry {
///                 name: "data-sync".to_string(),
///                 cron_expression: "0 */5 * * * *".to_string(),
///                 enabled: true,
///             },
///         ],
///         enabled: true,
///     });
///
/// builder.register_module(module)?;
/// ```
pub struct HutoolCronModule {
    config: HutoolCronConfig,
}

impl HutoolCronModule {
    /// 创建使用默认配置的定时任务模块。
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HutoolCronConfig::default(),
        }
    }

    /// 设置自定义定时任务配置。
    #[must_use]
    pub fn with_config(mut self, config: HutoolCronConfig) -> Self {
        self.config = config;
        self
    }

    /// 添加一个 cron 调度条目。
    #[must_use]
    pub fn with_entry(mut self, entry: HutoolCronEntry) -> Self {
        self.config.entries.push(entry);
        self
    }

    /// 返回定时任务配置的组件定义。
    fn definitions(&self) -> Vec<ComponentDefinition> {
        let config = self.config.clone();
        vec![ComponentDefinition::shared_value(config)]
    }
}

impl Default for HutoolCronModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationModule for HutoolCronModule {
    fn name(&self) -> &'static str {
        "hutool.cron"
    }

    fn configure(
        self,
        registrar: &mut ApplicationModuleRegistrar,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // 校验 cron 表达式格式
        for entry in &self.config.entries {
            if entry.enabled {
                hutool_cron::CronSchedule::parse(&entry.cron_expression).map_err(|e| {
                    format!("cron 表达式解析失败 [{}]: {}", entry.name, e)
                })?;
            }
        }

        registrar.register_all(self.definitions());

        tracing::debug!(
            entry_count = self.config.entries.len(),
            enabled = self.config.enabled,
            "Hutool 定时任务模块注册完成"
        );

        Ok(())
    }
}
