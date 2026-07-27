//! 对齐: `cn.hutool.http`
//! 来源: hutool-http/src/main/java/cn/hutool/http/
//! 中文说明: Hutool HTTP模块，提供HTTP客户端、请求、响应、工具类等完整HTTP功能

#![forbid(unsafe_code)]

use rand::Rng as _;
pub use reqwest::{Method, StatusCode, Url, header};
use serde::de::DeserializeOwned;
use std::{fmt, net::IpAddr, sync::Arc, time::Duration};
use thiserror::Error;

mod base;
pub mod prelude;
pub use base::{HTTP_1_0, HTTP_1_1, HttpBase, HttpBaseError};
mod body;
pub use body::{BytesBody, FormUrlEncodedBody, MultipartBody, MultipartOutputStream, RequestBody, ResourceBody};
mod config;
pub use config::{
    HostnameVerification, HttpConfig, HttpConfigError, HttpInterceptorError, HttpRequestContext,
    HttpResponseContext, RequestInterceptor, ResponseInterceptor, TlsProtocol,
};
mod cookie;
pub use cookie::{
    CookieJar, CookieManagerHandle, GlobalCookieManager, ThreadLocalCookieStore,
};
mod interceptor;
pub use interceptor::GlobalInterceptor;
mod input_stream;
pub use input_stream::HttpInputStream;
mod resource;
pub use resource::HttpResource;
mod downloader;
pub use downloader::HttpDownloader;
mod global_config;
pub use global_config::{HttpGlobalConfig, HttpGlobalConfigState, DEFAULT_BOUNDARY};
mod metadata;
pub use metadata::{ContentType, GlobalHeaders, Header, HttpStatus, Status};
mod progress;
pub use progress::{FnStreamProgress, NoopStreamProgress, StreamProgress};
mod query;
mod http_util;
pub use http_util::{form_map, HttpUtil};
mod request;
pub use request::HttpRequest;
mod response;
pub use response::{HttpCookie, HttpResponse};
mod exception;
pub use exception::HttpException;
mod http_connection;
pub use http_connection::{HttpConnection, StubHttpConnection};
pub mod server;
pub use server::{
    Action, DefaultExceptionFilter, Filter, FilterChain, HttpExchangeWrapper, HttpServerBase,
    HttpServerRequest, HttpServerResponse, RootAction, SimpleServer,
};
pub mod ssl;
pub use ssl::{
    CustomProtocolsSslFactory, CustomProtocolsSslFactoryImpl, TrustAnyHostnameVerifier,
    TrustAnyHostnameVerifierImpl,
};
pub mod webservice;
pub use webservice::{
    JakartaSoapClient, JakartaSoapProtocol, JakartaSoapUtil, SoapClient, SoapProtocol,
    SoapRuntimeException, SoapUtil,
};
use tokio::io::{AsyncWrite, AsyncWriteExt};

/// User-Agent parsing compatible with Hutool's `useragent` package.
#[cfg(feature = "useragent")]
pub mod useragent;

/// HTML helpers aligned with Hutool `HtmlUtil` / `HTMLFilter`.
#[cfg(feature = "html")]
pub mod html;

/// Validates a destination URL before network I/O.
pub trait UrlPolicy: Send + Sync {
    /// Returns `Ok` when the URL is allowed.
    fn validate(&self, url: &Url) -> Result<(), UrlPolicyError>;
}

/// A policy that accepts all syntactically valid URLs.
#[derive(Debug, Clone, Copy, Default)]
pub struct AllowAllUrls;

impl UrlPolicy for AllowAllUrls {
    fn validate(&self, _url: &Url) -> Result<(), UrlPolicyError> {
        Ok(())
    }
}

/// Rejects loopback, private, link-local, multicast, and unspecified literal
/// IP addresses, plus the `localhost` hostname.
///
/// Domain-name resolution is intentionally left to application-specific
/// policies so callers can defend against DNS rebinding in their environment.
#[derive(Debug, Clone, Copy, Default)]
pub struct DenyLocalTargets;

impl UrlPolicy for DenyLocalTargets {
    fn validate(&self, url: &Url) -> Result<(), UrlPolicyError> {
        Self::validate_parts(url.scheme(), url.host())
    }
}

