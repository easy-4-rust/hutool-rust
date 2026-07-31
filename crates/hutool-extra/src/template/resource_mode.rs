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

/// 资源加载方式，对齐 `cn.hutool.extra.template.TemplateConfig.ResourceMode`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceMode {
    /// 从 ClassPath 加载模板
    ClassPath,
    /// 从 File 目录加载模板
    File,
    /// 从 WebRoot 目录加载模板
    WebRoot,
    /// 从模板文本加载模板
    String,
    /// 复合加载（File → ClassPath → WebRoot → String 顺序尝试）
    Composite,
}
