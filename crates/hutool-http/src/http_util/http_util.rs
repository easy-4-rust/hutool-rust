//! 对齐: `cn.hutool.http.HttpUtil`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpUtil.java
//! 中文说明: HTTP工具类，提供URL编码、参数拼接、下载等静态便捷方法

use crate::progress::{NoopStreamProgress, StreamProgress};
use crate::request::HttpRequest;
use crate::{ContentType, HttpError, Method, UrlPolicy};
use crate::query::{normalize_params, split_url_params, QueryMap};
use encoding_rs::Encoding;
use hutool_core::base64_encode;
use indexmap::IndexMap;
use std::io::Write;
use std::path::Path as FsPath;
use std::sync::Arc;

/// Static helpers matching Hutool `HttpUtil`.
pub struct HttpUtil;

impl HttpUtil {
    /// Returns whether `url` starts with `http:` (case-insensitive).
    ///
    /// Java: `HttpUtil.isHttp(String url)`
    pub fn is_http(url: &str) -> bool {
        url.len() >= 5 && url[..5].eq_ignore_ascii_case("http:")
    }

    /// Returns whether `url` starts with `https:` (case-insensitive).
    ///
    /// Java: `HttpUtil.isHttps(String url)`
    pub fn is_https(url: &str) -> bool {
        url.len() >= 6 && url[..6].eq_ignore_ascii_case("https:")
    }

    /// Creates an [`HttpRequest`] with the given method.
    ///
    /// Java: `HttpUtil.createRequest(Method method, String url)`
    #[must_use]
    pub fn create_request(method: Method, url: impl Into<String>) -> HttpRequest {
        HttpRequest::new(method, url)
    }

    /// Java: `HttpUtil.createGet(String url)`
    #[must_use]
    pub fn create_get(url: impl Into<String>) -> HttpRequest {
        HttpRequest::get(url)
    }

    /// Java: `HttpUtil.createGet(String url, boolean isFollowRedirects)`
    #[must_use]
    pub fn create_get_with_redirects(url: impl Into<String>, follow_redirects: bool) -> HttpRequest {
        HttpRequest::get(url).set_follow_redirects(follow_redirects)
    }

    /// Java: `HttpUtil.createPost(String url)`
    #[must_use]
    pub fn create_post(url: impl Into<String>) -> HttpRequest {
        HttpRequest::post(url)
    }

    /// Sends GET and returns the response body text.
    ///
    /// Java: `HttpUtil.get(String urlString)`
    pub async fn get(url: &str) -> Result<String, HttpError> {
        Self::create_get(url).execute_body().await
    }

    /// Java: `HttpUtil.get(String urlString, int timeout)`
    pub async fn get_timeout(url: &str, timeout_ms: i64) -> Result<String, HttpError> {
        Self::create_get(url).timeout(timeout_ms).execute_body().await
    }

    /// Java: `HttpUtil.get(String urlString, Map paramMap)`
    pub async fn get_with_form(
        url: &str,
        form: &IndexMap<String, String>,
    ) -> Result<String, HttpError> {
        Self::create_get(url).form(form.clone()).execute_body().await
    }

    /// Java: `HttpUtil.get(String urlString, Map paramMap, int timeout)`
    pub async fn get_with_form_timeout(
        url: &str,
        form: &IndexMap<String, String>,
        timeout_ms: i64,
    ) -> Result<String, HttpError> {
        Self::create_get(url)
            .form(form.clone())
            .timeout(timeout_ms)
            .execute_body()
            .await
    }

    /// GET with an explicit URL policy (tests / trusted networks).
    pub async fn get_with_policy(
        url: &str,
        policy: Arc<dyn UrlPolicy>,
    ) -> Result<String, HttpError> {
        HttpRequest::get(url)
            .url_policy_arc(policy)
            .execute_body()
            .await
    }

    /// Java: `HttpUtil.post(String urlString, Map paramMap)`
    pub async fn post_form(
        url: &str,
        form: &IndexMap<String, String>,
    ) -> Result<String, HttpError> {
        Self::create_post(url).form(form.clone()).execute_body().await
    }

    /// Java: `HttpUtil.post(String urlString, Map paramMap, int timeout)`
    pub async fn post_form_timeout(
        url: &str,
        form: &IndexMap<String, String>,
        timeout_ms: i64,
    ) -> Result<String, HttpError> {
        Self::create_post(url)
            .form(form.clone())
            .timeout(timeout_ms)
            .execute_body()
            .await
    }

