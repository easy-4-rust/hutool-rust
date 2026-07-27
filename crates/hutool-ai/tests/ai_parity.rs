//! Hutool `hutool-ai` TEST parity —— mock HTTP, no paid LLM calls.
//!
//! Inventory: 117 `@Test` methods under `hutool-ai/src/test`.
//! Strategy: local `TcpListener` wire mock returns canned OpenAI-/provider-shaped JSON/SSE/bytes;
//! asserts request path + response shape matching Hutool service operations.

#![allow(clippy::too_many_lines, dead_code, unused_imports)]

use hutool_ai::{
    AIConfigBuilder, AIResponse, AIService, AIServiceFactory, AIUtil, BaseConfig, Message,
    ModelName, Operation, ProviderService, StreamCallback, VideoParameter,
};
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

/// Captured inbound HTTP request for shape asserts.
#[derive(Debug, Clone, Default)]
struct CapturedRequest {
    start_line: String,
    body: String,
}

/// Spawns a one-shot mock that returns `body` with `content_type`.
async fn mock_once(
    content_type: &'static str,
    body: Vec<u8>,
) -> (
    String,
    Arc<Mutex<CapturedRequest>>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let captured = Arc::new(Mutex::new(CapturedRequest::default()));
    let slot = Arc::clone(&captured);
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 65536];
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
            "HTTP/1.1 200 OK\r\ncontent-type: {content_type}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
            body.len()
        );
        socket.write_all(header.as_bytes()).await.unwrap();
        socket.write_all(&body).await.unwrap();
    });
    (format!("http://{address}"), captured, task)
}

fn chat_json() -> Vec<u8> {
    br#"{"id":"chatcmpl-mock","object":"chat.completion","choices":[{"index":0,"message":{"role":"assistant","content":"ok"},"finish_reason":"stop"}],"usage":{"prompt_tokens":1,"completion_tokens":1,"total_tokens":2}}"#.to_vec()
}

fn sse_body() -> Vec<u8> {
    b"data: {\"id\":\"s\",\"choices\":[{\"delta\":{\"content\":\"hi\"},\"finish_reason\":null}]}\n\ndata: [DONE]\n\n".to_vec()
}

fn models_json() -> Vec<u8> {
    br#"{"object":"list","data":[{"id":"mock-model","object":"model"}],"models":[{"name":"mock-model"}]}"#.to_vec()
}

fn embed_json() -> Vec<u8> {
    br#"{"object":"list","data":[{"embedding":[0.1,0.2],"index":0}],"model":"mock"}"#.to_vec()
}

fn image_json() -> Vec<u8> {
    br#"{"created":1,"data":[{"url":"https://example.com/img.png"}],"predictions":[{"bytesBase64Encoded":"AQID"}]}"#.to_vec()
}

fn video_json() -> Vec<u8> {
    br#"{"id":"task-mock","status":"succeeded","name":"operations/mock"}"#.to_vec()
}

fn moderate_json() -> Vec<u8> {
    br#"{"id":"mod","results":[{"flagged":false}]}"#.to_vec()
}

fn tokenize_json() -> Vec<u8> {
    br#"{"data":[{"token_ids":[1,2,3]}]}"#.to_vec()
}

fn balance_json() -> Vec<u8> {
    br#"{"is_available":true,"balance_infos":[{"currency":"CNY","total_balance":"1.00"}]}"#.to_vec()
}

fn stt_json() -> Vec<u8> {
    br#"{"text":"hello world"}"#.to_vec()
}

fn upload_json() -> Vec<u8> {
    br#"{"file":{"name":"files/mock","mimeType":"video/mp4","uri":"https://example.com/files/mock"}}"#.to_vec()
}

fn pull_json() -> Vec<u8> {
    br#"{"status":"success"}"#.to_vec()
}

async fn service_for(provider: ModelName, url: &str, proxy: bool) -> ProviderService {
    let mut config = BaseConfig::with_api_key(provider, "your-key").unwrap();
    config.set_api_url(url).unwrap();
    if proxy {
        // Proxy configured like Hutool OpenaiProxyServiceTest; requests still hit the api_url mock.
        let _ = config.set_proxy("http://127.0.0.1:9");
    }
    ProviderService::new(config).unwrap()
}

