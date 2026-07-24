//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

use crate::net::rfc3986::Rfc3986;
use crate::net::url_decoder::UrlDecoder;
use crate::string::{is_blank, trim};
use crate::{CoreError, Result};

mod hit_uri;
mod url_util;

pub use hit_uri::HitUri;
pub use url_util::UrlUtil;

fn validate_uri(location: &str) -> Result<()> {
    if location.is_empty() {
        return Ok(());
    }
    if location.contains(char::is_whitespace) {
        return Err(CoreError::Codec(format!("invalid URI: {location}")));
    }
    Ok(())
}

fn split_protocol(url: &str) -> (String, String) {
    if let Some(sep_index) = url.find("://") {
        if sep_index > 0 {
            let protocol = url[..sep_index + 3].to_string();
            let body = url[sep_index + 3..].to_string();
            return (protocol, body);
        }
    }
    ("http://".to_string(), url.to_string())
}

fn split_query(body: &str) -> (Option<String>, String) {
    if let Some(index) = body.find('?') {
        if index > 0 {
            let params = body[index..].to_string();
            let without_params = body[..index].to_string();
            return (Some(params), without_params);
        }
    }
    (None, body.to_string())
}

fn trim_leading_slashes(body: &str) -> &str {
    body.trim_start_matches(['\\', '/'])
}

fn collapse_slashes(body: &str) -> String {
    let mut output = String::with_capacity(body.len());
    let mut previous_slash = false;
    for ch in body.chars() {
        if ch == '/' {
            if !previous_slash {
                output.push(ch);
            }
            previous_slash = true;
        } else {
            previous_slash = false;
            output.push(ch);
        }
    }
    output
}

fn split_domain_and_path(body: &str) -> (String, Option<String>) {
    if let Some(index) = body.find('/') {
        if index > 0 {
            let domain = body[..index].to_string();
            let path = body[index..].to_string();
            return (domain, Some(path));
        }
    }
    (body.to_string(), None)
}

fn extract_path(raw: &str) -> Option<&str> {
    let trimmed = trim(raw);
    let path_start = if let Some(index) = trimmed.find("://") {
        let rest = &trimmed[index + 3..];
        rest.find('/').map(|offset| index + 3 + offset)
    } else if trimmed.starts_with('/') {
        Some(0)
    } else {
        None
    }?;

    let suffix = &trimmed[path_start..];
    let end = suffix
        .find(['?', '#'])
        .map_or(suffix.len(), |offset| offset);
    Some(&suffix[..end])
}

fn resolve_path(base_path: &str, other: &str) -> String {
    if other == "." {
        let mut path = base_path.to_string();
        if let Some(index) = path.rfind('/') {
            path.truncate(index + 1);
        } else {
            path.clear();
        }
        return path;
    }
    if other.starts_with('/') {
        return other.to_string();
    }
    let mut path = base_path.to_string();
    if !path.ends_with('/') {
        if let Some(index) = path.rfind('/') {
            path.truncate(index + 1);
        }
    }
    path.push_str(other);
    path
}