    /// Java: `HttpUtil.post(String urlString, String body)`
    pub async fn post_body(url: &str, body: &str) -> Result<String, HttpError> {
        Self::create_post(url).body(body).execute_body().await
    }

    /// Java: `HttpUtil.post(String urlString, String body, int timeout)`
    pub async fn post_body_timeout(
        url: &str,
        body: &str,
        timeout_ms: i64,
    ) -> Result<String, HttpError> {
        Self::create_post(url)
            .body(body)
            .timeout(timeout_ms)
            .execute_body()
            .await
    }

    /// Java: `HttpUtil.downloadBytes(String url)`
    pub async fn download_bytes(url: &str) -> Result<Vec<u8>, HttpError> {
        Self::create_get(url).execute_bytes().await
    }

    /// Java: `HttpUtil.downloadString(String url, String customCharsetName)`
    pub async fn download_string(url: &str, charset_name: &str) -> Result<String, HttpError> {
        Self::download_string_with_progress(url, charset_name, None).await
    }

    /// Java: `HttpUtil.downloadString(String url, Charset customCharset, StreamProgress)`
    pub async fn download_string_with_progress(
        url: &str,
        charset_name: &str,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<String, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let bytes = Self::download_bytes(url).await?;
        let total = bytes.len() as i64;
        progress.progress(total, total);
        let text = Self::get_string(&bytes, Some(charset_name), false);
        progress.finish();
        Ok(text)
    }

    /// Java: `HttpUtil.downloadFile(String url, String dest)` — returns written byte count.
    pub async fn download_file(url: &str, dest: impl AsRef<FsPath>) -> Result<u64, HttpError> {
        Self::download_file_with_progress(url, dest, None).await
    }

    /// Java: `HttpUtil.downloadFile(String url, File destFile, StreamProgress)`
    pub async fn download_file_with_progress(
        url: &str,
        dest: impl AsRef<FsPath>,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let bytes = Self::download_bytes(url).await?;
        let total = bytes.len() as i64;
        progress.progress(total, total);
        std::fs::write(dest.as_ref(), &bytes)?;
        progress.finish();
        Ok(bytes.len() as u64)
    }

    /// Java: `HttpUtil.downloadFile(String url, File destFile, int timeout)`
    pub async fn download_file_timeout(
        url: &str,
        dest: impl AsRef<FsPath>,
        timeout_ms: i64,
    ) -> Result<u64, HttpError> {
        Self::download_file_timeout_with_progress(url, dest, timeout_ms, None).await
    }

    /// Java: `HttpUtil.downloadFile(String url, File destFile, int timeout, StreamProgress)`
    pub async fn download_file_timeout_with_progress(
        url: &str,
        dest: impl AsRef<FsPath>,
        timeout_ms: i64,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let bytes = Self::create_get(url)
            .timeout(timeout_ms)
            .execute_bytes()
            .await?;
        let total = bytes.len() as i64;
        progress.progress(total, total);
        std::fs::write(dest.as_ref(), &bytes)?;
        progress.finish();
        Ok(bytes.len() as u64)
    }

    /// Downloads bytes and writes them to `out` (Hutool `OutputStream` overload).
    ///
    /// Java: `HttpUtil.download(String url, OutputStream out, boolean isCloseOut)`
    /// (`isCloseOut` is caller-owned in Rust; this helper always flushes.)
    pub async fn download(
        url: &str,
        out: &mut impl Write,
    ) -> Result<u64, HttpError> {
        Self::download_with_progress(url, out, None).await
    }

    /// Java: `HttpUtil.download(String url, OutputStream out, boolean isCloseOut, StreamProgress)`
    pub async fn download_with_progress(
        url: &str,
        out: &mut impl Write,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let bytes = Self::download_bytes(url).await?;
        let total = bytes.len() as i64;
        progress.progress(total, total);
        out.write_all(&bytes)?;
        out.flush()?;
        progress.finish();
        Ok(bytes.len() as u64)
    }

    /// Downloads to a path and returns the destination file path.
    ///
    /// Java: `HttpUtil.downloadFileFromUrl(String url, String dest)` /
    /// `downloadFileFromUrl(String url, File destFile)`
    pub async fn download_file_from_url(
        url: &str,
        dest: impl AsRef<FsPath>,
    ) -> Result<std::path::PathBuf, HttpError> {
        Self::download_file_from_url_with_progress(url, dest, None).await
    }

