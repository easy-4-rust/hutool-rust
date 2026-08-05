//! `AIUtil` 对象。
//!
//! 对齐 Java 来源: `cn.hutool.ai.AIUtil`
//!
//! Java 侧为静态工具门面，提供 `getAIService / getDeepSeekService / chat / ...` 入口；
//! Rust 侧沿用同构 API，内部全部委托给 `AIServiceFactory` 与 `core::AIService`。

use crate::core::{AIService, BaseConfig};
use crate::message::Message;
use crate::{AIException, AIServiceFactory};
use std::sync::Arc;

/// `AIUtil` 静态门面。
///
/// 对齐 Java 来源: `cn.hutool.ai.AIUtil`
pub struct AIUtil;

impl AIUtil {
    /// 创建内置服务实例。
    pub fn get_ai_service(config: BaseConfig) -> Result<Arc<dyn AIService>, AIException> {
        AIServiceFactory::get_ai_service(config)
    }

    /// 发送单条用户提示词。
    pub async fn chat(
        config: BaseConfig,
        prompt: impl Into<String>,
    ) -> Result<String, AIException> {
        let prompt = prompt.into();
        Self::chat_messages(config, vec![Message::user(&prompt)]).await
    }

    /// 发送完整消息序列。
    pub async fn chat_messages(
        config: BaseConfig,
        messages: Vec<Message>,
    ) -> Result<String, AIException> {
        let service = Self::get_ai_service(config)?;
        Ok(service.chat(messages).await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_util_is_a_zero_sized_static_facade() {
        // 静态门面没有运行时状态。
        let _ = std::mem::size_of::<AIUtil>();
    }
}
