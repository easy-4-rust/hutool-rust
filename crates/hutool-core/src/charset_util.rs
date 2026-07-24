use std::{
    fmt, fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

use encoding_rs::{Encoding, GBK, UTF_8};
use thiserror::Error;

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// A Java-compatible character set, including encodings intentionally absent from WHATWG.
#[derive(Clone, Copy)]
pub enum Charset {
    /// Exact ISO-8859-1 rather than the WHATWG Windows-1252 alias.
    Iso88591,
    /// Strict seven-bit US-ASCII.
    Ascii,
    /// BOM-aware UTF-16 with big-endian fallback.
    Utf16,
    /// A mature WHATWG encoding supplied by `encoding_rs`.
    Encoding(&'static Encoding),
}

impl Charset {
    /// ISO-8859-1 with its exact one-byte mapping.
    pub const ISO_8859_1: Self = Self::Iso88591;
    /// UTF-8, Rust's native string encoding.
    pub const UTF_8: Self = Self::Encoding(UTF_8);
    /// GBK backed by `encoding_rs`.
    pub const GBK: Self = Self::Encoding(GBK);
    /// BOM-aware UTF-16, defaulting to big endian when no BOM is present.
    pub const UTF_16: Self = Self::Utf16;

    /// Returns the canonical Java-style charset name.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Iso88591 => "ISO-8859-1",
            Self::Ascii => "US-ASCII",
            Self::Utf16 => "UTF-16",
            Self::Encoding(encoding) => encoding.name(),
        }
    }

    fn for_label(label: &str) -> Option<Self> {
        if is_iso_8859_1_label(label) {
            Some(Self::Iso88591)
        } else if is_ascii_label(label) {
            Some(Self::Ascii)
        } else if label.eq_ignore_ascii_case("UTF-16") || label.eq_ignore_ascii_case("UNICODE") {
            Some(Self::Utf16)
        } else {
            Encoding::for_label(label.as_bytes()).map(Self::Encoding)
        }
    }

    fn encode(self, source: &str) -> Vec<u8> {
        match self {
            Self::Iso88591 => source
                .chars()
                .map(|character| u8::try_from(u32::from(character)).unwrap_or(b'?'))
                .collect(),
            Self::Ascii => source
                .chars()
                .map(|character| {
                    u8::try_from(u32::from(character))
                        .ok()
                        .filter(u8::is_ascii)
                        .unwrap_or(b'?')
                })
                .collect(),
            Self::Utf16 => std::iter::once(0xFE)
                .chain(std::iter::once(0xFF))
                .chain(source.encode_utf16().flat_map(u16::to_be_bytes))
                .collect(),
            Self::Encoding(encoding) => encoding.encode(source).0.into_owned(),
        }
    }

    fn decode(self, bytes: &[u8]) -> String {
        match self {
            Self::Iso88591 => bytes.iter().map(|byte| char::from(*byte)).collect(),
            Self::Ascii => bytes
                .iter()
                .map(|byte| {
                    if byte.is_ascii() {
                        char::from(*byte)
                    } else {
                        char::REPLACEMENT_CHARACTER
                    }
                })
                .collect(),
            Self::Utf16 => decode_utf16(bytes),
            Self::Encoding(encoding) => encoding.decode(bytes).0.into_owned(),
        }
    }

    fn identifies(self, bytes: &[u8]) -> bool {
        match self {
            Self::Iso88591 => true,
            Self::Ascii => bytes.iter().all(u8::is_ascii),
            Self::Utf16 => identify_utf16(bytes),
            Self::Encoding(encoding) => encoding
                .decode_without_bom_handling_and_without_replacement(bytes)
                .is_some(),
        }
    }
}

impl fmt::Debug for Charset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Charset")
            .field(&self.name())
            .finish()
    }
}

impl PartialEq for Charset {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::Iso88591, Self::Iso88591)
            | (Self::Ascii, Self::Ascii)
            | (Self::Utf16, Self::Utf16) => true,
            (Self::Encoding(left), Self::Encoding(right)) => std::ptr::eq(left, right),
            _ => false,
        }
    }
}

impl Eq for Charset {}

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

fn convert_file_with_writer(
    path: &Path,
    source_charset: Charset,
    destination_charset: Charset,
    writer: &mut dyn FnMut(&Path, Vec<u8>) -> io::Result<()>,
) -> Result<PathBuf, CharsetError> {
    let source = fs::read(path)?;
    let decoded = source_charset.decode(&source);
    writer(path, destination_charset.encode(&decoded))?;
    Ok(path.to_path_buf())
}

