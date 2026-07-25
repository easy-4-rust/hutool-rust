//! 对齐: `cn.hutool.http.HttpDownloader`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpDownloader.java
//! 中文说明: HTTP下载器门面，提供字节数组、流和文件下载便捷方法

use crate::progress::{NoopStreamProgress, StreamProgress};
use crate::{HttpError, HttpUtil};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Thin download facade matching Hutool `HttpDownloader`.
///
/// Java: `cn.hutool.http.HttpDownloader`
pub struct HttpDownloader;

impl HttpDownloader {
    /// Java: `HttpDownloader.downloadBytes(String url)`
    pub async fn download_bytes(url: &str) -> Result<Vec<u8>, HttpError> {
        HttpUtil::download_bytes(url).await
    }

    /// Java: `HttpDownloader.downloadBytes(String url, int timeout)`
    pub async fn download_bytes_timeout(url: &str, timeout_ms: i64) -> Result<Vec<u8>, HttpError> {
        HttpUtil::create_get(url)
            .timeout(timeout_ms)
            .execute_bytes()
            .await
    }

    /// Java: `HttpDownloader.downloadString(String url, Charset, StreamProgress)`
    pub async fn download_string(
        url: &str,
        charset_name: &str,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<String, HttpError> {
        HttpUtil::download_string_with_progress(url, charset_name, progress).await
    }

    /// Java: `HttpDownloader.downloadFile(...)` — returns written byte count.
    pub async fn download_file(
        url: &str,
        dest: impl AsRef<Path>,
        timeout_ms: i64,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let bytes = HttpUtil::create_get(url)
            .timeout(timeout_ms)
            .execute_bytes()
            .await?;
        let total = bytes.len() as i64;
        progress.progress(total, total);
        std::fs::write(dest.as_ref(), &bytes)?;
        progress.finish();
        Ok(bytes.len() as u64)
    }

    /// Java: `HttpDownloader.downloadFile(..., String tempFileSuffix, ...)` —
    /// `temp_file_suffix` is accepted for signature parity (write goes to `dest`).
    pub async fn download_file_with_suffix(
        url: &str,
        dest: impl AsRef<Path>,
        _temp_file_suffix: &str,
        timeout_ms: i64,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        Self::download_file(url, dest, timeout_ms, progress).await
    }

    /// Java: `HttpDownloader.downloadForFile(...)` — returns the destination path.
    pub async fn download_for_file(
        url: &str,
        dest: impl AsRef<Path>,
        timeout_ms: i64,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<PathBuf, HttpError> {
        let progress = progress.unwrap_or(&NoopStreamProgress);
        progress.start();
        let path = HttpUtil::download_file_from_url_timeout(url, dest, timeout_ms).await?;
        progress.progress(-1, -1);
        progress.finish();
        Ok(path)
    }

    /// Java: `HttpDownloader.download(String, OutputStream, boolean, StreamProgress)`
    pub async fn download(
        url: &str,
        out: &mut impl Write,
        progress: Option<&dyn StreamProgress>,
    ) -> Result<u64, HttpError> {
        HttpUtil::download_with_progress(url, out, progress).await
    }
}
