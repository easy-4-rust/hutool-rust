//! minijinja 模板引擎，对齐 hutool 默认引擎（Freemarker/Velocity 风格）语义。
//!
//! 对齐 Java `cn.hutool.extra.template.engine.freemarker.FreemarkerEngine`：
//! - `init(config)`：设置资源模式（String/File），登记命名模板
//! - `get_template(resource)`：返回自包含 `Template`（持引擎 Arc 副本）
//! - `render(binding)`：渲染为字符串
//!
//! minijinja 语法兼容 Jinja2（`{{ var }}` 占位、`{% for %}` 控制流），
//! 与 hutool 默认 Thymeleaf/Velocity 的占位语义对齐。

use std::collections::HashMap;
use std::sync::Arc;

use super::resource_mode::ResourceMode;
use super::template::Template;
use super::template_binding::TemplateBinding;
use super::template_config::TemplateConfig;
use super::template_engine::TemplateEngine;
use super::template_exception::TemplateException;

/// minijinja 引擎封装。Environment 以 Arc 共享，供返回的 Template 自包含渲染。
pub struct MinijinjaEngine {
    env: Arc<minijinja::Environment<'static>>,
    /// 命名模板源码缓存（File/ClassPath 模式 init 时登记）。
    named: HashMap<String, String>,
}

impl Default for MinijinjaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MinijinjaEngine {
    /// 创建空引擎实例。
    #[must_use]
    pub fn new() -> Self {
        Self {
            env: Arc::new(minijinja::Environment::new()),
            named: HashMap::new(),
        }
    }
}

impl TemplateEngine for MinijinjaEngine {
    fn init(&mut self, config: &TemplateConfig) -> Result<(), TemplateException> {
        let _ = config.charset();
        if config.resource_mode() == ResourceMode::File
            && let Some(path) = config.path()
                && let Ok(source) = std::fs::read_to_string(path) {
                    let name = std::path::Path::new(path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("template")
                        .to_string();
                    self.named.insert(name, source);
                }
        Ok(())
    }

    fn raw_engine(&self) -> Option<&dyn std::any::Any> {
        Some(self.env.as_ref())
    }

    fn get_template(&self, resource: &str) -> Result<Box<dyn Template>, TemplateException> {
        // 已登记的命名模板：取出源码走 source 路径；否则 resource 视为内联源码
        let source = self
            .named
            .get(resource)
            .cloned()
            .or(Some(resource.to_string()));
        Ok(Box::new(MinijinjaTemplate {
            env: Arc::clone(&self.env),
            source,
        }))
    }
}

/// minijinja 模板句柄（持引擎 Arc 副本，自包含渲染）。
struct MinijinjaTemplate {
    env: Arc<minijinja::Environment<'static>>,
    source: Option<String>,
}

impl Template for MinijinjaTemplate {
    fn render_to_string(&self, binding: &TemplateBinding) -> Result<String, TemplateException> {
        let value = minijinja::Value::from_serialize(binding);
        match &self.source {
            Some(source) => self.env.render_str(source, value).map_err(map_mj_err),
            None => Err(TemplateException::Message(
                "template source is empty".into(),
            )),
        }
    }

    fn render_to_bytes(&self, binding: &TemplateBinding) -> Result<Vec<u8>, TemplateException> {
        Ok(self.render_to_string(binding)?.into_bytes())
    }
}

fn map_mj_err(error: minijinja::Error) -> TemplateException {
    TemplateException::WithCause {
        message: format!("minijinja error: {error}"),
        source: Box::new(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_string_template() {
        let engine = MinijinjaEngine::new();
        let template = engine
            .get_template("Hello {{ name }}!")
            .expect("get template");
        let mut binding = TemplateBinding::new();
        binding.insert("name".to_string(), serde_json::json!("hutool"));
        let out = template.render_to_string(&binding).expect("render");
        assert_eq!(out, "Hello hutool!");
    }

    #[test]
    fn render_loop_and_filter() {
        let engine = MinijinjaEngine::new();
        let src = "{% for item in items %}{{ item }}{% endfor %}";
        let template = engine.get_template(src).expect("get template");
        let mut binding = TemplateBinding::new();
        binding.insert("items".to_string(), serde_json::json!(["a", "b", "c"]));
        let out = template.render_to_string(&binding).expect("render");
        assert_eq!(out, "abc");
    }

    #[test]
    fn render_to_bytes_matches_string() {
        let engine = MinijinjaEngine::new();
        let template = engine.get_template("X={{ v }}").expect("get template");
        let mut binding = TemplateBinding::new();
        binding.insert("v".to_string(), serde_json::json!(42));
        let bytes = template.render_to_bytes(&binding).expect("render");
        assert_eq!(bytes, b"X=42");
    }

    #[test]
    fn syntax_error_propagates() {
        let engine = MinijinjaEngine::new();
        let template = engine.get_template("{{ unclosed").expect("get template");
        let result = template.render_to_string(&TemplateBinding::new());
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            TemplateException::WithCause { .. }
        ));
    }

    #[test]
    fn file_mode_init_loads_path() {
        let dir = std::env::temp_dir();
        let path = dir.join("mj_test_template.j2");
        std::fs::write(&path, "From file: {{ n }}").expect("write");
        let mut engine = MinijinjaEngine::new();
        let mut config = TemplateConfig::new();
        config
            .set_resource_mode(ResourceMode::File)
            .set_path(path.to_str().unwrap());
        engine.init(&config).expect("init");
        let template = engine
            .get_template("mj_test_template")
            .expect("get template");
        let mut binding = TemplateBinding::new();
        binding.insert("n".to_string(), serde_json::json!(7));
        let out = template.render_to_string(&binding).expect("render");
        assert_eq!(out, "From file: 7");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn raw_engine_returns_environment() {
        let engine = MinijinjaEngine::new();
        assert!(engine.raw_engine().is_some());
    }
}
