//! 对齐: `cn.hutool.http.HttpRequest`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpRequest.java
//! 中文说明: HTTP请求构建器，提供链式API设置方法、头部、超时和请求体

use crate::body::ResourceBody;
use crate::http_util::HttpUtil;
use crate::{
    ContentType, DenyLocalTargets, HttpClient, HttpConfig, HttpCookie, HttpError, HttpResponse,
    Method, UrlPolicy,
};
use indexmap::IndexMap;
use reqwest::header::{
    AUTHORIZATION, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, COOKIE, HeaderMap, HeaderName,
    HeaderValue,
};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

/// Hutool-style HTTP request builder that delegates to [`HttpClient`].
///
/// Java: `cn.hutool.http.HttpRequest`
#[derive(Clone)]
pub struct HttpRequest {
    method: Method,
    url: String,
    headers: HeaderMap,
    form: IndexMap<String, String>,
    body_text: Option<String>,
    body_bytes: Option<Vec<u8>>,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    follow_redirects: Option<bool>,
    max_redirect_count: Option<usize>,
    max_response_bytes: Option<usize>,
    url_policy: Arc<dyn UrlPolicy>,
    cookie_store: bool,
    disable_cache: bool,
    follow_redirects_cookie: Option<bool>,
    is_rest: bool,
    proxy_url: Option<String>,
    http_config: Option<HttpConfig>,
    keep_alive: Option<bool>,
    fixed_content_length: Option<i64>,
    chunked_block_size: Option<i32>,
    /// Multipart file parts: field name → (path, optional override file name).
    file_form: IndexMap<String, (PathBuf, Option<String>)>,
}

impl std::fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method)
            .field("url", &self.url)
            .field("timeout", &self.timeout)
            .field("follow_redirects", &self.follow_redirects)
            .field("form_len", &self.form.len())
            .field(
                "has_body",
                &(self.body_text.is_some() || self.body_bytes.is_some()),
            )
            .finish_non_exhaustive()
    }
}

impl HttpRequest {
    /// Creates a request for `url` with method `GET`.
    ///
    /// Java: `HttpRequest.of(String url)` / `HttpRequest(String url)`
    #[must_use]
    pub fn of(url: impl Into<String>) -> Self {
        Self::new(Method::GET, url)
    }

    /// Creates a request with an explicit method.
    ///
    /// Java: `HttpRequest.of(String url).method(Method)`
    #[must_use]
    pub fn new(method: Method, url: impl Into<String>) -> Self {
        Self {
            method,
            url: url.into(),
            headers: HeaderMap::new(),
            form: IndexMap::new(),
            body_text: None,
            body_bytes: None,
            timeout: None,
            connect_timeout: None,
            follow_redirects: None,
            max_redirect_count: None,
            max_response_bytes: None,
            url_policy: Arc::new(DenyLocalTargets),
            cookie_store: false,
            disable_cache: false,
            follow_redirects_cookie: None,
            is_rest: false,
            proxy_url: None,
            http_config: None,
            keep_alive: None,
            fixed_content_length: None,
            chunked_block_size: None,
            file_form: IndexMap::new(),
        }
    }

    /// Creates a request with an explicit charset tag (URL strings remain UTF-8).
    ///
    /// Java: `HttpRequest.of(String url, Charset charset)`
    #[must_use]
    pub fn of_charset(url: impl Into<String>, _charset: &str) -> Self {
        Self::of(url)
    }

    /// Java: `HttpRequest.get(String url)`
    #[must_use]
    pub fn get(url: impl Into<String>) -> Self {
        Self::new(Method::GET, url)
    }

    /// Java: `HttpRequest.post(String url)`
    #[must_use]
    pub fn post(url: impl Into<String>) -> Self {
        Self::new(Method::POST, url)
    }

    /// Java: `HttpRequest.put(String url)`
    #[must_use]
    pub fn put(url: impl Into<String>) -> Self {
        Self::new(Method::PUT, url)
    }

    /// Java: `HttpRequest.delete(String url)`
    #[must_use]
    pub fn delete(url: impl Into<String>) -> Self {
        Self::new(Method::DELETE, url)
    }

    /// Java: `HttpRequest.head(String url)`
    #[must_use]
    pub fn head(url: impl Into<String>) -> Self {
        Self::new(Method::HEAD, url)
    }

