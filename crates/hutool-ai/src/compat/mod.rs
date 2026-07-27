#![allow(clippy::missing_panics_doc)]

//! Hutool-compatible configuration, factory, and service facade.
/// 对齐: `cn.hutool.ai.AIConfig`
/// AI 兼容层
mod ai_service_factory;
mod ai_util;

pub use self::{ai_service_factory::AIServiceFactory, ai_util::AIUtil};

use crate::{AIResponse, Message, ModelName, Operation, ProviderError, StreamCallback};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Map, Value};
use std::{fmt, sync::Arc, time::Duration};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(180);
const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(300);
const MAX_MEDIA_BYTES: usize = 64 * 1024 * 1024;

/// Shared configuration contract for all Hutool AI providers.
pub trait AIConfig: fmt::Debug + Send + Sync {
    /// Provider name.
    fn model_name(&self) -> ModelName;
    /// Secret provider credential.
    fn api_key(&self) -> &SecretString;
    /// Base API URL.
    fn api_url(&self) -> &Url;
    /// Concrete model identifier.
    fn model(&self) -> &str;
    /// Additional request fields.
    fn additional(&self) -> &Map<String, Value>;
    /// Connection/request timeout.
    fn timeout(&self) -> Duration;
    /// Streaming read timeout.
    fn read_timeout(&self) -> Duration;
    /// Optional HTTP proxy URL.
    fn proxy(&self) -> Option<&Url>;
}

/// Owned provider configuration. Secrets are redacted from debug output.
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
    /// Creates the provider's Hutool-compatible defaults.
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

    /// Creates defaults with a provider credential.
    pub fn with_api_key(
        provider: ModelName,
        api_key: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let mut config = Self::new(provider).expect("built-in provider defaults are valid");
        config.set_api_key(api_key);
        Ok(config)
    }

    /// Replaces the credential.
    pub fn set_api_key(&mut self, api_key: impl Into<String>) -> &mut Self {
        self.api_key = Arc::new(SecretString::from(api_key.into()));
        self
    }
    /// Replaces the API root.
    pub fn set_api_url(&mut self, api_url: impl AsRef<str>) -> Result<&mut Self, ProviderError> {
        self.api_url = Url::parse(api_url.as_ref())?;
        Ok(self)
    }
    /// Replaces the concrete model.
    pub fn set_model(&mut self, model: impl Into<String>) -> &mut Self {
        self.model = model.into();
        self
    }
    /// Adds a provider-specific request field.
    pub fn put_additional(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.additional.insert(key.into(), value.into());
        self
    }
    /// Returns one provider-specific request field.
    #[must_use]
    pub fn get_additional(&self, key: &str) -> Option<&Value> {
        self.additional.get(key)
    }
    /// Replaces the request timeout when non-zero.
    pub fn set_timeout(&mut self, timeout: Duration) -> &mut Self {
        if !timeout.is_zero() {
            self.timeout = timeout;
        }
        self
    }
    /// Replaces the stream read timeout when non-zero.
    pub fn set_read_timeout(&mut self, timeout: Duration) -> &mut Self {
        if !timeout.is_zero() {
            self.read_timeout = timeout;
        }
        self
    }
    /// Configures an HTTP proxy URL.
    pub fn set_proxy(&mut self, proxy: impl AsRef<str>) -> Result<&mut Self, ProviderError> {
        self.proxy = Some(Url::parse(proxy.as_ref())?);
        Ok(self)
    }
    /// Removes the HTTP proxy.
    pub fn clear_proxy(&mut self) -> &mut Self {
        self.proxy = None;
        self
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

/// Chainable configuration builder corresponding to Hutool's builder.
#[derive(Debug, Clone)]
pub struct AIConfigBuilder {
    config: BaseConfig,
}

impl AIConfigBuilder {
    /// Creates a builder from a case-insensitive Hutool provider name.
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
        self.config.set_api_url(value)?;
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
        self.config.set_proxy(value)?;
        Ok(self)
    }
    /// Returns the validated owned configuration.
    #[must_use]
    pub fn build(self) -> BaseConfig {
        self.config
    }
}

