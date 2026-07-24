//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

/// SOAP 协议枚举，对齐 `cn.hutool.http.webservice.SoapProtocol`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoapProtocol {
    /// SOAP 1.1
    Soap11,
    /// SOAP 1.2
    Soap12,
}

impl Default for SoapProtocol {
    fn default() -> Self {
        SoapProtocol::Soap11
    }
}
