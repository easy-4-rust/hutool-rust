//! Provider-neutral chat model APIs and an OpenAI-compatible implementation.
//!
//! 包结构对齐 Java `cn.hutool.ai.*`：
//! - 顶级：`ai_exception / ai_service_factory / ai_util / models / message / base_config`
//! - `core/` 子包：`AIConfig / AIConfigBuilder / AIService / AIServiceProvider / BaseConfig / ProviderService`
//! - `operations/` 子包：`AIResponse / Operation / StreamCallback / VideoParameter`
//! - `providers/` 子包：`openai`（active）+ 其他 6 家厂商 stub

#![forbid(unsafe_code)]
#![allow(async_fn_in_trait)]

mod ai_exception;
mod ai_service_factory;
mod ai_util;
mod base_config;
mod core;
mod message;
mod models;
mod operations;
pub mod prelude;
mod providers;

pub use ai_exception::AIException;
pub use ai_service_factory::{registry as provider_registry, AIServiceFactory, ProviderRegistry};
pub use ai_util::AIUtil;
pub use core::{
    AIConfig, AIConfigBuilder, AIService, AIServiceProvider, BaseConfig, ProviderService,
};
pub use message::{Message, Role};
pub use models::*;
pub use operations::{AIResponse, Operation, StreamCallback, VideoParameter};
pub use providers::OpenAiCompatibleProvider;

use futures_core::Stream;
use hutool_http::{HttpClient, Url};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use thiserror::Error;

/// A provider-neutral chat request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Provider model identifier. `None` selects the provider default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Ordered conversation messages.
    pub messages: Vec<Message>,
    /// Optional sampling temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Optional output-token ceiling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl ChatRequest {
    /// Creates a request with one user message and provider defaults.
    #[must_use]
    pub fn user(content: &str) -> Self {
        Self {
            model: None,
            messages: vec![Message::user(content)],
            temperature: None,
            max_tokens: None,
        }
    }
}

/// Token accounting reported by a provider.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    /// Input/prompt token count.
    #[serde(default)]
    pub prompt_tokens: u64,
    /// Generated token count.
    #[serde(default)]
    pub completion_tokens: u64,
    /// Total token count.
    #[serde(default)]
    pub total_tokens: u64,
}

/// A normalized non-streaming chat response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// Provider request identifier.
    pub id: String,
    /// Model that produced the response.
    pub model: String,
    /// First returned assistant message.
    pub message: Message,
    /// Optional finish reason.
    pub finish_reason: Option<String>,
    /// Provider token accounting.
    pub usage: Usage,
}

/// One incremental streaming delta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatChunk {
    /// Provider request identifier.
    pub id: String,
    /// Incremental text, if any.
    pub content: Option<String>,
    /// Finish reason on the terminal chunk.
    pub finish_reason: Option<String>,
}

/// A boxed provider stream.
pub type ChatStream =
    Pin<Box<dyn Stream<Item = Result<ChatChunk, ProviderError>> + Send + 'static>>;

/// AI provider failures.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// HTTP transport or response decoding failed.
    #[error(transparent)]
    Http(#[from] hutool_http::HttpError),
    /// The base URL is invalid.
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// Provider payload was not valid JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Provider returned no choices.
    #[error("provider returned no chat choices")]
    EmptyChoices,
    /// Streaming is not implemented by this provider.
    #[error("provider does not support streaming")]
    StreamingUnsupported,
    /// One server-sent event exceeded the defensive parser limit.
    #[error("provider stream event exceeds {limit} bytes")]
    StreamEventTooLarge {
        /// Maximum accepted bytes per event.
        limit: usize,
    },
    /// The requested Hutool provider name is unknown.
    #[error("unsupported AI provider: {0}")]
    UnsupportedProvider(String),
    /// A bounded binary or JSON response exceeded the safety limit.
    #[error("provider response exceeds {limit} bytes")]
    ResponseTooLarge {
        /// Maximum accepted response size.
        limit: usize,
    },
}

impl From<ProviderError> for AIException {
    fn from(value: ProviderError) -> Self {
        AIException::Message(value.to_string())
    }
}

/// A provider-neutral asynchronous chat interface.
pub trait ChatProvider: Send + Sync {
    /// Completes one chat request.
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError>;

    /// Starts a streaming completion when supported.
    async fn stream(&self, _request: ChatRequest) -> Result<ChatStream, ProviderError> {
        Err(ProviderError::StreamingUnsupported)
    }
}

/// 防御性 SSE 事件上限（256 KiB）。
pub const MAX_SSE_EVENT_BYTES: usize = 256 * 1024;

/// 复用 SSE 字节流分帧器，同时供 `providers/openai.rs` 与 `lib.rs` 单元测试使用。
#[derive(Debug)]
pub struct SseDecoder {
    pending: Vec<u8>,
    data: Vec<u8>,
    max_event_bytes: usize,
}

impl SseDecoder {
    /// 创建 SSE 解码器，限定单事件最大字节数。
    #[must_use]
    pub fn new(max_event_bytes: usize) -> Self {
        Self {
            pending: Vec::new(),
            data: Vec::new(),
            max_event_bytes,
        }
    }

