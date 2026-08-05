//! 公共 `Message` 对象。
//!
//! 对齐 Java 来源: `cn.hutool.ai.core.Message`
//!
//! Java 端 `content` 字段是 `Object`，可用于承载纯文本或多模态数组；
//! Rust 侧沿用既有 `String` 实现，保留 `role` + `content` 核心字段。

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

/// 消息角色枚举。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.Message.role` 字段的取值约定。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System-level instructions.
    System,
    /// End-user content.
    User,
    /// Model-generated content.
    Assistant,
    /// Tool output supplied to the model.
    Tool,
}

/// 一条对话消息。
///
/// 对齐 Java 来源: `cn.hutool.ai.core.Message`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// 消息角色。
    pub role: Role,
    /// 文本内容。
    pub content: String,
}

impl Message {
    /// 创建一条系统消息。
    #[must_use]
    pub fn system(content: &str) -> Self {
        Self {
            role: Role::System,
            content: content.to_owned(),
        }
    }

    /// 创建一条用户消息。
    #[must_use]
    pub fn user(content: &str) -> Self {
        Self {
            role: Role::User,
            content: content.to_owned(),
        }
    }

    /// 创建一条助手消息。
    #[must_use]
    pub fn assistant(content: &str) -> Self {
        Self {
            role: Role::Assistant,
            content: content.to_owned(),
        }
    }

    /// 创建一条工具消息。
    #[must_use]
    pub fn tool(content: &str) -> Self {
        Self {
            role: Role::Tool,
            content: content.to_owned(),
        }
    }

    /// 创建一个自由角色消息。
    #[must_use]
    pub fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_new_accepts_any_role_and_content() {
        let message = Message::new(Role::Tool, "tool-result");
        assert_eq!(message.role, Role::Tool);
        assert_eq!(message.content, "tool-result");
    }
}
