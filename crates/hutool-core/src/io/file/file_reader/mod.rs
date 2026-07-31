//! 对齐: `cn.hutool.core.io.file.FileReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileReader.java
//!
//! 文件读取门面；委托 [`crate::FileUtil`] / [`crate::IoUtil`]。

mod file_reader;
mod reader_handler;

pub use file_reader::FileReader;
pub use reader_handler::ReaderHandler;
