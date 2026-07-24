//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

mod soap_protocol;
mod jakarta_soap_protocol;
mod soap_runtime_exception;
mod soap_client;
mod jakarta_soap_client;
mod soap_util;
mod jakarta_soap_util;

pub use soap_protocol::SoapProtocol;
pub use jakarta_soap_protocol::JakartaSoapProtocol;
pub use soap_runtime_exception::SoapRuntimeException;
pub use soap_client::SoapClient;
pub use jakarta_soap_client::JakartaSoapClient;
pub use soap_util::SoapUtil;
pub use jakarta_soap_util::JakartaSoapUtil;