fn is_iso_8859_1_label(label: &str) -> bool {
    [
        "ISO-8859-1",
        "ISO_8859-1",
        "LATIN1",
        "LATIN-1",
        "L1",
        "IBM819",
        "CP819",
        "CSISOLATIN1",
    ]
    .iter()
    .any(|candidate| label.eq_ignore_ascii_case(candidate))
}

fn is_ascii_label(label: &str) -> bool {
    ["US-ASCII", "ASCII", "ISO646-US", "ANSI_X3.4-1968"]
        .iter()
        .any(|candidate| label.eq_ignore_ascii_case(candidate))
}

fn default_detection_charsets() -> [Charset; 7] {
    [
        Charset::UTF_8,
        Charset::GBK,
        Charset::Encoding(encoding_rs::GB18030),
        Charset::Encoding(encoding_rs::UTF_16BE),
        Charset::Encoding(encoding_rs::UTF_16LE),
        Charset::UTF_16,
        Charset::Encoding(encoding_rs::BIG5),
    ]
}

fn decode_utf16(bytes: &[u8]) -> String {
    let (bytes, little_endian) = if bytes.starts_with(&[0xFE, 0xFF]) {
        (&bytes[2..], false)
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        (&bytes[2..], true)
    } else {
        (bytes, false)
    };
    let mut chunks = bytes.chunks_exact(2);
    let units: Vec<u16> = chunks
        .by_ref()
        .map(|chunk| {
            let pair = [chunk[0], chunk[1]];
            if little_endian {
                u16::from_le_bytes(pair)
            } else {
                u16::from_be_bytes(pair)
            }
        })
        .collect();
    let mut decoded = String::from_utf16_lossy(&units);
    if !chunks.remainder().is_empty() {
        decoded.push(char::REPLACEMENT_CHARACTER);
    }
    decoded
}

