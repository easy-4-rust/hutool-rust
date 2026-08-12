//! 对齐: `cn.hutool.http.HttpConfig` (错误类型部分)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpConfig.java
//! 中文说明: HTTP配置构建错误类型，处理超时、代理等配置异常

use thiserror::Error;

/// Errors returned while building Hutool-compatible HTTP configuration.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HttpConfigError {
    /// Timeout values must be non-negative.
    #[error("HTTP timeout must be non-negative, got {0} ms")]
    NegativeTimeout(i64),
    /// A proxy URL could not be accepted by Reqwest.
    #[error("invalid HTTP proxy URL: {0}")]
    InvalidProxy(String),
    /// A TLS protocol name was blank.
    #[error("TLS protocol must not be blank")]
    BlankTlsProtocol,
    /// Rustls intentionally does not support the requested protocol.
    #[error("unsupported or insecure TLS protocol: {0}")]
    UnsupportedTlsProtocol(String),
}
