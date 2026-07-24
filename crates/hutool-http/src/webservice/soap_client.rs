//! WebService (SOAP) facade，对齐 hutool 的 `cn.hutool.http.webservice.*`。
//!
//! 提供 SOAP 客户端和工具的 trait 抽象。
//! 具体实现依赖 `javax.xml.soap.SOAPMessage` / Jakarta SOAP，属于 unsafe-to-copy。
//! Rust 用户推荐使用 `soap-rs` 或 `xml-rs` 等替代。

use std::collections::HashMap;

use crate::HttpException;

use super::soap_protocol::SoapProtocol;

/// SOAP 客户端，对齐 `cn.hutool.http.webservice.SoapClient`。
///
/// Java 基于 `javax.xml.soap.SOAPConnection`；Rust 用 trait 抽象。
/// 注意：所有 setter 返回 `()`（不返回 `&mut Self`）以保持 dyn-compatible。
pub trait SoapClient: Send + Sync {
    /// 对齐 `SoapClient.setCharset(Charset)`
    fn set_charset(&mut self, charset: &str);

    /// 对齐 `SoapClient.getCharset()`
    fn get_charset(&self) -> &str;

    /// 对齐 `SoapClient.setUrl(String)`
    fn set_url(&mut self, url: &str);

    /// 对齐 `SoapClient.getUrl()`
    fn get_url(&self) -> &str;

    /// 对齐 `SoapClient.setMethod(String)`
    fn set_method(&mut self, method: &str);

    /// 对齐 `SoapClient.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `SoapClient.setTimeout(int)`
    fn set_timeout(&mut self, timeout_ms: u64);

    /// 对齐 `SoapClient.getTimeout()`
    fn get_timeout(&self) -> u64;

    /// 对齐 `SoapClient.setProtocol(SoapProtocol)`
    fn set_protocol(&mut self, protocol: SoapProtocol);

    /// 对齐 `SoapClient.getProtocol()`
    fn get_protocol(&self) -> SoapProtocol;

    /// 对齐 `SoapClient.addBody(Map<String, Object>)`
    fn add_body(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.setBody(Map<String, Object>)`
    fn set_body(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.getBody()`
    fn get_body(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addHeader(String, Object)`
    fn add_header(&mut self, name: &str, value: &str);

    /// 对齐 `SoapClient.getHeaders()`
    fn get_headers(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addSOAPHeader(Map<String, String>)`
    fn add_soap_header(&mut self, headers: HashMap<String, String>);

    /// 对齐 `SoapClient.getSOAPHeaders()`
    fn get_soap_headers(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.setParameters(Map<String, Object>)`
    fn set_parameters(&mut self, params: HashMap<String, String>);

    /// 对齐 `SoapClient.getParameters()`
    fn get_parameters(&self) -> &HashMap<String, String>;

    /// 对齐 `SoapClient.addParameter(String, Object)`
    fn add_parameter(&mut self, name: &str, value: &str);

    /// 对齐 `SoapClient.getMessages(boolean pretty)`
    fn get_messages(&self, pretty: bool) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.send()`
    fn send(&self) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.send(boolean pretty)`
    fn send_pretty(&self, pretty: bool) -> Result<String, HttpException>;

    /// 对齐 `SoapClient.sendForString()`
    fn send_for_string(&self) -> Result<String, HttpException> {
        self.send()
    }

    /// 对齐 `SoapClient.sendForDocument()` — Rust 版返回 String（Java 返回 org.w3c.dom.Document）
    fn send_for_document(&self) -> Result<String, HttpException> {
        self.send()
    }
}
