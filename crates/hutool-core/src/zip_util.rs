//! 对齐: `cn.hutool.core.util.ZipUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ZipUtil.java
//!
//! 基于 `compress` 模块提供 gzip/zlib 与 ZIP 文件操作。

use std::fs::{self, File};
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};

use crate::compress::{ZipLimits, ZipReader, ZipWriter, memory_zip_writer};
use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ZipUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ZipUtil;

impl ZipUtil {
    /// 对齐 Java: `ZipUtil.gzip(byte[])`
    pub fn gzip(buf: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(buf)?;
        Ok(encoder.finish()?)
    }

    /// 对齐 Java: `ZipUtil.gzip(String content, String charset)`（UTF-8）
    pub fn gzip_str(content: &str) -> Result<Vec<u8>> {
        Self::gzip(content.as_bytes())
    }

    /// 对齐 Java: `ZipUtil.gzip(File)` — 读取文件后 gzip。
    pub fn gzip_file(file: &Path) -> Result<Vec<u8>> {
        Self::gzip(&fs::read(file)?)
    }

    /// 对齐 Java: `ZipUtil.unGzip(byte[])`
    pub fn un_gzip(buf: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(buf);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out)?;
        Ok(out)
    }

    /// 对齐 Java: `ZipUtil.unGzip(byte[], String charset)` — UTF-8 字符串。
    pub fn un_gzip_str(buf: &[u8]) -> Result<String> {
        let bytes = Self::un_gzip(buf)?;
        String::from_utf8(bytes).map_err(|_| CoreError::InvalidArgument {
            name: "unGzip",
            reason: "invalid utf-8",
        })
    }

    /// 对齐 Java: `ZipUtil.zlib(byte[], int level)`
    pub fn zlib(buf: &[u8], level: u32) -> Result<Vec<u8>> {
        let compression = if level == 0 {
            Compression::none()
        } else {
            Compression::new(level.min(9))
        };
        let mut encoder = ZlibEncoder::new(Vec::new(), compression);
        encoder.write_all(buf)?;
        Ok(encoder.finish()?)
    }

    /// 对齐 Java: `ZipUtil.zlib(String content, String charset, int level)`（UTF-8）
    pub fn zlib_str(content: &str, level: u32) -> Result<Vec<u8>> {
        Self::zlib(content.as_bytes(), level)
    }

    /// 对齐 Java: `ZipUtil.zlib(File, int level)`
    pub fn zlib_file(file: &Path, level: u32) -> Result<Vec<u8>> {
        Self::zlib(&fs::read(file)?, level)
    }

    /// 对齐 Java: `ZipUtil.unZlib(byte[])`
    pub fn un_zlib(buf: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(buf);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out)?;
        Ok(out)
    }

    /// 对齐 Java: `ZipUtil.unZlib(byte[], String charset)` — UTF-8 字符串。
    pub fn un_zlib_str(buf: &[u8]) -> Result<String> {
        let bytes = Self::un_zlib(buf)?;
        String::from_utf8(bytes).map_err(|_| CoreError::InvalidArgument {
            name: "unZlib",
            reason: "invalid utf-8",
        })
    }

    /// 对齐 Java: `ZipUtil.zip(String srcPath)` — 默认写出同名 `.zip`。
    pub fn zip(src: &Path) -> Result<PathBuf> {
        let dest = src.with_extension("zip");
        Self::zip_dir(src, &dest, false)
    }

    /// 对齐 Java: `ZipUtil.zip(File srcFile)`
    pub fn zip_dir(src: &Path, dest_zip: &Path, with_src_dir: bool) -> Result<PathBuf> {
        let file = File::create(dest_zip)?;
        let mut writer = ZipWriter::new(file);
        writer.add_path(src, with_src_dir)?;
        writer.finish()?;
        Ok(dest_zip.to_path_buf())
    }

    /// 对齐 Java: `ZipUtil.unzip(File zipFile, File outFile)`
    pub fn unzip(zip_file: &Path, out_dir: &Path) -> Result<PathBuf> {
        let file = File::open(zip_file)?;
        let mut reader = ZipReader::new(file)?;
        reader.read_to(out_dir, |_| true)
    }

    /// 对齐 Java: `ZipUtil.unzip(InputStream in, File outFile, Charset charset)`
    pub fn unzip_bytes(data: &[u8], out_dir: &Path) -> Result<PathBuf> {
        let mut reader = ZipReader::new(Cursor::new(data.to_vec()))?;
        reader.read_to(out_dir, |_| true)
    }

    /// 对齐 Java: `ZipUtil.unzipFileBytes(File zipFile, String name)`
    pub fn unzip_file_bytes(zip_file: &Path, entry_name: &str) -> Result<Option<Vec<u8>>> {
        let file = File::open(zip_file)?;
        let mut reader = ZipReader::new(file)?;
        reader.get(entry_name)
    }

    /// 对齐 Java: `ZipUtil.append(Path zipPath, Path appendFilePath)`
    pub fn append(zip_path: &Path, append_path: &Path) -> Result<()> {
        let existing = fs::read(zip_path)?;
        let mut old_reader = ZipReader::new(Cursor::new(existing.clone()))?;
        let mut names = Vec::new();
        old_reader.read(|entry| names.push(entry.name.clone()))?;
        let mut old_reader = ZipReader::new(Cursor::new(existing))?;
        let file = File::create(zip_path)?;
        let mut writer = ZipWriter::new(file);
        for name in names {
            if let Some(bytes) = old_reader.get(&name)? {
                if name.ends_with('/') {
                    writer.add_directory(name.trim_end_matches('/'))?;
                } else {
                    writer.add_bytes(&name, &bytes)?;
                }
            }
        }

        if append_path.is_dir() {
            writer.add_path(append_path, true)?;
        } else if append_path.is_file() {
            let name = append_path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or_else(|| CoreError::InvalidArgument {
                    name: "append_path",
                    reason: "missing file name",
                })?;
            writer.add_bytes(name, &fs::read(append_path)?)?;
        } else {
            return Err(CoreError::InvalidArgument {
                name: "append_path",
                reason: "must be an existing file or directory",
            });
        }
        writer.finish()?;
        Ok(())
    }

    /// 对齐 Java: `ZipUtil.unzip(ZipFile zipFile, File outFile, long limit)`
    pub fn unzip_with_limit(zip_file: &Path, out_dir: &Path, limit: u64) -> Result<PathBuf> {
        let file = File::open(zip_file)?;
        let mut reader = ZipReader::new(file)?;
        reader.set_limits(ZipLimits {
            max_uncompressed_bytes: limit,
            ..ZipLimits::default()
        });
        reader.read_to(out_dir, |_| true)
    }

    /// 对齐 Java: `ZipUtil.zip(OutputStream out, String[] paths, InputStream[] ins)`
    pub fn zip_streams(paths: &[&str], contents: &[&[u8]]) -> Result<Vec<u8>> {
        if paths.len() != contents.len() {
            return Err(CoreError::InvalidArgument {
                name: "paths",
                reason: "paths and contents length mismatch",
            });
        }
        let mut writer = memory_zip_writer();
        for (path, bytes) in paths.iter().zip(contents.iter()) {
            writer.add_bytes(path, bytes)?;
        }
        Ok(writer.finish()?.into_inner())
    }

    /// 对齐 Java: `ZipUtil.zip(File zipFile, boolean withSrcDir, File... srcFiles)`
    pub fn zip_files(dest_zip: &Path, with_src_dir: bool, sources: &[&Path]) -> Result<PathBuf> {
        let file = File::create(dest_zip)?;
        let mut writer = ZipWriter::new(file);
        for source in sources {
            writer.add_path(source, with_src_dir)?;
        }
        writer.finish()?;
        Ok(dest_zip.to_path_buf())
    }

    /// 列出 ZIP 一级条目名称。
    pub fn list_entry_names(zip_file: &Path) -> Result<Vec<String>> {
        let file = File::open(zip_file)?;
        let mut reader = ZipReader::new(file)?;
        let mut names = Vec::new();
        reader.read(|entry| names.push(entry.name.clone()))?;
        Ok(names)
    }

    /// 对齐 Java: `ZipUtil.listFileNames(ZipFile, String dir)` — 指定前缀下的文件名。
    pub fn list_file_names(zip_file: &Path, dir: &str) -> Result<Vec<String>> {
        let prefix = if dir.is_empty() || dir == "/" {
            String::new()
        } else {
            let mut p = dir.trim_matches('/').to_string();
            p.push('/');
            p
        };
        Ok(Self::list_entry_names(zip_file)?
            .into_iter()
            .filter(|name| {
                if prefix.is_empty() {
                    !name.contains('/')
                } else {
                    name.starts_with(&prefix) && !name[prefix.len()..].contains('/')
                }
            })
            .collect())
    }

    /// 对齐 Java: `ZipUtil.zip(File zipFile, String path, InputStream in)` — 单条目写入。
    pub fn zip_bytes_entry(dest_zip: &Path, entry_path: &str, data: &[u8]) -> Result<PathBuf> {
        Self::zip_streams_to_file(dest_zip, &[entry_path], &[data])
    }

    /// 将对齐 `zip(paths, streams)` 结果写到目标文件。
    pub fn zip_streams_to_file(
        dest_zip: &Path,
        paths: &[&str],
        contents: &[&[u8]],
    ) -> Result<PathBuf> {
        let bytes = Self::zip_streams(paths, contents)?;
        if let Some(parent) = dest_zip.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::write(dest_zip, bytes)?;
        Ok(dest_zip.to_path_buf())
    }

    /// 对齐 Java: `ZipUtil.read(ZipFile, Consumer)` — 遍历条目名。
    pub fn read_names<F>(zip_file: &Path, mut consumer: F) -> Result<()>
    where
        F: FnMut(&str),
    {
        for name in Self::list_entry_names(zip_file)? {
            consumer(&name);
        }
        Ok(())
    }

    /// 对齐 Java: `ZipUtil.get(File, Charset, String path)` — 按路径取条目字节。
    pub fn get(zip_file: &Path, path: &str) -> Result<Option<Vec<u8>>> {
        Self::unzip_file_bytes(zip_file, path)
    }
}
