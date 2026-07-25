//! 对齐: `cn.hutool.http.MultipartOutputStream`
//! 来源: hutool-http/src/main/java/cn/hutool/http/MultipartOutputStream.java
//! 中文说明: 流式multipart/form-data写入器，增量写入多部分请求体

use crate::DEFAULT_BOUNDARY;
use std::io::{self, Write};

/// Streaming multipart/form-data writer.
///
/// Java: `cn.hutool.http.MultipartOutputStream`
#[derive(Debug)]
pub struct MultipartOutputStream<W: Write> {
    out: W,
    boundary: String,
    charset: String,
    finished: bool,
}

impl MultipartOutputStream<Vec<u8>> {
    /// Creates a multipart writer backed by an in-memory buffer.
    #[must_use]
    pub fn into_vec(charset: &str) -> Self {
        Self::new(Vec::new(), charset, DEFAULT_BOUNDARY)
    }
}

impl<W: Write> MultipartOutputStream<W> {
    /// Java: `new MultipartOutputStream(OutputStream, Charset)`
    pub fn new(out: W, charset: &str, boundary: &str) -> Self {
        Self {
            out,
            boundary: boundary.to_string(),
            charset: charset.to_string(),
            finished: false,
        }
    }

    /// Java: `new MultipartOutputStream(OutputStream, Charset, String boundary)`
    pub fn with_boundary(out: W, charset: &str, boundary: impl Into<String>) -> Self {
        Self {
            out,
            boundary: boundary.into(),
            charset: charset.to_string(),
            finished: false,
        }
    }

    /// Returns the multipart boundary.
    #[must_use]
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Returns the charset name retained for Hutool parity.
    #[must_use]
    pub fn charset(&self) -> &str {
        &self.charset
    }

    /// Java: `MultipartOutputStream.write(String formFieldName, Object value)`
    pub fn write_field(&mut self, form_field_name: &str, value: &str) -> io::Result<&mut Self> {
        if self.finished {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "multipart stream already finished",
            ));
        }
        write!(
            self.out,
            "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
            self.boundary, form_field_name, value
        )?;
        Ok(self)
    }

    /// Java: `MultipartOutputStream.write(int b)` — writes a raw byte into the stream.
    pub fn write_byte(&mut self, b: u8) -> io::Result<()> {
        self.out.write_all(&[b])
    }

    /// Java: `MultipartOutputStream.finish()` — writes the closing boundary.
    pub fn finish(&mut self) -> io::Result<()> {
        if !self.finished {
            write!(self.out, "--{}--\r\n", self.boundary)?;
            self.finished = true;
        }
        self.out.flush()
    }

    /// Java: `MultipartOutputStream.close()` — finishes then drops the writer.
    pub fn close(mut self) -> io::Result<W> {
        self.finish()?;
        Ok(self.out)
    }

    /// Returns a mutable reference to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multipart_output_stream_writes_fields() {
        let mut stream = MultipartOutputStream::into_vec("UTF-8");
        stream.write_field("a", "1").unwrap();
        stream.write_field("b", "2").unwrap();
        let bytes = stream.close().unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("name=\"a\""));
        assert!(text.contains("1"));
        assert!(text.contains("------HiToolHttpBoundary--"));
    }
}
