//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

use super::soap_client::SoapClient;

/// Jakarta SOAP 客户端，对齐 `cn.hutool.http.webservice.JakartaSoapClient`。
///
/// Java 9+ Jakarta EE 命名空间版本（javax.xml.soap → jakarta.xml.soap）。
/// Rust 中与 `SoapClient` 等价，无 javax/jakarta 区分。
pub trait JakartaSoapClient: SoapClient {}