    /// Java: `HttpUtil.downloadFileFromUrl(String url, File destFile, StreamProgress)`
    pub async fn download_file_from_url_with_progress(
        url: &str,
        dest: impl AsRef<FsPath>,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<std::path::PathBuf, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let response = Self::create_get(url).execute().await?;
        let path = response.write_body_for_file(dest.as_ref())?;
        progress.progress(-1, -1);
        progress.finish();
        Ok(path)
    }

    /// Java: `HttpUtil.downloadFileFromUrl(String url, File destFile, int timeout)`
    pub async fn download_file_from_url_timeout(
        url: &str,
        dest: impl AsRef<FsPath>,
        timeout_ms: i64,
    ) -> Result<std::path::PathBuf, HttpError> {
        Self::download_file_from_url_timeout_with_progress(url, dest, timeout_ms, None).await
    }

    /// Java: `HttpUtil.downloadFileFromUrl(String url, File destFile, int timeout, StreamProgress)`
    pub async fn download_file_from_url_timeout_with_progress(
        url: &str,
        dest: impl AsRef<FsPath>,
        timeout_ms: i64,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<std::path::PathBuf, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let response = Self::create_get(url).timeout(timeout_ms).execute().await?;
        let path = response.write_body_for_file(dest.as_ref())?;
        progress.progress(-1, -1);
        progress.finish();
        Ok(path)
    }

    /// Decodes a query string into a multi-value map.
    ///
    /// Java: `HttpUtil.decodeParams(...)`
    pub fn decode_params(params: &str) -> IndexMap<String, Vec<String>> {
        Self::decode_params_form(params, false)
    }

    /// Decodes a query string with optional form-urlencoded semantics.
    ///
    /// Java: `HttpUtil.decodeParams(..., boolean isFormUrlEncoded)`
    pub fn decode_params_form(params: &str, form: bool) -> IndexMap<String, Vec<String>> {
        let map = QueryMap::parse(params, true, form);
        let mut out = IndexMap::new();
        for (key, value) in map.pairs() {
            out.entry(key.clone())
                .or_insert_with(Vec::new)
                .push(value.clone().unwrap_or_default());
        }
        out
    }

