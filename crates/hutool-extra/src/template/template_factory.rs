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

use super::template_config::TemplateConfig;
use super::template_engine::TemplateEngine;
use super::template_exception::TemplateException;
use super::template_util::TemplateUtil;

/// 模板引擎工厂，对齐 `cn.hutool.extra.template.engine.TemplateFactory`。
pub struct TemplateFactory;

impl TemplateFactory {
    /// 对齐 `TemplateFactory.create(TemplateConfig config)`
    pub fn create(config: &TemplateConfig) -> Result<Box<dyn TemplateEngine>, TemplateException> {
        TemplateUtil::create_engine_with_config(config)
    }

    /// 对齐 `TemplateFactory.get()`：使用默认配置。
    pub fn get() -> Result<Box<dyn TemplateEngine>, TemplateException> {
        TemplateUtil::create_engine_with_config(default_config())
    }
}

use super::{DEFAULT_CONFIG, default_config};