impl DenyLocalTargets {
    fn validate_parts(scheme: &str, host: Option<url::Host<&str>>) -> Result<(), UrlPolicyError> {
        if !matches!(scheme, "http" | "https") {
            return Err(UrlPolicyError::UnsupportedScheme);
        }
        let host = host.ok_or(UrlPolicyError::MissingHost)?;
        let denied = match host {
            url::Host::Domain(domain) => domain.eq_ignore_ascii_case("localhost"),
            url::Host::Ipv4(address) => is_denied_ip(IpAddr::V4(address)),
            url::Host::Ipv6(address) => is_denied_ip(IpAddr::V6(address)),
        };
        if denied {
            return Err(UrlPolicyError::DeniedTarget);
        }
        Ok(())
    }
}

fn is_denied_ip(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => {
            address.is_private()
                || address.is_loopback()
                || address.is_link_local()
                || address.is_multicast()
                || address.is_unspecified()
                || address.is_broadcast()
                || address.is_documentation()
        }
        IpAddr::V6(address) => {
            address.is_loopback()
                || address.is_multicast()
                || address.is_unspecified()
                || address.is_unique_local()
                || address.is_unicast_link_local()
        }
    }
}

/// URL policy failures.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum UrlPolicyError {
    /// Only HTTP and HTTPS are supported.
    #[error("only http and https URL schemes are allowed")]
    UnsupportedScheme,
    /// URL does not contain a host.
    #[error("URL must contain a host")]
    MissingHost,
    /// URL targets a network location denied by policy.
    #[error("URL target is denied by policy")]
    DeniedTarget,
}

