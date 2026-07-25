//! Hutool-Rust 到 Vernal `ApplicationContext` 的具名原子装配模块。

use std::{error::Error, sync::Arc};

use vernal_context::{ApplicationModule, ApplicationModuleRegistrar, PropertySource};

use crate::{HutoolHttpComponents, HutoolSettingPropertySource};

/// 将 Hutool-Rust 工具组件、配置来源与 Profile 组织为一个 Vernal 应用模块。
///
/// 本对象属于消费方 Bridge，而不是 Hutool 工具内核或 Vernal 内核。应用先用
/// Hutool-Rust 完成 `.setting` 文件加载、变量展开和 HTTP 安全策略配置，再把最终
/// 快照交给本模块；Vernal 负责组件依赖图、属性来源优先级、Profile 和上下文生命周期。
///
/// 模块注册具有具名事务语义：HTTP Definition、Setting `PropertySource` 与 Profile
/// 要么一起进入 [`vernal_context::VernalApplicationBuilder`]，要么在任何预检失败
/// 时全部回滚。模块不使用全局 Service Locator，也不触发 classpath 式自动发现。
#[derive(Default)]
pub struct HutoolApplicationModule {
    http_components: Option<HutoolHttpComponents>,
    setting_sources_first: Vec<Arc<dyn PropertySource>>,
    setting_sources_last: Vec<Arc<dyn PropertySource>>,
    active_profiles: Vec<String>,
    default_profiles: Vec<String>,
}

impl HutoolApplicationModule {
    /// 创建尚未携带任何贡献的 Hutool 应用模块。
    ///
    /// 至少应继续声明 HTTP 组件、Setting 来源或 Profile；空模块会由 Vernal
    /// fail-closed 拒绝，避免成功注册一个没有实际能力的品牌占位。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 把一组已经完成安全策略配置的 Hutool HTTP 组件加入模块。
    #[must_use]
    pub fn with_http_components(mut self, components: HutoolHttpComponents) -> Self {
        self.http_components = Some(components);
        self
    }

    /// 使用 Hutool 默认 HTTP 配置与默认 URL 策略加入 HTTP 组件。
    ///
    /// 服务端处理用户可控 URL 时应优先显式传入
    /// [`HutoolHttpComponents::deny_local_targets`] 后的组件包。
    #[must_use]
    pub fn with_default_http(self) -> Self {
        self.with_http_components(HutoolHttpComponents::default())
    }

    /// 在 Environment 最高优先级一侧加入一份不可变 Hutool Setting 快照。
    ///
    /// 多次调用时保留声明顺序；Vernal 会在模块提交前统一校验来源名称冲突。
    #[must_use]
    pub fn setting_first(mut self, source: HutoolSettingPropertySource) -> Self {
        self.setting_sources_first.push(Arc::new(source));
        self
    }

    /// 在 Environment 最低优先级一侧加入一份不可变 Hutool Setting 快照。
    ///
    /// 该模式适合提供应用默认值，让命令行、环境变量或业务来源覆盖 Hutool 配置。
    #[must_use]
    pub fn setting_last(mut self, source: HutoolSettingPropertySource) -> Self {
        self.setting_sources_last.push(Arc::new(source));
        self
    }

    /// 声明一个 Vernal Active Profile。
    #[must_use]
    pub fn active_profile(mut self, profile: impl Into<String>) -> Self {
        self.active_profiles.push(profile.into());
        self
    }

    /// 声明一个 Vernal Default Profile。
    #[must_use]
    pub fn default_profile(mut self, profile: impl Into<String>) -> Self {
        self.default_profiles.push(profile.into());
        self
    }
}

impl ApplicationModule for HutoolApplicationModule {
    /// 返回参与应用级去重和诊断的稳定模块身份。
    fn name(&self) -> &'static str {
        "hutool.application"
    }

    /// 把全部 Hutool 贡献写入 Vernal 提供的隔离 Registrar。
    ///
    /// 本方法只声明装配意图，不直接修改真实 Registry 或 Environment；后续条件、
    /// Environment 与 `IoC` 预检均由 `register_module` 统一完成。
    fn configure(
        self,
        registrar: &mut ApplicationModuleRegistrar,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        // HTTP 配置和客户端必须作为同一个 Definition 批次登记，确保依赖图能够看到
        // HttpClient 对 HttpConfig 的显式依赖。
        if let Some(http_components) = self.http_components {
            registrar.register_all(http_components.definitions());
        }

        // 高、低优先级来源分别保持调用方声明顺序。这里仅移动已校验快照，不读取文件，
        // 因而模块重放不会受到工作目录或配置文件变化的隐式影响。
        for source in self.setting_sources_first {
            registrar.property_source_first(source);
        }
        for source in self.setting_sources_last {
            registrar.property_source_last(source);
        }

        // Profile 由 Vernal 在隔离 Environment 克隆上校验；非法名称会使整个模块回滚。
        for profile in self.active_profiles {
            registrar.active_profile(profile);
        }
        for profile in self.default_profiles {
            registrar.default_profile(profile);
        }

        Ok(())
    }
}