    /// 推入一段字节并解析出完整事件。
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, ProviderError> {
        self.pending.extend_from_slice(chunk);
        let mut events = Vec::new();
        while let Some(newline) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line: Vec<u8> = self.pending.drain(..=newline).collect();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                if !self.data.is_empty() {
                    self.data.pop();
                    events.push(std::mem::take(&mut self.data));
                }
                continue;
            }
            if let Some(value) = line.strip_prefix(b"data:") {
                let value = value.strip_prefix(b" ").unwrap_or(value);
                let next_len = self.data.len().saturating_add(value.len() + 1);
                if next_len > self.max_event_bytes {
                    return Err(ProviderError::StreamEventTooLarge {
                        limit: self.max_event_bytes,
                    });
                }
                self.data.extend_from_slice(value);
                self.data.push(b'\n');
            }
        }
        if self.pending.len() > self.max_event_bytes {
            return Err(ProviderError::StreamEventTooLarge {
                limit: self.max_event_bytes,
            });
        }
        Ok(events)
    }
}

/// 兼容访问器：旧调用方通过 `OpenAiCompatibleProvider.client/base_url/default_model` 访问。
impl OpenAiCompatibleProvider {
    /// 暴露内部 HTTP 客户端。
    #[must_use]
    pub fn http_client(&self) -> &HttpClient {
        &self.client
    }
    /// 暴露 base URL。
    #[must_use]
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
    /// 暴露默认模型。
    #[must_use]
    pub fn default_model(&self) -> &str {
        &self.default_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_constructor_is_provider_neutral() {
        let request = ChatRequest::user("hello");
        assert_eq!(request.messages, [Message::user("hello")]);
        assert!(request.model.is_none());
        assert_eq!(Message::system("rules").role, Role::System);
        assert_eq!(Message::assistant("answer").role, Role::Assistant);
        let tool = Message::tool("result");
        assert_eq!(
            serde_json::from_str::<Message>(&serde_json::to_string(&tool).unwrap()).unwrap(),
            tool
        );
        for role in [Role::System, Role::User, Role::Assistant, Role::Tool] {
            let encoded = serde_json::to_string(&role).unwrap();
            assert_eq!(serde_json::from_str::<Role>(&encoded).unwrap(), role);
        }
        let mut complete = ChatRequest::user("all fields");
        complete.model = Some("model".into());
        complete.temperature = Some(0.1);
        complete.max_tokens = Some(2);
        let encoded = serde_json::to_string(&complete).unwrap();
        let decoded: ChatRequest = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.model.as_deref(), Some("model"));
        let minimal = serde_json::to_string(&ChatRequest::user("minimal")).unwrap();
        assert!(!minimal.contains("temperature"));
    }

    #[test]
    fn response_types_and_errors_round_trip() {
        let response = ChatResponse {
            id: "id".into(),
            model: "model".into(),
            message: Message::assistant("ok"),
            finish_reason: Some("stop".into()),
            usage: Usage {
                prompt_tokens: 1,
                completion_tokens: 2,
                total_tokens: 3,
            },
        };
        let encoded = serde_json::to_string(&response).unwrap();
        let decoded: ChatResponse = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.id, "id");
        let chunk = ChatChunk {
            id: "id".into(),
            content: None,
            finish_reason: Some("stop".into()),
        };
        assert_eq!(
            serde_json::from_str::<ChatChunk>(&serde_json::to_string(&chunk).unwrap()).unwrap(),
            chunk
        );
        assert_eq!(
            Usage::default(),
            Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0
            }
        );
        for error in [
            ProviderError::EmptyChoices,
            ProviderError::StreamingUnsupported,
            ProviderError::StreamEventTooLarge { limit: 1 },
            ProviderError::UnsupportedProvider("x".into()),
            ProviderError::ResponseTooLarge { limit: 2 },
            ProviderError::Url(Url::parse("bad url").unwrap_err()),
            ProviderError::Json(serde_json::from_str::<serde_json::Value>("{").unwrap_err()),
        ] {
            assert!(!error.to_string().is_empty());
        }
    }

    #[derive(Debug)]
    struct NonStreaming;
    impl ChatProvider for NonStreaming {
        async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
            Ok(ChatResponse {
                id: "local".into(),
                model: "local".into(),
                message: request.messages[0].clone(),
                finish_reason: None,
                usage: Usage::default(),
            })
        }
    }

    #[tokio::test]
    async fn default_provider_stream_is_explicitly_unsupported() {
        assert_eq!(
            NonStreaming.chat(ChatRequest::user("x")).await.unwrap().id,
            "local"
        );
        assert_eq!(
            NonStreaming
                .stream(ChatRequest::user("x"))
                .await
                .err()
                .map(|error| error.to_string()),
            Some("provider does not support streaming".into())
        );
    }

    #[test]
    fn sse_decoder_handles_chunk_boundaries_and_crlf() {
        let mut decoder = SseDecoder::new(1024);
        assert!(decoder.push(b"data: {\"id\":\"1").unwrap().is_empty());
        let events = decoder
            .push(b"\",\"choices\":[]}\r\n\r\ndata: [DONE]\n\n")
            .unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], br#"{"id":"1","choices":[]}"#);
        assert_eq!(events[1], b"[DONE]");
    }

    #[test]
    fn sse_decoder_bounds_unterminated_input() {
        let mut decoder = SseDecoder::new(4);
        assert_eq!(
            decoder.push(b"12345").unwrap_err().to_string(),
            "provider stream event exceeds 4 bytes"
        );
        let mut decoder = SseDecoder::new(4);
        assert_eq!(
            decoder.push(b"data: 12345\n").unwrap_err().to_string(),
            "provider stream event exceeds 4 bytes"
        );
        let mut decoder = SseDecoder::new(20);
        assert!(decoder.push(b"event: ping\n\n").unwrap().is_empty());
    }
}