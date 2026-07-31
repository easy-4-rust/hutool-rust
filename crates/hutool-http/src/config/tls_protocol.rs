//! 对齐: `cn.hutool.http.HttpConfig` (TLS协议部分)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpConfig.java
//! 中文说明: TLS协议版本枚举，支持TLS 1.2和TLS 1.3

use reqwest::tls::Version;

/// TLS protocol versions supported by the Rustls transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsProtocol {
    /// TLS 1.2 only.
    Tls12,
    /// TLS 1.3 only.
    Tls13,
}

impl TlsProtocol {
    pub(crate) const fn reqwest(self) -> Version {
        match self {
            Self::Tls12 => Version::TLS_1_2,
            Self::Tls13 => Version::TLS_1_3,
        }
    }
}
