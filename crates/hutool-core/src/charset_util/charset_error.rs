use std::{
    fmt, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use encoding_rs::{Encoding, GBK, UTF_8};
use thiserror::Error;

/// Errors returned by charset resolution and bounded I/O operations.
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

use super::{DEFAULT_BUFFER_SIZE, convert_file_with_writer, decode_utf16, default_detection_charsets, identify_utf16, is_ascii_label, is_iso_8859_1_label};
