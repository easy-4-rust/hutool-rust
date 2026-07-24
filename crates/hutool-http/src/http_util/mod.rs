//! Hutool-aligned HTTP utility helpers (`cn.hutool.http.HttpUtil`).
//!
//! Offline param/URL helpers plus network facades that delegate to
//! [`crate::HttpRequest`] / [`crate::HttpClient`] with secure defaults.

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

mod http_util;
mod form_map;

pub use http_util::HttpUtil;
pub use form_map::FormMap;

fn extract_meta_charset(content: &str) -> Option<String> {
    let lower = content.to_ascii_lowercase();
    if !lower.contains("<meta") {
        return None;
    }
    HttpUtil::get_charset(content)
}

pub fn form_map(pairs: &[(&str, &str)]) -> FormMap {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

pub fn param_list_map(pairs: &[(&str, &str)]) -> IndexMap<String, Vec<String>> {
    let mut map = IndexMap::new();
    for (k, v) in pairs {
        map.entry(k.to_string())
            .or_insert_with(Vec::new)
            .push(v.to_string());
    }
    map
}
