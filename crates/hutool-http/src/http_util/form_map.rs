//! 对齐: `cn.hutool.http.FormMap`
//! 来源: hutool-http/src/main/java/cn/hutool/http/FormMap.java
//! 中文说明: 表单参数映射，有序键值对集合，支持URL编码和表单提交

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
