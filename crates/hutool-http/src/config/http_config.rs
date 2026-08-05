//! 对齐: `cn.hutool.http.HttpConfig`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpConfig.java
//! 中文说明: HTTP客户端配置，管理超时、代理、TLS、拦截器等设置

use std::{fmt, sync::Arc, time::Duration};

use super::hostname_verification::HostnameVerification;
use super::http_config_error::HttpConfigError;
use super::http_interceptor_error::HttpInterceptorError;
use super::http_request_context::HttpRequestContext;
use super::http_response_context::HttpResponseContext;
use super::request_interceptor::RequestInterceptor;
use super::response_interceptor::ResponseInterceptor;
use super::tls_protocol::TlsProtocol;

/// HTTP client construction and Hutool compatibility settings.
#[derive(Clone)]
pub struct HttpConfig {
    /// TCP/TLS connection establishment timeout.
    pub connect_timeout: Duration,
    /// Entire request/read timeout.
    pub timeout: Duration,
    /// Maximum response body accepted by convenience methods.
    pub max_response_bytes: usize,
    /// User-Agent header value.
    pub user_agent: String,
    /// Maximum redirects followed by the client.
    pub redirect_limit: usize,
    /// Whether request-cache headers are disabled explicitly.
    pub disable_cache: bool,
    /// Preferred multipart / streaming block size hint (Hutool `blockSize`).
    pub block_size: usize,
    /// Whether truncated EOF bodies are tolerated when copying (stored for parity).
    pub ignore_eof_error: bool,
    /// Whether URL query decoding is requested before send (stored for parity).
    pub decode_url: bool,
    /// Whether interceptors run on redirect hops (stored for parity).
    pub interceptor_on_redirect: bool,
    /// Whether cookies are forwarded across redirects.
    pub follow_redirects_cookie: bool,
    /// Whether a default Content-Type is injected when missing.
    pub use_default_content_type_if_null: bool,
    /// Whether declared Content-Length should be ignored by helpers.
    pub ignore_content_length: bool,
    pub(crate) proxy_url: Option<String>,
    pub(crate) hostname_verification: HostnameVerification,
    pub(crate) tls_identity: Option<reqwest::Identity>,
    pub(crate) root_certificates: Vec<reqwest::Certificate>,
    pub(crate) tls_protocol: Option<TlsProtocol>,
    pub(crate) request_interceptors: Vec<RequestInterceptor>,
    pub(crate) response_interceptors: Vec<ResponseInterceptor>,
}

impl fmt::Debug for HttpConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpConfig")
            .field("connect_timeout", &self.connect_timeout)
            .field("timeout", &self.timeout)
            .field("max_response_bytes", &self.max_response_bytes)
            .field("user_agent", &self.user_agent)
            .field("redirect_limit", &self.redirect_limit)
            .field("disable_cache", &self.disable_cache)
            .field("block_size", &self.block_size)
            .field("ignore_eof_error", &self.ignore_eof_error)
            .field("decode_url", &self.decode_url)
            .field("interceptor_on_redirect", &self.interceptor_on_redirect)
            .field("follow_redirects_cookie", &self.follow_redirects_cookie)
            .field(
                "use_default_content_type_if_null",
                &self.use_default_content_type_if_null,
            )
            .field("ignore_content_length", &self.ignore_content_length)
            .field("proxy_url", &self.proxy_url.as_ref().map(|_| "<redacted>"))
            .field("hostname_verification", &self.hostname_verification)
            .field(
                "tls_identity",
                &self.tls_identity.as_ref().map(|_| "configured"),
            )
            .field("root_certificates", &self.root_certificates.len())
            .field("tls_protocol", &self.tls_protocol)
            .field("request_interceptors", &self.request_interceptors.len())
            .field("response_interceptors", &self.response_interceptors.len())
            .finish()
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
            max_response_bytes: 8 * 1024 * 1024,
            user_agent: concat!("hutool-http/", env!("CARGO_PKG_VERSION")).to_owned(),
            redirect_limit: 5,
            disable_cache: false,
            block_size: 8192,
            ignore_eof_error: false,
            decode_url: false,
            interceptor_on_redirect: false,
            follow_redirects_cookie: true,
            use_default_content_type_if_null: true,
            ignore_content_length: false,
            proxy_url: None,
            hostname_verification: HostnameVerification::Strict,
            tls_identity: None,
            root_certificates: Vec::new(),
            tls_protocol: None,
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
        }
    }
}

impl HttpConfig {
    /// Creates the default Hutool-aligned configuration.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Sets both connection and read timeouts in milliseconds.
    pub fn timeout_millis(&mut self, milliseconds: i64) -> Result<&mut Self, HttpConfigError> {
        let timeout = duration(milliseconds)?;
        self.connect_timeout = timeout;
        self.timeout = timeout;
        Ok(self)
    }

    /// Sets the connection timeout in milliseconds.
    pub fn set_connection_timeout_millis(
        &mut self,
        milliseconds: i64,
    ) -> Result<&mut Self, HttpConfigError> {
        self.connect_timeout = duration(milliseconds)?;
        Ok(self)
    }

    /// Sets the request/read timeout in milliseconds.
    pub fn set_read_timeout_millis(
        &mut self,
        milliseconds: i64,
    ) -> Result<&mut Self, HttpConfigError> {
        self.timeout = duration(milliseconds)?;
        Ok(self)
    }

    /// Disables caches through standard request headers.
    ///
    /// Java: `HttpConfig.disableCache()`
    pub const fn disable_cache(&mut self) -> &mut Self {
        self.disable_cache = true;
        self
    }

