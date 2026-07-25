//! 对齐: `cn.hutool.http.MultipartBody`
//! 来源: hutool-http/src/main/java/cn/hutool/http/MultipartBody.java
//! 中文说明: multipart/form-data请求体构建器，用于表单文件上传

use std::collections::HashMap;
use std::fmt;

/// RFC 2388 multipart/form-data body builder.
#[derive(Debug, Clone)]
pub struct MultipartBody {
    form: HashMap<String, String>,
    boundary: String,
    charset: String,
}

impl MultipartBody {
    /// Creates a multipart body from string form fields.
    ///
    /// Java: `MultipartBody.create(Map, Charset)` / `MultipartBody(Map, Charset)`
    pub fn create(form: HashMap<String, String>, charset: &str) -> Self {
        Self {
            form,
            boundary: "----HiToolHttpBoundary".to_string(),
            charset: charset.to_string(),
        }
    }

    /// Alias for [`Self::create`] matching the Hutool constructor name.
    ///
    /// Java: `new MultipartBody(Map, Charset)`
    #[must_use]
    pub fn new(form: HashMap<String, String>, charset: &str) -> Self {
        Self::create(form, charset)
    }

    /// Returns the `Content-Type` header value including boundary.
    ///
    /// Java: `MultipartBody.getContentType()`
    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }

    /// Serializes the multipart payload.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (key, value) in &self.form {
            out.extend_from_slice(format!("--{}\r\n", self.boundary).as_bytes());
            out.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{}\"\r\n\r\n",
                    key
                )
                .as_bytes(),
            );
            out.extend_from_slice(value.as_bytes());
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(format!("--{}--\r\n", self.boundary).as_bytes());
        out
    }

    /// Writes the multipart payload to an output stream.
    ///
    /// Java: `MultipartBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        out.write_all(&self.to_bytes())?;
        out.flush()
    }
}

impl fmt::Display for MultipartBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MultipartBody{{charset={}, fields={}}}",
            self.charset,
            self.form.len()
        )
    }
}
