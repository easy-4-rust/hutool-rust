//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

use std::{
    io::{Read, Write},
    path::Path,
};

use base64::Engine as _;
use encoding_rs::Encoding;

use crate::{CoreError, Result};

mod base16_codec;
mod decoder;
mod encoder;
mod percent_codec;

pub use base16_codec::Base16Codec;
pub use decoder::Decoder;
pub use encoder::Encoder;
pub use percent_codec::PercentCodec;

/// 按配置 Base64 编码（支持换行/URL 安全）。
pub fn base64_encode_config(input: &[u8], multiline: bool, url_safe: bool) -> String {
    let encoded = if url_safe {
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
    } else {
        base64::engine::general_purpose::STANDARD.encode(input)
    };
    if !multiline || encoded.len() <= 76 {
        return encoded;
    }
    let mut output = String::with_capacity(encoded.len() + encoded.len() / 76 * 2);
    for (index, character) in encoded.chars().enumerate() {
        if index > 0 && index % 76 == 0 {
            output.push_str("\r\n");
        }
        output.push(character);
    }
    output
}

/// 无填充 Base64 编码。
pub fn base64_encode_without_padding(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(input)
}

/// 宽松 Base64 解码（忽略非法字符）。
pub fn base64_decode_tolerant(input: impl AsRef<[u8]>) -> Vec<u8> {
    let sextets: Vec<u8> = input
        .as_ref()
        .iter()
        .filter_map(|byte| base64_sextet(*byte))
        .collect();
    let mut output = Vec::with_capacity(sextets.len().saturating_mul(3) / 4);
    for chunk in sextets.chunks(4) {
        if chunk.len() >= 2 {
            output.push((chunk[0] << 2) | (chunk[1] >> 4));
        }
        if chunk.len() >= 3 {
            output.push((chunk[1] << 4) | (chunk[2] >> 2));
        }
        if chunk.len() == 4 {
            output.push((chunk[2] << 6) | chunk[3]);
        }
    }
    output
}

/// 宽松解码指定范围子串。
///
/// # Errors
///
/// 范围越界时返回 [`CoreError::Codec`]。
pub fn base64_decode_range_tolerant(
    input: &[u8],
    position: usize,
    length: usize,
) -> Result<Vec<u8>> {
    let end = position
        .checked_add(length)
        .filter(|end| *end <= input.len())
        .ok_or_else(|| CoreError::Codec("Base64 range is out of bounds".into()))?;
    Ok(base64_decode_tolerant(&input[position..end]))
}

/// 是否 Base64 码字符。
pub const fn is_base64_code(byte: u8) -> bool {
    matches!(
        byte,
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'+' | b'-' | b'/' | b'_' | b'='
    )
}

/// 是否合法 Base64 文本。
pub fn is_base64(input: impl AsRef<[u8]>) -> bool {
    let input = input.as_ref();
    if input.len() < 3 || !input.is_ascii() {
        return false;
    }
    let mut padding = false;
    for byte in input {
        if padding {
            if *byte != b'=' {
                return false;
            }
        } else if *byte == b'=' {
            padding = true;
        } else if !is_base64_code(*byte) && !matches!(*byte, b' ' | b'\n' | b'\r' | b'\t') {
            return false;
        }
    }
    true
}

/// 按标签解析字符集。
///
/// # Errors
///
/// 未知字符集时返回 [`CoreError::Codec`]。
pub fn encoding_for_label(label: &str) -> Result<&'static Encoding> {
    Encoding::for_label(label.as_bytes())
        .ok_or_else(|| CoreError::Codec(format!("unknown character encoding: {label}")))
}

/// 按字符集编码后 Base64 编码。
pub fn base64_encode_text(input: &str, encoding: &'static Encoding, url_safe: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    base64_encode_config(&bytes, false, url_safe)
}

/// Base64 解码后按字符集解码为文本。
pub fn base64_decode_text(input: &str, encoding: &'static Encoding) -> String {
    let bytes = base64_decode_tolerant(input);
    encoding.decode(&bytes).0.into_owned()
}

/// 从 Reader 读取并 Base64 编码。
///
/// # Errors
///
/// 读取失败时返回 IO 错误。
pub fn base64_encode_reader(
    mut reader: impl Read,
    multiline: bool,
    url_safe: bool,
) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(base64_encode_config(&input, multiline, url_safe))
}

/// 从文件读取并 Base64 编码。
///
/// # Errors
///
/// 打开/读取文件失败时返回 IO 错误。
pub fn base64_encode_file(path: impl AsRef<Path>, url_safe: bool) -> Result<String> {
    base64_encode_reader(std::fs::File::open(path)?, false, url_safe)
}

/// Base64 解码并写入 Writer。
///
/// # Errors
///
/// 写入失败时返回 IO 错误。
pub fn base64_decode_to_writer(input: &str, mut writer: impl Write) -> Result<usize> {
    let decoded = base64_decode_tolerant(input);
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Base64 解码并写入文件。
///
/// # Errors
///
/// 创建/写入文件失败时返回 IO 错误。
pub fn base64_decode_to_file(input: &str, path: impl AsRef<Path>) -> Result<usize> {
    base64_decode_to_writer(input, std::fs::File::create(path)?)
}

fn base64_sextet(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' | b'-' => Some(62),
        b'/' | b'_' => Some(63),
        _ => None,
    }
}
