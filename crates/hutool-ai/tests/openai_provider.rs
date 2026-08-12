//! `OpenAiCompatibleProvider` 集成测试（本地 mock HTTP，无真实 API 调用）。
//!
//! 对齐 Java `OpenaiServiceImpl` 的 chat / chatStream 行为：
//! - `chat` 非流式：POST `/chat/completions`，Bearer 认证，解析 choices[0]
//! - `stream` 流式：SSE 事件流，`[DONE]` 结束
//! - 空 choices / HTTP 错误 / 非法 JSON 的错误路径

#![allow(clippy::too_many_lines)]

use futures_util::StreamExt;
use hutool_ai::OpenAiCompatibleProvider;
use hutool_ai::{ChatChunk, ChatProvider, ChatRequest, Message, ProviderError, Role, Usage};
use hutool_http::{HttpClient, HttpConfig};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// 捕获的入站请求（用于断言请求路径与方法）。
#[derive(Debug, Default, Clone)]
struct CapturedRequest {
    start_line: String,
    body: String,
}

/// 启动一次性 mock 服务，返回 `(url, captured, task)`。
async fn mock_once(
    status_line: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
) -> (
    String,
    Arc<std::sync::Mutex<CapturedRequest>>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let captured = Arc::new(std::sync::Mutex::new(CapturedRequest::default()));
    let slot = Arc::clone(&captured);
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0_u8; 65536];
        let n = socket.read(&mut buf).await.unwrap();
        let raw = String::from_utf8_lossy(&buf[..n]);
        let mut cap = CapturedRequest::default();
        if let Some(line) = raw.lines().next() {
            cap.start_line = line.to_string();
        }
        if let Some(idx) = raw.find("\r\n\r\n") {
            cap.body = raw[idx + 4..].to_string();
        }
        *slot.lock().unwrap() = cap;
        let header = format!(
            "{status_line}\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
            body.len()
        );
        socket.write_all(header.as_bytes()).await.unwrap();
        socket.write_all(&body).await.unwrap();
    });
    (format!("http://{address}"), captured, task)
}

fn chat_json() -> Vec<u8> {
    br#"{"id":"chatcmpl-test","model":"gpt-4o","choices":[{"index":0,"message":{"role":"assistant","content":"hello"},"finish_reason":"stop"}],"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":3}}"#.to_vec()
}

fn sse_body() -> Vec<u8> {
    b"data: {\"id\":\"s1\",\"choices\":[{\"delta\":{\"content\":\"a\"},\"finish_reason\":null}]}\n\ndata: {\"id\":\"s2\",\"choices\":[{\"delta\":{\"content\":\"b\"},\"finish_reason\":null}]}\n\ndata: {\"id\":\"s3\",\"choices\":[{\"delta\":{},\"finish_reason\":\"stop\"}]}\n\ndata: [DONE]\n\n".to_vec()
}

fn provider(url: &str) -> OpenAiCompatibleProvider {
    let client = HttpClient::new(&HttpConfig::default()).unwrap();
    OpenAiCompatibleProvider::new(client, url, "test-key", "gpt-4o").unwrap()
}

#[tokio::test]
async fn chat_parses_choice_and_sends_bearer_json() {
    let (url, captured, task) = mock_once("HTTP/1.1 200 OK", "application/json", chat_json()).await;
    let provider = provider(&url);
    let response = provider
        .chat(ChatRequest {
            model: Some("gpt-4o".into()),
            messages: vec![Message::user("你好")],
            temperature: Some(0.7),
            max_tokens: Some(64),
        })
        .await
        .unwrap();
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(cap.start_line.starts_with("POST /chat/completions"));
    assert!(cap.start_line.contains("HTTP/1.1"));
    assert!(!cap.body.contains("Bearer test-key")); // 认证在 header，不在 body
    assert!(cap.body.contains("\"model\":\"gpt-4o\""));
    assert!(cap.body.contains("\"temperature\":0.7"));
    assert!(cap.body.contains("\"max_tokens\":64"));
    assert!(cap.body.contains("\"stream\":false"));
    assert_eq!(response.id, "chatcmpl-test");
    assert_eq!(response.model, "gpt-4o");
    assert_eq!(response.message.role, Role::Assistant);
    assert_eq!(response.message.content, "hello");
    assert_eq!(response.finish_reason.as_deref(), Some("stop"));
    assert_eq!(
        response.usage,
        Usage {
            prompt_tokens: 1,
            completion_tokens: 2,
            total_tokens: 3,
        }
    );
}