/// Errors returned by bounded response helpers.
#[derive(Debug, Error)]
pub enum HttpError {
    /// The HTTP stack rejected or failed the operation.
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    /// JSON decoding failed.
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    /// Streaming output failed.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Destination was rejected before I/O.
    #[error(transparent)]
    UrlPolicy(#[from] UrlPolicyError),
    /// Retry configuration is invalid.
    #[error(transparent)]
    RetryPolicy(#[from] RetryPolicyError),
    /// HTTP client configuration is invalid or unavailable in this build.
    #[error(transparent)]
    Config(#[from] HttpConfigError),
    /// A request or response interceptor rejected the operation.
    #[error(transparent)]
    Interceptor(#[from] HttpInterceptorError),
    /// Automatic retries are restricted to HTTP idempotent methods.
    #[error("automatic retry is not allowed for HTTP method {0}")]
    NonIdempotentRetry(Method),
    /// The request body cannot be cloned safely for another attempt.
    #[error("request body cannot be cloned for retry")]
    UncloneableRetryRequest,
    /// The response status was not successful for a helper that requires 2xx.
    #[error("unexpected HTTP status {status}")]
    UnexpectedStatus {
        /// Observed response status.
        status: StatusCode,
    },
    /// Every retry attempt failed.
    #[error("HTTP request failed after {attempts} attempts: {source}")]
    RetriesExhausted {
        /// Number of attempts made, including the first request.
        attempts: usize,
        /// Last transport or HTTP status failure.
        #[source]
        source: reqwest::Error,
    },
    /// The response exceeded the configured in-memory limit.
    #[error("response body contains {actual} bytes, exceeding limit {limit}")]
    ResponseTooLarge {
        /// Configured byte limit.
        limit: usize,
        /// Observed response size.
        actual: usize,
    },
}

/// Validation errors for [`RetryPolicy`].
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum RetryPolicyError {
    /// At least one request attempt is required.
    #[error("retry policy must allow at least one attempt")]
    ZeroAttempts,
}

/// Explicit retry policy for idempotent asynchronous requests.
///
/// Retries use capped exponential backoff with jitter and honor numeric
/// `Retry-After` seconds up to the configured maximum delay.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    max_attempts: usize,
    base_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    /// Creates a retry policy, counting the initial request as one attempt.
    pub fn new(max_attempts: usize) -> Result<Self, RetryPolicyError> {
        if max_attempts == 0 {
            return Err(RetryPolicyError::ZeroAttempts);
        }
        Ok(Self {
            max_attempts,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
        })
    }

    /// Sets the initial backoff duration.
    #[must_use]
    pub const fn base_delay(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }

    /// Sets the cap applied to backoff and `Retry-After` delays.
    #[must_use]
    pub const fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    fn delay(&self, failed_attempt: usize, retry_after: Option<Duration>) -> Duration {
        if let Some(delay) = retry_after {
            return delay.min(self.max_delay);
        }
        let exponent = u32::try_from(failed_attempt.saturating_sub(1).min(31)).unwrap_or(31);
        let ceiling = self
            .base_delay
            .saturating_mul(1_u32 << exponent)
            .min(self.max_delay);
        let max_millis = u64::try_from(ceiling.as_millis()).unwrap_or(u64::MAX);
        Duration::from_millis(rand::rng().random_range(0..=max_millis))
    }
}

/// A cheap-to-clone asynchronous HTTP client.
#[derive(Clone)]
pub struct HttpClient {
    inner: reqwest::Client,
    max_response_bytes: usize,
    url_policy: Arc<dyn UrlPolicy>,
    config: Arc<HttpConfig>,
}

impl fmt::Debug for HttpClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpClient")
            .field("max_response_bytes", &self.max_response_bytes)
            .field("url_policy", &"dyn UrlPolicy")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl HttpClient {
    /// Creates a configurable client builder.
    #[must_use]
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::default()
    }

    /// Builds a pooled client with explicit time and body limits.
    pub fn new(config: &HttpConfig) -> Result<Self, HttpError> {
        HttpClientBuilder::from_config(config.clone()).build()
    }

    /// Returns the underlying request builder.
    pub fn request(&self, method: Method, url: impl reqwest::IntoUrl) -> reqwest::RequestBuilder {
        self.inner.request(method, url)
    }

    /// Sends a GET request and decodes a bounded response body.
    pub async fn get_text(&self, url: impl reqwest::IntoUrl) -> Result<String, HttpError> {
        self.get_text_request(self.inner.get(url)).await
    }

    /// Sends a GET request and deserializes a bounded JSON response body.
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: impl reqwest::IntoUrl,
    ) -> Result<T, HttpError> {
        self.send_json(self.inner.get(url)).await
    }

    /// Sends a prepared request and deserializes a bounded JSON response.
    pub async fn send_json<T: DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<T, HttpError> {
        let response = self.send_response_checked(request).await?;
        Ok(serde_json::from_slice(response.body_bytes())?)
    }

    async fn get_text_request(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<String, HttpError> {
        Ok(self.send_response_checked(request).await?.body())
    }

    /// Sends a GET request and returns a bounded, repeatable response facade.
    pub async fn get_response(
        &self,
        url: impl reqwest::IntoUrl,
    ) -> Result<HttpResponse, HttpError> {
        self.send_response(self.inner.get(url)).await
    }

    /// Sends a prepared request and buffers the response under the configured
    /// limit without converting non-success statuses into transport errors.
    pub async fn send_response(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<HttpResponse, HttpError> {
        let response = self.execute(request).await?;
        self.buffer_response(response).await
    }

    /// Sends a prepared request after applying URL policy and HTTP status
    /// validation. The response body remains streaming.
    pub async fn send(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, HttpError> {
        Ok(self.execute(request).await?.error_for_status()?)
    }

    async fn execute(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, HttpError> {
        let mut request = request.build()?;
        apply_async_request_interceptors(&self.config, &mut request)?;
        self.url_policy.validate(request.url())?;
        let mut response = self.inner.execute(request).await?;
        apply_response_interceptors(&self.config, response.status(), response.headers_mut())?;
        Ok(response)
    }

    async fn send_response_checked(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<HttpResponse, HttpError> {
        let response = self.execute(request).await?.error_for_status()?;
        self.buffer_response(response).await
    }

    async fn buffer_response(
        &self,
        mut response: reqwest::Response,
    ) -> Result<HttpResponse, HttpError> {
        let status = response.status();
        let headers = response.headers().clone();
        let url = response.url().to_string();
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            self.ensure_size(body.len().saturating_add(chunk.len()))?;
            body.extend_from_slice(&chunk);
        }
        Ok(HttpResponse::new(status, headers, body).with_url(url))
    }

    /// Sends an idempotent request with an explicit retry policy.
    ///
    /// Only `GET`, `HEAD`, `PUT`, `DELETE`, `OPTIONS`, and `TRACE` are accepted.
    /// Requests with streaming or otherwise non-cloneable bodies are rejected.
    /// Retryable failures are connection/timeouts and status codes 408, 429,
    /// 500, 502, 503, and 504.
    pub async fn send_idempotent(
        &self,
        request: reqwest::RequestBuilder,
        policy: RetryPolicy,
    ) -> Result<reqwest::Response, HttpError> {
        let mut template = request.build()?;
        apply_async_request_interceptors(&self.config, &mut template)?;
        if !is_idempotent(template.method()) {
            return Err(HttpError::NonIdempotentRetry(template.method().clone()));
        }
        self.url_policy.validate(template.url())?;

        let mut attempt = 1_usize;
        loop {
            let Some(current) = template.try_clone() else {
                return Err(HttpError::UncloneableRetryRequest);
            };
            match self.inner.execute(current).await {
                Ok(mut response)
                    if !response.status().is_client_error()
                        && !response.status().is_server_error() =>
                {
                    apply_response_interceptors(
                        &self.config,
                        response.status(),
                        response.headers_mut(),
                    )?;
                    return Ok(response);
                }
                Ok(mut response) => {
                    apply_response_interceptors(
                        &self.config,
                        response.status(),
                        response.headers_mut(),
                    )?;
                    let retry_after = retry_after(&response);
                    let retryable = is_retryable_status(response.status());
                    let error = response
                        .error_for_status()
                        .expect_err("status is not successful");
                    if !retryable || attempt == policy.max_attempts {
                        return if retryable && policy.max_attempts > 1 {
                            Err(HttpError::RetriesExhausted {
                                attempts: attempt,
                                source: error,
                            })
                        } else {
                            Err(error.into())
                        };
                    }
                    tokio::time::sleep(policy.delay(attempt, retry_after)).await;
                    attempt += 1;
                }
                Err(error) => {
                    let retryable = error.is_connect() || error.is_timeout();
                    if !retryable || attempt == policy.max_attempts {
                        return if retryable && policy.max_attempts > 1 {
                            Err(HttpError::RetriesExhausted {
                                attempts: attempt,
                                source: error,
                            })
                        } else {
                            Err(error.into())
                        };
                    }
                    tokio::time::sleep(policy.delay(attempt, None)).await;
                    attempt += 1;
                }
            }
        }
    }

    /// Streams a response into an asynchronous writer while enforcing the
    /// configured byte ceiling.
    pub async fn download_to<W: AsyncWrite + Unpin>(
        &self,
        request: reqwest::RequestBuilder,
        writer: &mut W,
    ) -> Result<u64, HttpError> {
        let mut response = self.send(request).await?;
        let mut written = 0_usize;
        while let Some(chunk) = response.chunk().await? {
            written = written.saturating_add(chunk.len());
            self.ensure_size(written)?;
            writer.write_all(&chunk).await?;
        }
        writer.flush().await?;
        Ok(written as u64)
    }

    fn ensure_size(&self, actual: usize) -> Result<(), HttpError> {
        if actual > self.max_response_bytes {
            return Err(HttpError::ResponseTooLarge {
                limit: self.max_response_bytes,
                actual,
            });
        }
        Ok(())
    }
}

fn is_idempotent(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS | Method::TRACE
    )
}

fn is_retryable_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::REQUEST_TIMEOUT
            | StatusCode::TOO_MANY_REQUESTS
            | StatusCode::INTERNAL_SERVER_ERROR
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

fn retry_after(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get(header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .parse::<u64>()
        .ok()
        .map(Duration::from_secs)
}

fn apply_async_request_interceptors(
    config: &HttpConfig,
    request: &mut reqwest::Request,
) -> Result<(), HttpError> {
    let mut context = HttpRequestContext::new(
        request.method().clone(),
        request.url().clone(),
        request.headers().clone(),
    );
    config.intercept_request(&mut context)?;
    let (method, url, headers) = context.into_parts();
    *request.method_mut() = method;
    *request.url_mut() = url;
    *request.headers_mut() = headers;
    Ok(())
}

#[cfg(feature = "blocking")]
fn apply_blocking_request_interceptors(
    config: &HttpConfig,
    request: &mut reqwest::blocking::Request,
) -> Result<(), HttpError> {
    let mut context = HttpRequestContext::new(
        request.method().clone(),
        request.url().clone(),
        request.headers().clone(),
    );
    config.intercept_request(&mut context)?;
    let (method, url, headers) = context.into_parts();
    *request.method_mut() = method;
    *request.url_mut() = url;
    *request.headers_mut() = headers;
    Ok(())
}

fn apply_response_interceptors(
    config: &HttpConfig,
    status: StatusCode,
    headers: &mut header::HeaderMap,
) -> Result<(), HttpError> {
    let mut context = HttpResponseContext::new(status, headers.clone());
    config.intercept_response(&mut context)?;
    *headers = context.into_headers();
    Ok(())
}

fn configure_async_builder(
    mut builder: reqwest::ClientBuilder,
    config: &HttpConfig,
) -> Result<reqwest::ClientBuilder, HttpError> {
    if config.disable_cache {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache, no-store"),
        );
        headers.insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
        builder = builder.default_headers(headers);
    }
    if let Some(proxy_url) = &config.proxy_url {
        builder = builder.proxy(
            reqwest::Proxy::all(proxy_url)
                .map_err(|_| HttpConfigError::InvalidProxy(proxy_url.clone()))?,
        );
    }
    builder = builder.danger_accept_invalid_hostnames(matches!(
        config.hostname_verification,
        HostnameVerification::DangerousAcceptInvalid
    ));
    if let Some(identity) = &config.tls_identity {
        builder = builder.identity(identity.clone());
    }
    for certificate in &config.root_certificates {
        builder = builder.add_root_certificate(certificate.clone());
    }
    if let Some(protocol) = config.tls_protocol {
        let version = protocol.reqwest();
        builder = builder.min_tls_version(version).max_tls_version(version);
    }
    Ok(builder)
}

#[cfg(feature = "blocking")]
fn configure_blocking_builder(
    mut builder: reqwest::blocking::ClientBuilder,
    config: &HttpConfig,
) -> Result<reqwest::blocking::ClientBuilder, HttpError> {
    if config.disable_cache {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CACHE_CONTROL,
            header::HeaderValue::from_static("no-cache, no-store"),
        );
        headers.insert(header::PRAGMA, header::HeaderValue::from_static("no-cache"));
        builder = builder.default_headers(headers);
    }
    if let Some(proxy_url) = &config.proxy_url {
        builder = builder.proxy(
            reqwest::Proxy::all(proxy_url)
                .map_err(|_| HttpConfigError::InvalidProxy(proxy_url.clone()))?,
        );
    }
    builder = builder.danger_accept_invalid_hostnames(matches!(
        config.hostname_verification,
        HostnameVerification::DangerousAcceptInvalid
    ));
    if let Some(identity) = &config.tls_identity {
        builder = builder.identity(identity.clone());
    }
    for certificate in &config.root_certificates {
        builder = builder.add_root_certificate(certificate.clone());
    }
    if let Some(protocol) = config.tls_protocol {
        let version = protocol.reqwest();
        builder = builder.min_tls_version(version).max_tls_version(version);
    }
    Ok(builder)
}

/// Builder for [`HttpClient`].
pub struct HttpClientBuilder {
    config: HttpConfig,
    url_policy: Arc<dyn UrlPolicy>,
    cookie_store: bool,
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self {
            config: HttpConfig::default(),
            url_policy: Arc::new(AllowAllUrls),
            cookie_store: false,
        }
    }
}

impl HttpClientBuilder {
    /// Starts from a complete configuration value.
    #[must_use]
    pub fn from_config(config: HttpConfig) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// Sets the connection establishment timeout.
    #[must_use]
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    /// Sets the total request timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Sets the response byte ceiling for buffered and streaming helpers.
    #[must_use]
    pub fn max_response_size(mut self, bytes: usize) -> Self {
        self.config.max_response_bytes = bytes;
        self
    }

    /// Sets the maximum redirect count.
    #[must_use]
    pub fn redirect_limit(mut self, limit: usize) -> Self {
        self.config.redirect_limit = limit;
        self
    }

    /// Sets the User-Agent header.
    #[must_use]
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.config.user_agent = user_agent.into();
        self
    }

    /// Routes requests through an HTTP, HTTPS, or SOCKS proxy.
    pub fn proxy(mut self, proxy_url: impl AsRef<str>) -> Result<Self, HttpError> {
        self.config.set_proxy(proxy_url.as_ref().to_owned())?;
        Ok(self)
    }

    /// Installs an application-specific URL/SSRF policy.
    #[must_use]
    pub fn url_policy<P: UrlPolicy + 'static>(mut self, policy: P) -> Self {
        self.url_policy = Arc::new(policy);
        self
    }

