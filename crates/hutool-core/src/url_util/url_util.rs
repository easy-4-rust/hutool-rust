//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

use crate::net::rfc3986::Rfc3986;
use crate::net::url_decoder::UrlDecoder;
use crate::string::{is_blank, trim};
use crate::Result;

use super::hit_uri::HitUri;

/// 对齐 Java: `cn.hutool.core.util.URLUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlUtil;

impl UrlUtil {
    // ── URL 验证 ──

    /// 对齐 Java: `URLUtil.isUrl(CharSequence)`
    pub fn is_url(value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("ftp://")
    }

    /// 对齐 Java: `URLUtil.isHttp(CharSequence)`
    pub fn is_http(value: &str) -> bool {
        value.starts_with("http://")
    }

    /// 对齐 Java: `URLUtil.isHttps(CharSequence)`
    pub fn is_https(value: &str) -> bool {
        value.starts_with("https://")
    }

    // ── URL 解析 ──

    /// 对齐 Java: `URLUtil.getHost(URL)` 的字符串简化版。
    pub fn get_host(url: &str) -> Option<&str> {
        let url = url
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .trim_start_matches("ftp://");
        url.split('/').next().and_then(|host| {
            if host.is_empty() {
                None
            } else {
                Some(host)
            }
        })
    }

    /// 对齐 Java: `URLUtil.getPath(String uriStr)`
    pub fn get_path(uri_str: &str) -> String {
        Self::to_uri(uri_str)
            .ok()
            .and_then(|uri| uri.path().map(str::to_owned))
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| "/".to_string())
    }

    /// 对齐 Java: `URLUtil.getProtocol(URL)`
    pub fn get_protocol(url: &str) -> Option<&str> {
        if url.starts_with("https://") {
            Some("https")
        } else if url.starts_with("http://") {
            Some("http")
        } else if url.starts_with("ftp://") {
            Some("ftp")
        } else {
            None
        }
    }

    // ── URL 编码 ──

    /// 对齐 Java: `URLUtil.encode(String)` → `RFC3986.PATH`
    pub fn encode(value: &str) -> String {
        Rfc3986::encode_path(value)
    }

    /// 对齐 Java: `URLUtil.encodeQuery(String)` → `RFC3986.QUERY`
    pub fn encode_query(value: &str) -> String {
        Rfc3986::encode_query(value)
    }

    /// 对齐 Java: `URLUtil.decode(String)`
    pub fn decode(value: &str) -> String {
        UrlDecoder::decode(value)
    }

    // ── URI 转换 ──

    /// 对齐 Java: `URLUtil.toURI(String location)`
    pub fn to_uri(location: &str) -> Result<HitUri> {
        Self::to_uri_with_encode(location, false)
    }

    /// 对齐 Java: `URLUtil.toURI(String location, boolean isEncode)`
    pub fn to_uri_with_encode(location: &str, is_encode: bool) -> Result<HitUri> {
        let mut normalized = trim(location).to_string();
        if is_encode {
            normalized = Self::encode(&normalized);
        }
        validate_uri(&normalized)?;
        Ok(HitUri { raw: normalized })
    }

    // ── URL 构建 ──

    /// 对齐 Java: `URLUtil.normalize(String)`
    pub fn normalize(url: &str) -> String {
        Self::normalize_with_encode_path(url, false)
    }

    /// 对齐 Java: `URLUtil.normalize(String, boolean isEncodePath)`
    pub fn normalize_with_encode_path(url: &str, is_encode_path: bool) -> String {
        Self::normalize_full(url, is_encode_path, false)
    }

    /// 对齐 Java: `URLUtil.normalize(String, boolean isEncodePath, boolean replaceSlash)`
    pub fn normalize_full(url: &str, is_encode_path: bool, replace_slash: bool) -> String {
        if is_blank(url) {
            return url.to_string();
        }

        let (protocol, mut body) = split_protocol(url);

        let (params, body_without_params) = split_query(body.as_str());
        body = body_without_params;

        if !body.is_empty() {
            body = trim_leading_slashes(&body).to_string();
            body = body.replace('\\', "/");
            if replace_slash {
                body = collapse_slashes(&body);
            }
        }

        let (domain, path) = split_domain_and_path(&body);
        let encoded_path = if is_encode_path {
            path.as_ref().map(|segment| Self::encode(segment))
        } else {
            path
        };

        format!(
            "{protocol}{domain}{}{}",
            encoded_path.unwrap_or_default(),
            params.unwrap_or_default()
        )
    }

    /// 对齐 Java: `URLUtil.completeUrl(String, String)`
    pub fn complete_url(base: &str, path: &str) -> String {
        let base = base.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    /// 对齐 Java: `URLUtil.encodeBlank(String)` — 空白转 `%20`。
    pub fn encode_blank(value: &str) -> String {
        value.replace(' ', "%20")
    }

    /// 对齐 Java: `URLUtil.buildQuery(Map)` — `k=v&` 拼接（值已编码）。
    pub fn build_query(params: &[(&str, &str)]) -> String {
        params
            .iter()
            .map(|(k, v)| format!("{k}={}", Self::encode_query(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// 对齐 Java: `URLUtil.isFileURL(URL)` — `file:` scheme。
    pub fn is_file_url(url: &str) -> bool {
        url.trim().to_ascii_lowercase().starts_with("file:")
    }

    /// 对齐 Java: `URLUtil.isJarURL(URL)`
    pub fn is_jar_url(url: &str) -> bool {
        let lower = url.trim().to_ascii_lowercase();
        lower.starts_with("jar:") || lower.contains(".jar!")
    }

    /// 对齐 Java: `URLUtil.isJarFileURL(URL)`
    pub fn is_jar_file_url(url: &str) -> bool {
        Self::is_file_url(url) && url.to_ascii_lowercase().contains(".jar")
    }

    /// 对齐 Java: `URLUtil.getDecodedPath(URL)`
    pub fn get_decoded_path(url: &str) -> String {
        Self::decode(&Self::get_path(url))
    }

    /// 对齐 Java: `URLUtil.getDataUri` / `getDataUriBase64`
    pub fn get_data_uri(mime: &str, base64: &str) -> String {
        format!("data:{mime};base64,{base64}")
    }

    /// 对齐 Java: `URLUtil.getStringURI(String)` — 规范化字符串 URI。
    pub fn get_string_uri(location: &str) -> String {
        Self::to_uri(location)
            .map(|u| u.as_str().to_string())
            .unwrap_or_else(|_| trim(location).to_string())
    }

    /// 对齐 Java: `URLUtil.url(String)` / `toUrlForHttp` — HTTP(S) 规范化。
    pub fn url(location: &str) -> String {
        let trimmed = trim(location);
        if Self::is_url(trimmed) {
            Self::normalize(trimmed)
        } else if trimmed.starts_with("//") {
            format!("http:{}", trimmed)
        } else {
            Self::normalize(trimmed)
        }
    }

    /// 对齐 Java: `URLUtil.toUrlForHttp(String)`
    pub fn to_url_for_http(location: &str) -> String {
        Self::url(location)
    }
}

use super::{collapse_slashes, split_domain_and_path, split_protocol, split_query, trim_leading_slashes, validate_uri};

