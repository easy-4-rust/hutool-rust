//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

/// SOAP 运行时异常，对齐 `cn.hutool.http.webservice.SoapRuntimeException`。
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct SoapRuntimeException {
    message: String,
}

impl SoapRuntimeException {
    /// 对齐 `SoapRuntimeException(String)`
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// 对齐 `SoapRuntimeException(Throwable)`
    pub fn from_error<E: std::error::Error>(e: E) -> Self {
        Self {
            message: e.to_string(),
        }
    }

    /// 获取消息
    pub fn get_message(&self) -> &str {
        &self.message
    }
}
