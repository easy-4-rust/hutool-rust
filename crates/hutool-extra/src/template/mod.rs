//! 模板引擎配置 facade。
//!
//! 1:1 对齐 hutool 的 `cn.hutool.extra.template.*` 包（不含各 template engine 适配）。
//!
//! - 原 Java 包：`cn.hutool.extra.template`
//! - 本文件覆盖：`TemplateConfig`、`ResourceMode`、`TemplateException`、`Template` interface、
//!   `AbstractTemplate` 抽象类、`TemplateUtil` 静态门面、`TemplateEngine` 接口
//! - 各 engine 适配（Beetl / Enjoy / Freemarker / Jetbrick / Rythm / Thymeleaf / Velocity / Wit）
//!   在各自的 `engine/<name>.rs` 子模块；本文件只提供配置 + facade。
//! - 迁移状态：✅ 已实现（Phase 1.4 工作）

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

/// 默认模板配置（进程级懒加载单例），对齐 hutool 的 `TemplateConfig` 默认实例。
pub static DEFAULT_CONFIG: std::sync::OnceLock<TemplateConfig> = std::sync::OnceLock::new();

/// 获取默认模板配置，首次调用时初始化。
pub fn default_config() -> &'static TemplateConfig {
    DEFAULT_CONFIG.get_or_init(TemplateConfig::new)
}
