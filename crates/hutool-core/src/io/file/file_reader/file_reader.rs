//! 对齐: `cn.hutool.core.io.file.FileReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileReader.java
//!
//! 文件读取门面；委托 [`crate::FileUtil`] / [`crate::IoUtil`]。

use crate::{FileUtil, IoUtil};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

use super::reader_handler::ReaderHandler;

/// 对齐 Java 类: `cn.hutool.core.io.file.FileReader`
#[derive(Debug, Clone)]
pub struct FileReader {
    path: PathBuf,
}

impl FileReader {
    /// 对齐 Java: `FileReader.create(File)` / `create(String)`
    pub fn create(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// 对齐 Java: `FileReader(File)` / `FileReader(String)` 等构造。
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self::create(path)
    }

    /// 对齐 Java: `FileReader.readBytes()`
    pub fn read_bytes(&self) -> io::Result<Vec<u8>> {
        FileUtil::read_bytes(&self.path.to_string_lossy())
    }

    /// 对齐 Java: `FileReader.readString()`
    pub fn read_string(&self) -> io::Result<String> {
        FileUtil::read_utf8_string(&self.path.to_string_lossy())
    }

    /// 对齐 Java: `FileReader.readLines()`
    pub fn read_lines(&self) -> io::Result<Vec<String>> {
        FileUtil::read_utf8_lines(&self.path.to_string_lossy())
    }

    /// 对齐 Java: `FileReader.read(ReaderHandler)` — 整文件交给闭包。
    pub fn read_with<R>(&self, handler: impl FnOnce(&[u8]) -> R) -> io::Result<R> {
        let bytes = self.read_bytes()?;
        Ok(handler(&bytes))
    }

    /// 对齐 Java: `FileReader.getReader()`
    pub fn get_reader(&self) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(File::open(&self.path)?))
    }

    /// 对齐 Java: `FileReader.getInputStream()`
    pub fn get_input_stream(&self) -> io::Result<File> {
        File::open(&self.path)
    }

    /// 对齐 Java: `FileReader.writeToStream(OutputStream)`
    pub fn write_to_stream<W: Write>(&self, writer: &mut W) -> io::Result<u64> {
        let mut file = self.get_input_stream()?;
        IoUtil::copy(&mut file, writer)
    }

    /// 底层路径。
    pub fn path(&self) -> &Path {
        &self.path
    }
}