    /// Decodes the query portion of a URL into a single-value map.
    ///
    /// Java: `HttpUtil.decodeParamMap(String paramsStr, Charset charset)`
    pub fn decode_param_map(params: &str) -> IndexMap<String, String> {
        Self::decode_params(params)
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().next().unwrap_or_default()))
            .collect()
    }

    /// Encodes a map into a query string without form-url-encoding mode.
    ///
    /// Java: `HttpUtil.toParams(Map)`
    pub fn to_params(params: &IndexMap<String, Vec<String>>) -> String {
        QueryMap::from_multi(params).build(false)
    }

    /// Encodes a flat map into a query string.
    pub fn to_params_map(params: &IndexMap<String, String>) -> String {
        QueryMap::from_flat(params).build(false)
    }

    /// Encodes a map with optional form-urlencoded semantics.
    ///
    /// Java: `HttpUtil.toParams(Map, Charset, boolean isFormUrlEncoded)`
    pub fn to_params_form(params: &IndexMap<String, String>, form: bool) -> String {
        let mut map = QueryMap::with_form_encoding(form);
        for (key, value) in params {
            map.insert(key.clone(), Some(value.clone()));
        }
        map.build(form)
    }

    /// Normalizes and optionally encodes a parameter string.
    ///
    /// Java: `HttpUtil.encodeParams(String urlWithParams, Charset charset)`
    pub fn encode_params(input: &str) -> String {
        if input.trim().is_empty() {
            return String::new();
        }
        let (url_part, param_part) = split_url_params(input);
        if param_part.is_empty() {
            return url_part.map(|u| u.into_owned()).unwrap_or_default();
        }
        let normalized = normalize_params(&param_part, true);
        match url_part {
            Some(url) if !url.is_empty() => format!("{url}?{normalized}"),
            _ => normalized,
        }
    }

    /// Normalizes parameter text; `encode=false` keeps literal pairs.
    ///
    /// Java: `HttpUtil.normalizeParams(String paramPart, Charset charset)`
    pub fn normalize_params(param_part: &str, encode: bool) -> String {
        normalize_params(param_part, encode)
    }

    /// Appends encoded form parameters to a URL.
    ///
    /// Java: `HttpUtil.urlWithForm(String url, Map, Charset, boolean)`
    pub fn url_with_form(
        url: &str,
        form: &IndexMap<String, String>,
        encode: bool,
    ) -> String {
        let query = if encode {
            Self::to_params_form(form, true)
        } else {
            Self::to_params_map(form)
        };
        Self::url_with_form_query(url, &query, encode)
    }

    /// Appends a pre-built query string to a URL.
    ///
    /// Java: `HttpUtil.urlWithForm(String url, String queryString, Charset, boolean)`
    pub fn url_with_form_query(url: &str, query: &str, encode: bool) -> String {
        if query.trim().is_empty() {
            if url.contains('?') {
                return if encode {
                    Self::encode_params(url)
                } else {
                    url.to_string()
                };
            }
            return url.to_string();
        }
        let mut out = String::with_capacity(url.len() + query.len() + 16);
        if url.contains('?') {
            let base = if encode {
                Self::encode_params(url)
            } else {
                url.to_string()
            };
            out.push_str(&base);
            if !base.ends_with('&') {
                out.push('&');
            }
        } else {
            out.push_str(url);
            out.push('?');
        }
        let encoded_query = normalize_params(query, true);
        out.push_str(if encode { &encoded_query } else { query });
        out
    }

    /// Appends form parameters using form-urlencoded encoding (issue #3536).
    ///
    /// Java: `HttpUtil.urlWithFormUrlEncoded(String url, Map, Charset)`
    pub fn url_with_form_url_encoded(url: &str, form: &IndexMap<String, String>) -> String {
        let raw = Self::to_params_map(form);
        Self::url_with_form_query(url, &raw, true)
    }

    /// Extracts a charset name from a Content-Type header value.
    ///
    /// Java: `HttpUtil.getCharset(String contentType)`
    pub fn get_charset(content_type: &str) -> Option<String> {
        let lower = content_type.to_ascii_lowercase();
        let idx = lower.find("charset")?;
        let rest = content_type[idx + "charset".len()..].trim_start();
        let rest = rest.strip_prefix('=')?.trim_start();
        let rest = rest.trim_start_matches(['\'', '"']);
        let end = rest
            .find(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
            .unwrap_or(rest.len());
        let name = rest[..end].trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    /// Decodes bytes to a string, optionally re-detecting charset from HTML meta.
    ///
    /// Java: `HttpUtil.getString(byte[] contentBytes, Charset charset, boolean)`
    pub fn get_string(content_bytes: &[u8], charset_name: Option<&str>, from_content: bool) -> String {
        let primary = charset_name
            .and_then(|name| Encoding::for_label(name.as_bytes()))
            .unwrap_or(encoding_rs::UTF_8);
        let (cow, _, _) = primary.decode(content_bytes);
        let mut content = cow.into_owned();
        if from_content {
            if let Some(meta) = extract_meta_charset(&content) {
                if let Some(enc) = Encoding::for_label(meta.as_bytes()) {
                    if enc != primary {
                        let (again, _, _) = enc.decode(content_bytes);
                        content = again.into_owned();
                    }
                }
            }
        }
        content
    }

    /// Guesses MIME type from a file name extension (minimal Hutool parity).
    ///
    /// Java: `HttpUtil.getMimeType(String filePath)`
    pub fn get_mime_type(path: &str) -> Option<&'static str> {
        match path.rsplit('.').next().unwrap_or("") {
            "html" | "htm" => Some("text/html"),
            "json" => Some("application/json"),
            "txt" => Some("text/plain"),
            "xml" => Some("application/xml"),
            "png" => Some("image/png"),
            "jpg" | "jpeg" => Some("image/jpeg"),
            "gif" => Some("image/gif"),
            "css" => Some("text/css"),
            "js" => Some("application/javascript"),
            "pdf" => Some("application/pdf"),
            _ => None,
        }
    }

    /// Java: `HttpUtil.getMimeType(String filePath, String defaultValue)`
    pub fn get_mime_type_or(path: &str, default: &str) -> String {
        Self::get_mime_type(path)
            .unwrap_or(default)
            .to_string()
    }

    /// Detects Content-Type from a request body (JSON / XML).
    ///
    /// Java: `HttpUtil.getContentTypeByRequestBody(String body)`
    pub fn get_content_type_by_request_body(body: &str) -> Option<String> {
        ContentType::detect(body).map(|ct| ct.to_string())
    }

    /// Builds a `Basic` Authorization header value.
    ///
    /// Java: `HttpUtil.buildBasicAuth(String username, String password, Charset)`
    pub fn build_basic_auth(username: &str, password: &str) -> String {
        let data = format!("{username}:{password}");
        format!("Basic {}", base64_encode(data))
    }
}

use super::{extract_meta_charset};
