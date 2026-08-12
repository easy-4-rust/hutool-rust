//! `AIService` 异步服务 trait。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.AIService`
//!
//! Java 端为同步 + `Consumer<String>` 回调；Rust 端使用 `async fn` + `StreamCallback`。

use super::provider_service::ProviderService;
use crate::ProviderError;
use crate::message::Message;
use crate::operations::{AIResponse, Operation, StreamCallback};
use async_trait::async_trait;
use std::fmt;

/// 通用异步 AI 服务契约。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.AIService`
#[async_trait]
pub trait AIService: fmt::Debug + Send + Sync {
    /// 执行一条 provider 操作。
    async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError>;

    /// 以流式回调执行一条 provider 操作。
    async fn execute_stream(
        &self,
        operation: Operation,
        callback: StreamCallback,
    ) -> Result<(), ProviderError>;

    /// 单轮 chat 便捷入口（默认行为对齐 Java `default chat(String prompt)`）。
    async fn chat(&self, messages: Vec<Message>) -> Result<String, ProviderError> {
        Ok(self
            .execute(Operation::Chat { messages })
            .await?
            .into_text())
    }

    /// 流式 chat 便捷入口。
    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        callback: StreamCallback,
    ) -> Result<(), ProviderError> {
        self.execute_stream(Operation::Chat { messages }, callback)
            .await
    }
}

/// `BaseAIService` 抽象基类。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.BaseAIService`
/// Rust 端没有继承，统一通过 `AIService` trait + `ProviderService` 实现来表达。
/// 该 trait 为 Java 镜像占位，暂无实现方。
#[allow(dead_code)]
pub trait BaseAIService: AIService {
    /// 持有的 `ProviderService`。
    fn provider(&self) -> &ProviderService;
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serde_json::json;

    #[derive(Debug)]
    struct Dummy;

    #[async_trait]
    impl AIService for Dummy {
        async fn execute(&self, operation: Operation) -> Result<AIResponse, ProviderError> {
            assert!(matches!(operation, Operation::Chat { .. }));
            Ok(AIResponse::Json(json!({"answer": 1})))
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

    #[tokio::test]
    async fn chat_default_delegates_to_execute() {
        let dummy = Dummy;
        let result = dummy.chat(vec![Message::user("hi")]).await.unwrap();
        assert_eq!(result, r#"{"answer":1}"#);
    }

    #[tokio::test]
    async fn chat_stream_default_delegates_to_execute_stream() {
        let dummy = Dummy;
        let events = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let slot = std::sync::Arc::clone(&events);
        dummy
            .chat_stream(
                vec![Message::user("hi")],
                std::sync::Arc::new(move |event| slot.lock().unwrap().push(event)),
            )
            .await
            .unwrap();
        assert_eq!(events.lock().unwrap().as_slice(), &["event"]);
    }
}
