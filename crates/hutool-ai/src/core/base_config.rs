//! `BaseConfig` 实现。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.BaseConfig`
//!
//! Java 端使用可变 Bean 风格 setter；Rust 端实现 `AIConfig` + `AIConfigMut` 双 trait，
//! 保留旧 `set_api_key / set_api_url / ...` 链式写法。

use super::ai_config::{AIConfig, AIConfigMut};
use crate::{ModelName, ProviderError};
use secrecy::SecretString;
use serde_json::{Map, Value};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// 默认连接超时（毫秒）。对齐 Java `BaseConfig.timeout = 180_000`。
pub const DEFAULT_TIMEOUT: Duration = Duration::from_mins(3);

/// 默认读取超时（毫秒）。对齐 Java `BaseConfig.readTimeout = 300_000`。
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_mins(5);

/// 通用 `BaseConfig` 实现。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.BaseConfig`
#[derive(Clone)]
pub struct BaseConfig {
    provider: ModelName,
    api_key: Arc<SecretString>,
    api_url: Url,
    model: String,
    additional: Map<String, Value>,
    timeout: Duration,
    read_timeout: Duration,
    proxy: Option<Url>,
}

impl fmt::Debug for BaseConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BaseConfig")
            .field("provider", &self.provider)
            .field("api_key", &"[REDACTED]")
            .field("api_url", &self.api_url)
            .field("model", &self.model)
            .field("additional", &self.additional)
            .field("timeout", &self.timeout)
            .field("read_timeout", &self.read_timeout)
            .field("proxy", &self.proxy)
            .finish()
    }
}

impl BaseConfig {
    /// 创建内置 provider 默认配置。
    pub fn new(provider: ModelName) -> Result<Self, ProviderError> {
        let (url, model) = provider.defaults();
        Ok(Self {
            provider,
            api_key: Arc::new(SecretString::from(String::new())),
            api_url: Url::parse(url).expect("built-in provider URL constants are valid"),
            model: model.into(),
            additional: Map::new(),
            timeout: DEFAULT_TIMEOUT,
            read_timeout: DEFAULT_READ_TIMEOUT,
            proxy: None,
        })
    }

    /// 创建带密钥的内置配置。
    pub fn with_api_key(
        provider: ModelName,
        api_key: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let mut config = Self::new(provider).expect("built-in provider defaults are valid");
        config.set_api_key(api_key);
        Ok(config)
    }
}

impl AIConfig for BaseConfig {
    fn model_name(&self) -> ModelName {
        self.provider
    }
    fn api_key(&self) -> &SecretString {
        &self.api_key
    }
    fn api_url(&self) -> &Url {
        &self.api_url
    }
    fn model(&self) -> &str {
        &self.model
    }
    fn additional(&self) -> &Map<String, Value> {
        &self.additional
    }
    fn timeout(&self) -> Duration {
        self.timeout
    }
    fn read_timeout(&self) -> Duration {
        self.read_timeout
    }
    fn proxy(&self) -> Option<&Url> {
        self.proxy.as_ref()
    }
}

// 为了保留旧式 `set_api_url(...) / set_proxy(...)` / `get_additional(...)` 调用点
// 在未导入 `AIConfigMut` 时也能编译通过，额外提供 inherent 方法包装。
impl BaseConfig {
    /// 读取一个附加字段。Java `getAdditionalConfigByKey` 等价。
    #[must_use]
    pub fn get_additional(&self, key: &str) -> Option<&Value> {
        self.additional.get(key)
    }

    /// 替换 API URL，便于未导入 `AIConfigMut` 时直接调用。
    pub fn set_api_url(&mut self, value: &str) -> Result<(), url::ParseError> {
        self.api_url = Url::parse(value)?;
        Ok(())
    }

    /// 替换代理，便于未导入 `AIConfigMut` 时直接调用。
    pub fn set_proxy(&mut self, value: &str) -> Result<(), url::ParseError> {
        self.proxy = Some(Url::parse(value)?);
        Ok(())
    }
}

impl AIConfigMut for BaseConfig {
    fn set_api_key(&mut self, api_key: impl Into<String>) {
        self.api_key = Arc::new(SecretString::from(api_key.into()));
    }

    fn set_api_url(&mut self, api_url: &str) -> Result<(), url::ParseError> {
        BaseConfig::set_api_url(self, api_url)
    }

    fn set_model(&mut self, model: impl Into<String>) {
        self.model = model.into();
    }

    fn put_additional(&mut self, key: impl Into<String>, value: impl Into<Value>) {
        self.additional.insert(key.into(), value.into());
    }

    fn set_timeout(&mut self, timeout: Duration) {
        if !timeout.is_zero() {
            self.timeout = timeout;
        }
    }

    fn set_read_timeout(&mut self, read_timeout: Duration) {
        if !read_timeout.is_zero() {
            self.read_timeout = read_timeout;
        }
    }

    fn set_proxy(&mut self, proxy: &str) -> Result<(), url::ParseError> {
        BaseConfig::set_proxy(self, proxy)
    }

