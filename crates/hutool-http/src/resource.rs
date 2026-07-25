//! 对齐: `cn.hutool.http.HttpResource`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpResource.java
//! 中文说明: HTTP资源包装器，封装文件和字节资源用于上传

use crate::body::ResourceBody;
use std::io::{Cursor, Read};
use std::path::Path;

/// Named resource with optional content type for HTTP uploads.
///
/// Java: `cn.hutool.http.HttpResource`
#[derive(Debug, Clone)]
pub struct HttpResource {
    inner: ResourceBody,
}

impl HttpResource {
    /// Java: `new HttpResource(Resource, String contentType)`
    #[must_use]
    pub fn new(bytes: impl Into<Vec<u8>>, content_type: impl Into<String>) -> Self {
        Self {
            inner: ResourceBody::create(bytes).with_content_type(content_type),
        }
    }

    /// Builds from a filesystem path and content type.
    pub fn from_path(
        path: impl AsRef<Path>,
        content_type: impl Into<String>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            inner: ResourceBody::from_path(path)?.with_content_type(content_type),
        })
    }

    /// Wraps an existing [`ResourceBody`].
    #[must_use]
    pub fn from_resource_body(body: ResourceBody) -> Self {
        Self { inner: body }
    }

    /// Java: `HttpResource.getName()`
    #[must_use]
    pub fn get_name(&self) -> Option<&str> {
        self.inner.name()
    }

    /// Sets the resource display name (fluent).
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.inner = self.inner.with_name(name);
        self
    }

    /// Java: `HttpResource.getUrl()` — returns a `file:` URL when sourced from a path.
    #[must_use]
    pub fn get_url(&self) -> Option<String> {
        self.inner.source_path().map(|p| format!("file://{}", p.display()))
    }

    /// Java: `HttpResource.getStream()` — returns a readable cursor over the bytes.
    #[must_use]
    pub fn get_stream(&self) -> Cursor<&[u8]> {
        Cursor::new(self.inner.bytes())
    }

    /// Java: `HttpResource.getContentType()`
    #[must_use]
    pub fn get_content_type(&self) -> Option<&str> {
        self.inner.content_type()
    }

    /// Returns the underlying resource body.
    #[must_use]
    pub fn into_body(self) -> ResourceBody {
        self.inner
    }

    /// Borrows the underlying resource body.
    #[must_use]
    pub fn as_body(&self) -> &ResourceBody {
        &self.inner
    }

    /// Reads all bytes via [`Self::get_stream`].
    pub fn read_all(&self) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.get_stream().read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_resource_name_type_and_stream() {
        let res = HttpResource::new(b"hello".as_slice(), "text/plain").with_name("a.txt");
        assert_eq!(res.get_name(), Some("a.txt"));
        assert_eq!(res.get_content_type(), Some("text/plain"));
        assert_eq!(res.read_all().unwrap(), b"hello");
        assert!(res.get_url().is_none());
    }
}
