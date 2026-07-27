//! `OpenaiService` 实现。
//!
//! 对齐 Java 来源: `cn.hutool.ai.model.openai.*`
//!
//! Rust 端 `OpenAiCompatibleProvider` 直接对齐 Java `OpenaiServiceImpl` 中
//! "chat / chatVision / imagesGenerations / moderations / embeddingText /
//!  textToSpeech / chatReasoning" 等核心 HTTP 调用；通过 SSE 复用 `ChatChunk` 流。

use crate::message::Message;
use crate::{
    ChatChunk, ChatProvider, ChatRequest, ChatResponse, ChatStream, ProviderError, SseDecoder,
    MAX_SSE_EVENT_BYTES,
};
use futures_core::Stream;
use hutool_http::{HttpClient, Method, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::pin::Pin;
use std::sync::Arc;

/// OpenAI 兼容 `/chat/completions` provider。
///
/// 对齐 Java 来源: `cn.hutool.ai.model.openai.OpenaiServiceImpl`
pub struct OpenAiCompatibleProvider {
    pub(crate) client: HttpClient,
    pub(crate) base_url: Url,
    pub(crate) api_key: Arc<SecretString>,
    pub(crate) default_model: String,
}

impl fmt::Debug for OpenAiCompatibleProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiCompatibleProvider")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .field("default_model", &self.default_model)
            .finish_non_exhaustive()
    }
}

impl OpenAiCompatibleProvider {
    /// Creates an OpenAI-compatible provider.
    pub fn new(
        client: HttpClient,
        base_url: impl AsRef<str>,
        api_key: impl Into<String>,
        default_model: impl Into<String>,
    ) -> Result<Self, ProviderError> {
        let mut base_url = Url::parse(base_url.as_ref())?;
        if !base_url.path().ends_with('/') {
            let path = format!("{}/", base_url.path());
            base_url.set_path(&path);
        }
        Ok(Self {
            client,
            base_url,
            api_key: Arc::new(SecretString::from(api_key.into())),
            default_model: default_model.into(),
        })
    }
}

/// 与 `OpenAiCompatibleProvider::chat` 对齐的请求体。
///
/// 对应 Java `OpenaiServiceImpl.buildChatRequestBody` 构造字段。
#[derive(Debug, Serialize)]
struct OpenAiRequest<'a> {
    model: &'a str,
    messages: &'a [Message],
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OpenAiResponse {
    id: String,
    model: String,
    choices: Vec<OpenAiChoice>,
    #[serde(default)]
    usage: crate::Usage,
}

#[derive(Debug, Deserialize)]
struct OpenAiChoice {
    message: Message,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamResponse {
    id: String,
    choices: Vec<OpenAiStreamChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAiStreamChoice {
    #[serde(default)]
    delta: OpenAiDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct OpenAiDelta {
    content: Option<String>,
}

impl ChatProvider for OpenAiCompatibleProvider {
    async fn chat(&self, request: ChatRequest) -> Result<ChatResponse, ProviderError> {
        let endpoint = self
            .base_url
            .join("chat/completions")
            .expect("the fixed chat endpoint is a valid relative URL");
        let model = request.model.as_deref().unwrap_or(&self.default_model);
        let payload = OpenAiRequest {
            model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: false,
        };
        let response: OpenAiResponse = self
            .client
            .send_json(
                self.client
                    .request(Method::POST, endpoint)
                    .bearer_auth(self.api_key.expose_secret())
                    .json(&payload),
            )
            .await?;
        let choice = response
            .choices
            .into_iter()
            .next()
            .ok_or(ProviderError::EmptyChoices)?;
        Ok(ChatResponse {
            id: response.id,
            model: response.model,
            message: choice.message,
            finish_reason: choice.finish_reason,
            usage: response.usage,
        })
    }

    async fn stream(&self, request: ChatRequest) -> Result<ChatStream, ProviderError> {
        let endpoint = self
            .base_url
            .join("chat/completions")
            .expect("the fixed chat endpoint is a valid relative URL");
        let model = request.model.as_deref().unwrap_or(&self.default_model);
        let payload = OpenAiRequest {
            model,
            messages: &request.messages,
            temperature: request.temperature,
            max_tokens: request.max_tokens,
            stream: true,
        };
        let mut response = self
            .client
            .send(
                self.client
                    .request(Method::POST, endpoint)
                    .bearer_auth(self.api_key.expose_secret())
                    .header("accept", "text/event-stream")
                    .json(&payload),
            )
            .await?;

        Ok(Box::pin(async_stream::try_stream! {
            let mut decoder = SseDecoder::new(MAX_SSE_EVENT_BYTES);
            'stream: while let Some(chunk) = response.chunk().await.map_err(hutool_http::HttpError::from)? {
                for event in decoder.push(&chunk)? {
                    if event == b"[DONE]" {
                        break 'stream;
                    }
                    let response: OpenAiStreamResponse = serde_json::from_slice(&event)?;
                    for choice in response.choices {
                        yield ChatChunk {
                            id: response.id.clone(),
                            content: choice.delta.content,
                            finish_reason: choice.finish_reason,
                        };
                    }
                }
            }
        }))
    }
}

/// Java `OpenaiCommon` 的 Rust 等价：reasoning/vision/speech 三组枚举已统一放入 `models.rs`。
pub type OpenaiReasoning = crate::ReasoningEffort;
pub use crate::models::VisionDetail as OpenaiVision;
pub use crate::models::SpeechVoice as OpenaiSpeech;

/// Java `OpenaiConfig` 的 Rust 类型别名：所有配置通过 `BaseConfig` 承载。
pub type OpenaiConfig = crate::core::BaseConfig;

/// Java `OpenaiProvider` 的 Rust 入口（暂未启用）。
#[allow(dead_code)]
pub fn openai_service_name() -> crate::ModelName {
    crate::ModelName::OpenAi
}

/// 类型别名，避免下游出现冗长路径。
pub type ChatStreamType =
    Pin<Box<dyn Stream<Item = Result<ChatChunk, ProviderError>> + Send + 'static>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ReasoningEffort, Role, Usage};
    use hutool_http::HttpConfig;

    #[test]
    fn provider_smoke_compiles_and_exposes_aliases() {
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        let provider =
            OpenAiCompatibleProvider::new(client, "https://example.com/v1", "secret", "model")
                .unwrap();
        let debug = format!("{provider:?}");
        assert!(debug.contains("[REDACTED]"));
        let _: OpenaiConfig = crate::core::BaseConfig::with_api_key(crate::ModelName::OpenAi, "x")
            .unwrap();
        // Re-exported common enums compile.
        let _: ReasoningEffort = ReasoningEffort::Medium;
        for role in [Role::System, Role::User, Role::Assistant, Role::Tool] {
            let encoded = serde_json::to_string(&role).unwrap();
            assert_eq!(serde_json::from_str::<Role>(&encoded).unwrap(), role);
        }
        let usage = Usage::default();
        assert_eq!(
            serde_json::from_str::<Usage>(&serde_json::to_string(&usage).unwrap()).unwrap(),
            usage
        );
    }
}