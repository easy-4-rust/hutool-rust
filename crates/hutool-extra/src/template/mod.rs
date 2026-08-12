//! 模板引擎 facade，对齐 hutool 的 `cn.hutool.extra.template.*`。
//!
//! - 配置/POJO：`TemplateConfig`/`ResourceMode`/`TemplateException`/`Template` trait
//! - 默认引擎：`MinijinjaEngine`（feature `template`，基于 [`minijinja`]）

mod abstract_template;
mod resource_mode;
mod template;
mod template_binding;
mod template_config;
mod template_engine;
mod template_exception;
mod template_factory;
mod template_util;
mod template_value;

#[cfg(feature = "template")]
mod minijinja_engine;

pub use abstract_template::AbstractTemplate;
pub use resource_mode::ResourceMode;
pub use template::Template;
pub use template_binding::TemplateBinding;
pub use template_config::TemplateConfig;
pub use template_engine::TemplateEngine;
pub use template_exception::TemplateException;
pub use template_factory::TemplateFactory;
pub use template_util::TemplateUtil;
pub use template_value::TemplateValue;

#[cfg(feature = "template")]
pub use minijinja_engine::MinijinjaEngine;

/// 默认模板配置（进程级懒加载单例），对齐 hutool 的 `TemplateConfig` 默认实例。
pub static DEFAULT_CONFIG: std::sync::OnceLock<TemplateConfig> = std::sync::OnceLock::new();

/// 获取默认模板配置，首次调用时初始化。
pub fn default_config() -> &'static TemplateConfig {
    DEFAULT_CONFIG.get_or_init(TemplateConfig::new)
}
