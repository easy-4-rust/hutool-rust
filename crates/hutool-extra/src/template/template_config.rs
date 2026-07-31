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

use super::resource_mode::ResourceMode;

/// 模板配置，对齐 `cn.hutool.extra.template.TemplateConfig`。
///
/// 默认值：
/// - charset = UTF-8
/// - path = `None`（ClassPath 根）
/// - resourceMode = `ResourceMode::String`
/// - useCache = `true`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateConfig {
    charset: Option<String>,
    path: Option<String>,
    resource_mode: ResourceMode,
    custom_engine: Option<String>,
    use_cache: bool,
}

impl TemplateConfig {
    /// 默认构造，使用 UTF-8 编码，默认从 String 加载模板。
    /// 对齐 `TemplateConfig()`。
    pub fn new() -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: None,
            resource_mode: ResourceMode::String,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 构造，path 显式指定，UTF-8 编码。
    /// 对齐 `TemplateConfig(String path)`。
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: Some(path.into()),
            resource_mode: ResourceMode::String,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 构造，path + resourceMode。
    /// 对齐 `TemplateConfig(String path, ResourceMode resourceMode)`。
    pub fn with_path_and_mode(path: impl Into<String>, resource_mode: ResourceMode) -> Self {
        Self {
            charset: Some("UTF-8".to_string()),
            path: Some(path.into()),
            resource_mode,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 全参数构造。
    /// 对齐 `TemplateConfig(Charset charset, String path, ResourceMode resourceMode)`。
    pub fn full(
        charset: impl AsRef<str>,
        path: impl Into<String>,
        resource_mode: ResourceMode,
    ) -> Self {
        Self {
            charset: Some(charset.as_ref().to_string()),
            path: Some(path.into()),
            resource_mode,
            custom_engine: None,
            use_cache: true,
        }
    }

    /// 对齐 `getCharset()`
    pub fn charset(&self) -> Option<&str> {
        self.charset.as_deref()
    }

    /// 对齐 `getCharsetStr()`：返回 charset 的字符串表示，`None` 时返回 `None`。
    pub fn charset_str(&self) -> Option<&str> {
        self.charset.as_deref()
    }

    /// 对齐 `setCharset(Charset)`
    pub fn set_charset(&mut self, charset: impl AsRef<str>) -> &mut Self {
        self.charset = Some(charset.as_ref().to_string());
        self
    }

    /// 对齐 `getPath()`
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// 对齐 `setPath(String)`
    pub fn set_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.path = Some(path.into());
        self
    }

    /// 对齐 `getResourceMode()`
    pub fn resource_mode(&self) -> ResourceMode {
        self.resource_mode
    }

    /// 对齐 `setResourceMode(ResourceMode)`
    pub fn set_resource_mode(&mut self, mode: ResourceMode) -> &mut Self {
        self.resource_mode = mode;
        self
    }

    /// 对齐 `getCustomEngine()`：返回自定义引擎类名（Java 用 `Class<? extends TemplateEngine>`，Rust 用字符串）。
    pub fn custom_engine(&self) -> Option<&str> {
        self.custom_engine.as_deref()
    }

    /// 对齐 `setCustomEngine(Class<? extends TemplateEngine>)`：Rust 用引擎名称字符串代替 Class 对象。
    pub fn set_custom_engine(&mut self, engine_name: impl Into<String>) -> &mut Self {
        self.custom_engine = Some(engine_name.into());
        self
    }

    /// 对齐 `isUseCache()`
    pub fn is_use_cache(&self) -> bool {
        self.use_cache
    }

    /// 对齐 `setUseCache(boolean)`
    pub fn set_use_cache(&mut self, use_cache: bool) -> &mut Self {
        self.use_cache = use_cache;
        self
    }
}

impl Default for TemplateConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ResourceMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceMode::ClassPath => write!(f, "CLASSPATH"),
            ResourceMode::File => write!(f, "FILE"),
            ResourceMode::WebRoot => write!(f, "WEB_ROOT"),
            ResourceMode::String => write!(f, "STRING"),
            ResourceMode::Composite => write!(f, "COMPOSITE"),
        }
    }
}