    /// Installs a shared URL/SSRF policy handle.
    #[must_use]
    pub fn url_policy_arc(mut self, policy: Arc<dyn UrlPolicy>) -> Self {
        self.url_policy = policy;
        self
    }

    /// Replaces the full configuration value while preserving policy/cookie flags.
    #[must_use]
    pub fn with_config(mut self, config: HttpConfig) -> Self {
        self.config = config;
        self
    }

    /// Enables reqwest's cookie jar for redirect/cookie parity tests.
    #[must_use]
    pub const fn cookie_store(mut self, enable: bool) -> Self {
        self.cookie_store = enable;
        self
    }

    /// Builds a pooled Rustls client.
    pub fn build(self) -> Result<HttpClient, HttpError> {
        let mut builder = reqwest::Client::builder()
            .connect_timeout(self.config.connect_timeout)
            .timeout(self.config.timeout)
            .redirect(reqwest::redirect::Policy::limited(
                self.config.redirect_limit,
            ))
            .user_agent(&self.config.user_agent)
            .cookie_store(self.cookie_store);
        builder = configure_async_builder(builder, &self.config)?;
        let inner = builder.build()?;
        let max_response_bytes = self.config.max_response_bytes;
        Ok(HttpClient {
            inner,
            max_response_bytes,
            url_policy: self.url_policy,
            config: Arc::new(self.config),
        })
    }
}

