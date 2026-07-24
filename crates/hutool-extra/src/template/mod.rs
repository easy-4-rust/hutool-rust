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

use std::fmt;
use std::path::Path;

use thiserror::Error;

mod template_exception;
mod resource_mode;
mod template_engine;
mod template;
mod template_binding;
mod template_value;
mod template_config;
mod abstract_template;
mod template_util;
mod template_factory;

pub use template_exception::TemplateException;
pub use resource_mode::ResourceMode;
pub use template_engine::TemplateEngine;
pub use template::Template;
pub use template_binding::TemplateBinding;
pub use template_value::TemplateValue;
pub use template_config::TemplateConfig;
pub use abstract_template::AbstractTemplate;
pub use template_util::TemplateUtil;
pub use template_factory::TemplateFactory;

pub static DEFAULT_CONFIG: std::sync::OnceLock<TemplateConfig> = std::sync::OnceLock::new();

pub fn default_config() -> &'static TemplateConfig {
    DEFAULT_CONFIG.get_or_init(TemplateConfig::new)
}
