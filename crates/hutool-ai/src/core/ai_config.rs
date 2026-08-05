//! `AIConfig` trait。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.AIConfig`
//!
//! Java 端使用接口 + Bean 风格（`setApiKey/getApiKey` 等），Rust 端将其映射为读写访问器，
//! 默认实现仅暴露读取接口。

use crate::ModelName;
use secrecy::SecretString;
use serde_json::{Map, Value};
use std::time::Duration;
use url::Url;

/// 所有 Hutool AI 提供商共享的配置契约。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.AIConfig`
pub trait AIConfig: std::fmt::Debug + Send + Sync {
    /// 厂商名称。
    fn model_name(&self) -> ModelName;

    /// API 凭据。
    fn api_key(&self) -> &SecretString;

    /// API 根地址。
    fn api_url(&self) -> &Url;

    /// 具体模型标识。
    fn model(&self) -> &str;

    /// 动态扩展字段集合。
    fn additional(&self) -> &Map<String, Value>;

    /// 连接/请求超时。
    fn timeout(&self) -> Duration;

    /// 流式读取超时。
    fn read_timeout(&self) -> Duration;

    /// 可选 HTTP 代理。
    fn proxy(&self) -> Option<&Url>;
}

/// Java 端使用 `setApiKey` 等可变接口，Rust 端将其抽象为独立 trait 以维持不可变性。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.AIConfig` 的 `setApiKey/setApiUrl/...` 方法。
pub trait AIConfigMut: AIConfig {
    /// 替换 API key。
    fn set_api_key(&mut self, value: impl Into<String>);

    /// 替换 API URL。
    /// 注：`BaseConfig` 提供同名 inherent 方法，本 trait 方法作为 Java 镜像保留。
    #[allow(dead_code)]
    fn set_api_url(&mut self, value: &str) -> Result<(), url::ParseError>;

    /// 替换具体模型。
    fn set_model(&mut self, value: impl Into<String>);

    /// 写入动态字段。
    fn put_additional(&mut self, key: impl Into<String>, value: impl Into<Value>);

    /// 设置超时（`Duration::ZERO` 保持原值）。
    fn set_timeout(&mut self, value: Duration);

    /// 设置读取超时。
    fn set_read_timeout(&mut self, value: Duration);

    /// 配置代理。
    /// 注：`BaseConfig` 提供同名 inherent 方法，本 trait 方法作为 Java 镜像保留。
    #[allow(dead_code)]
    fn set_proxy(&mut self, proxy: &str) -> Result<(), url::ParseError>;

    /// 清除代理。
    /// 注：`BaseConfig` 提供同名 inherent 方法，本 trait 方法作为 Java 镜像保留。
    #[allow(dead_code)]
    fn clear_proxy(&mut self);
}
