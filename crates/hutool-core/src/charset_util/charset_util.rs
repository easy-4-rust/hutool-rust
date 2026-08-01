/// 对齐: `cn.hutool.core.util.CharsetUtil`
/// 字符集工具类

use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};


use super::charset::Charset;
use super::charset_error::CharsetError;

/// Character-set resolution, conversion, file rewriting, and candidate detection.
#[derive(Debug, Clone, Copy, Default)]
pub struct CharsetUtil;

impl CharsetUtil {
    /// Resolves a label; a missing or blank label selects Rust's UTF-8 default.
    pub fn charset(name: Option<&str>) -> Result<Charset, CharsetError> {
        match name {
            None => Ok(Self::default_charset()),
            Some(name) if name.trim().is_empty() => Ok(Self::default_charset()),
            Some(name) => {
                Charset::for_label(name).ok_or_else(|| CharsetError::Unsupported(name.to_owned()))
            }
        }
    }

    /// Resolves a label, falling back to the supplied charset on failure or blank input.
    #[must_use]
    pub fn parse(name: Option<&str>, default: Charset) -> Charset {
        name.and_then(Charset::for_label).unwrap_or(default)
    }

    /// Resolves a label with UTF-8 as the fallback.
    #[must_use]
    pub fn parse_default(name: Option<&str>) -> Charset {
        Self::parse(name, Self::default_charset())
    }

    /// Repairs a string by re-encoding it with `source` and decoding it with `destination`.
    pub fn convert(
        source: &str,
        source_name: &str,
        destination_name: &str,
    ) -> Result<String, CharsetError> {
        let source_charset = Self::charset(Some(source_name))?;
        let destination_charset = Self::charset(Some(destination_name))?;
        Ok(Self::convert_with_charsets(
            source,
            Some(source_charset),
            Some(destination_charset),
        ))
    }

    /// Repairs a string using ISO-8859-1 and UTF-8 when either charset is absent.
    #[must_use]
    pub fn convert_with_charsets(
        source: &str,
        source_charset: Option<Charset>,
        destination_charset: Option<Charset>,
    ) -> String {
        let source_charset = source_charset.unwrap_or(Charset::ISO_8859_1);
        let destination_charset = destination_charset.unwrap_or(Charset::UTF_8);
        if source.trim().is_empty() || source_charset == destination_charset {
            return source.to_owned();
        }
        destination_charset.decode(&source_charset.encode(source))
    }

    /// Rewrites a file from one encoding to another and returns the same path.
    pub fn convert_file(
        path: &Path,
        source_charset: Charset,
        destination_charset: Charset,
    ) -> Result<PathBuf, CharsetError> {
        let mut writer = |path: &Path, bytes: Vec<u8>| fs::write(path, bytes);
        convert_file_with_writer(path, source_charset, destination_charset, &mut writer)
    }

    /// Returns the OS-oriented charset used by Hutool (`GBK` on Windows, UTF-8 elsewhere).
    #[cfg(windows)]
    #[must_use]
    pub const fn system_charset() -> Charset {
        Charset::GBK
    }

    /// Returns the OS-oriented charset used by Hutool (`GBK` on Windows, UTF-8 elsewhere).
    #[cfg(not(windows))]
    #[must_use]
    pub const fn system_charset() -> Charset {
        Charset::UTF_8
    }

    /// Returns the canonical system charset name.
    #[must_use]
    pub fn system_charset_name() -> &'static str {
        Self::system_charset().name()
    }

    /// Returns UTF-8, the native and deterministic Rust string charset.
    #[must_use]
    pub const fn default_charset() -> Charset {
        Charset::UTF_8
    }

    /// Returns the canonical Rust default charset name.
    #[must_use]
    pub fn default_charset_name() -> &'static str {
        Self::default_charset().name()
    }

    /// Detects the first charset that can strictly decode the initial bounded reader chunk.
    pub fn default_charset_from_reader(
        reader: &mut dyn Read,
        candidates: &[Charset],
    ) -> Result<Option<Charset>, CharsetError> {
        Self::default_charset_from_reader_with_buffer_size(DEFAULT_BUFFER_SIZE, reader, candidates)
    }

    /// Detects a charset using a caller-supplied positive buffer size.
    pub fn default_charset_from_reader_with_buffer_size(
        buffer_size: usize,
        reader: &mut dyn Read,
        candidates: &[Charset],
    ) -> Result<Option<Charset>, CharsetError> {
        if buffer_size == 0 {
            return Err(CharsetError::InvalidBufferSize);
        }
        let mut buffer = vec![0_u8; buffer_size];
        let read = loop {
            match reader.read(&mut buffer) {
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                result => break result?,
            }
        };
        if read == 0 {
            return Ok(None);
        }
        buffer.truncate(read);
        let defaults = default_detection_charsets();
        let candidates = if candidates.is_empty() {
            defaults.as_slice()
        } else {
            candidates
        };
        Ok(candidates
            .iter()
            .copied()
            .find(|charset| charset.identifies(&buffer)))
    }
}

use super::{DEFAULT_BUFFER_SIZE, convert_file_with_writer, default_detection_charsets};