    /// Sets the preferred block size hint used by streaming helpers.
    ///
    /// Java: `HttpConfig.setBlockSize(int blockSize)`
    pub const fn set_block_size(&mut self, block_size: usize) -> &mut Self {
        self.block_size = block_size;
        self
    }

    /// Controls whether truncated EOF copy errors are ignored.
    ///
    /// Java: `HttpConfig.setIgnoreEOFError(boolean ignoreEOFError)`
    pub const fn set_ignore_eof_error(&mut self, ignore: bool) -> &mut Self {
        self.ignore_eof_error = ignore;
        self
    }

    /// Controls whether URL query decoding is requested before send.
    ///
    /// Java: `HttpConfig.setDecodeUrl(boolean decodeUrl)`
    pub const fn set_decode_url(&mut self, decode_url: bool) -> &mut Self {
        self.decode_url = decode_url;
        self
    }

    /// Controls whether interceptors run on redirect hops.
    ///
    /// Java: `HttpConfig.setInterceptorOnRedirect(boolean interceptorOnRedirect)`
    pub const fn set_interceptor_on_redirect(&mut self, enabled: bool) -> &mut Self {
        self.interceptor_on_redirect = enabled;
        self
    }

    /// Controls whether cookies follow redirects.
    ///
    /// Java: `HttpConfig.setFollowRedirectsCookie(boolean followRedirectsCookie)`
    pub const fn set_follow_redirects_cookie(&mut self, follow: bool) -> &mut Self {
        self.follow_redirects_cookie = follow;
        self
    }

    /// Controls default Content-Type injection when the header is absent.
    ///
    /// Java: `HttpConfig.setUseDefaultContentTypeIfNull(boolean)`
    pub const fn set_use_default_content_type_if_null(&mut self, enabled: bool) -> &mut Self {
        self.use_default_content_type_if_null = enabled;
        self
    }

    /// Controls whether helpers ignore declared Content-Length.
    ///
    /// Java: `HttpConfig.setIgnoreContentLength(boolean ignoreContentLength)`
    pub const fn set_ignore_content_length(&mut self, ignore: bool) -> &mut Self {
        self.ignore_content_length = ignore;
        self
    }

    /// Sets the redirect limit, clamping negative values to zero.
    pub fn set_max_redirect_count(&mut self, count: i32) -> &mut Self {
        self.redirect_limit = usize::try_from(count).unwrap_or_default();
        self
    }

    /// Sets the TLS hostname-verification policy.
    pub const fn set_hostname_verifier(&mut self, verification: HostnameVerification) -> &mut Self {
        self.hostname_verification = verification;
        self
    }

    /// Configures an HTTP proxy from a host and port.
    pub fn set_http_proxy(&mut self, host: &str, port: u16) -> Result<&mut Self, HttpConfigError> {
        self.set_proxy(format!("http://{}:{port}", host.trim()))
    }

    /// Configures an HTTP, HTTPS, or SOCKS proxy URL.
    pub fn set_proxy(
        &mut self,
        proxy_url: impl Into<String>,
    ) -> Result<&mut Self, HttpConfigError> {
        let proxy_url = proxy_url.into();
        reqwest::Proxy::all(&proxy_url)
            .map_err(|_| HttpConfigError::InvalidProxy(proxy_url.clone()))?;
        self.proxy_url = Some(proxy_url);
        Ok(self)
    }

    /// Configures a PKCS#12 or PEM client identity instead of a Java socket factory.
    pub fn set_ssl_identity(&mut self, identity: reqwest::Identity) -> &mut Self {
        self.tls_identity = Some(identity);
        self
    }

    /// Adds a trusted PEM or DER root certificate.
    pub fn add_root_certificate(&mut self, certificate: reqwest::Certificate) -> &mut Self {
        self.root_certificates.push(certificate);
        self
    }

    /// Selects TLS 1.2 or TLS 1.3; obsolete protocols are rejected.
    pub fn set_ssl_protocol(&mut self, protocol: &str) -> Result<&mut Self, HttpConfigError> {
        let protocol = protocol.trim();
        if protocol.is_empty() {
            return Err(HttpConfigError::BlankTlsProtocol);
        }
        self.tls_protocol = Some(match protocol.to_ascii_uppercase().as_str() {
            "TLSV1.2" | "TLS1.2" => TlsProtocol::Tls12,
            "TLSV1.3" | "TLS1.3" => TlsProtocol::Tls13,
            _ => return Err(HttpConfigError::UnsupportedTlsProtocol(protocol.to_owned())),
        });
        Ok(self)
    }

    /// Adds an owned request interceptor.
    pub fn add_request_interceptor<F>(&mut self, interceptor: F) -> &mut Self
    where
        F: Fn(&mut HttpRequestContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        self.request_interceptors.push(Arc::new(interceptor));
        self
    }

    /// Adds an owned response interceptor.
    pub fn add_response_interceptor<F>(&mut self, interceptor: F) -> &mut Self
    where
        F: Fn(&mut HttpResponseContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
    {
        self.response_interceptors.push(Arc::new(interceptor));
        self
    }

    pub(crate) fn intercept_request(
        &self,
        context: &mut HttpRequestContext,
    ) -> Result<(), HttpInterceptorError> {
        for interceptor in &self.request_interceptors {
            interceptor(context)?;
        }
        Ok(())
    }

    pub(crate) fn intercept_response(
        &self,
        context: &mut HttpResponseContext,
    ) -> Result<(), HttpInterceptorError> {
        for interceptor in &self.response_interceptors {
            interceptor(context)?;
        }
        Ok(())
    }
}

use super::duration;