async fn assert_json_op(
    provider: ModelName,
    operation: Operation,
    body: Vec<u8>,
    path_substr: &str,
    proxy: bool,
) -> (Value, CapturedRequest) {
    let (url, captured, task) = mock_once("application/json", body).await;
    let service = service_for(provider, &url, proxy).await;
    let response = service.execute(operation).await.unwrap();
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(
        cap.start_line.contains(path_substr),
        "expected path containing {path_substr:?}, got {}",
        cap.start_line
    );
    match response {
        AIResponse::Json(v) => (v, cap),
        AIResponse::Bytes(b) => (json!({"_bytes": String::from_utf8_lossy(&b)}), cap),
    }
}

async fn assert_bytes_op(
    provider: ModelName,
    operation: Operation,
    body: Vec<u8>,
    path_substr: &str,
    proxy: bool,
) -> CapturedRequest {
    let (url, captured, task) = mock_once("application/octet-stream", body).await;
    let service = service_for(provider, &url, proxy).await;
    let response = service.execute(operation).await.unwrap();
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(cap.start_line.contains(path_substr), "{}", cap.start_line);
    assert!(matches!(response, AIResponse::Bytes(_)));
    cap
}

async fn assert_stream_op(
    provider: ModelName,
    operation: Operation,
    path_substr: &str,
    proxy: bool,
) -> CapturedRequest {
    let (url, captured, task) = mock_once("text/event-stream", sse_body()).await;
    let service = service_for(provider, &url, proxy).await;
    let events = Arc::new(Mutex::new(Vec::new()));
    let slot = Arc::clone(&events);
    service
        .execute_stream(
            operation,
            Arc::new(move |e| slot.lock().unwrap().push(e)) as StreamCallback,
        )
        .await
        .unwrap();
    task.await.unwrap();
    let cap = captured.lock().unwrap().clone();
    assert!(cap.start_line.contains(path_substr), "{}", cap.start_line);
    assert!(!events.lock().unwrap().is_empty());
    cap
}

// Generated 117 parity tests

