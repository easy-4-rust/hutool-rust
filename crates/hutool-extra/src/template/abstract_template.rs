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

use std::path::Path;

use super::template_binding::TemplateBinding;
use super::template_config::TemplateConfig;
use super::template_exception::TemplateException;
use super::template_value::TemplateValue;

/// 模板抽象基类，对齐 `cn.hutool.extra.template.AbstractTemplate`。
///
/// 持有配置和绑定映射，提供 `render(File)` 和 `render(Map)` 两种重载。
pub struct AbstractTemplate {
    // 保留给具体 engine 委托（Phase 1.4 子任务）使用
    #[allow(dead_code)]
    config: TemplateConfig,
    binding: TemplateBinding,
}

impl AbstractTemplate {
    /// 创建抽象模板实例（包级调用）
    pub fn new(config: TemplateConfig) -> Self {
        Self {
            config,
            binding: TemplateBinding::new(),
        }
    }

    /// 设置 binding
    pub fn with_binding(mut self, binding: TemplateBinding) -> Self {
        self.binding = binding;
        self
    }

    /// 添加单个 binding
    pub fn bind(mut self, key: impl Into<String>, value: impl Into<TemplateValue>) -> Self {
        self.binding.insert(key.into(), value.into());
        self
    }

    /// 对齐 `render(Map, File)`：渲染到文件
    pub fn render_to_file(
        &self,
        _file: &Path,
    ) -> Result<(), TemplateException> {
        // TODO: 委托到具体 Template 实现（Phase 1.4 子任务）
        Err(TemplateException::Message(
            "AbstractTemplate::render(File) requires a concrete TemplateEngine; see engine/<name>.rs".into(),
        ))
    }

    /// 对齐 `render(Map)`：返回字符串
    pub fn render(&self) -> Result<String, TemplateException> {
        // TODO: 委托到具体 Template 实现
        Err(TemplateException::Message(
            "AbstractTemplate::render() requires a concrete TemplateEngine; see engine/<name>.rs".into(),
        ))
    }
}