/// Common asynchronous AI service contract.
#[async_trait]
pub trait AIService: fmt::Debug + Send + Sync {
    /// Executes one typed provider operation.
    async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError>;
    /// Executes an operation and delivers stream events.
    async fn execute_stream(
        &self,
        operation: Operation,
        callback: StreamCallback,
    ) -> Result<(), ProviderError>;
    /// Convenience chat operation.
    async fn chat(&self, messages: Vec<Message>) -> Result<String, ProviderError> {
        Ok(self
            .execute(Operation::Chat { messages })
            .await?
            .into_text())
    }
}

/// Provider plug-in contract used by custom registries.
pub trait AIServiceProvider: fmt::Debug + Send + Sync {
    /// Provider identifier.
    fn service_name(&self) -> ModelName;
    /// Creates a service from explicit configuration.
    fn create(&self, config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError>;
}

/// Unified production HTTP implementation for Hutool provider capabilities.
#[derive(Debug, Clone)]
pub struct ProviderService {
    config: BaseConfig,
    client: HttpClient,
    max_response_bytes: usize,
}

impl ProviderService {
    /// Builds a pooled, Rustls-backed provider client.
    pub fn new(config: BaseConfig) -> Result<Self, ProviderError> {
        let mut builder = HttpClient::builder()
            .timeout(config.timeout())
            .max_response_size(MAX_MEDIA_BYTES);
        if let Some(proxy) = config.proxy() {
            builder = builder
                .proxy(proxy.as_str())
                .expect("validated proxy URLs are accepted by reqwest");
        }
        let client = builder
            .build()
            .expect("fixed Rustls HTTP client configuration is valid");
        Ok(Self {
            config,
            client,
            max_response_bytes: MAX_MEDIA_BYTES,
        })
    }

    /// Builds a service from an application-managed client and response limit.
    pub fn with_client(
        config: BaseConfig,
        client: HttpClient,
        max_response_bytes: usize,
    ) -> Result<Self, ProviderError> {
        if max_response_bytes == 0 {
            return Err(ProviderError::ResponseTooLarge { limit: 0 });
        }
        Ok(Self {
            config,
            client,
            max_response_bytes,
        })
    }

    fn request(&self, operation: &Operation, stream: bool) -> reqwest::RequestBuilder {
        let endpoint = operation.endpoint(self.config.model_name(), self.config.model());
        let mut url = self.config.api_url().clone();
        let root = url.path().trim_end_matches('/');
        url.set_path(&format!("{root}{endpoint}"));
        if self.config.model_name() == ModelName::Gemini {
            url.query_pairs_mut()
                .append_pair("key", self.config.api_key().expose_secret());
        }
        let method = match operation {
            Operation::ListModels
            | Operation::ListLanguageModels
            | Operation::Balance
            | Operation::GetModel { .. }
            | Operation::GetLanguageModel { .. }
            | Operation::GetVideo { .. }
            | Operation::DeferredCompletion { .. } => Method::GET,
            Operation::DeleteModel { .. } => Method::DELETE,
            _ => Method::POST,
        };
        let payload = operation.payload(self.config.model(), self.config.additional(), stream);
        let mut request = self
            .client
            .request(method, url)
            .header("accept", "application/json");
        if !matches!(
            self.config.model_name(),
            ModelName::Gemini | ModelName::Ollama
        ) {
            request = request.bearer_auth(self.config.api_key().expose_secret());
        }
        request.json(&payload)
    }
}

#[async_trait]
impl AIService for ProviderService {
    async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError> {
        let binary = matches!(operation, Operation::TextToSpeech { .. });
        let response = self.client.send(self.request(&operation, false)).await?;
        let bytes = response
            .bytes()
            .await
            .map_err(hutool_http::HttpError::from)?;
        if bytes.len() > self.max_response_bytes {
            return Err(ProviderError::ResponseTooLarge {
                limit: self.max_response_bytes,
            });
        }
        if binary {
            Ok(AIResponse::Bytes(bytes.to_vec()))
        } else {
            Ok(AIResponse::Json(serde_json::from_slice(&bytes)?))
        }
    }

