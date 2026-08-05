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

use super::template::Template;
use super::template_config::TemplateConfig;
use super::template_exception::TemplateException;

/// 模板引擎抽象，对齐 `cn.hutool.extra.template.TemplateEngine` 接口。
///
/// 各具体引擎（Beetl/Enjoy/Freemarker/Rythm/Thymeleaf/Velocity/Wit/Jetbrick）由
/// `engine/<name>.rs` 子模块实现 `TemplateEngine` trait。
pub trait TemplateEngine: Send + Sync {
    /// 初始化引擎
    fn init(&mut self, config: &TemplateConfig) -> Result<(), TemplateException>;

    /// 获取原始引擎（Java `getRawEngine()` 返回 `Object`）
    fn raw_engine(&self) -> Option<&dyn std::any::Any>;

    /// 根据资源名获取模板
    fn get_template(&self, resource: &str) -> Result<Box<dyn Template>, TemplateException>;
}
