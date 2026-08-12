//! Hutool 文件存储模块到 Vernal Context 的桥接。
//!
//! 将 hutool-extra 的文件存储能力注入 Vernal ApplicationContext。
//! 支持本地文件系统存储。
//!
//! # 设计来源
//!
//! 对标 tx_di 的 `tx_di_file` 插件模式：基于 OpenDAL 的统一文件存储抽象。
//! 通过 Vernal 的 `ApplicationModule` trait 统一生命周期管理。

use std::error::Error;

use vernal_context::{ApplicationModule, ApplicationModuleRegistrar};
use vernal_ioc::ComponentDefinition;

/// Hutool 文件存储配置。
///
/// 配置文件存储的根目录和行为参数。
#[derive(Debug, Clone)]
pub struct HutoolFileStorageConfig {
    /// 文件存储根目录
    pub base_dir: String,
    /// 是否自动创建不存在的目录
    pub auto_create_dir: bool,
    /// 单个文件最大大小（字节），0 表示不限制
    pub max_file_size: u64,
}

impl Default for HutoolFileStorageConfig {
    fn default() -> Self {
        Self {
            base_dir: "./storage".to_string(),
            auto_create_dir: true,
            max_file_size: 0,
        }
    }
}

/// Hutool 文件存储模块。
///
/// 将文件存储能力注入 Vernal ApplicationContext。
/// 注册 `HutoolFileStorageConfig` 作为单例配置组件。
///
/// # 使用方式
///
/// ```rust,ignore
/// use hutool_vernal::{HutoolFileStorageModule, HutoolFileStorageConfig};
///
/// let module = HutoolFileStorageModule::new()
///     .with_config(HutoolFileStorageConfig {
///         base_dir: "/data/files".to_string(),
///         auto_create_dir: true,
///         max_file_size: 100 * 1024 * 1024, // 100MB
///     });
///
/// builder.register_module(module)?;
/// ```
pub struct HutoolFileStorageModule {
    config: HutoolFileStorageConfig,
}

impl HutoolFileStorageModule {
    /// 创建使用默认配置的文件存储模块。
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HutoolFileStorageConfig::default(),
        }
    }

    /// 设置自定义文件存储配置。
    #[must_use]
    pub fn with_config(mut self, config: HutoolFileStorageConfig) -> Self {
        self.config = config;
        self
    }

    /// 返回文件存储配置的组件定义。
    fn definitions(&self) -> Vec<ComponentDefinition> {
        let config = self.config.clone();
        vec![ComponentDefinition::shared_value(config)]
    }
}

impl Default for HutoolFileStorageModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationModule for HutoolFileStorageModule {
    fn name(&self) -> &'static str {
        "hutool.file-storage"
    }

    fn configure(
        self,
        registrar: &mut ApplicationModuleRegistrar,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // 如果启用了自动创建目录，尝试创建根目录
        if self.config.auto_create_dir {
            let base_dir = std::path::Path::new(&self.config.base_dir);
            if !base_dir.exists() {
                std::fs::create_dir_all(base_dir).map_err(|e| {
                    format!("创建文件存储目录失败: {}: {}", self.config.base_dir, e)
                })?;
            }
        }

        registrar.register_all(self.definitions());

        tracing::debug!(
            base_dir = %self.config.base_dir,
            auto_create = self.config.auto_create_dir,
            "Hutool 文件存储模块注册完成"
        );

        Ok(())
    }
}
