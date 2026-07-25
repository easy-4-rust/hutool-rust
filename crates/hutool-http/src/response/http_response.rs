//! 对齐: `cn.hutool.http.HttpResponse`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpResponse.java
//! 中文说明: 缓冲HTTP响应门面，持有状态码、头部和响应体供消费

use crate::{HttpError, StatusCode, header};
use encoding_rs::Encoding;
use std::{
    fmt,
    io::Cursor,
    path::{Path, PathBuf},
};

use super::http_cookie::HttpCookie;

/// Buffered HTTP response facade aligned with Hutool's `HttpResponse`.
///
/// The transport remains reqwest. This facade owns the status, headers and a
/// body that was collected under the client's configured byte limit, making
/// repeated body reads and synchronous writes deterministic.
#[derive(Clone, PartialEq, Eq)]
pub struct HttpResponse {
    status: StatusCode,
    headers: header::HeaderMap,
    body: Vec<u8>,
    /// Final request URL when available (used for download file-name completion).
    url: Option<String>,
}

impl HttpResponse {
    pub(crate) const fn new(status: StatusCode, headers: header::HeaderMap, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
            url: None,
        }
    }

    /// Attaches the final request URL used for disposition / path file-name fallback.
    #[must_use]
    pub(crate) fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Returns the numeric response status.
    ///
    /// Java: `HttpResponse.getStatus()`
    #[must_use]
    pub const fn get_status(&self) -> u16 {
        self.status.as_u16()
    }

    /// Returns the typed response status.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Reports whether the status is in the `200..=299` range.
    ///
    /// Java: `HttpResponse.isOk()`
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status.is_success()
    }

    /// Returns all response headers.
    #[must_use]
    pub const fn headers(&self) -> &header::HeaderMap {
        &self.headers
    }

    /// Returns the first response-header value when it is valid text.
    #[must_use]
    pub fn header(&self, name: impl header::AsHeaderName) -> Option<&str> {
        self.headers.get(name)?.to_str().ok()
    }

    /// Returns the `Content-Encoding` value.
    ///
    /// Java: `HttpResponse.contentEncoding()`
    #[must_use]
    pub fn content_encoding(&self) -> Option<&str> {
        self.header(header::CONTENT_ENCODING)
    }

    /// Returns the protocol-valid content length, or `-1` when unknown.
    ///
    /// As in Hutool, a declared length is invalidated by transfer or content
    /// encoding because it no longer describes the decoded body exposed here.
    ///
    /// Java: `HttpResponse.contentLength()`
    #[must_use]
    pub fn content_length(&self) -> i64 {
        let length = self
            .header(header::CONTENT_LENGTH)
            .and_then(|value| value.parse::<i64>().ok())
            .unwrap_or(-1);
        if length > 0 && (self.is_chunked() || self.has_content_encoding()) {
            -1
        } else {
            length
        }
    }

    /// Reports `Content-Encoding: gzip`.
    ///
    /// Java: `HttpResponse.isGzip()`
    #[must_use]
    pub fn is_gzip(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| value.eq_ignore_ascii_case("gzip"))
    }

    /// Reports `Content-Encoding: deflate`.
    ///
    /// Java: `HttpResponse.isDeflate()`
    #[must_use]
    pub fn is_deflate(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| value.eq_ignore_ascii_case("deflate"))
    }

    /// Reports `Transfer-Encoding: chunked`.
    ///
    /// Java: `HttpResponse.isChunked()`
    #[must_use]
    pub fn is_chunked(&self) -> bool {
        self.header(header::TRANSFER_ENCODING)
            .is_some_and(|value| value.eq_ignore_ascii_case("chunked"))
    }

    /// Returns the first raw `Set-Cookie` header.
    ///
    /// Java: `HttpResponse.getCookieStr()`
    #[must_use]
    pub fn get_cookie_str(&self) -> Option<&str> {
        self.header(header::SET_COOKIE)
    }

    /// Parses all `Set-Cookie` headers into name/value cookies.
    ///
    /// Java: `HttpResponse.getCookies()`
    #[must_use]
    pub fn get_cookies(&self) -> Vec<HttpCookie> {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .filter_map(parse_set_cookie)
            .collect()
    }

    /// Returns the first cookie matching `name`.
    ///
    /// Java: `HttpResponse.getCookie(String name)`
    #[must_use]
    pub fn get_cookie(&self, name: &str) -> Option<HttpCookie> {
        self.get_cookies().into_iter().find(|cookie| cookie.name() == name)
    }

    /// Returns the value of the first cookie matching `name`.
    ///
    /// Java: `HttpResponse.getCookieValue(String name)`
    #[must_use]
    pub fn get_cookie_value(&self, name: &str) -> Option<String> {
        self.get_cookie(name).map(|cookie| cookie.value().to_string())
    }

    /// No-op sync: bodies are always buffered under the client byte limit.
    ///
    /// Java: `HttpResponse.sync()`
    #[must_use]
    pub fn sync(self) -> Self {
        self
    }

    /// Releases retained body buffers (idempotent; no live connection to close).
    ///
    /// Java: `HttpResponse.close()`
    pub fn close(&mut self) {
        self.body.clear();
        self.body.shrink_to_fit();
    }

    /// Returns a new reader over the buffered response body.
    ///
    /// Java: `HttpResponse.bodyStream()`
    #[must_use]
    pub fn body_stream(&self) -> Cursor<&[u8]> {
        Cursor::new(&self.body)
    }

    /// Returns the response bytes without another allocation.
    ///
    /// Java: `HttpResponse.bodyBytes()`
    #[must_use]
    pub fn body_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Replaces the buffered response body.
    ///
    /// Java: `HttpResponse.body(byte[] bodyBytes)`
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) -> &mut Self {
        self.body = body.into();
        self
    }

    /// Decodes the body using the response charset, defaulting to UTF-8.
    ///
    /// Java: `HttpResponse.body()`
    #[must_use]
    pub fn body(&self) -> String {
        let encoding = self
            .response_charset()
            .and_then(|label| Encoding::for_label(label.as_bytes()))
            .unwrap_or(encoding_rs::UTF_8);
        let (text, _, _) = encoding.decode(&self.body);
        text.into_owned()
    }

    /// Writes the entire buffered body and returns the number of bytes written.
    ///
    /// Java: `HttpResponse.writeBody(OutputStream out, boolean isCloseOut, StreamProgress)`
    /// (`StreamProgress` is omitted; callers may wrap the writer themselves.)
    pub fn write_body(&self, output: &mut impl std::io::Write) -> Result<u64, HttpError> {
        output.write_all(&self.body)?;
        output.flush()?;
        Ok(self.body.len() as u64)
    }

    /// Writes the body to a file or directory path (completing the file name when needed).
    ///
    /// Java: `HttpResponse.writeBody(File targetFileOrDir)` / `writeBody(String)`
    pub fn write_body_to_path(&self, target_file_or_dir: impl AsRef<Path>) -> Result<u64, HttpError> {
        let path = self.complete_file_name_from_header(target_file_or_dir.as_ref())?;
        let mut file = std::fs::File::create(&path)?;
        self.write_body(&mut file)
    }

    /// Writes via a temporary sibling file then renames into place.
    ///
    /// Java: `HttpResponse.writeBody(File, String tempFileSuffix, StreamProgress)`
    pub fn write_body_with_temp_suffix(
        &self,
        target_file_or_dir: impl AsRef<Path>,
        temp_file_suffix: Option<&str>,
    ) -> Result<u64, HttpError> {
        let out_file = self.complete_file_name_from_header(target_file_or_dir.as_ref())?;
        let suffix = match temp_file_suffix {
            None | Some("") => ".temp".to_string(),
            Some(s) if s.starts_with('.') => s.to_string(),
            Some(s) => format!(".{s}"),
        };
        let file_name = out_file
            .file_name()
            .map(|n| n.to_owned())
            .unwrap_or_else(|| std::ffi::OsString::from("download"));
        let mut temp_name = file_name.clone();
        temp_name.push(&suffix);
        let temp_path = out_file.with_file_name(temp_name);
        let write_result = (|| {
            let mut file = std::fs::File::create(&temp_path)?;
            let written = self.write_body(&mut file)?;
            std::fs::rename(&temp_path, &out_file)?;
            Ok::<u64, HttpError>(written)
        })();
        if write_result.is_err() {
            let _ = std::fs::remove_file(&temp_path);
        }
        write_result
    }

    /// Writes the body and returns the resolved destination path.
    ///
    /// Java: `HttpResponse.writeBodyForFile(File targetFileOrDir, StreamProgress)`
    pub fn write_body_for_file(
        &self,
        target_file_or_dir: impl AsRef<Path>,
    ) -> Result<PathBuf, HttpError> {
        let path = self.complete_file_name_from_header(target_file_or_dir.as_ref())?;
        let mut file = std::fs::File::create(&path)?;
        self.write_body(&mut file)?;
        Ok(path)
    }

    /// Completes a download path using `Content-Disposition` or the response URL.
    ///
    /// Java: `HttpResponse.completeFileNameFromHeader(File targetFileOrDir)`
    pub fn complete_file_name_from_header(
        &self,
        target_file_or_dir: impl AsRef<Path>,
    ) -> Result<PathBuf, HttpError> {
        let target = target_file_or_dir.as_ref();
        if !target.is_dir() {
            return Ok(target.to_path_buf());
        }
        let file_name = self
            .get_file_name_from_disposition()
            .filter(|name| !name.trim().is_empty())
            .or_else(|| self.fallback_file_name_from_url())
            .unwrap_or_else(|| "download".to_string());
        Ok(target.join(file_name))
    }

    /// Reads a file name from `Content-Disposition` (`filename*` preferred).
    ///
    /// Java: `HttpResponse.getFileNameFromDisposition()`
    #[must_use]
    pub fn get_file_name_from_disposition(&self) -> Option<String> {
        self.get_file_name_from_disposition_param(None)
    }

    /// Reads a file name from `Content-Disposition` using a custom parameter name.
    ///
    /// Java: `HttpResponse.getFileNameFromDisposition(String paramName)`
    #[must_use]
    pub fn get_file_name_from_disposition_param(
        &self,
        param_name: Option<&str>,
    ) -> Option<String> {
        let param = param_name.unwrap_or("filename");
        let dispositions: Vec<&str> = self
            .headers
            .get_all(header::CONTENT_DISPOSITION)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .collect();
        if dispositions.is_empty() {
            return None;
        }
        let starred = format!("{param}*");
        if let Some(name) = filename_from_dispositions(&dispositions, &starred) {
            return Some(decode_rfc5987(&name));
        }
        filename_from_dispositions(&dispositions, param).map(|name| strip_quotes(&name))
    }

    fn has_content_encoding(&self) -> bool {
        self.content_encoding()
            .is_some_and(|value| !value.trim().is_empty())
    }

    fn response_charset(&self) -> Option<&str> {
        self.header(header::CONTENT_TYPE)?
            .split(';')
            .skip(1)
            .find_map(|parameter| {
                let (name, value) = parameter.trim().split_once('=')?;
                name.trim()
                    .eq_ignore_ascii_case("charset")
                    .then(|| value.trim().trim_matches(['\'', '"']))
            })
    }

    fn fallback_file_name_from_url(&self) -> Option<String> {
        let url = self.url.as_deref()?;
        let path = url.split('?').next().unwrap_or(url);
        let name = path.rsplit('/').next().unwrap_or("");
        if name.is_empty() {
            None
        } else {
            Some(percent_decode_lightweight(name))
        }
    }
}

impl fmt::Debug for HttpResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HttpResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body_bytes", &self.body.len())
            .field("url", &self.url)
            .finish()
    }
}

impl fmt::Display for HttpResponse {
    /// Java: `HttpResponse.toString()`
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "Response Headers: ")?;
        for (name, value) in &self.headers {
            writeln!(
                formatter,
                "    {:?}: {:?}",
                name,
                value.to_str().unwrap_or("<binary>")
            )?;
        }
        writeln!(formatter, "Response Body: ")?;
        write!(formatter, "    {}", self.body())
    }
}

use super::{decode_rfc5987, filename_from_dispositions, from_hex, parse_set_cookie, percent_decode_lightweight, strip_quotes};
