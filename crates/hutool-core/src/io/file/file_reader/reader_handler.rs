//! 对齐: `cn.hutool.core.io.file.FileReader`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/file/FileReader.java
//!
//! 文件读取门面；委托 [`crate::FileUtil`] / [`crate::IoUtil`]。

/// 对齐 Java: `FileReader.ReaderHandler` — 函数式读处理器别名。
pub type ReaderHandler<R> = Box<dyn FnOnce(&[u8]) -> R>;
