//! 对齐: `cn.hutool.http.HttpBaseError`
//! 来源: hutool-http/src/main/java/cn/hutool/http/base/HttpBaseError.java
//! 中文说明: HTTP基础元数据错误类型，处理编码不支持等异常

use thiserror::Error;

/// Errors returned while configuring shared HTTP metadata.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HttpBaseError {
    /// The requested character encoding is not supported by Encoding Standard.
    #[error("unsupported HTTP character encoding: {0}")]
    UnsupportedCharset(String),
}