/// Blocking HTTP support, available through the `blocking` feature.
#[cfg(feature = "blocking")]
pub mod blocking {
    use super::{
        AllowAllUrls, HttpConfig, HttpError, HttpResponse, UrlPolicy,
        apply_blocking_request_interceptors, apply_response_interceptors,
        configure_blocking_builder,
    };
    use serde::de::DeserializeOwned;
    use std::{fmt, io::Write, sync::Arc};

    /// A pooled blocking HTTP client for synchronous applications.
    #[derive(Clone)]
    pub struct HttpClient {
        inner: reqwest::blocking::Client,
        max_response_bytes: usize,
        url_policy: Arc<dyn UrlPolicy>,
        config: Arc<HttpConfig>,
    }

    impl fmt::Debug for HttpClient {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter
                .debug_struct("blocking::HttpClient")
                .field("max_response_bytes", &self.max_response_bytes)
                .field("url_policy", &"dyn UrlPolicy")
                .field("config", &self.config)
                .finish_non_exhaustive()
        }
    }

    impl HttpClient {
        /// Builds a blocking client from the shared configuration model.
        pub fn new(config: &HttpConfig) -> Result<Self, HttpError> {
            Self::new_with_policy(config, AllowAllUrls)
        }

        /// Builds a blocking client with an application-specific URL policy.
        pub fn new_with_policy<P: UrlPolicy + 'static>(
            config: &HttpConfig,
            url_policy: P,
        ) -> Result<Self, HttpError> {
            Self::build(config, Arc::new(url_policy))
        }

        fn build(config: &HttpConfig, url_policy: Arc<dyn UrlPolicy>) -> Result<Self, HttpError> {
            let builder = reqwest::blocking::Client::builder()
                .connect_timeout(config.connect_timeout)
                .timeout(config.timeout)
                .redirect(reqwest::redirect::Policy::limited(config.redirect_limit))
                .user_agent(&config.user_agent);
            let inner = configure_blocking_builder(builder, config)?.build()?;
            Ok(Self {
                inner,
                max_response_bytes: config.max_response_bytes,
                url_policy,
                config: Arc::new(config.clone()),
            })
        }

        /// Sends a prepared request after applying URL and status policies.
        pub fn send(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<reqwest::blocking::Response, HttpError> {
            Ok(self.execute(request)?.error_for_status()?)
        }

        /// Sends a GET request and returns a bounded, repeatable response facade.
        pub fn get_response(&self, url: impl reqwest::IntoUrl) -> Result<HttpResponse, HttpError> {
            self.send_response(self.inner.get(url))
        }

        /// Sends a prepared request and buffers non-success responses without
        /// discarding their status, headers, or body.
        pub fn send_response(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<HttpResponse, HttpError> {
            let response = self.execute(request)?;
            self.buffer_response(response)
        }

        fn execute(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<reqwest::blocking::Response, HttpError> {
            let mut request = request.build()?;
            apply_blocking_request_interceptors(&self.config, &mut request)?;
            self.url_policy.validate(request.url())?;
            let mut response = self.inner.execute(request)?;
            apply_response_interceptors(&self.config, response.status(), response.headers_mut())?;
            Ok(response)
        }

        fn buffer_response(
            &self,
            mut response: reqwest::blocking::Response,
        ) -> Result<HttpResponse, HttpError> {
            let status = response.status();
            let headers = response.headers().clone();
            let url = response.url().to_string();
            let mut body = Vec::new();
            let mut buffer = [0_u8; 16 * 1024];
            loop {
                let read = std::io::Read::read(&mut response, &mut buffer)?;
                if read == 0 {
                    break;
                }
                self.ensure_size(body.len().saturating_add(read))?;
                body.extend_from_slice(&buffer[..read]);
            }
            Ok(HttpResponse::new(status, headers, body).with_url(url))
        }

        fn send_response_checked(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<HttpResponse, HttpError> {
            let response = self.execute(request)?.error_for_status()?;
            self.buffer_response(response)
        }

        /// Sends a GET request and decodes a bounded response body.
        pub fn get_text(&self, url: impl reqwest::IntoUrl) -> Result<String, HttpError> {
            self.get_text_request(self.inner.get(url))
        }

        /// Sends a GET request and decodes a bounded JSON body.
        pub fn get_json<T: DeserializeOwned>(
            &self,
            url: impl reqwest::IntoUrl,
        ) -> Result<T, HttpError> {
            self.send_json(self.inner.get(url))
        }

        /// Sends a prepared request and decodes a bounded JSON response.
        pub fn send_json<T: DeserializeOwned>(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<T, HttpError> {
            let response = self.send_response_checked(request)?;
            Ok(serde_json::from_slice(response.body_bytes())?)
        }

        fn get_text_request(
            &self,
            request: reqwest::blocking::RequestBuilder,
        ) -> Result<String, HttpError> {
            Ok(self.send_response_checked(request)?.body())
        }

        /// Streams a response into a writer while enforcing the byte ceiling.
        pub fn download_to<W: Write>(
            &self,
            request: reqwest::blocking::RequestBuilder,
            writer: &mut W,
        ) -> Result<u64, HttpError> {
            let mut response = self.send(request)?;
            let mut written = 0_usize;
            let mut buffer = [0_u8; 16 * 1024];
            loop {
                let read = std::io::Read::read(&mut response, &mut buffer)?;
                if read == 0 {
                    break;
                }
                written = written.saturating_add(read);
                self.ensure_size(written)?;
                writer.write_all(&buffer[..read])?;
            }
            writer.flush()?;
            Ok(written as u64)
        }

        fn ensure_size(&self, actual: usize) -> Result<(), HttpError> {
            if actual > self.max_response_bytes {
                return Err(HttpError::ResponseTooLarge {
                    limit: self.max_response_bytes,
                    actual,
                });
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_defaults_build_without_native_tls() {
        let client = HttpClient::new(&HttpConfig::default()).unwrap();
        assert!(client.ensure_size(8 * 1024 * 1024).is_ok());
        assert!(client.ensure_size(8 * 1024 * 1024 + 1).is_err());
    }

    #[test]
    fn local_target_policy_rejects_literal_private_destinations() {
        let policy = DenyLocalTargets;
        assert_eq!(
            policy.validate(&Url::parse("http://127.0.0.1/admin").unwrap()),
            Err(UrlPolicyError::DeniedTarget)
        );
        assert_eq!(
            policy.validate(&Url::parse("http://10.0.0.1/admin").unwrap()),
            Err(UrlPolicyError::DeniedTarget)
        );
        assert!(
            policy
                .validate(&Url::parse("https://example.com").unwrap())
                .is_ok()
        );
    }

    #[test]
    fn documented_builder_surface_is_available() {
        let client = HttpClient::builder()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(10))
            .max_response_size(1_024)
            .redirect_limit(2)
            .url_policy(DenyLocalTargets)
            .build()
            .unwrap();
        assert_eq!(client.max_response_bytes, 1_024);
    }
}

#[cfg(test)]
mod coverage_tests;
