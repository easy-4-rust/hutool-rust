//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::request_body::RequestBody;

/// Resource-backed body (bytes + optional name / content-type).
///
/// Java: `cn.hutool.http.body.ResourceBody` over Hutool `Resource`.
#[derive(Debug, Clone)]
pub struct ResourceBody {
    name: Option<String>,
    content_type: Option<String>,
    bytes: Vec<u8>,
    source: Option<PathBuf>,
}

impl ResourceBody {
    /// Creates a resource body from in-memory bytes.
    ///
    /// Java: `ResourceBody.create(Resource)` / constructor — Rust takes bytes directly.
    #[must_use]
    pub fn create(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            name: None,
            content_type: None,
            bytes: bytes.into(),
            source: None,
        }
    }

    /// Creates a resource body by reading a filesystem path.
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned());
        Ok(Self {
            name,
            content_type: None,
            bytes,
            source: Some(path.to_path_buf()),
        })
    }

    /// Sets the resource display name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the content type.
    #[must_use]
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Java: `Resource.getName()`
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Java: `Resource.getContentType()` / HttpResource content type.
    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Returns the body bytes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the filesystem source path when created via [`Self::from_path`].
    #[must_use]
    pub fn source_path(&self) -> Option<&Path> {
        self.source.as_deref()
    }

    /// Java: `ResourceBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(&self.bytes)?;
        out.flush()
    }
}

impl fmt::Display for ResourceBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResourceBody{{name={:?}, bytes={}, source={:?}}}",
            self.name,
            self.bytes.len(),
            self.source
        )
    }
}

impl RequestBody for ResourceBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(&self.bytes)?;
        out.flush()
    }
}