    async fn execute_stream(
        &self,
        operation: Operation,
        callback: StreamCallback,
    ) -> Result<(), ProviderError> {
        let mut response = self.client.send(self.request(&operation, true)).await?;
        let mut decoder = crate::SseDecoder::new(crate::MAX_SSE_EVENT_BYTES);
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(hutool_http::HttpError::from)?
        {
            for event in decoder.push(&chunk)? {
                if event == b"[DONE]" {
                    return Ok(());
                }
                callback(String::from_utf8_lossy(&event).into_owned());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hutool_http::HttpConfig;
    use serde_json::json;
    use std::sync::Mutex;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    async fn server(
        responses: Vec<(&'static str, Vec<u8>)>,
    ) -> (String, tokio::task::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            for (content_type, body) in responses {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = vec![0; 8192];
                let _ = socket.read(&mut request).await.unwrap();
                let header = format!(
                    "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                    body.len()
                );
                socket.write_all(header.as_bytes()).await.unwrap();
                socket.write_all(&body).await.unwrap();
            }
        });
        (format!("http://{address}"), task)
    }

    #[test]
    fn configuration_builder_covers_validation_mutation_and_redaction() {
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
        config
            .set_api_key("next")
            .set_model("custom")
            .put_additional("temperature", 1);
        config
            .set_timeout(Duration::ZERO)
            .set_read_timeout(Duration::ZERO);
        config
            .set_timeout(Duration::from_secs(2))
            .set_read_timeout(Duration::from_secs(3));
        config.set_api_url("https://example.com/v1").unwrap();
        config.set_proxy("http://proxy.example:8080").unwrap();
        assert_eq!(config.get_additional("temperature"), Some(&json!(1)));
        assert_eq!(config.model(), "custom");
        assert_eq!(config.timeout(), Duration::from_secs(2));
        assert_eq!(config.read_timeout(), Duration::from_secs(3));
        assert!(config.proxy().is_some());
        config.clear_proxy();
        assert!(config.proxy().is_none());
        assert!(config.set_api_url("not a url").is_err());
        assert!(config.set_proxy("not a url").is_err());

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
        assert_eq!(built.get_additional("x"), Some(&json!(true)));
        assert!(AIConfigBuilder::new("missing").is_err());
        assert!(
            AIConfigBuilder::new("openai")
                .unwrap()
                .api_url("bad")
                .is_err()
        );
        assert!(
            AIConfigBuilder::new("openai")
                .unwrap()
                .proxy("bad")
                .is_err()
        );
    }

    #[test]
    fn requests_apply_method_auth_query_and_provider_paths() {
        let openai =
            ProviderService::new(BaseConfig::with_api_key(ModelName::OpenAi, "key").unwrap())
                .unwrap();
        let get = openai
            .request(&Operation::ListModels, false)
            .build()
            .unwrap();
        assert_eq!(get.method(), Method::GET);
        assert_eq!(get.headers()["authorization"], "Bearer key");
        let delete = openai
            .request(&Operation::DeleteModel { name: "m".into() }, false)
            .build()
            .unwrap();
        assert_eq!(delete.method(), Method::DELETE);
        let post = openai
            .request(
                &Operation::Chat {
                    messages: vec![Message::user("hi")],
                },
                true,
            )
            .build()
            .unwrap();
        assert_eq!(post.method(), Method::POST);
        assert!(post.body().is_some());

        let gemini =
            ProviderService::new(BaseConfig::with_api_key(ModelName::Gemini, "g-key").unwrap())
                .unwrap();
        let request = gemini
            .request(&Operation::ListModels, false)
            .build()
            .unwrap();
        assert_eq!(request.url().query(), Some("key=g-key"));
        assert!(!request.headers().contains_key("authorization"));
        let ollama = ProviderService::new(BaseConfig::new(ModelName::Ollama).unwrap()).unwrap();
        assert!(
            !ollama
                .request(&Operation::ListModels, false)
                .build()
                .unwrap()
                .headers()
                .contains_key("authorization")
        );

        let mut proxied = BaseConfig::new(ModelName::OpenAi).unwrap();
        proxied.set_proxy("http://127.0.0.1:8888").unwrap();
        assert!(ProviderService::new(proxied).is_ok());
        assert!(HttpClient::builder().proxy("not a proxy").is_err());
    }

    #[tokio::test]
    async fn service_executes_json_binary_limits_and_sse() {
        let sse = b"data: {\"delta\":\"one\"}\n\ndata: [DONE]\n\n".to_vec();
        let (url, task) = server(vec![
            ("application/json", br#"{"ok":true}"#.to_vec()),
            ("application/octet-stream", b"wav".to_vec()),
            ("application/json", b"this response is too large".to_vec()),
            ("text/event-stream", sse),
        ])
        .await;
        let mut config = BaseConfig::with_api_key(ModelName::OpenAi, "key").unwrap();
        config.set_api_url(&url).unwrap();
        let client = HttpClient::builder()
            .max_response_size(1024)
            .build()
            .unwrap();
        let service = ProviderService::with_client(config, client, 16).unwrap();
        assert_eq!(
            service.execute(Operation::ListModels).await.unwrap(),
            AIResponse::Json(json!({"ok":true}))
        );
        assert_eq!(
            service
                .execute(Operation::TextToSpeech {
                    input: "x".into(),
                    voice: "alloy".into()
                })
                .await
                .unwrap(),
            AIResponse::Bytes(b"wav".to_vec())
        );
        assert_eq!(
            service
                .execute(Operation::ListModels)
                .await
                .unwrap_err()
                .to_string(),
            "provider response exceeds 16 bytes"
        );
        let events = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&events);
        service
            .execute_stream(
                Operation::Chat {
                    messages: vec![Message::user("x")],
                },
                Arc::new(move |event| captured.lock().unwrap().push(event)),
            )
            .await
            .unwrap();
        assert_eq!(events.lock().unwrap().as_slice(), ["{\"delta\":\"one\"}"]);
        assert!(
            ProviderService::with_client(
                BaseConfig::new(ModelName::OpenAi).unwrap(),
                HttpClient::new(&HttpConfig::default()).unwrap(),
                0
            )
            .is_err()
        );
        task.await.unwrap();
    }

    #[derive(Debug)]
    struct FakeService;
    #[async_trait]
    impl AIService for FakeService {
        async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError> {
            assert!(matches!(operation, Operation::Chat { .. }));
            Ok(AIResponse::Json(json!({"answer":42})))
        }
        async fn execute_stream(
            &self,
            _operation: Operation,
            callback: StreamCallback,
        ) -> Result<(), ProviderError> {
            callback("event".into());
            Ok(())
        }
    }

    #[derive(Debug)]
    struct FakeProvider;
    impl AIServiceProvider for FakeProvider {
        fn service_name(&self) -> ModelName {
            ModelName::Hutool
        }
        fn create(&self, _config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
            Ok(Arc::new(FakeService))
        }
    }

    #[tokio::test]
    async fn service_trait_factory_and_util_facades_are_usable() {
        let provider = FakeProvider;
        assert_eq!(provider.service_name(), ModelName::Hutool);
        let fake = provider
            .create(BaseConfig::new(ModelName::Hutool).unwrap())
            .unwrap();
        assert_eq!(
            fake.chat(vec![Message::user("hi")]).await.unwrap(),
            "{\"answer\":42}"
        );
        let events = Arc::new(Mutex::new(Vec::new()));
        let captured = Arc::clone(&events);
        fake.execute_stream(
            Operation::ListModels,
            Arc::new(move |value| captured.lock().unwrap().push(value)),
        )
        .await
        .unwrap();
        assert_eq!(events.lock().unwrap().as_slice(), ["event"]);
        let config = BaseConfig::new(ModelName::OpenAi).unwrap();
        assert!(AIServiceFactory::get_ai_service(config.clone()).is_ok());
        assert!(AIUtil::get_ai_service(config).is_ok());
    }

    #[tokio::test]
    async fn ai_util_chat_conveniences_execute_through_the_factory() {
        let (url, task) = server(vec![
            ("application/json", br#"{"one":1}"#.to_vec()),
            ("application/json", br#"{"two":2}"#.to_vec()),
        ])
        .await;
        let mut first = BaseConfig::with_api_key(ModelName::OpenAi, "key").unwrap();
        first.set_api_url(&url).unwrap();
        assert_eq!(AIUtil::chat(first, "hello").await.unwrap(), "{\"one\":1}");
        let mut second = BaseConfig::with_api_key(ModelName::OpenAi, "key").unwrap();
        second.set_api_url(&url).unwrap();
        assert_eq!(
            AIUtil::chat_messages(second, vec![Message::user("hello")])
                .await
                .unwrap(),
            "{\"two\":2}"
        );
        task.await.unwrap();
    }
}
