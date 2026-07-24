//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::request_body::RequestBody;

/// Raw byte body.
///
/// Java: `cn.hutool.http.body.BytesBody`
#[derive(Debug, Clone)]
pub struct BytesBody {
    content: Vec<u8>,
}

impl BytesBody {
    /// Java: `BytesBody.create(byte[])` / `new BytesBody(byte[])`
    #[must_use]
    pub fn create(content: impl Into<Vec<u8>>) -> Self {
        Self {
            content: content.into(),
        }
    }

    /// Alias for [`Self::create`].
    #[must_use]
    pub fn new(content: impl Into<Vec<u8>>) -> Self {
        Self::create(content)
    }

    /// Returns the owned content bytes.
    #[must_use]
    pub fn content(&self) -> &[u8] {
        &self.content
    }

    /// Java: `BytesBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(&self.content)?;
        out.flush()
    }
}

impl RequestBody for BytesBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(&self.content)?;
        out.flush()
    }
}
