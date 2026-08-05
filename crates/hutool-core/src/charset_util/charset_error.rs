use std::io;

use thiserror::Error;

/// Errors returned by charset resolution and bounded I/O operations.

/// 对齐: `cn.hutool.core.util.CharsetUtil`
/// 字符集错误
#[derive(Debug, Error)]
pub enum CharsetError {
    /// The requested label is not supported by `encoding_rs` or the Java compatibility layer.
    #[error("unsupported character set: {0}")]
    Unsupported(String),
    /// Detection requires a positive read buffer.
    #[error("charset detection buffer size must be positive")]
    InvalidBufferSize,
    /// A file or reader operation failed.
    #[error(transparent)]
    Io(#[from] io::Error),
}
