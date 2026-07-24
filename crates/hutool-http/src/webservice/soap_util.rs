//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

use super::soap_client::SoapClient;

/// SOAP 工具类，对齐 `cn.hutool.http.webservice.SoapUtil`。
pub struct SoapUtil;

impl SoapUtil {
    /// 对齐 `SoapUtil.createClient(String url)`
    pub fn create_client(_url: &str) -> Result<Box<dyn SoapClient>, HttpException> {
        Err(HttpException::new(
            "SoapUtil::create_client requires javax.xml.soap; consider soap-rs crate in Rust",
        ))
    }

    /// 对齐 `SoapUtil.createClient(String url, String method)`
    pub fn create_client_with_method(
        _url: &str,
        _method: &str,
    ) -> Result<Box<dyn SoapClient>, HttpException> {
        Self::create_client(_url)
    }
}
