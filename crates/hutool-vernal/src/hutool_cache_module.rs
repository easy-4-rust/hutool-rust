//! Hutool 缓存模块到 Vernal Context 的桥接。
//!
//! 将 hutool-cache 的缓存能力注入 Vernal ApplicationContext。
//! 支持内存缓存（moka）和可选的 TTL 配置。
//!
//! # 设计来源
//!
//! 对标 tx_di 的 `tx_di_cache` 插件模式：配置组件 + trait + 实现组件。
//! 通过 Vernal 的 `ApplicationModule` trait 统一生命周期管理。

use std::error::Error;

use vernal_context::{ApplicationModule, ApplicationModuleRegistrar};
use vernal_ioc::ComponentDefinition;

/// Hutool 缓存配置。
///
/// 从 Vernal `ApplicationEnvironment` 绑定缓存参数。
/// 对标 tx_di 的 `#[component(conf)]` 配置组件模式。
#[derive(Debug, Clone)]
pub struct HutoolCacheConfig {
    /// 缓存最大容量（条目数）
    pub max_capacity: u64,
    /// 默认 TTL（秒），0 表示永不过期
    pub ttl_secs: u64,
    /// 默认 TTI（秒），0 表示不启用空闲过期
    pub tti_secs: u64,
}

impl Default for HutoolCacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            ttl_secs: 300,  // 5 分钟
            tti_secs: 0,    // 不启用
        }
    }
}

/// Hutool 缓存模块。
///
/// 将 hutool-cache 的缓存能力注入 Vernal ApplicationContext。
/// 注册 `HutoolCacheConfig` 作为单例配置组件。
///
/// # 使用方式
///
/// ```rust,ignore
/// use hutool_vernal::{HutoolCacheModule, HutoolCacheConfig};
///
/// let module = HutoolCacheModule::new()
///     .with_config(HutoolCacheConfig {
///         max_capacity: 50_000,
///         ttl_secs: 600,
///         tti_secs: 120,
///     });
///
/// builder.register_module(module)?;
/// ```
pub struct HutoolCacheModule {
    config: HutoolCacheConfig,
}

impl HutoolCacheModule {
    /// 创建使用默认配置的缓存模块。
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: HutoolCacheConfig::default(),
        }
    }

    /// 设置自定义缓存配置。
    #[must_use]
    pub fn with_config(mut self, config: HutoolCacheConfig) -> Self {
        self.config = config;
        self
    }

    /// 返回缓存配置的组件定义。
    fn definitions(&self) -> Vec<ComponentDefinition> {
        let config = self.config.clone();
        vec![
            // 注册 HutoolCacheConfig 为共享值单例
            ComponentDefinition::shared_value(config),
        ]
    }
}

impl Default for HutoolCacheModule {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationModule for HutoolCacheModule {
    fn name(&self) -> &'static str {
        "hutool.cache"
    }

    fn configure(
        self,
        registrar: &mut ApplicationModuleRegistrar,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // 注册缓存配置组件
        registrar.register_all(self.definitions());

        tracing::debug!(
            max_capacity = self.config.max_capacity,
            ttl_secs = self.config.ttl_secs,
            tti_secs = self.config.tti_secs,
            "Hutool 缓存模块注册完成"
        );

        Ok(())
    }
}