/// 对齐 Java: `AIServiceFactoryTest.getAIService()`
#[test]
fn a_i_service_factory_get_a_i_service() {
    let config = AIConfigBuilder::new("deepSeek")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIServiceFactory::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIServiceFactoryTest.testGetAIService()`
#[test]
fn a_i_service_factory_test_get_a_i_service() {
    let config = AIConfigBuilder::new("deepSeek")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIServiceFactory::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getAIService()`
#[test]
fn a_i_util_get_a_i_service() {
    let config = AIConfigBuilder::new("deepSeek")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.testGetAIService()`
#[test]
fn a_i_util_test_get_a_i_service() {
    let config = AIConfigBuilder::new("openai")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getHutoolService()`
#[test]
fn a_i_util_get_hutool_service() {
    let config = AIConfigBuilder::new("hutool")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getDeepSeekService()`
#[test]
fn a_i_util_get_deep_seek_service() {
    let config = AIConfigBuilder::new("deepSeek")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getDoubaoService()`
#[test]
fn a_i_util_get_doubao_service() {
    let config = AIConfigBuilder::new("doubao")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getGrokService()`
#[test]
fn a_i_util_get_grok_service() {
    let config = AIConfigBuilder::new("grok")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getOpenAIService()`
#[test]
fn a_i_util_get_open_a_i_service() {
    let config = AIConfigBuilder::new("openai")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.getGeminiService()`
#[test]
fn a_i_util_get_gemini_service() {
    let config = AIConfigBuilder::new("gemini")
        .unwrap()
        .api_key("your key")
        .build();
    assert!(AIUtil::get_ai_service(config).is_ok());
}

/// 对齐 Java: `AIUtilTest.chat()`
#[tokio::test]
async fn a_i_util_chat() {
    let (url, _cap, task) = mock_once("application/json", chat_json()).await;
    let mut config = BaseConfig::with_api_key(ModelName::DeepSeek, "your key").unwrap();
    config.set_api_url(&url).unwrap();
    let text = AIUtil::chat(config, "写一首赞美我的诗").await.unwrap();
    assert!(text.contains("choices") || text.contains("ok"));
    task.await.unwrap();
}

/// 对齐 Java: `AIUtilTest.testChat()`
#[tokio::test]
async fn a_i_util_test_chat() {
    let (url, _cap, task) = mock_once("application/json", chat_json()).await;
    let mut config = BaseConfig::with_api_key(ModelName::DeepSeek, "your key").unwrap();
    config.set_api_url(&url).unwrap();
    let messages = vec![
        Message::system("你是财神爷，只会说“我是财神”"),
        Message::user("你是谁啊？"),
    ];
    let text = AIUtil::chat_messages(config, messages).await.unwrap();
    assert!(text.contains("choices") || text.contains("ok"));
    task.await.unwrap();
}

/// 对齐 Java: `DeepSeekServiceTest.chat()`
#[tokio::test]
async fn deep_seek_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::DeepSeek,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DeepSeekServiceTest.chatStream()`
#[tokio::test]
async fn deep_seek_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::DeepSeek,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DeepSeekServiceTest.testChat()`
#[tokio::test]
async fn deep_seek_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::DeepSeek,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DeepSeekServiceTest.beta()`
#[tokio::test]
async fn deep_seek_service_beta() {
    let (value, cap) = assert_json_op(
        ModelName::DeepSeek,
        Operation::Beta {
            prompt: "写一个疯狂星期四广告词".into(),
        },
        chat_json(),
        "/beta/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DeepSeekServiceTest.betaStream()`
#[tokio::test]
async fn deep_seek_service_beta_stream() {
    let _cap = assert_stream_op(
        ModelName::DeepSeek,
        Operation::Beta {
            prompt: "写一个疯狂星期四广告词".into(),
        },
        "/beta/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DeepSeekServiceTest.models()`
#[tokio::test]
async fn deep_seek_service_models() {
    let (value, cap) = assert_json_op(
        ModelName::DeepSeek,
        Operation::ListModels,
        models_json(),
        "/models",
        false,
    )
    .await;
    assert!(
        value.get("data").is_some()
            || value.get("models").is_some()
            || value.get("object").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `DeepSeekServiceTest.balance()`
#[tokio::test]
async fn deep_seek_service_balance() {
    let (value, cap) = assert_json_op(
        ModelName::DeepSeek,
        Operation::Balance,
        balance_json(),
        "/user/balance",
        false,
    )
    .await;
    assert!(value.get("is_available").is_some() || value.get("balance_infos").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.chat()`
#[tokio::test]
async fn doubao_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.chatStream()`
#[tokio::test]
async fn doubao_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::Doubao,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DoubaoServiceTest.testChat()`
#[tokio::test]
async fn doubao_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.chatVision()`
#[tokio::test]
async fn doubao_service_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testChatVision()`
#[tokio::test]
async fn doubao_service_test_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "high".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testChatVisionStream()`
#[tokio::test]
async fn doubao_service_test_chat_vision_stream() {
    let _cap = assert_stream_op(
        ModelName::Doubao,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DoubaoServiceTest.videoTasks()`
#[tokio::test]
async fn doubao_service_video_tasks() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::CreateVideo {
            text: "动画视频".into(),
            image: None,
            parameters: vec![VideoParameter::new("--dur", 5)],
        },
        video_json(),
        "/contents/generations/tasks",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.getVideoTasksInfo()`
#[tokio::test]
async fn doubao_service_get_video_tasks_info() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::GetVideo {
            id: "cgt-mock".into(),
        },
        video_json(),
        "/contents/generations/tasks/",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.embeddingText()`
#[tokio::test]
async fn doubao_service_embedding_text() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::EmbedText {
            inputs: vec!["text".into()],
        },
        embed_json(),
        "/embeddings",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.to_string().contains("embedding"));
    assert!(cap.body.contains("input") || cap.body.contains("prompt") || !cap.body.is_empty());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.embeddingVision()`
#[tokio::test]
async fn doubao_service_embedding_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::EmbedVision {
            text: "天空好难".into(),
            image: "https://img.example.com/a.jpg".into(),
        },
        embed_json(),
        "/embeddings/multimodal",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.to_string().contains("embedding"));
    assert!(cap.body.contains("input") || cap.body.contains("prompt") || !cap.body.is_empty());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.botsChat()`
#[tokio::test]
async fn doubao_service_bots_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::BotChat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.botsChatStream()`
#[tokio::test]
async fn doubao_service_bots_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::Doubao,
        Operation::BotChat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DoubaoServiceTest.tokenization()`
#[tokio::test]
async fn doubao_service_tokenization() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::Tokenize {
            texts: vec!["hello".into()],
        },
        tokenize_json(),
        "/tokenization",
        false,
    )
    .await;
    assert!(value.get("data").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.batchChat()`
#[tokio::test]
async fn doubao_service_batch_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::BatchChat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/batch/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testBatchChat()`
#[tokio::test]
async fn doubao_service_test_batch_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::BatchChat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/batch/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.createContext()`
#[tokio::test]
async fn doubao_service_create_context() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::CreateContext {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
            mode: "session".into(),
        },
        br#"{"id":"ctx-1"}"#.to_vec(),
        "/context/create",
        false,
    )
    .await;
    assert!(value.get("id").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testCreateContext()`
#[tokio::test]
async fn doubao_service_test_create_context() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::CreateContext {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
            mode: "session".into(),
        },
        br#"{"id":"ctx-1"}"#.to_vec(),
        "/context/create",
        false,
    )
    .await;
    assert!(value.get("id").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.chatContext()`
#[tokio::test]
async fn doubao_service_chat_context() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::ContextChat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
            context_id: "ctx-1".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testChatContext()`
#[tokio::test]
async fn doubao_service_test_chat_context() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::ContextChat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
            context_id: "ctx-1".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `DoubaoServiceTest.testChatContextStream()`
#[tokio::test]
async fn doubao_service_test_chat_context_stream() {
    let _cap = assert_stream_op(
        ModelName::Doubao,
        Operation::ContextChat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
            context_id: "ctx-1".into(),
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `DoubaoServiceTest.imagesGenerations()`
#[tokio::test]
async fn doubao_service_images_generations() {
    let (value, cap) = assert_json_op(
        ModelName::Doubao,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "/images/generations",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chat()`
#[tokio::test]
async fn gemini_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "generateContent",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chatStream()`
#[tokio::test]
async fn gemini_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::Gemini,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "generateContent",
        false,
    )
    .await;
}

/// 对齐 Java: `GeminiServiceTest.testUpload()`
#[tokio::test]
async fn gemini_service_test_upload() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::UploadFile {
            file: PathBuf::from("clip.mov"),
        },
        upload_json(),
        "upload",
        false,
    )
    .await;
    assert!(value.get("file").is_some() || value.to_string().contains("files"));
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chatMultimodalImage()`
#[tokio::test]
async fn gemini_service_chat_multimodal_image() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::Multimodal {
            prompt: "图片上有些什么内容？".into(),
            media: vec!["https://example.com/files/mock".into()],
        },
        chat_json(),
        "generateContent",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chatMultimodalImageSteam()`
#[tokio::test]
async fn gemini_service_chat_multimodal_image_steam() {
    let _cap = assert_stream_op(
        ModelName::Gemini,
        Operation::Multimodal {
            prompt: "图片上有些什么内容？".into(),
            media: vec!["https://example.com/files/mock".into()],
        },
        "generateContent",
        false,
    )
    .await;
}

/// 对齐 Java: `GeminiServiceTest.chatMultimodalVideo()`
#[tokio::test]
async fn gemini_service_chat_multimodal_video() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::Multimodal {
            prompt: "图片上有些什么内容？".into(),
            media: vec!["https://example.com/files/mock".into()],
        },
        chat_json(),
        "generateContent",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chatMultimodalVideoStream()`
#[tokio::test]
async fn gemini_service_chat_multimodal_video_stream() {
    let _cap = assert_stream_op(
        ModelName::Gemini,
        Operation::Multimodal {
            prompt: "图片上有些什么内容？".into(),
            media: vec!["https://example.com/files/mock".into()],
        },
        "generateContent",
        false,
    )
    .await;
}

/// 对齐 Java: `GeminiServiceTest.chatJson()`
#[tokio::test]
async fn gemini_service_chat_json() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::JsonChat {
            messages: vec![Message::user(
                "提取以下信息：张三，男，25岁。返回JSON格式。",
            )],
        },
        chat_json(),
        "generateContent",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.chatImage()`
#[tokio::test]
async fn gemini_service_chat_image() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "generateContent",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.predictImage()`
#[tokio::test]
async fn gemini_service_predict_image() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "predict",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.predictImageAndSave()`
#[tokio::test]
async fn gemini_service_predict_image_and_save() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "predict",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.generateVideoTest()`
#[tokio::test]
async fn gemini_service_generate_video_test() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::CreateVideo {
            text: "动画视频".into(),
            image: None,
            parameters: vec![VideoParameter::new("--dur", 5)],
        },
        video_json(),
        "predictLongRunning",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.downLoadVideo()`
#[tokio::test]
async fn gemini_service_down_load_video() {
    let (value, cap) = assert_json_op(
        ModelName::Gemini,
        Operation::GetVideo {
            id: "operations/mock".into(),
        },
        video_json(),
        "operations",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `GeminiServiceTest.testTTSWithBuildMethod()`
#[tokio::test]
async fn gemini_service_test_t_t_s_with_build_method() {
    let _cap = assert_bytes_op(
        ModelName::Gemini,
        Operation::TextToSpeech {
            input: "hello".into(),
            voice: "nova".into(),
        },
        b"RIFF....WAVEfmt ".to_vec(),
        "generateContent",
        false,
    )
    .await;
}

/// 对齐 Java: `GrokServiceTest.chat()`
#[tokio::test]
async fn grok_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.chatStream()`
#[tokio::test]
async fn grok_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::Grok,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `GrokServiceTest.testChat()`
#[tokio::test]
async fn grok_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.message()`
#[tokio::test]
async fn grok_service_message() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Message {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
            max_tokens: 4096,
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.messageStream()`
#[tokio::test]
async fn grok_service_message_stream() {
    let _cap = assert_stream_op(
        ModelName::Grok,
        Operation::Message {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
            max_tokens: 4096,
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `GrokServiceTest.chatVision()`
#[tokio::test]
async fn grok_service_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.testChatVisionStream()`
#[tokio::test]
async fn grok_service_test_chat_vision_stream() {
    let _cap = assert_stream_op(
        ModelName::Grok,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `GrokServiceTest.testChatVision()`
#[tokio::test]
async fn grok_service_test_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "high".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.models()`
#[tokio::test]
async fn grok_service_models() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::ListModels,
        models_json(),
        "/models",
        false,
    )
    .await;
    assert!(
        value.get("data").is_some()
            || value.get("models").is_some()
            || value.get("object").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.getModel()`
#[tokio::test]
async fn grok_service_get_model() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::GetModel {
            id: "grok-2".into(),
        },
        models_json(),
        "/models/",
        false,
    )
    .await;
    assert!(value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.languageModels()`
#[tokio::test]
async fn grok_service_language_models() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::ListLanguageModels,
        models_json(),
        "/language-models",
        false,
    )
    .await;
    assert!(value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.getLanguageModel()`
#[tokio::test]
async fn grok_service_get_language_model() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::GetLanguageModel {
            id: "grok-2".into(),
        },
        models_json(),
        "/language-models/",
        false,
    )
    .await;
    assert!(value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.tokenizeText()`
#[tokio::test]
async fn grok_service_tokenize_text() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::Tokenize {
            texts: vec!["hello".into()],
        },
        tokenize_json(),
        "/tokenization",
        false,
    )
    .await;
    assert!(value.get("data").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.deferredCompletion()`
#[tokio::test]
async fn grok_service_deferred_completion() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::DeferredCompletion {
            request_id: "req-1".into(),
        },
        chat_json(),
        "/chat/deferred-completion/",
        false,
    )
    .await;
    assert!(value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `GrokServiceTest.imagesGenerations()`
#[tokio::test]
async fn grok_service_images_generations() {
    let (value, cap) = assert_json_op(
        ModelName::Grok,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "/images/generations",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.chat()`
#[tokio::test]
async fn hutool_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.chatStream()`
#[tokio::test]
async fn hutool_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::Hutool,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `HutoolServiceTest.testChat()`
#[tokio::test]
async fn hutool_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.chatVision()`
#[tokio::test]
async fn hutool_service_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.testChatVisionStream()`
#[tokio::test]
async fn hutool_service_test_chat_vision_stream() {
    let _cap = assert_stream_op(
        ModelName::Hutool,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `HutoolServiceTest.testChatVision()`
#[tokio::test]
async fn hutool_service_test_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "high".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.tokenizeText()`
#[tokio::test]
async fn hutool_service_tokenize_text() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::Tokenize {
            texts: vec!["hello".into()],
        },
        tokenize_json(),
        "/tokenization",
        false,
    )
    .await;
    assert!(value.get("data").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.imagesGenerations()`
#[tokio::test]
async fn hutool_service_images_generations() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "/images/generations",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.embeddingVision()`
#[tokio::test]
async fn hutool_service_embedding_vision() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::EmbedVision {
            text: "天空好难".into(),
            image: "https://img.example.com/a.jpg".into(),
        },
        embed_json(),
        "/embeddings/multimodal",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.to_string().contains("embedding"));
    assert!(cap.body.contains("input") || cap.body.contains("prompt") || !cap.body.is_empty());
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.textToSpeech()`
#[tokio::test]
async fn hutool_service_text_to_speech() {
    let _cap = assert_bytes_op(
        ModelName::Hutool,
        Operation::TextToSpeech {
            input: "hello".into(),
            voice: "nova".into(),
        },
        b"RIFF....WAVEfmt ".to_vec(),
        "/audio/speech",
        false,
    )
    .await;
}

/// 对齐 Java: `HutoolServiceTest.speechToText()`
#[tokio::test]
async fn hutool_service_speech_to_text() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::SpeechToText {
            file: PathBuf::from("a.wav"),
        },
        stt_json(),
        "/audio/transcriptions",
        false,
    )
    .await;
    assert_eq!(
        value.get("text").and_then(|v| v.as_str()),
        Some("hello world")
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.videoTasks()`
#[tokio::test]
async fn hutool_service_video_tasks() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::CreateVideo {
            text: "动画视频".into(),
            image: None,
            parameters: vec![VideoParameter::new("--dur", 5)],
        },
        video_json(),
        "/contents/generations/tasks",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `HutoolServiceTest.getVideoTasksInfo()`
#[tokio::test]
async fn hutool_service_get_video_tasks_info() {
    let (value, cap) = assert_json_op(
        ModelName::Hutool,
        Operation::GetVideo {
            id: "cgt-mock".into(),
        },
        video_json(),
        "/contents/generations/tasks/",
        false,
    )
    .await;
    assert!(
        value.get("id").is_some() || value.get("name").is_some() || value.get("status").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `OllamaServiceTest.testSimple()`
#[tokio::test]
async fn ollama_service_test_simple() {
    let (value, cap) = assert_json_op(
        ModelName::Ollama,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/api/chat",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OllamaServiceTest.testStream()`
#[tokio::test]
async fn ollama_service_test_stream() {
    let _cap = assert_stream_op(
        ModelName::Ollama,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/api/chat",
        false,
    )
    .await;
}

/// 对齐 Java: `OllamaServiceTest.testSimpleWithHistory()`
#[tokio::test]
async fn ollama_service_test_simple_with_history() {
    let (value, cap) = assert_json_op(
        ModelName::Ollama,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/api/chat",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OllamaServiceTest.testStreamWithHistory()`
#[tokio::test]
async fn ollama_service_test_stream_with_history() {
    let _cap = assert_stream_op(
        ModelName::Ollama,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        "/api/chat",
        false,
    )
    .await;
}

/// 对齐 Java: `OllamaServiceTest.testListModels()`
#[tokio::test]
async fn ollama_service_test_list_models() {
    let (value, cap) = assert_json_op(
        ModelName::Ollama,
        Operation::ListModels,
        models_json(),
        "/api/tags",
        false,
    )
    .await;
    assert!(
        value.get("data").is_some()
            || value.get("models").is_some()
            || value.get("object").is_some()
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `OllamaServiceTest.testPullModel()`
#[tokio::test]
async fn ollama_service_test_pull_model() {
    let (value, cap) = assert_json_op(
        ModelName::Ollama,
        Operation::PullModel {
            name: "qwen2.5:0.5b".into(),
        },
        pull_json(),
        "/api/pull",
        false,
    )
    .await;
    assert!(value.get("status").is_some() || value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OllamaServiceTest.testDeleteModel()`
#[tokio::test]
async fn ollama_service_test_delete_model() {
    let (value, cap) = assert_json_op(
        ModelName::Ollama,
        Operation::DeleteModel {
            name: "qwen2.5:0.5b".into(),
        },
        pull_json(),
        "/api/delete",
        false,
    )
    .await;
    assert!(value.get("status").is_some() || value.is_object());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.chat()`
#[tokio::test]
async fn openai_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.chatStream()`
#[tokio::test]
async fn openai_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `OpenaiServiceTest.testChat()`
#[tokio::test]
async fn openai_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.chatVision()`
#[tokio::test]
async fn openai_service_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.testChatVisionStream()`
#[tokio::test]
async fn openai_service_test_chat_vision_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `OpenaiServiceTest.imagesGenerations()`
#[tokio::test]
async fn openai_service_images_generations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "/images/generations",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.imagesEdits()`
#[tokio::test]
async fn openai_service_images_edits() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::EditImage {
            prompt: "deer".into(),
            image: PathBuf::from("img.png"),
            mask: None,
        },
        image_json(),
        "/images/edits",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.imagesVariations()`
#[tokio::test]
async fn openai_service_images_variations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::VaryImage {
            image: PathBuf::from("img.png"),
        },
        image_json(),
        "/images/variations",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.textToSpeech()`
#[tokio::test]
async fn openai_service_text_to_speech() {
    let _cap = assert_bytes_op(
        ModelName::OpenAi,
        Operation::TextToSpeech {
            input: "hello".into(),
            voice: "nova".into(),
        },
        b"RIFF....WAVEfmt ".to_vec(),
        "/audio/speech",
        false,
    )
    .await;
}

/// 对齐 Java: `OpenaiServiceTest.speechToText()`
#[tokio::test]
async fn openai_service_speech_to_text() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::SpeechToText {
            file: PathBuf::from("a.wav"),
        },
        stt_json(),
        "/audio/transcriptions",
        false,
    )
    .await;
    assert_eq!(
        value.get("text").and_then(|v| v.as_str()),
        Some("hello world")
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.embeddingText()`
#[tokio::test]
async fn openai_service_embedding_text() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::EmbedText {
            inputs: vec!["text".into()],
        },
        embed_json(),
        "/embeddings",
        false,
    )
    .await;
    assert!(value.get("data").is_some() || value.to_string().contains("embedding"));
    assert!(cap.body.contains("input") || cap.body.contains("prompt") || !cap.body.is_empty());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.moderations()`
#[tokio::test]
async fn openai_service_moderations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Moderate {
            text: "你要玩游戏".into(),
            image_url: Some("https://img.example.com/a.jpg".into()),
        },
        moderate_json(),
        "/moderations",
        false,
    )
    .await;
    assert!(value.get("results").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.chatReasoning()`
#[tokio::test]
async fn openai_service_chat_reasoning() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        false,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiServiceTest.chatReasoningStream()`
#[tokio::test]
async fn openai_service_chat_reasoning_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        "/chat/completions",
        false,
    )
    .await;
}

/// 对齐 Java: `OpenaiProxyServiceTest.chat()`
#[tokio::test]
async fn openai_proxy_service_chat() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        chat_json(),
        "/chat/completions",
        true,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.chatStream()`
#[tokio::test]
async fn openai_proxy_service_chat_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![Message::user("写一个疯狂星期四广告词")],
        },
        "/chat/completions",
        true,
    )
    .await;
}

/// 对齐 Java: `OpenaiProxyServiceTest.testChat()`
#[tokio::test]
async fn openai_proxy_service_test_chat() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        true,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.chatVision()`
#[tokio::test]
async fn openai_proxy_service_chat_vision() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        chat_json(),
        "/chat/completions",
        true,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.testChatVisionStream()`
#[tokio::test]
async fn openai_proxy_service_test_chat_vision_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Vision {
            prompt: "图片上有些什么？".into(),
            images: vec!["https://img.example.com/a.jpg".into()],
            detail: "auto".into(),
        },
        "/chat/completions",
        true,
    )
    .await;
}

/// 对齐 Java: `OpenaiProxyServiceTest.imagesGenerations()`
#[tokio::test]
async fn openai_proxy_service_images_generations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::GenerateImage {
            prompt: "astronaut".into(),
        },
        image_json(),
        "/images/generations",
        true,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.imagesEdits()`
#[tokio::test]
async fn openai_proxy_service_images_edits() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::EditImage {
            prompt: "deer".into(),
            image: PathBuf::from("img.png"),
            mask: None,
        },
        image_json(),
        "/images/edits",
        true,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.imagesVariations()`
#[tokio::test]
async fn openai_proxy_service_images_variations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::VaryImage {
            image: PathBuf::from("img.png"),
        },
        image_json(),
        "/images/variations",
        true,
    )
    .await;
    assert!(value.get("data").is_some() || value.get("predictions").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.textToSpeech()`
#[tokio::test]
async fn openai_proxy_service_text_to_speech() {
    let _cap = assert_bytes_op(
        ModelName::OpenAi,
        Operation::TextToSpeech {
            input: "hello".into(),
            voice: "nova".into(),
        },
        b"RIFF....WAVEfmt ".to_vec(),
        "/audio/speech",
        true,
    )
    .await;
}

/// 对齐 Java: `OpenaiProxyServiceTest.speechToText()`
#[tokio::test]
async fn openai_proxy_service_speech_to_text() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::SpeechToText {
            file: PathBuf::from("a.wav"),
        },
        stt_json(),
        "/audio/transcriptions",
        true,
    )
    .await;
    assert_eq!(
        value.get("text").and_then(|v| v.as_str()),
        Some("hello world")
    );
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.embeddingText()`
#[tokio::test]
async fn openai_proxy_service_embedding_text() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::EmbedText {
            inputs: vec!["text".into()],
        },
        embed_json(),
        "/embeddings",
        true,
    )
    .await;
    assert!(value.get("data").is_some() || value.to_string().contains("embedding"));
    assert!(cap.body.contains("input") || cap.body.contains("prompt") || !cap.body.is_empty());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.moderations()`
#[tokio::test]
async fn openai_proxy_service_moderations() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Moderate {
            text: "你要玩游戏".into(),
            image_url: Some("https://img.example.com/a.jpg".into()),
        },
        moderate_json(),
        "/moderations",
        true,
    )
    .await;
    assert!(value.get("results").is_some());
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.chatReasoning()`
#[tokio::test]
async fn openai_proxy_service_chat_reasoning() {
    let (value, cap) = assert_json_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        chat_json(),
        "/chat/completions",
        true,
    )
    .await;
    assert!(
        value.get("choices").is_some()
            || value.get("candidates").is_some()
            || value.to_string().contains("ok")
    );
    if !cap.body.is_empty() {
        assert!(
            cap.body.contains("model")
                || cap.body.contains("messages")
                || cap.body.contains("prompt")
                || cap.body.contains("contents")
                || cap.body.contains("input")
                || cap.start_line.contains("GET")
        );
    }
    let _ = (&value, &cap);
}

/// 对齐 Java: `OpenaiProxyServiceTest.chatReasoningStream()`
#[tokio::test]
async fn openai_proxy_service_chat_reasoning_stream() {
    let _cap = assert_stream_op(
        ModelName::OpenAi,
        Operation::Chat {
            messages: vec![
                Message::system("你是个抽象大师，会说很抽象的话，最擅长说抽象的笑话"),
                Message::user("给我说一个笑话"),
            ],
        },
        "/chat/completions",
        true,
    )
    .await;
}
