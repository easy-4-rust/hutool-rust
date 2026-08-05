//! Configurable radix codecs and Rust-native stream/file overloads.

use std::{
    io::{Read, Write},
    path::Path,
};

use encoding_rs::{Encoding, GBK};

use crate::{
    CoreError, Result, advanced_codec::convert_base, base32_decode, base32_encode,
    base32_hex_decode, base32_hex_encode, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode,
};

mod base32_decoder;
mod base32_encoder;
mod base58_decoder;
mod base58_encoder;
mod base62_decoder;
mod base62_encoder;

pub use base32_decoder::Base32Decoder;
pub use base32_encoder::Base32Encoder;
pub use base58_decoder::Base58Decoder;
pub use base58_encoder::Base58Encoder;
pub use base62_decoder::Base62Decoder;
pub use base62_encoder::Base62Encoder;

const BASE32_STANDARD_BYTES: [u8; 32] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

const BASE32_HEX_BYTES: [u8; 32] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

const BASE58_BITCOIN_BYTES: [u8; 58] =
    *b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

const BASE62_GMP_BYTES: [u8; 62] =
    *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const BASE62_INVERTED_BYTES: [u8; 62] =
    *b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// 按字符集编码后 Base32 编码。
pub fn base32_encode_text(input: &str, encoding: &'static Encoding, use_hex: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    if use_hex {
        base32_hex_encode(bytes)
    } else {
        base32_encode(bytes)
    }
}

/// Base32 解码后按字符集解码为文本。
///
/// # Errors
///
/// 输入非法时返回 [`CoreError::Codec`]。
pub fn base32_decode_text(
    input: &str,
    encoding: &'static Encoding,
    use_hex: bool,
) -> Result<String> {
    let bytes = if use_hex {
        base32_hex_decode(input)?
    } else {
        base32_decode(input)?
    };
    Ok(encoding.decode(&bytes).0.into_owned())
}

/// 从 Reader 读取并 Base32 编码。
///
/// # Errors
///
/// 读取失败时返回 IO 错误。
pub fn base32_encode_reader(mut reader: impl Read, use_hex: bool) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(if use_hex {
        base32_hex_encode(input)
    } else {
        base32_encode(input)
    })
}

/// 从文件读取并 Base32 编码。
///
/// # Errors
///
/// 打开/读取文件失败时返回 IO 错误。
pub fn base32_encode_file(path: impl AsRef<Path>, use_hex: bool) -> Result<String> {
    base32_encode_reader(std::fs::File::open(path)?, use_hex)
}

/// Base32 解码并写入 Writer。
///
/// # Errors
///
/// 解码或写入失败时返回错误。
pub fn base32_decode_to_writer(
    input: &str,
    mut writer: impl Write,
    use_hex: bool,
) -> Result<usize> {
    let decoded = if use_hex {
        base32_hex_decode(input)?
    } else {
        base32_decode(input)?
    };
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Base32 解码并写入文件。
///
/// # Errors
///
/// 解码或写入失败时返回错误。
pub fn base32_decode_to_file(input: &str, path: impl AsRef<Path>, use_hex: bool) -> Result<usize> {
    base32_decode_to_writer(input, std::fs::File::create(path)?, use_hex)
}

/// 按字符集编码后 Base62 编码。
pub fn base62_encode_text(input: &str, encoding: &'static Encoding, inverted: bool) -> String {
    let (bytes, _, _) = encoding.encode(input);
    if inverted {
        base62_inverted_encode(bytes)
    } else {
        base62_encode(bytes)
    }
}

/// Base62 解码后按字符集解码为文本。
///
/// # Errors
///
/// 输入非法时返回 [`CoreError::Codec`]。
pub fn base62_decode_text(
    input: &str,
    encoding: &'static Encoding,
    inverted: bool,
) -> Result<String> {
    let bytes = if inverted {
        base62_inverted_decode(input)?
    } else {
        base62_decode(input)?
    };
    Ok(encoding.decode(&bytes).0.into_owned())
}

/// Base62 解码（GBK 字符集）。
///
/// # Errors
///
/// 输入非法时返回 [`CoreError::Codec`]。
pub fn base62_decode_text_gbk(input: &str) -> Result<String> {
    base62_decode_text(input, GBK, false)
}

/// 从 Reader 读取并 Base62 编码。
///
/// # Errors
///
/// 读取失败时返回 IO 错误。
pub fn base62_encode_reader(mut reader: impl Read, inverted: bool) -> Result<String> {
    let mut input = Vec::new();
    reader.read_to_end(&mut input)?;
    Ok(if inverted {
        base62_inverted_encode(input)
    } else {
        base62_encode(input)
    })
}

/// 从文件读取并 Base62 编码。
///
/// # Errors
///
/// 打开/读取文件失败时返回 IO 错误。
pub fn base62_encode_file(path: impl AsRef<Path>, inverted: bool) -> Result<String> {
    base62_encode_reader(std::fs::File::open(path)?, inverted)
}

/// Base62 解码并写入 Writer。
///
/// # Errors
///
/// 解码或写入失败时返回错误。
pub fn base62_decode_to_writer(
    input: &str,
    mut writer: impl Write,
    inverted: bool,
) -> Result<usize> {
    let decoded = if inverted {
        base62_inverted_decode(input)?
    } else {
        base62_decode(input)?
    };
    writer.write_all(&decoded)?;
    Ok(decoded.len())
}

/// Base62 解码并写入文件。
///
/// # Errors
///
/// 解码或写入失败时返回错误。
pub fn base62_decode_to_file(input: &str, path: impl AsRef<Path>, inverted: bool) -> Result<usize> {
    base62_decode_to_writer(input, std::fs::File::create(path)?, inverted)
}

/// BCD 编码（取前 `length` 字节）。
///
/// # Errors
///
/// `length` 超出输入长度或含非十六进制字节时返回 [`CoreError::Codec`]。
pub fn bcd_encode_ascii_prefix(input: &[u8], length: usize) -> Result<Vec<u8>> {
    if length > input.len() {
        return Err(CoreError::Codec("BCD prefix exceeds input length".into()));
    }
    input[..length]
        .chunks(2)
        .map(|pair| {
            let high = bcd_nibble(pair[0])?;
            let low = pair.get(1).copied().map_or(Ok(0), bcd_nibble)?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn validate_alphabet<const N: usize>(alphabet: &str, name: &str) -> Result<[u8; N]> {
    let bytes: [u8; N] = alphabet.as_bytes().try_into().map_err(|_| {
        CoreError::Codec(format!(
            "{name} alphabet must contain exactly {N} ASCII bytes"
        ))
    })?;
    if !bytes.is_ascii() {
        return Err(CoreError::Codec(format!("{name} alphabet must be ASCII")));
    }
    let mut sorted = bytes;
    sorted.sort_unstable();
    if sorted.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(CoreError::Codec(format!(
            "{name} alphabet characters must be unique"
        )));
    }
    Ok(bytes)
}

fn decode_alphabet(input: &[u8], alphabet: &[u8], radix: u32) -> Result<Vec<u8>> {
    let digits = input
        .iter()
        .enumerate()
        .map(|(index, byte)| {
            alphabet
                .iter()
                .position(|candidate| candidate == byte)
                .and_then(|digit| u8::try_from(digit).ok())
                .ok_or_else(|| CoreError::Codec(format!("invalid radix byte at index {index}")))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(convert_base(&digits, radix, 256))
}

fn bcd_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CoreError::Codec("invalid BCD hexadecimal digit".into())),
    }
}
