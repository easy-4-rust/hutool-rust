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

use super::template_binding::TemplateBinding;
use super::template_exception::TemplateException;

/// 模板接口，对齐 `cn.hutool.extra.template.Template`。
///
/// 注意：原 Java 接口有 `render(Map, Writer)` 重载；Rust 用 dyn-compatibility 约束，
/// 因此 `render` 返回 `String` 而非泛型 Writer。如需流式输出，调用方可在 `render_to_string`
/// 后自行 write。
pub trait Template: Send + Sync {
    /// 渲染模板并返回字符串（对齐 `render(Map)` Java 重载）。
    fn render_to_string(&self, binding: &TemplateBinding) -> Result<String, TemplateException>;

    /// 渲染模板并写入字节缓冲（对齐 `render(Map, File)` Java 重载）。
    fn render_to_bytes(&self, binding: &TemplateBinding) -> Result<Vec<u8>, TemplateException>;
}

use super::{DEFAULT_CONFIG, default_config};