fn identify_utf16(bytes: &[u8]) -> bool {
    if bytes.len() % 2 != 0 {
        return false;
    }
    let (bytes, little_endian) = if bytes.starts_with(&[0xFE, 0xFF]) {
        (&bytes[2..], false)
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        (&bytes[2..], true)
    } else {
        (bytes, false)
    };
    let units: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| {
            let pair = [chunk[0], chunk[1]];
            if little_endian {
                u16::from_le_bytes(pair)
            } else {
                u16::from_be_bytes(pair)
            }
        })
        .collect();
    String::from_utf16(&units).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct InterruptedThenData(bool);

    impl Read for InterruptedThenData {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if !self.0 {
                self.0 = true;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            buffer[0] = b'x';
            Ok(1)
        }
    }

    struct BrokenReader;

    impl Read for BrokenReader {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::other("broken"))
        }
    }

    #[test]
    fn resolution_parsing_names_and_special_java_charsets_are_explicit() {
        assert_eq!(CharsetUtil::charset(None).unwrap(), Charset::UTF_8);
        assert_eq!(CharsetUtil::charset(Some("  ")).unwrap(), Charset::UTF_8);
        assert_eq!(
            CharsetUtil::charset(Some("latin1")).unwrap(),
            Charset::ISO_8859_1
        );
        assert_eq!(CharsetUtil::charset(Some("ASCII")).unwrap(), Charset::Ascii);
        assert_eq!(
            CharsetUtil::charset(Some("unicode")).unwrap(),
            Charset::UTF_16
        );
        assert_eq!(CharsetUtil::charset(Some("GBK")).unwrap(), Charset::GBK);
        assert!(matches!(
            CharsetUtil::charset(Some("not-a-charset")),
            Err(CharsetError::Unsupported(name)) if name == "not-a-charset"
        ));
        assert_eq!(
            CharsetUtil::parse(Some("invalid"), Charset::Ascii),
            Charset::Ascii
        );
        assert_eq!(CharsetUtil::parse(None, Charset::GBK), Charset::GBK);
        assert_eq!(CharsetUtil::parse_default(Some("UTF-8")), Charset::UTF_8);
        assert_eq!(CharsetUtil::default_charset_name(), "UTF-8");
        assert_eq!(
            CharsetUtil::system_charset_name(),
            CharsetUtil::system_charset().name()
        );
        assert_eq!(
            format!("{:?}", Charset::ISO_8859_1),
            "Charset(\"ISO-8859-1\")"
        );
        assert_eq!(Charset::Ascii.name(), "US-ASCII");
        assert_eq!(Charset::UTF_16.name(), "UTF-16");
        assert_ne!(Charset::ISO_8859_1, Charset::Ascii);
        CharsetUtil::convert("text", "invalid", "UTF-8").unwrap_err();
        CharsetUtil::convert("text", "UTF-8", "invalid").unwrap_err();
    }

    #[test]
    fn string_conversion_uses_exact_single_byte_utf16_and_encoding_rs_engines() {
        let (gbk, _, _) = GBK.encode("你好");
        let mojibake: String = gbk.iter().map(|byte| char::from(*byte)).collect();
        assert_eq!(
            CharsetUtil::convert_with_charsets(
                &mojibake,
                Some(Charset::ISO_8859_1),
                Some(Charset::GBK)
            ),
            "你好"
        );
        assert_eq!(
            CharsetUtil::convert(&mojibake, "ISO-8859-1", "GBK").unwrap(),
            "你好"
        );
        assert_eq!(CharsetUtil::convert_with_charsets("", None, None), "");
        assert_eq!(
            CharsetUtil::convert_with_charsets("same", Some(Charset::UTF_8), Some(Charset::UTF_8)),
            "same"
        );
        assert_eq!(Charset::ISO_8859_1.encode("é中"), [0xE9, b'?']);
        assert_eq!(Charset::ISO_8859_1.decode(&[0xE9]), "é");
        assert_eq!(Charset::Ascii.encode("Aé"), b"A?");
        assert_eq!(Charset::Ascii.decode(&[b'A', 0xFF]), "A�");

        let utf16 = Charset::UTF_16.encode("A😀");
        assert!(utf16.starts_with(&[0xFE, 0xFF]));
        assert_eq!(Charset::UTF_16.decode(&utf16), "A😀");
        assert_eq!(Charset::UTF_16.decode(&[0xFF, 0xFE, 0x41, 0x00]), "A");
        assert_eq!(Charset::UTF_16.decode(&[0x00, 0x41]), "A");
        assert_eq!(Charset::UTF_16.decode(&[0x00]), "�");
    }

    #[test]
    fn files_and_candidate_detection_are_bounded_and_report_io_failures() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("gbk.txt");
        let (gbk, _, _) = GBK.encode("你好");
        fs::write(&path, &gbk).unwrap();
        assert_eq!(
            CharsetUtil::convert_file(&path, Charset::GBK, Charset::UTF_8).unwrap(),
            path
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "你好");
        let missing = directory.path().join("missing");
        CharsetUtil::convert_file(&missing, Charset::UTF_8, Charset::GBK).unwrap_err();
        let mut failing_writer = |_: &Path, _: Vec<u8>| Err(io::Error::other("write failed"));
        convert_file_with_writer(&path, Charset::UTF_8, Charset::GBK, &mut failing_writer)
            .unwrap_err();

        let mut reader = Cursor::new(b"hello".to_vec());
        assert_eq!(
            CharsetUtil::default_charset_from_reader(&mut reader, &[]).unwrap(),
            Some(Charset::UTF_8)
        );
        let (gbk, _, _) = GBK.encode("你好");
        let mut reader = Cursor::new(gbk.into_owned());
        assert_eq!(
            CharsetUtil::default_charset_from_reader(&mut reader, &[Charset::UTF_8, Charset::GBK])
                .unwrap(),
            Some(Charset::GBK)
        );
        let mut reader = Cursor::new(vec![0xFF]);
        assert_eq!(
            CharsetUtil::default_charset_from_reader(
                &mut reader,
                &[Charset::Ascii, Charset::ISO_8859_1]
            )
            .unwrap(),
            Some(Charset::ISO_8859_1)
        );
        let mut reader = Cursor::new(Vec::new());
        assert_eq!(
            CharsetUtil::default_charset_from_reader(&mut reader, &[]).unwrap(),
            None
        );
        let mut reader = Cursor::new(b"x".to_vec());
        CharsetUtil::default_charset_from_reader_with_buffer_size(0, &mut reader, &[]).unwrap_err();

        let mut reader = InterruptedThenData(false);
        assert_eq!(
            CharsetUtil::default_charset_from_reader(&mut reader, &[]).unwrap(),
            Some(Charset::UTF_8)
        );

        let mut reader = BrokenReader;
        CharsetUtil::default_charset_from_reader(&mut reader, &[]).unwrap_err();
    }

    #[test]
    fn strict_utf16_identification_rejects_odd_and_unpaired_inputs() {
        assert!(identify_utf16(&[0xFE, 0xFF, 0x00, 0x41]));
        assert!(identify_utf16(&[0xFF, 0xFE, 0x41, 0x00]));
        assert!(identify_utf16(&[0x00, 0x41]));
        assert!(!identify_utf16(&[0x00]));
        assert!(!identify_utf16(&[0xD8, 0x00]));
        assert!(Charset::UTF_16.identifies(&[0x00, 0x41]));
        assert!(Charset::ISO_8859_1.identifies(&[0xFF]));
        assert!(!Charset::Ascii.identifies(&[0xFF]));
    }
}
