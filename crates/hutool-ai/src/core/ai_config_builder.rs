//! `AIConfigBuilder` 链式 builder。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.AIConfigBuilder`
//!
//! Java 端通过 `AIConfigRegistry` 反射获取厂商配置类；Rust 端改为接受字符串厂商名，
//! 并使用 `BaseConfig` 兜底实现。返回的 `BaseConfig` 直接满足 `AIConfig`。

use super::ai_config::AIConfigMut;
use super::base_config::BaseConfig;
use crate::{ModelName, ProviderError};
use serde_json::Value;
use std::time::Duration;

/// 链式 builder。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.AIConfigBuilder`
#[derive(Debug, Clone)]
pub struct AIConfigBuilder {
    config: BaseConfig,
}

impl AIConfigBuilder {
    /// 从厂商名（不区分大小写）创建 builder。
    pub fn new(model_name: &str) -> Result<Self, ProviderError> {
        let provider = ModelName::parse(model_name)
            .ok_or_else(|| ProviderError::UnsupportedProvider(model_name.into()))?;
        Ok(Self {
            config: BaseConfig::new(provider).expect("built-in provider defaults are valid"),
        })
    }

    /// Sets the API key.
    #[must_use]
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.config.set_api_key(value);
        self
    }

    /// Sets the API URL.
    pub fn api_url(mut self, value: impl AsRef<str>) -> Result<Self, ProviderError> {
        self.config
            .set_api_url(value.as_ref())
            .map_err(|_| ProviderError::UnsupportedProvider(value.as_ref().into()))?;
        Ok(self)
    }

    /// Sets the model.
    #[must_use]
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.config.set_model(value);
        self
    }

    /// Adds a dynamic request field.
    #[must_use]
    pub fn additional(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.config.put_additional(key, value);
        self
    }

    /// Sets the request timeout.
    #[must_use]
    pub fn timeout(mut self, value: Duration) -> Self {
        self.config.set_timeout(value);
        self
    }

    /// Sets the read timeout.
    #[must_use]
    pub fn read_timeout(mut self, value: Duration) -> Self {
        self.config.set_read_timeout(value);
        self
    }

    /// Sets an HTTP proxy.
    pub fn proxy(mut self, value: impl AsRef<str>) -> Result<Self, ProviderError> {
        self.config
            .set_proxy(value.as_ref())
            .map_err(|_| ProviderError::UnsupportedProvider(value.as_ref().into()))?;
        Ok(self)
    }

    /// Returns the validated owned configuration.
    #[must_use]
    pub fn build(self) -> BaseConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ai_config::{AIConfig, AIConfigMut};

    #[test]
    fn builder_validates_unsupported_provider_and_chains_fields() {
        assert!(AIConfigBuilder::new("missing").is_err());
        let built = AIConfigBuilder::new("DEEPSEEK")
            .unwrap()
            .api_key("key")
            .model("reasoner")
            .additional("x", true)
            .timeout(Duration::from_secs(4))
            .read_timeout(Duration::from_secs(5))
            .api_url("https://example.com")
            .unwrap()
            .proxy("http://proxy.example:8080")
            .unwrap()
            .build();
        assert_eq!(built.model_name(), ModelName::DeepSeek);
        assert_eq!(built.model(), "reasoner");
        assert!(built.get_additional("x").is_some());
        assert!(AIConfigBuilder::new("openai")
            .unwrap()
            .api_url("bad")
            .is_err());
        assert!(AIConfigBuilder::new("openai")
            .unwrap()
            .proxy("bad")
            .is_err());
    }
}