    fn clear_proxy(&mut self) {
        self.proxy = None;
    }
}

// 保留旧式链式 API：Java Hutool 风格 setter。
impl BaseConfig {
    /// 链式 setter：API key。
    pub fn with_api_key_mut(mut self, api_key: impl Into<String>) -> Self {
        self.set_api_key(api_key);
        self
    }

    /// 链式 setter：API URL。
    pub fn with_api_url(mut self, api_url: &str) -> Result<Self, ProviderError> {
        self.set_api_url(api_url)?;
        Ok(self)
    }

    /// 链式 setter：模型。
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.set_model(model);
        self
    }

    /// 链式 setter：附加字段。
    pub fn with_additional(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.put_additional(key, value);
        self
    }

    /// 链式 setter：超时。
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.set_timeout(timeout);
        self
    }

    /// 链式 setter：读取超时。
    pub fn with_read_timeout(mut self, read_timeout: Duration) -> Self {
        self.set_read_timeout(read_timeout);
        self
    }

    /// 链式 setter：代理。
    pub fn with_proxy(mut self, proxy: &str) -> Result<Self, ProviderError> {
        self.set_proxy(proxy)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;

    #[test]
    fn built_in_defaults_redact_secret_and_allow_mutation() {
        use crate::core::ai_config::AIConfigMut;
        let mut config = BaseConfig::with_api_key(ModelName::OpenAi, "secret").unwrap();
        assert!(!format!("{config:?}").contains("secret"));
        assert_eq!(config.model_name(), ModelName::OpenAi);
        assert_eq!(config.api_key().expose_secret(), "secret");
        assert!(config.api_url().as_str().contains("openai"));
        assert_eq!(config.model(), "gpt-4o");
        assert!(config.additional().is_empty());
        assert_eq!(config.timeout(), DEFAULT_TIMEOUT);
        assert_eq!(config.read_timeout(), DEFAULT_READ_TIMEOUT);
        assert!(config.proxy().is_none());

        config.set_api_key("next");
        config.set_model("custom");
        config.put_additional("temperature", 1);
        config.set_timeout(Duration::ZERO);
        config.set_read_timeout(Duration::ZERO);
        config.set_timeout(Duration::from_secs(2));
        config.set_read_timeout(Duration::from_secs(3));
        config.set_api_url("https://example.com/v1").unwrap();
        config.set_proxy("http://proxy.example:8080").unwrap();
        assert_eq!(config.model(), "custom");
        assert_eq!(config.timeout(), Duration::from_secs(2));
        assert_eq!(config.read_timeout(), Duration::from_secs(3));
        assert!(config.proxy().is_some());
        config.clear_proxy();
        assert!(config.proxy().is_none());
        assert!(config.set_api_url("not a url").is_err());
        assert!(config.set_proxy("not a url").is_err());

        let chain = BaseConfig::with_api_key(ModelName::OpenAi, "x")
            .unwrap()
            .with_model("m")
            .with_timeout(Duration::from_secs(7));
        assert_eq!(chain.model(), "m");
        assert_eq!(chain.timeout(), Duration::from_secs(7));
    }

    #[test]
    fn trait_mutators_and_full_chain_cover_all_setters() {
        // 通过 AIConfigMut trait 方法调用各 setter（覆盖 trait 委托路径）
        let mut config = BaseConfig::with_api_key(ModelName::OpenAi, "k").unwrap();
        AIConfigMut::set_api_url(&mut config, "https://example.com/v1").unwrap();
        AIConfigMut::set_model(&mut config, "trait-model");
        AIConfigMut::set_proxy(&mut config, "http://proxy.example:3128").unwrap();
        assert_eq!(config.api_url().as_str(), "https://example.com/v1");
        assert_eq!(config.model(), "trait-model");
        assert!(config.proxy().is_some());
        AIConfigMut::clear_proxy(&mut config);
        assert!(config.proxy().is_none());

        // 链式全量：with_api_url / with_additional / with_read_timeout / with_proxy
        let chained = BaseConfig::with_api_key(ModelName::OpenAi, "k")
            .unwrap()
            .with_api_url("https://example.com/v1")
            .unwrap()
            .with_model("chain-model")
            .with_additional("temperature", 0.5)
            .with_timeout(Duration::from_secs(9))
            .with_read_timeout(Duration::from_secs(10))
            .with_proxy("http://proxy.example:8080")
            .unwrap();
        assert_eq!(chained.model(), "chain-model");
        assert_eq!(chained.timeout(), Duration::from_secs(9));
        assert_eq!(chained.read_timeout(), Duration::from_secs(10));
        assert!(chained.proxy().is_some());
        assert_eq!(chained.additional().get("temperature").unwrap(), 0.5);
        // 非法 URL 在链式方法中同样报错
        assert!(
            BaseConfig::with_api_key(ModelName::OpenAi, "k")
                .unwrap()
                .with_proxy("not a url")
                .is_err()
        );
    }
}
