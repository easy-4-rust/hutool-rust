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

/// Convenience alias for building ordered form maps in tests.
pub type FormMap = IndexMap<String, String>;

use super::{extract_meta_charset};
