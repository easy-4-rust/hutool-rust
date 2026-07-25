//! 对齐: `cn.hutool.http.HttpInputStream`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpInputStream.java
//! 中文说明: 响应体输入流，提供可seek的响应体读取视图

use crate::HttpResponse;
use std::io::{self, Cursor, Read, Seek, SeekFrom};

/// Seekable view over an [`HttpResponse`] body.
///
/// Java: `cn.hutool.http.HttpInputStream`
#[derive(Debug, Clone)]
pub struct HttpInputStream {
    cursor: Cursor<Vec<u8>>,
}

impl HttpInputStream {
    /// Java: `new HttpInputStream(HttpResponse response)`
    #[must_use]
    pub fn new(response: &HttpResponse) -> Self {
        Self {
            cursor: Cursor::new(response.body_bytes().to_vec()),
        }
    }

    /// Creates a stream from owned body bytes.
    #[must_use]
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            cursor: Cursor::new(bytes.into()),
        }
    }

    /// Java: `HttpInputStream.available()`
    #[must_use]
    pub fn available(&self) -> usize {
        let pos = self.cursor.position() as usize;
        self.cursor.get_ref().len().saturating_sub(pos)
    }

    /// Java: `HttpInputStream.markSupported()`
    #[must_use]
    pub const fn mark_supported() -> bool {
        true
    }

    /// Java: `HttpInputStream.mark(int)` — Cursor always supports seek; mark is a no-op position save.
    pub fn mark(&mut self, _readlimit: usize) {
        // Cursor is fully seekable; mark/reset use SeekFrom::Start(0) semantics via reset().
    }

    /// Java: `HttpInputStream.reset()`
    pub fn reset(&mut self) -> io::Result<()> {
        self.cursor.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    /// Java: `HttpInputStream.skip(long)`
    pub fn skip(&mut self, n: u64) -> io::Result<u64> {
        let before = self.cursor.position();
        self.cursor.seek(SeekFrom::Current(n as i64))?;
        Ok(self.cursor.position().saturating_sub(before))
    }

    /// Java: `HttpInputStream.close()`
    pub fn close(&mut self) {
        self.cursor = Cursor::new(Vec::new());
    }

    /// Returns remaining unread bytes.
    #[must_use]
    pub fn remaining_bytes(&self) -> &[u8] {
        let pos = self.cursor.position() as usize;
        &self.cursor.get_ref()[pos..]
    }
}

impl Read for HttpInputStream {
    /// Java: `HttpInputStream.read()` / `read(byte[], int, int)`
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.cursor.read(buf)
    }
}

impl Seek for HttpInputStream {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor.seek(pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_input_stream_read_skip_reset() {
        let mut stream = HttpInputStream::from_bytes(b"abcdef");
        assert_eq!(stream.available(), 6);
        assert!(HttpInputStream::mark_supported());
        let mut buf = [0u8; 2];
        assert_eq!(stream.read(&mut buf).unwrap(), 2);
        assert_eq!(&buf, b"ab");
        assert_eq!(stream.skip(2).unwrap(), 2);
        assert_eq!(stream.remaining_bytes(), b"ef");
        stream.reset().unwrap();
        assert_eq!(stream.available(), 6);
        stream.close();
        assert_eq!(stream.available(), 0);
    }
}
