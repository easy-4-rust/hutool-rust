//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Marker trait for request body writers (Hutool `RequestBody`).
///
/// Java: `cn.hutool.http.body.RequestBody`
pub trait RequestBody {
    /// Writes the body bytes to `out`.
    ///
    /// Java: `RequestBody.write(OutputStream out)`
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()>;
}