    /// Java: `HttpRequest.options(String url)`
    #[must_use]
    pub fn options(url: impl Into<String>) -> Self {
        Self::new(Method::OPTIONS, url)
    }

    /// Java: `HttpRequest.patch(String url)`
    #[must_use]
    pub fn patch(url: impl Into<String>) -> Self {
        Self::new(Method::PATCH, url)
    }

    /// Java: `HttpRequest.trace(String url)`
    #[must_use]
    pub fn trace(url: impl Into<String>) -> Self {
        Self::new(Method::TRACE, url)
    }

    /// Java: `HttpRequest.getUrl()`
    #[must_use]
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Java: `HttpRequest.setUrl(String url)`
    #[must_use]
    pub fn set_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Java: `HttpRequest.getMethod()`
    #[must_use]
    pub fn get_method(&self) -> &Method {
        &self.method
    }

    /// Java: `HttpRequest.setMethod(Method)` / `method(Method)`
    #[must_use]
    pub fn method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    /// Alias for [`Self::method`].
    ///
    /// Java: `HttpRequest.setMethod(Method method)`
    #[must_use]
    pub fn set_method(self, method: Method) -> Self {
        self.method(method)
    }

    /// Sets both connect and total timeouts in milliseconds (`< 0` keeps defaults).
    ///
    /// Java: `HttpRequest.timeout(int milliseconds)`
    #[must_use]
    pub fn timeout(mut self, milliseconds: i64) -> Self {
        if milliseconds >= 0 {
            let duration = Duration::from_millis(milliseconds as u64);
            self.timeout = Some(duration);
            self.connect_timeout = Some(duration);
        }
        self
    }

    /// Java: `HttpRequest.setConnectionTimeout(int milliseconds)`
    #[must_use]
    pub fn set_connection_timeout(mut self, milliseconds: i64) -> Self {
        if milliseconds >= 0 {
            self.connect_timeout = Some(Duration::from_millis(milliseconds as u64));
        }
        self
    }

    /// Java: `HttpRequest.setReadTimeout(int milliseconds)`
    #[must_use]
    pub fn set_read_timeout(mut self, milliseconds: i64) -> Self {
        if milliseconds >= 0 {
            self.timeout = Some(Duration::from_millis(milliseconds as u64));
        }
        self
    }

