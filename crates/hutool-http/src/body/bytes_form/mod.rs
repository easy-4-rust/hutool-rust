//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

mod request_body;
mod bytes_body;
mod form_url_encoded_body;
mod resource_body;

pub use request_body::RequestBody;
pub use bytes_body::BytesBody;
pub use form_url_encoded_body::FormUrlEncodedBody;
pub use resource_body::ResourceBody;
