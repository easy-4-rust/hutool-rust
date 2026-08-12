//! 对齐: `cn.hutool.http.HttpConfig` (主机名验证部分)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpConfig.java
//! 中文说明: TLS主机名验证策略枚举，支持严格验证和宽松模式

/// Explicit hostname-verification policy for TLS connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostnameVerification {
    /// Verify certificate hostnames using Rustls and `WebPKI`.
    #[default]
    Strict,
    /// Accept invalid certificate hostnames. This is dangerous outside tests.
    DangerousAcceptInvalid,
}
