//! 通用 `ProviderService` HTTP 实现。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.BaseAIService` / `cn.hutool.ai.core.AIService`
//!
//! 原 `compat/mod.rs` 单源 dump 已拆分，本文件仅承载 "通用 reqwest 实现" 部分。
//! - Java `OpenaiServiceImpl` 等厂商的差异化逻辑放到 `providers/openai.rs`；
//! - Java `BaseAIService.sendGet / sendPost / sendPostStream` 的通用 HTTP 行为放在本文件。

#![allow(clippy::missing_panics_doc)]

use super::ai_service::AIService;
use super::base_config::BaseConfig;
use crate::core::ai_config::AIConfig;
use crate::message::Message;
use crate::operations::{AIResponse, Operation, StreamCallback};
use crate::{ModelName, ProviderError};
use async_trait::async_trait;
use hutool_http::{HttpClient, Method};
use secrecy::ExposeSecret;
use std::fmt;

/// 最大响应体字节数（64 MiB），对齐 Java `BaseAIService` 默认读取上限。
pub const MAX_MEDIA_BYTES: usize = 64 * 1024 * 1024;

/// 通用 provider HTTP 实现。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.BaseAIService` + `cn.hutool.ai.model.*.*ServiceImpl`
#[derive(Debug, Clone)]
pub struct ProviderService {
    config: BaseConfig,
    client: HttpClient,
    max_response_bytes: usize,
}

impl ProviderService {
    /// 构建一个池化、Rustls 后端的 provider 客户端。
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

    /// 通过外部管理的 `HttpClient` 与响应上限创建服务。
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

    /// 访问底层配置。
    #[must_use]
    pub fn config(&self) -> &BaseConfig {
        &self.config
    }

    /// 访问底层 HTTP 客户端。
    #[must_use]
    pub fn client(&self) -> &HttpClient {
        &self.client
    }

    /// 构建针对给定操作的 `reqwest::RequestBuilder`。
    ///
    /// 这是 Java `BaseAIService.sendPost(endpoint, paramJson)` 的 Rust 等价实现。
    pub fn request(&self, operation: &Operation, stream: bool) -> reqwest::RequestBuilder {
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

/// Java `AIService` 接口里的 `chat(List<Message>)` 缺省实现，Rust 端在 trait 中给出。
/// 该函数保留为 Java 镜像占位，暂无调用方。
#[allow(dead_code)]
pub fn default_chat_prompt(prompt: &str) -> Vec<Message> {
    let mut messages = Vec::with_capacity(2);
    messages.push(Message::system("You are a helpful assistant"));
    messages.push(Message::user(prompt));
    messages
}

impl fmt::Display for ProviderService {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderService")
            .field("model_name", &self.config.model_name())
            .field("api_url", &self.config.api_url())
            .field("model", &self.config.model())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ai_service::AIService;
    use hutool_http::HttpConfig;
    use serde_json::json;
    use std::sync::{Arc, Mutex};
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
}
