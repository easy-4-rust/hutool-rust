use std::{
    fs, io,
    path::{Path, PathBuf},
};

mod charset;
mod charset_error;
mod charset_util;

pub use charset::Charset;
pub use charset_error::CharsetError;
pub use charset_util::CharsetUtil;

const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

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