    /// Java: `HttpRequest.setFollowRedirects(boolean isFollowRedirects)`
    #[must_use]
    pub fn set_follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = Some(follow);
        self
    }

    /// Java: `HttpRequest.contentType(String contentType)`
    #[must_use]
    pub fn content_type(mut self, content_type: impl AsRef<str>) -> Self {
        if let Ok(value) = HeaderValue::from_str(content_type.as_ref()) {
            self.headers.insert(CONTENT_TYPE, value);
        }
        self
    }

    /// Sets a request header (last write wins for the same name).
    ///
    /// Java: `HttpBase.header(String name, String value)` via `HttpRequest`
    #[must_use]
    pub fn header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_bytes(name.as_ref().as_bytes()),
            HeaderValue::from_str(value.as_ref()),
        ) {
            self.headers.insert(name, value);
        }
        self
    }

    /// Java: `HttpRequest.form(String name, Object value)`
    #[must_use]
    pub fn form_pair(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.form.insert(name.into(), value.into());
        self
    }

    /// Java: `HttpRequest.form(Map<String, Object> formMap)`
    #[must_use]
    pub fn form(mut self, form: IndexMap<String, String>) -> Self {
        for (key, value) in form {
            self.form.insert(key, value);
        }
        self
    }

    /// Java: `HttpRequest.body(String body)`
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        let text = body.into();
        if !self.headers.contains_key(CONTENT_TYPE) {
            if let Some(detected) = ContentType::detect(&text) {
                self = self.content_type(detected.value());
            }
        }
        self.body_text = Some(text);
        self.body_bytes = None;
        self
    }

    /// Java: `HttpRequest.body(String body, String contentType)`
    #[must_use]
    pub fn body_with_type(self, body: impl Into<String>, content_type: impl AsRef<str>) -> Self {
        self.content_type(content_type).body(body)
    }

    /// Java: `HttpRequest.body(byte[] bodyBytes)`
    #[must_use]
    pub fn body_bytes(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body_bytes = Some(body.into());
        self.body_text = None;
        self
    }

    /// Java: `HttpRequest.auth(String content)`
    #[must_use]
    pub fn auth(mut self, content: impl AsRef<str>) -> Self {
        if let Ok(value) = HeaderValue::from_str(content.as_ref()) {
            self.headers.insert(AUTHORIZATION, value);
        }
        self
    }

    /// Java: `HttpRequest.basicAuth(String username, String password)`
    #[must_use]
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        self.auth(HttpUtil::build_basic_auth(username, password))
    }

    /// Java: `HttpRequest.bearerAuth(String token)`
    #[must_use]
    pub fn bearer_auth(self, token: impl AsRef<str>) -> Self {
        self.auth(format!("Bearer {}", token.as_ref()))
    }

    /// Java: `HttpRequest.cookie(String cookie)`
    #[must_use]
    pub fn cookie(mut self, cookie: impl AsRef<str>) -> Self {
        if let Ok(value) = HeaderValue::from_str(cookie.as_ref()) {
            self.headers.insert(COOKIE, value);
        }
        self
    }

    /// Java: `HttpRequest.cookie(Collection<HttpCookie>)` / `cookie(HttpCookie...)`
    #[must_use]
    pub fn cookies(self, cookies: impl IntoIterator<Item = HttpCookie>) -> Self {
        let header = cookies
            .into_iter()
            .map(|c| format!("{}={}", c.name(), c.value()))
            .collect::<Vec<_>>()
            .join("; ");
        if header.is_empty() {
            self
        } else {
            self.cookie(header)
        }
    }

    /// Disables the request cookie jar and clears a previously set Cookie header.
    ///
    /// Java: `HttpRequest.disableCookie()`
    #[must_use]
    pub fn disable_cookie(mut self) -> Self {
        self.cookie_store = false;
        self.headers.remove(COOKIE);
        self
    }

    /// Java: `HttpRequest.formStr(Map<String, String>)`
    #[must_use]
    pub fn form_str(self, form: IndexMap<String, String>) -> Self {
        self.form(form)
    }

    /// Adds a filesystem file as a multipart form part.
    ///
    /// Java: `HttpRequest.form(String name, File file)` / `form(String name, File file, String fileName)`
    #[must_use]
    pub fn form_file(
        mut self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
        file_name: Option<String>,
    ) -> Self {
        self.file_form
            .insert(name.into(), (path.as_ref().to_path_buf(), file_name));
        self
    }

    /// Adds multiple files under the same form field name.
    ///
    /// Java: `HttpRequest.form(String name, File... files)`
    #[must_use]
    pub fn form_files(mut self, name: impl Into<String>, paths: &[&Path]) -> Self {
        let name = name.into();
        for (idx, path) in paths.iter().enumerate() {
            let field = if idx == 0 {
                name.clone()
            } else {
                format!("{name}_{idx}")
            };
            self = self.form_file(field, path, None);
        }
        self
    }

    /// Sets the body from a [`ResourceBody`] (Hutool `Resource` stand-in).
    ///
    /// Java: `HttpRequest.body(Resource)` / `form(String, Resource)` via body bytes.
    #[must_use]
    pub fn body_resource(mut self, resource: &ResourceBody) -> Self {
        if let Some(ct) = resource.content_type() {
            self = self.content_type(ct);
        }
        self.body_bytes(resource.bytes().to_vec())
    }

    /// Returns a snapshot of registered multipart file parts (path as string).
    ///
    /// Java: `HttpRequest.fileForm()` — Rust returns path strings instead of Java `Resource`.
    #[must_use]
    pub fn file_form(&self) -> IndexMap<String, String> {
        self.file_form
            .iter()
            .map(|(k, (path, _))| (k.clone(), path.display().to_string()))
            .collect()
    }

    /// Sets the `Content-Length` header explicitly.
    ///
    /// Java: `HttpRequest.contentLength(int value)` / `setFixedContentLength(long)`
    #[must_use]
    pub fn content_length(mut self, value: i64) -> Self {
        self.fixed_content_length = Some(value);
        if value >= 0 {
            if let Ok(header) = HeaderValue::from_str(&value.to_string()) {
                self.headers.insert(CONTENT_LENGTH, header);
            }
        }
        self
    }

    /// Returns the configured Content-Length header value, if any.
    ///
    /// Java: `HttpRequest.contentLength()` (String)
    #[must_use]
    pub fn content_length_str(&self) -> Option<String> {
        self.headers
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned)
            .or_else(|| self.fixed_content_length.map(|v| v.to_string()))
    }

    /// Alias for [`Self::content_length`] matching Hutool `setFixedContentLength`.
    ///
    /// Java: `HttpRequest.setFixedContentLength(long contentLength)`
    #[must_use]
    pub fn set_fixed_content_length(self, content_length: i64) -> Self {
        self.content_length(content_length)
    }

    /// Sets the `Connection` keep-alive / close header.
    ///
    /// Java: `HttpRequest.keepAlive(boolean)`
    #[must_use]
    pub fn keep_alive(mut self, enabled: bool) -> Self {
        self.keep_alive = Some(enabled);
        let header = if enabled {
            HeaderValue::from_static("keep-alive")
        } else {
            HeaderValue::from_static("close")
        };
        self.headers.insert(CONNECTION, header);
        self
    }

    /// Java: `HttpRequest.isKeepAlive()`
    #[must_use]
    pub fn is_keep_alive(&self) -> bool {
        self.keep_alive.unwrap_or(true)
    }

    /// Records a preferred chunk size (informational for Hutool callers).
    ///
    /// Java: `HttpRequest.setChunkedStreamingMode(int blockSize)` — reqwest streams by default;
    /// the size is retained for parity.
    #[must_use]
    pub fn set_chunked_streaming_mode(mut self, block_size: i32) -> Self {
        self.chunked_block_size = Some(block_size);
        self
    }

    /// Returns the configured chunked block size, if any.
    #[must_use]
    pub fn chunked_block_size(&self) -> Option<i32> {
        self.chunked_block_size
    }

    /// Attaches a request interceptor via the underlying [`HttpConfig`].
    ///
    /// Java: `HttpRequest.addRequestInterceptor` / `addInterceptor`
    #[must_use]
    pub fn add_request_interceptor<F>(mut self, interceptor: F) -> Self
    where
        F: Fn(&mut crate::HttpRequestContext) -> Result<(), crate::HttpInterceptorError>
            + Send
            + Sync
            + 'static,
    {
        let mut config = self.http_config.take().unwrap_or_default();
        config.add_request_interceptor(interceptor);
        self.http_config = Some(config);
        self
    }

    /// Attaches a response interceptor via the underlying [`HttpConfig`].
    ///
    /// Java: `HttpRequest.addResponseInterceptor`
    #[must_use]
    pub fn add_response_interceptor<F>(mut self, interceptor: F) -> Self
    where
        F: Fn(&mut crate::HttpResponseContext) -> Result<(), crate::HttpInterceptorError>
            + Send
            + Sync
            + 'static,
    {
        let mut config = self.http_config.take().unwrap_or_default();
        config.add_response_interceptor(interceptor);
        self.http_config = Some(config);
        self
    }

    /// Enables the reqwest cookie jar (Hutool default-cookie family, per-request).
    ///
    /// Java: `HttpRequest.enableDefaultCookie()` (idiomatic: request-scoped jar)
    #[must_use]
    pub fn enable_default_cookie(mut self) -> Self {
        self.cookie_store = true;
        self
    }

    /// Applies a shared [`HttpConfig`] overlay when building the client.
    ///
    /// Java: `HttpRequest.setConfig(HttpConfig config)`
    #[must_use]
    pub fn set_config(mut self, config: HttpConfig) -> Self {
        self.http_config = Some(config);
        self
    }

    /// Disables HTTP caches via request Cache-Control headers on the client.
    ///
    /// Java: `HttpRequest.disableCache()`
    #[must_use]
    pub fn disable_cache(mut self) -> Self {
        self.disable_cache = true;
        self
    }

    /// Sets the maximum redirect count for this request.
    ///
    /// Java: `HttpRequest.setMaxRedirectCount(int maxRedirectCount)`
    #[must_use]
    pub fn set_max_redirect_count(mut self, count: i32) -> Self {
        self.max_redirect_count = Some(usize::try_from(count).unwrap_or_default());
        self
    }

    /// Controls whether cookies follow redirects (reqwest cookie jar when enabled).
    ///
    /// Java: `HttpRequest.setFollowRedirectsCookie(boolean followRedirectsCookie)`
    #[must_use]
    pub fn set_follow_redirects_cookie(mut self, follow: bool) -> Self {
        self.follow_redirects_cookie = Some(follow);
        if follow {
            self.cookie_store = true;
        }
        self
    }

    /// Marks the request as REST-oriented (informational flag for callers).
    ///
    /// Java: `HttpRequest.setRest(boolean isRest)`
    #[must_use]
    pub fn set_rest(mut self, is_rest: bool) -> Self {
        self.is_rest = is_rest;
        self
    }

    /// Configures an HTTP proxy from host and port.
    ///
    /// Java: `HttpRequest.setHttpProxy(String host, int port)`
    pub fn set_http_proxy(mut self, host: &str, port: u16) -> Result<Self, HttpError> {
        self.proxy_url = Some(format!("http://{}:{port}", host.trim()));
        Ok(self)
    }

    /// Configures a proxy URL (http/https/socks).
    ///
    /// Java: `HttpRequest.setProxy(Proxy proxy)` — Rust accepts a proxy URL string.
    pub fn set_proxy(mut self, proxy_url: impl Into<String>) -> Result<Self, HttpError> {
        let proxy_url = proxy_url.into();
        reqwest::Proxy::all(&proxy_url).map_err(HttpError::from)?;
        self.proxy_url = Some(proxy_url);
        Ok(self)
    }

    /// Selects TLS 1.2 / 1.3 for the request client.
    ///
    /// Java: `HttpRequest.setSSLProtocol(String protocol)`
    pub fn set_ssl_protocol(mut self, protocol: &str) -> Result<Self, HttpError> {
        let mut config = self.http_config.take().unwrap_or_default();
        config.set_ssl_protocol(protocol)?;
        self.http_config = Some(config);
        Ok(self)
    }

    /// Sets Dangerous/Strict hostname verification (Hutool HostnameVerifier stand-in).
    ///
    /// Java: `HttpRequest.setHostnameVerifier(HostnameVerifier hostnameVerifier)`
    #[must_use]
    pub fn set_hostname_verifier(mut self, verification: crate::HostnameVerification) -> Self {
        let mut config = self.http_config.take().unwrap_or_default();
        config.set_hostname_verifier(verification);
        self.http_config = Some(config);
        self
    }

    /// Sets proxy Authorization using Basic credentials.
    ///
    /// Java: `HttpRequest.basicProxyAuth(String username, String password)`
    #[must_use]
    pub fn basic_proxy_auth(self, username: &str, password: &str) -> Self {
        self.proxy_auth(HttpUtil::build_basic_auth(username, password))
    }

    /// Sets the raw `Proxy-Authorization` header content.
    ///
    /// Java: `HttpRequest.proxyAuth(String content)`
    #[must_use]
    pub fn proxy_auth(mut self, content: impl AsRef<str>) -> Self {
        if let Ok(value) = HeaderValue::from_str(content.as_ref()) {
            self.headers
                .insert(HeaderName::from_static("proxy-authorization"), value);
        }
        self
    }

    /// Async execute alias matching Hutool's non-blocking naming.
    ///
    /// Java: `HttpRequest.executeAsync()`
    pub async fn execute_async(self) -> Result<HttpResponse, HttpError> {
        self.execute().await
    }

    /// Executes then invokes `consumer` with the response.
    ///
    /// Java: `HttpRequest.then(Consumer<HttpResponse> consumer)`
    pub async fn then<F>(self, consumer: F) -> Result<(), HttpError>
    where
        F: FnOnce(HttpResponse),
    {
        let response = self.execute().await?;
        consumer(response);
        Ok(())
    }

    /// Executes then maps the response through `function`.
    ///
    /// Java: `HttpRequest.thenFunction(Function<HttpResponse, T> function)`
    pub async fn then_function<T, F>(self, function: F) -> Result<T, HttpError>
    where
        F: FnOnce(HttpResponse) -> T,
    {
        Ok(function(self.execute().await?))
    }

    /// Overrides the default [`DenyLocalTargets`] policy (tests / trusted nets).
    #[must_use]
    pub fn url_policy<P: UrlPolicy + 'static>(mut self, policy: P) -> Self {
        self.url_policy = Arc::new(policy);
        self
    }

    /// Overrides the URL policy using a shared handle.
    #[must_use]
    pub fn url_policy_arc(mut self, policy: Arc<dyn UrlPolicy>) -> Self {
        self.url_policy = policy;
        self
    }

    /// Caps buffered response bytes for this request.
    #[must_use]
    pub fn max_response_bytes(mut self, bytes: usize) -> Self {
        self.max_response_bytes = Some(bytes);
        self
    }

    /// Executes the request and returns a bounded [`HttpResponse`].
    ///
    /// Java: `HttpRequest.execute()` — async because Rust networking is async-first.
    pub async fn execute(self) -> Result<HttpResponse, HttpError> {
        let client = self.build_client()?;
        let request = self.build_reqwest(&client)?;
        client.send_response(request).await
    }

    /// Executes and returns the response body text when status is success.
    pub async fn execute_body(self) -> Result<String, HttpError> {
        let response = self.execute().await?;
        ensure_success(&response)?;
        Ok(response.body().to_owned())
    }

    /// Executes and returns raw response bytes when status is success.
    pub async fn execute_bytes(self) -> Result<Vec<u8>, HttpError> {
        let response = self.execute().await?;
        ensure_success(&response)?;
        Ok(response.body_bytes().to_vec())
    }

    fn build_client(&self) -> Result<HttpClient, HttpError> {
        let mut config = self.http_config.clone().unwrap_or_default();
        if let Some(timeout) = self.timeout {
            config.timeout = timeout;
        }
        if let Some(connect) = self.connect_timeout {
            config.connect_timeout = connect;
        }
        if let Some(count) = self.max_redirect_count {
            config.redirect_limit = count;
        } else if let Some(follow) = self.follow_redirects {
            config.redirect_limit = if follow { 10 } else { 0 };
        }
        if let Some(max) = self.max_response_bytes {
            config.max_response_bytes = max;
        }
        if self.disable_cache {
            config.disable_cache = true;
        }
        if let Some(follow) = self.follow_redirects_cookie {
            config.follow_redirects_cookie = follow;
        }
        if let Some(proxy) = &self.proxy_url {
            config.set_proxy(proxy.clone())?;
        }
        let _ = self.is_rest; // retained for Hutool REST flag parity
        HttpClient::builder()
            .with_config(config)
            .url_policy_arc(Arc::clone(&self.url_policy))
            .cookie_store(self.cookie_store)
            .build()
    }

    fn build_reqwest(&self, client: &HttpClient) -> Result<reqwest::RequestBuilder, HttpError> {
        let url = self.resolve_url();
        let mut builder = client.request(self.method.clone(), &url);
        for (name, value) in self.headers.iter() {
            builder = builder.header(name.clone(), value.clone());
        }
        if !self.file_form.is_empty() {
            let mut form = reqwest::multipart::Form::new();
            for (key, value) in &self.form {
                form = form.text(key.clone(), value.clone());
            }
            for (name, (path, file_name)) in &self.file_form {
                let bytes = std::fs::read(path)?;
                let file_name = file_name
                    .clone()
                    .or_else(|| path.file_name().map(|n| n.to_string_lossy().into_owned()))
                    .unwrap_or_else(|| "file".to_string());
                let part = reqwest::multipart::Part::bytes(bytes).file_name(file_name);
                form = form.part(name.clone(), part);
            }
            builder = builder.multipart(form);
        } else if let Some(bytes) = &self.body_bytes {
            builder = builder.body(bytes.clone());
        } else if let Some(text) = &self.body_text {
            builder = builder.body(text.clone());
        } else if !self.form.is_empty() && self.method != Method::GET && self.method != Method::HEAD
        {
            let encoded = HttpUtil::to_params_form(&self.form, true);
            builder = builder
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(encoded);
        }
        Ok(builder)
    }

    fn resolve_url(&self) -> String {
        if self.form.is_empty() {
            return self.url.clone();
        }
        if self.method == Method::GET || self.method == Method::HEAD {
            return HttpUtil::url_with_form(&self.url, &self.form, true);
        }
        self.url.clone()
    }
}

