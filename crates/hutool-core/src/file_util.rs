//! 对齐: `cn.hutool.core.io.FileUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FileUtil.java
//!
//! Rust 版本提供文件操作的 idiomatic 实现。

use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

use sha2::{Digest, Sha256};

/// 对齐 Java: `cn.hutool.core.io.FileUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct FileUtil;

impl FileUtil {
    // ── 路径操作 ──

    /// 对齐 Java: `FileUtil.getName(File)`
    /// 路径以分隔符结尾时返回空串（对齐 Hutool）。
    pub fn name(path: &Path) -> &str {
        let s = path.to_str().unwrap_or("");
        if s.ends_with('/') || s.ends_with('\\') {
            return "";
        }
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.getSuffix(File)`
    pub fn suffix(path: &Path) -> &str {
        path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.mainName(File)`
    pub fn main_name(path: &Path) -> &str {
        path.file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    /// 对齐 Java: `FileUtil.getName(String)`
    pub fn name_from_str(path: &str) -> &str {
        Self::name(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.getSuffix(String)`
    pub fn suffix_from_str(path: &str) -> &str {
        Self::suffix(Path::new(path))
    }

    // ── 路径构建 ──

    /// 对齐 Java: `FileUtil.getPath(String...)`
    pub fn join_paths(parts: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for part in parts {
            path.push(part);
        }
        path
    }

    /// 对齐 Java: `FileUtil.file(String...)`
    pub fn file(parts: &[&str]) -> PathBuf {
        Self::join_paths(parts)
    }

    // ── 文件判断 ──

    /// 对齐 Java: `FileUtil.exist(String)`
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// 对齐 Java: `FileUtil.isFile(String)`
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// 对齐 Java: `FileUtil.isDirectory(String)`
    pub fn is_directory(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    // ── 文件大小 ──

    /// 对齐 Java: `FileUtil.size(File)`
    pub fn size(path: &Path) -> u64 {
        fs::metadata(path).map(|m| m.len()).unwrap_or(0)
    }

    // ── 文件读取 ──

    /// 对齐 Java: `FileUtil.readUtf8String(File)`
    pub fn read_utf8_string(path: &str) -> std::io::Result<String> {
        fs::read_to_string(path)
    }

    /// 对齐 Java: `FileUtil.readBytes(File)`
    pub fn read_bytes(path: &str) -> std::io::Result<Vec<u8>> {
        fs::read(path)
    }

    // ── 文件写入 ──

    /// 对齐 Java: `FileUtil.writeUtf8String(String, File)`
    pub fn write_utf8_string(path: &str, content: &str) -> std::io::Result<()> {
        fs::write(path, content)
    }

    /// 对齐 Java: `FileUtil.writeBytes(byte[], File)`
    pub fn write_bytes(path: &str, content: &[u8]) -> std::io::Result<()> {
        fs::write(path, content)
    }

    // ── 文件操作 ──

    /// 对齐 Java: `FileUtil.copy(File, File)`（拷贝前创建目标父目录，对齐 PathUtil.copyFile）
    pub fn copy(from: &str, to: &str) -> std::io::Result<u64> {
        let to_path = Path::new(to);
        if let Some(parent) = to_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::copy(from, to)
    }

    /// 对齐 Java: `FileUtil.del(File)`
    pub fn delete(path: &str) -> std::io::Result<()> {
        let p = Path::new(path);
        if p.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }

    /// 对齐 Java: `FileUtil.mkdir(File)`
    pub fn mkdir(path: &str) -> std::io::Result<()> {
        fs::create_dir_all(path)
    }

    /// 对齐 Java: `FileUtil.rename(File, String)`
    pub fn rename(from: &str, to: &str) -> std::io::Result<()> {
        fs::rename(from, to)
    }

    // ── 文件列表 ──

    /// 对齐 Java: `FileUtil.listFileNames(String)`
    pub fn list_file_names(path: &str) -> std::io::Result<Vec<String>> {
        let entries = fs::read_dir(path)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
        }
        Ok(names)
    }

    /// 列出目录下的所有目录名
    pub fn list_dir_names(path: &str) -> std::io::Result<Vec<String>> {
        let entries = fs::read_dir(path)?;
        let mut names = Vec::new();
        for entry in entries {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
        }
        Ok(names)
    }

    // ── 临时文件 ──

    /// 对齐 Java: `FileUtil.getTmpDirPath()` / `getTmpDir`
    pub fn tmp_dir() -> PathBuf {
        std::env::temp_dir()
    }

    /// 对齐 Java: `FileUtil.getTmpDirPath()` 字符串形式。
    pub fn tmp_dir_str() -> String {
        Self::tmp_dir().to_string_lossy().into_owned()
    }

    /// 对齐 Java: `FileUtil.isWindows()`
    pub fn is_windows() -> bool {
        cfg!(windows)
    }

    /// 对齐 Java: `FileUtil.extName(String)`（无点前缀）
    pub fn ext_name(path: &str) -> &str {
        Self::suffix(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.getAbsolutePath(String)`
    pub fn absolute_path(path: &str) -> PathBuf {
        let p = Path::new(path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(p)
        }
    }

    /// 对齐 Java: `FileUtil.getParent(String, int)`（向上 level 级）
    pub fn parent(path: &str, level: usize) -> Option<PathBuf> {
        let mut cur = Path::new(path).parent().map(Path::to_path_buf)?;
        for _ in 1..level {
            cur = cur.parent()?.to_path_buf();
        }
        Some(cur)
    }

    /// 对齐 Java: `FileUtil.touch(String)`（创建空文件及父目录）
    pub fn touch(path: &str) -> std::io::Result<PathBuf> {
        let p = Path::new(path);
        if let Some(parent) = p.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        if !p.exists() {
            fs::File::create(p)?;
        }
        Ok(p.to_path_buf())
    }

    /// 对齐 Java: `FileUtil.ls(String)`（列出目录项名）
    pub fn ls(path: &str) -> std::io::Result<Vec<String>> {
        let mut names = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
        Ok(names)
    }

    /// 对齐 Java: `FileUtil.contentEquals(File, File)`
    pub fn content_equals(path1: &str, path2: &str) -> std::io::Result<bool> {
        Ok(fs::read(path1)? == fs::read(path2)?)
    }

    /// 对齐 Java: `FileUtil.clean(File)`（清空目录内容，保留目录本身）
    pub fn clean(path: &str) -> std::io::Result<()> {
        let p = Path::new(path);
        if !p.is_dir() {
            return Ok(());
        }
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let child = entry.path();
            if child.is_dir() {
                fs::remove_dir_all(&child)?;
            } else {
                fs::remove_file(&child)?;
            }
        }
        Ok(())
    }

    /// 对齐 Java: `FileUtil.size(String)`
    pub fn size_of(path: &str) -> u64 {
        Self::size(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.equals(File, File)`（路径规范化比较）
    pub fn path_equals(path1: &str, path2: &str) -> bool {
        Path::new(path1) == Path::new(path2)
    }

    /// 对齐 Java: `FileUtil.readUtf8Lines(File)`
    pub fn read_utf8_lines(path: &str) -> std::io::Result<Vec<String>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        reader.lines().collect()
    }

    /// 对齐 Java: `FileUtil.writeUtf8Lines(Collection, File)`
    pub fn write_utf8_lines(path: &str, lines: &[impl AsRef<str>]) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for line in lines {
            writeln!(file, "{}", line.as_ref())?;
        }
        Ok(())
    }

    /// 对齐 Java: `FileUtil.appendUtf8String(String, File)`
    pub fn append_utf8_string(path: &str, content: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        file.write_all(content.as_bytes())
    }

    /// 对齐 Java: `FileUtil.appendUtf8Lines`
    pub fn append_utf8_lines(path: &str, lines: &[impl AsRef<str>]) -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        for line in lines {
            writeln!(file, "{}", line.as_ref())?;
        }
        Ok(())
    }

    /// 对齐 Java: `FileUtil.getTotalLines(File)`
    pub fn total_lines(path: &str) -> std::io::Result<usize> {
        Ok(Self::read_utf8_lines(path)?.len())
    }

    /// 对齐 Java: `FileUtil.createTempFile()`
    pub fn create_temp_file() -> std::io::Result<PathBuf> {
        let path = std::env::temp_dir().join(format!(
            "hutool-{}-{}.tmp",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        File::create(&path)?;
        Ok(path)
    }

    /// 对齐 Java: `FileUtil.move(File, File)`
    pub fn move_path(from: &str, to: &str) -> std::io::Result<()> {
        let to_path = Path::new(to);
        if let Some(parent) = to_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        fs::rename(from, to)
    }

    /// 对齐 Java: `FileUtil.normalize(String)`
    pub fn normalize(path: &str) -> String {
        let mut out = PathBuf::new();
        for component in Path::new(path).components() {
            match component {
                Component::ParentDir => {
                    out.pop();
                }
                Component::CurDir => {}
                other => out.push(other.as_os_str()),
            }
        }
        out.to_string_lossy().replace('\\', "/")
    }

    /// 对齐 Java: `FileUtil.isAbsolutePath(String)`
    pub fn is_absolute_path(path: &str) -> bool {
        Path::new(path).is_absolute()
    }

    /// 对齐 Java: `FileUtil.getUserHomePath()`
    pub fn user_home_path() -> PathBuf {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    }

    /// 对齐 Java: `FileUtil.isEmpty(File)` / `isDirEmpty`
    pub fn is_dir_empty(path: &str) -> std::io::Result<bool> {
        let mut entries = fs::read_dir(path)?;
        Ok(entries.next().is_none())
    }

    /// 对齐 Java: `FileUtil.isEmpty(File)` — 空文件或空目录。
    pub fn is_empty(path: &str) -> bool {
        let p = Path::new(path);
        if p.is_file() {
            Self::size(p) == 0
        } else if p.is_dir() {
            Self::is_dir_empty(path).unwrap_or(false)
        } else {
            true
        }
    }

    /// 对齐 Java: `FileUtil.lastModifiedTime(File)`
    pub fn last_modified_time(path: &str) -> std::io::Result<SystemTime> {
        fs::metadata(path)?.modified()
    }

    /// 对齐 Java: `FileUtil.newerThan(File, File)`
    pub fn newer_than(path1: &str, path2: &str) -> std::io::Result<bool> {
        Ok(Self::last_modified_time(path1)? > Self::last_modified_time(path2)?)
    }

    /// 对齐 Java: `FileUtil.readableFileSize(long)`
    pub fn readable_file_size(size: u64) -> String {
        const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
        let mut value = size as f64;
        let mut unit = 0usize;
        while value >= 1024.0 && unit < UNITS.len() - 1 {
            value /= 1024.0;
            unit += 1;
        }
        if unit == 0 {
            format!("{size} {}", UNITS[unit])
        } else {
            format!("{value:.1} {}", UNITS[unit])
        }
    }

    /// 对齐 Java: `FileUtil.checksum` — SHA-256 hex。
    pub fn checksum_sha256(path: &str) -> std::io::Result<String> {
        let bytes = fs::read(path)?;
        let digest = Sha256::digest(&bytes);
        Ok(hex::encode(digest))
    }

    /// 对齐 Java: `FileUtil.checksumCRC32` — IEEE CRC32。
    pub fn checksum_crc32(path: &str) -> std::io::Result<u32> {
        Ok(crc32_ieee(&fs::read(path)?))
    }

    /// 对齐 Java: `FileUtil.getMimeType(String)` — 按扩展名粗分。
    pub fn mime_type(path: &str) -> &'static str {
        match Self::ext_name(path).to_ascii_lowercase().as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "json" => "application/json",
            "xml" => "application/xml",
            "txt" | "log" => "text/plain",
            "html" | "htm" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "pdf" => "application/pdf",
            "zip" => "application/zip",
            _ => "application/octet-stream",
        }
    }

    /// 对齐 Java: `FileUtil.getLineSeparator()`
    pub fn line_separator() -> &'static str {
        if cfg!(windows) { "\r\n" } else { "\n" }
    }

    /// 对齐 Java: `FileUtil.contentEqualsIgnoreEOL`
    pub fn content_equals_ignore_eol(path1: &str, path2: &str) -> std::io::Result<bool> {
        let a = Self::read_utf8_lines(path1)?;
        let b = Self::read_utf8_lines(path2)?;
        Ok(a == b)
    }

    /// 对齐 Java: `FileUtil.subPath` — 相对路径片段。
    pub fn sub_path(base: &str, full: &str) -> Option<String> {
        let full = Path::new(full);
        full.strip_prefix(base)
            .ok()
            .map(|p| p.to_string_lossy().replace('\\', "/"))
    }

    /// 对齐 Java: `FileUtil.getPrefix(String)` — 主文件名。
    pub fn prefix(path: &str) -> &str {
        Self::main_name(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.checkSlip` — 拒绝 `..` 穿越。
    pub fn check_slip(base: &Path, child: &Path) -> bool {
        let joined = base.join(child);
        let normalized = PathBuf::from(Self::normalize(&joined.to_string_lossy()));
        normalized.starts_with(base)
    }

    /// 对齐 Java: `FileUtil.isSymlink`
    pub fn is_symlink(path: &str) -> bool {
        fs::symlink_metadata(path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false)
    }

    /// 对齐 Java: `FileUtil.mkParentDirs`
    pub fn mk_parent_dirs(path: &str) -> std::io::Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        Ok(())
    }

    /// 对齐 Java: `FileUtil.isNotEmpty(File)`
    pub fn is_not_empty(path: &str) -> bool {
        !Self::is_empty(path)
    }

    /// 对齐 Java: `FileUtil.getCanonicalPath` — 规范化绝对路径。
    pub fn canonical_path(path: &str) -> PathBuf {
        PathBuf::from(Self::normalize(&Self::absolute_path(path).to_string_lossy()))
    }

    /// 对齐 Java: `FileUtil.newFile(String)` — 构造路径。
    pub fn new_file(path: &str) -> PathBuf {
        PathBuf::from(path)
    }

    /// 对齐 Java: `FileUtil.getUserHomeDir`
    pub fn user_home_dir() -> PathBuf {
        Self::user_home_path()
    }

    /// 对齐 Java: `FileUtil.cleanEmpty` — 删除空目录。
    pub fn clean_empty(path: &str) -> std::io::Result<()> {
        if Self::is_dir_empty(path)? {
            fs::remove_dir(path)?;
        }
        Ok(())
    }

    /// 对齐 Java: `FileUtil.pathEndsWith`
    pub fn path_ends_with(path: &str, suffix: &str) -> bool {
        Path::new(path)
            .to_string_lossy()
            .replace('\\', "/")
            .ends_with(&suffix.replace('\\', "/"))
    }

    /// 对齐 Java: `FileUtil.lastIndexOfSeparator`
    pub fn last_index_of_separator(path: &str) -> Option<usize> {
        path.rfind(['/', '\\'])
    }

    /// 对齐 Java: `FileUtil.loopFiles(File)` — 非递归列文件。
    pub fn loop_files(path: &str) -> std::io::Result<Vec<PathBuf>> {
        crate::io::file::path_util::PathUtil::loop_files(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.walkFiles(File, FileProcessor)` — 递归收集文件。
    pub fn walk_files(path: &str) -> std::io::Result<Vec<PathBuf>> {
        crate::io::file::path_util::PathUtil::walk_files(Path::new(path))
    }

    /// 对齐 Java: `FileUtil.mkdirsSafely(File)` — create_dir_all，已存在则成功。
    pub fn mkdirs_safely(path: &str) -> bool {
        fs::create_dir_all(path).is_ok()
    }

    /// 对齐 Java: `FileUtil.copyContent(File, File, boolean)`
    pub fn copy_content(src: &str, target: &str) -> std::io::Result<PathBuf> {
        crate::io::file::path_util::PathUtil::copy_content(Path::new(src), Path::new(target))
    }

    /// 对齐 Java: `FileUtil.copyFilesFromDir(File, File, boolean)`
    pub fn copy_files_from_dir(src: &str, target: &str) -> std::io::Result<PathBuf> {
        Self::copy_content(src, target)
    }

    /// 对齐 Java: `FileUtil.moveContent(File, File, boolean)`
    pub fn move_content(src: &str, target: &str) -> std::io::Result<PathBuf> {
        crate::io::file::path_util::PathUtil::move_content(Path::new(src), Path::new(target))
    }

    /// 对齐 Java: `FileUtil.isModified(File, long)` — 与 epoch millis 比较。
    pub fn is_modified(path: &str, last_modified_millis: u64) -> bool {
        Self::last_modified_time(path)
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64 != last_modified_millis)
            .unwrap_or(true)
    }

    /// 对齐 Java: `FileUtil.isModifed`（历史拼写别名）。
    pub fn is_modifed(path: &str, last_modified_millis: u64) -> bool {
        Self::is_modified(path, last_modified_millis)
    }

    /// 对齐 Java: `FileUtil.readLine(File, Charset)` — 读第一行。
    pub fn read_line(path: &str) -> std::io::Result<Option<String>> {
        let mut lines = Self::read_utf8_lines(path)?;
        Ok(if lines.is_empty() {
            None
        } else {
            Some(lines.remove(0))
        })
    }

    /// 对齐 Java: `FileUtil.writeFromStream(InputStream, File)`
    pub fn write_from_stream<R: std::io::Read>(
        path: &str,
        reader: &mut R,
    ) -> std::io::Result<u64> {
        crate::io::file::path_util::PathUtil::write_from_reader(Path::new(path), reader)
    }

    /// 对齐 Java: `FileUtil.writeToStream(File, OutputStream)`
    pub fn write_to_stream<W: Write>(path: &str, writer: &mut W) -> std::io::Result<u64> {
        crate::io::file::path_util::PathUtil::write_to_writer(Path::new(path), writer)
    }

    /// 对齐 Java: `FileUtil.writeUtf8Map` / `writeMap` — `k=v` 行。
    pub fn write_utf8_map(path: &str, entries: &[(String, String)]) -> std::io::Result<()> {
        let lines: Vec<String> = entries
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect();
        Self::write_utf8_lines(path, &lines)
    }

    /// 对齐 Java: `FileUtil.cleanInvalid(String)` — 委托 FileNameUtil。
    pub fn clean_invalid(file_name: &str) -> String {
        crate::io::file::file_name_util::FileNameUtil::clean_invalid(file_name)
    }

    /// 对齐 Java: `FileUtil.containsInvalid(String)`
    pub fn contains_invalid(file_name: &str) -> bool {
        crate::io::file::file_name_util::FileNameUtil::contains_invalid(file_name)
    }

    /// 对齐 Java: `FileUtil.isSub(File, File)`
    pub fn is_sub(parent: &str, child: &str) -> bool {
        crate::io::file::path_util::PathUtil::is_sub(Path::new(parent), Path::new(child))
    }

    /// 对齐 Java: `FileUtil.getUtf8Reader(File)` — BufReader。
    pub fn utf8_reader(path: &str) -> std::io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(path)?))
    }

    /// 对齐 Java: `FileUtil.getInputStream(File)`
    pub fn input_stream(path: &str) -> std::io::Result<File> {
        File::open(path)
    }

    /// 对齐 Java: `FileUtil.getOutputStream(File)`
    pub fn output_stream(path: &str) -> std::io::Result<File> {
        Self::mk_parent_dirs(path)?;
        File::create(path)
    }

    /// 对齐 Java: `FileUtil.convertLineSeparator` — 统一换行。
    pub fn convert_line_separator(path: &str, sep: &str) -> std::io::Result<()> {
        let lines = Self::read_utf8_lines(path)?;
        let content = lines.join(sep);
        let mut out = content;
        if !out.ends_with(sep) && !lines.is_empty() {
            out.push_str(sep);
        }
        Self::write_utf8_string(path, &out)
    }
}

/// IEEE CRC-32（对齐 Java `java.util.zip.CRC32`）。
fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (!(crc & 1)).wrapping_add(1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
