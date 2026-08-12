//! 对齐: `cn.hutool.http` (配置模块)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpConfig.java
//! 中文说明: HTTP配置模块，包含TLS、拦截器、请求/响应上下文等配置组件

use std::time::Duration;

mod hostname_verification;
mod http_config;
mod http_config_error;
mod http_interceptor_error;
mod http_request_context;
mod http_response_context;
mod request_interceptor;
mod response_interceptor;
mod tls_protocol;

pub use hostname_verification::HostnameVerification;
pub use http_config::HttpConfig;
pub use http_config_error::HttpConfigError;
pub use http_interceptor_error::HttpInterceptorError;
pub use http_request_context::HttpRequestContext;
pub use http_response_context::HttpResponseContext;
pub use request_interceptor::RequestInterceptor;
pub use response_interceptor::ResponseInterceptor;
pub use tls_protocol::TlsProtocol;

fn duration(milliseconds: i64) -> Result<Duration, HttpConfigError> {
    let milliseconds =
        u64::try_from(milliseconds).map_err(|_| HttpConfigError::NegativeTimeout(milliseconds))?;
    Ok(Duration::from_millis(milliseconds))
}