impl std::fmt::Display for HttpRequest {
    /// Java: `HttpRequest.toString()`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpRequest{{method={:?}, url={}, form={}, rest={}}}",
            self.method,
            self.url,
            self.form.len(),
            self.is_rest
        )
    }
}

fn ensure_success(response: &HttpResponse) -> Result<(), HttpError> {
    if response.status().is_success() {
        return Ok(());
    }
    Err(HttpError::UnexpectedStatus {
        status: response.status(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AllowAllUrls;

    #[test]
    fn factories_set_method_and_url() {
        assert_eq!(
            HttpRequest::get("https://example.com").get_method(),
            &Method::GET
        );
        assert_eq!(
            HttpRequest::post("https://example.com").get_method(),
            &Method::POST
        );
        assert_eq!(
            HttpRequest::of("https://example.com/a").get_url(),
            "https://example.com/a"
        );
    }

    #[test]
    fn timeout_negative_keeps_default() {
        let req = HttpRequest::get("https://example.com").timeout(-1);
        assert!(req.timeout.is_none());
    }

    #[test]
    fn basic_auth_header_is_set() {
        let req = HttpRequest::get("https://example.com").basic_auth("user", "pass");
        let value = req.headers.get(AUTHORIZATION).unwrap().to_str().unwrap();
        assert!(value.starts_with("Basic "));
    }

    #[test]
    fn get_form_appends_query() {
        let mut form = IndexMap::new();
        form.insert("a".into(), "1".into());
        let req = HttpRequest::get("https://example.com/path").form(form);
        assert_eq!(req.resolve_url(), "https://example.com/path?a=1");
    }

    #[test]
    fn request_config_and_callback_helpers() {
        let mut config = HttpConfig::create();
        config.set_block_size(4096);
        config.set_ignore_eof_error(true);
        config.set_decode_url(true);
        config.set_interceptor_on_redirect(true);
        config.set_follow_redirects_cookie(false);
        config.set_use_default_content_type_if_null(false);
        config.set_ignore_content_length(true);
        assert_eq!(config.block_size, 4096);
        assert!(config.ignore_eof_error);

        let req = HttpRequest::get("https://example.com")
            .set_config(config)
            .disable_cache()
            .set_max_redirect_count(2)
            .set_rest(true)
            .set_follow_redirects_cookie(true)
            .basic_proxy_auth("u", "p");
        assert!(req.disable_cache);
        assert!(req.is_rest);
        assert!(format!("{req}").contains("HttpRequest"));
        assert!(req.headers.contains_key("proxy-authorization"));
    }

    #[test]
    fn cookie_keepalive_content_length_and_form_helpers() {
        let req = HttpRequest::of_charset("https://example.com", "UTF-8")
            .cookies([HttpCookie::new("a", "1"), HttpCookie::new("b", "2")])
            .keep_alive(false)
            .content_length(12)
            .set_chunked_streaming_mode(8192)
            .form_str({
                let mut m = IndexMap::new();
                m.insert("k".into(), "v".into());
                m
            })
            .disable_cookie()
            .add_request_interceptor(|_| Ok(()))
            .add_response_interceptor(|_| Ok(()));
        assert!(!req.is_keep_alive());
        assert_eq!(req.content_length_str().as_deref(), Some("12"));
        assert_eq!(req.chunked_block_size(), Some(8192));
        assert!(!req.headers.contains_key(COOKIE));
        assert!(!req.cookie_store);
        assert_eq!(req.form.get("k").map(String::as_str), Some("v"));
        assert!(req.file_form().is_empty());
    }

    #[tokio::test]
    async fn deny_local_targets_by_default() {
        let err = HttpRequest::get("http://127.0.0.1:9/")
            .timeout(100)
            .execute()
            .await
            .unwrap_err();
        assert!(matches!(err, HttpError::UrlPolicy(_)));
    }

    #[tokio::test]
    async fn allow_all_can_reach_local_mock() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move {
            let (mut sock, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 1024];
            let _ = sock.read(&mut buf).await;
            let body = b"ok";
            let resp = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\nok",
                body.len()
            );
            let _ = sock.write_all(resp.as_bytes()).await;
        });
        let text = HttpRequest::get(format!("http://{addr}/"))
            .url_policy(AllowAllUrls)
            .timeout(5_000)
            .execute()
            .await
            .unwrap()
            .body()
            .to_owned();
        assert_eq!(text, "ok");
        task.await.unwrap();
    }
}
