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

use super::default_config;
use super::template_config::TemplateConfig;
use super::template_engine::TemplateEngine;
use super::template_exception::TemplateException;

/// 模板工具类，对齐 `cn.hutool.extra.template.TemplateUtil`。
pub struct TemplateUtil;

impl TemplateUtil {
    /// 对齐 `TemplateUtil.createEngine()`：根据默认配置创建引擎。
    pub fn create_engine() -> Result<Box<dyn TemplateEngine>, TemplateException> {
        Self::create_engine_with_config(default_config())
    }

    /// 对齐 `TemplateUtil.createEngine(TemplateConfig config)`：根据指定配置创建引擎。
    ///
    /// Rust 用 `dyn TemplateEngine` trait object 而非 Java `Class<? extends TemplateEngine>` 反射。
    /// 启用 `template` feature 时返回 `MinijinjaEngine`（Java 默认走 Freemarker）。
    #[cfg(feature = "template")]
    pub fn create_engine_with_config(
        config: &TemplateConfig,
    ) -> Result<Box<dyn TemplateEngine>, TemplateException> {
        let mut engine = super::MinijinjaEngine::new();
        engine.init(config)?;
        Ok(Box::new(engine))
    }

    #[cfg(not(feature = "template"))]
    pub fn create_engine_with_config(
        _config: &TemplateConfig,
    ) -> Result<Box<dyn TemplateEngine>, TemplateException> {
        Err(TemplateException::Message(
            "template feature not enabled; add `hutool-extra/template` to create engine".into(),
        ))
    }
}
