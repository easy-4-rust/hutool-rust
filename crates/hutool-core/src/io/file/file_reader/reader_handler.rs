//! 对齐: `cn.hutool.core.io.file.FileReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileReader.java
//!
//! 文件读取门面；委托 [`crate::FileUtil`] / [`crate::IoUtil`]。

use crate::{FileUtil, IoUtil};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::{Path, PathBuf};

use super::file_reader::FileReader;

/// 对齐 Java: `FileReader.ReaderHandler` — 函数式读处理器别名。
pub type ReaderHandler<R> = Box<dyn FnOnce(&[u8]) -> R>;
