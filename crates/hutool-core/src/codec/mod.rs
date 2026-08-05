//! Base64, hexadecimal, and URL-component codecs.
//!
//! The public operations were adapted from yimi-rutool 0.2.5 (Apache-2.0)
//! and changed to return typed errors rather than a crate-wide string error.

use base64::Engine as _;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};

use crate::Result;

/// Encodes bytes with standard padded Base64.
#[must_use]
pub fn base64_encode(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD.encode(input)
}

/// Decodes standard padded Base64.
///
/// # Errors
///
/// Returns an error when the input is not valid Base64.
pub fn base64_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    Ok(base64::engine::general_purpose::STANDARD.decode(input)?)
}

/// Encodes bytes with unpadded URL-safe Base64.
#[must_use]
pub fn base64_url_encode(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(input)
}

/// Decodes unpadded URL-safe Base64.
///
/// # Errors
///
/// Returns an error when the input is not valid URL-safe Base64.
pub fn base64_url_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(input)?)
}

/// Encodes bytes as lowercase hexadecimal text.
#[must_use]
pub fn hex_encode(input: impl AsRef<[u8]>) -> String {
    hex::encode(input)
}

/// Decodes hexadecimal text.
///
/// # Errors
///
/// Returns an error for odd-length or non-hexadecimal input.
pub fn hex_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    Ok(hex::decode(input)?)
}

/// Percent-encodes a complete URL component.
#[must_use]
pub fn percent_encode_component(input: &str) -> String {
    utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
}

/// Percent-decodes UTF-8 text.
///
/// # Errors
///
/// Returns an error when decoded bytes are not valid UTF-8.
pub fn percent_decode(input: &str) -> Result<String> {
    Ok(percent_decode_str(input).decode_utf8()?.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn base64_round_trip() {
        let encoded = base64_encode("HiTool 你好");
        assert_eq!(base64_decode(encoded).unwrap(), "HiTool 你好".as_bytes());
    }

    #[test]
    fn url_safe_base64_has_no_padding() {
        let encoded = base64_url_encode([0xfb, 0xff]);
        assert!(!encoded.contains(['+', '/', '=']));
        assert_eq!(base64_url_decode(encoded).unwrap(), [0xfb, 0xff]);
    }

    #[test]
    fn hex_round_trip_and_validation() {
        assert_eq!(hex_decode(hex_encode("abc")).unwrap(), b"abc");
        assert!(hex_decode("xyz").is_err());
    }

    #[test]
    fn percent_encoding_round_trip() {
        let encoded = percent_encode_component("a/b 你好");
        assert_eq!(percent_decode(&encoded).unwrap(), "a/b 你好");
    }

    proptest! {
        #[test]
        fn binary_codecs_round_trip(input in proptest::collection::vec(any::<u8>(), 0..4096)) {
            let base64 = base64_decode(base64_encode(&input)).unwrap();
            let base64_url = base64_url_decode(base64_url_encode(&input)).unwrap();
            prop_assert_eq!(base64.as_slice(), input.as_slice());
            prop_assert_eq!(base64_url.as_slice(), input.as_slice());
            prop_assert_eq!(hex_decode(hex_encode(&input)).unwrap(), input);
        }

        #[test]
        fn percent_codec_round_trips_unicode(input in ".{0,1024}") {
            let encoded = percent_encode_component(&input);
            prop_assert_eq!(percent_decode(&encoded).unwrap(), input);
        }
    }
}
