//! 对齐: `cn.hutool.jwt.JWTException`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/JWTException.java`
//! 中文说明: Hutool JWT 异常类型，统一封装解析、编码、签名与算法校验错误。

use std::fmt;

/// JWT 兼容层统一异常。
#[derive(Debug, thiserror::Error)]
pub enum JWTException {
    /// Token、claim 或算法参数不合法。
    #[error("{0}")]
    Invalid(String),
    /// JSON 序列化或反序列化失败。
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Base64 解码失败。
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    /// 底层密码学引擎返回错误。
    #[error(transparent)]
    Crypto(#[from] jsonwebtoken::errors::Error),
}

impl JWTException {
    /// 创建消息型异常，保持 Hutool `new JWTException(String)` 的入口形态。
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self::Invalid(message.into())
    }

    /// 按顺序替换 `{}` 占位符，兼容 Hutool `StrUtil.format` 的常见用法。
    #[must_use]
    pub fn formatted(template: &str, values: &[&dyn fmt::Display]) -> Self {
        let mut message = String::new();
        let mut remaining = template;
        for value in values {
            if let Some(index) = remaining.find("{}") {
                message.push_str(&remaining[..index]);
                message.push_str(&value.to_string());
                remaining = &remaining[index + 2..];
            } else {
                break;
            }
        }
        message.push_str(remaining);
        Self::new(message)
    }
}
