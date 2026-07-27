//! `AIUtil` 对象。
//! 对齐 Java 来源: `cn.hutool.ai.AIUtil`
//! 说明: 保留 Java 静态工具门面语义，内部转调 `AIServiceFactory` 和 `AIService`。

use super::{AIService, AIServiceFactory, BaseConfig};
use crate::{Message, ProviderError};
use std::sync::Arc;

/// Hutool 兼容层工具门面。
///
/// 对齐 Java 来源: `cn.hutool.ai.AIUtil`
pub struct AIUtil;

impl AIUtil {
    /// 创建内置服务实例。
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, ProviderError> {
        AIServiceFactory::get_ai_service(config)
    }

    /// 发送单条用户提示词。
    pub async fn chat(
        config: BaseConfig,
        prompt: impl Into<String>,
    ) -> Result<String, ProviderError> {
        let prompt = prompt.into();
        Self::chat_messages(config, vec![Message::user(&prompt)]).await
    }

    /// 发送完整消息序列。
    pub async fn chat_messages(
        config: BaseConfig,
        messages: Vec<Message>,
    ) -> Result<String, ProviderError> {
        Self::get_ai_service(config)
            .expect("validated built-in provider configuration creates a service")
            .chat(messages)
            .await
    }
}