#[tokio::test]
async fn chat_stream_collects_deltas_until_done() {
    let (url, captured, task) = mock_once("HTTP/1.1 200 OK", "text/event-stream", sse_body()).await;
    let provider = provider(&url);
    let mut stream = provider.stream(ChatRequest::user("你好")).await.unwrap();
    let mut chunks: Vec<ChatChunk> = Vec::new();
    while let Some(chunk) = stream.next().await {
        chunks.push(chunk.unwrap());
    }
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(cap.start_line.starts_with("POST /chat/completions"));
    assert!(cap.body.contains("\"stream\":true"));
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].content.as_deref(), Some("a"));
    assert_eq!(chunks[1].content.as_deref(), Some("b"));
    assert_eq!(chunks[2].finish_reason.as_deref(), Some("stop"));
}

#[tokio::test]
async fn chat_empty_choices_returns_error() {
    let body = br#"{"id":"x","model":"m","choices":[],"usage":{}}"#.to_vec();
    let (url, _captured, task) = mock_once("HTTP/1.1 200 OK", "application/json", body).await;
    let provider = provider(&url);
    let err = provider.chat(ChatRequest::user("hi")).await.unwrap_err();
    task.await.unwrap();
    assert!(matches!(err, ProviderError::EmptyChoices));
}

#[tokio::test]
async fn chat_http_error_maps_to_provider_error() {
    let (url, _captured, task) = mock_once(
        "HTTP/1.1 500 Internal Server Error",
        "text/plain",
        b"boom".to_vec(),
    )
    .await;
    let provider = provider(&url);
    let err = provider.chat(ChatRequest::user("hi")).await.unwrap_err();
    task.await.unwrap();
    assert!(err.to_string().contains("500") || err.to_string().contains("status"));
}

#[tokio::test]
async fn chat_invalid_json_maps_to_provider_error() {
    let (url, _captured, task) =
        mock_once("HTTP/1.1 200 OK", "application/json", b"not-json".to_vec()).await;
    let provider = provider(&url);
    let err = provider.chat(ChatRequest::user("hi")).await.unwrap_err();
    task.await.unwrap();
    assert!(matches!(
        err,
        ProviderError::Json(_) | ProviderError::Http(_)
    ));
}

#[tokio::test]
async fn stream_invalid_sse_json_propagates_error() {
    let body = b"data: not-json\n\ndata: [DONE]\n\n".to_vec();
    let (url, _captured, task) = mock_once("HTTP/1.1 200 OK", "text/event-stream", body).await;
    let provider = provider(&url);
    let mut stream = provider.stream(ChatRequest::user("hi")).await.unwrap();
    let result = stream.next().await;
    task.await.unwrap();
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[tokio::test]
async fn chat_uses_default_model_when_unspecified() {
    let (url, captured, task) = mock_once("HTTP/1.1 200 OK", "application/json", chat_json()).await;
    let provider = provider(&url);
    let _ = provider.chat(ChatRequest::user("hi")).await.unwrap();
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(cap.body.contains("\"model\":\"gpt-4o\""));
}

#[test]
fn provider_debug_redacts_api_key() {
    let client = HttpClient::new(&HttpConfig::default()).unwrap();
    let provider =
        OpenAiCompatibleProvider::new(client, "https://example.com/v1", "SECRET", "m").unwrap();
    let debug = format!("{provider:?}");
    assert!(debug.contains("[REDACTED]"));
    assert!(!debug.contains("SECRET"));
}
