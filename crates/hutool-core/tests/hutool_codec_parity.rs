//! hutool_codec module parity tests
//! 对齐: hutool-core Base64Test/PercentCodec tests

use encoding_rs::UTF_8;
use hutool_core::{
    Base16Codec, PercentCodec, base64_decode_text, base64_decode_tolerant, base64_encode_config,
    base64_encode_text, base64_encode_without_padding, encoding_for_label, is_base64,
    is_base64_code,
};

// ── Base16Codec ──

#[test]
fn base16_codec_encode_decode() {
    let codec = Base16Codec::new(true);
    let encoded = codec.encode_bytes(b"hello");
    let decoded = codec.decode_text(&encoded).unwrap();
    assert_eq!(decoded, b"hello");
}

#[test]
fn base16_codec_uppercase() {
    let codec = Base16Codec::new(false);
    let encoded = codec.encode_bytes(b"hello");
    assert!(
        encoded
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
    );
}

#[test]
fn base16_roundtrip_various() {
    let codec = Base16Codec::new(true);
    let inputs: Vec<&[u8]> = vec![
        b"",
        b"a",
        b"ab",
        b"abc",
        b"Hello, World!",
        &[0u8, 1, 2, 255],
    ];
    for input in inputs {
        let encoded = codec.encode_bytes(input);
        let decoded = codec.decode_text(&encoded).unwrap();
        assert_eq!(decoded, input, "roundtrip failed");
    }
}

// ── PercentCodec ──

#[test]
fn percent_codec_encode() {
    let codec = PercentCodec::new();
    let encoded = codec.encode("hello world", UTF_8, &[]);
    assert_eq!(encoded, "%68%65%6C%6C%6F%20%77%6F%72%6C%64");
}

#[test]
fn percent_codec_special_chars() {
    let codec = PercentCodec::new();
    let encoded = codec.encode("a=b&c=d", UTF_8, &[]);
    assert!(encoded.contains("%3D"));
    assert!(encoded.contains("%26"));
}

#[test]
fn percent_codec_safe_chars() {
    let codec = PercentCodec::with_safe(vec!['=', '&']);
    let encoded = codec.encode("a=b&c=d", UTF_8, &[]);
    assert!(encoded.contains('='));
    assert!(encoded.contains('&'));
}

#[test]
fn percent_codec_space_as_plus() {
    let mut codec = PercentCodec::new();
    codec.set_encode_space_as_plus(true);
    let encoded = codec.encode("hello world", UTF_8, &[]);
    assert!(encoded.contains('+'));
}

// ── base64 ──

#[test]
fn base64_encode_config_standard() {
    let encoded = base64_encode_config(b"hello", false, false);
    assert!(!encoded.is_empty());
}

#[test]
fn base64_config_url_safe() {
    let input = b"Hello+World/Test";
    let standard = base64_encode_config(input, false, false);
    let url_safe = base64_encode_config(input, false, true);
    assert!(!url_safe.contains('+'));
    assert!(!url_safe.contains('/'));
    assert_ne!(standard, url_safe);
}

#[test]
fn base64_without_padding() {
    let input = b"Hello";
    let without_padding = base64_encode_without_padding(input);
    assert!(!without_padding.ends_with('='));
    let decoded = base64_decode_tolerant(&without_padding);
    assert_eq!(decoded, input);
}

#[test]
fn base64_decode_tolerant_basic() {
    let encoded = base64_encode_config(b"hello", false, false);
    let decoded = base64_decode_tolerant(&encoded);
    assert_eq!(decoded, b"hello");
}

// ── is_base64 ──

#[test]
fn is_base64_valid() {
    let encoded = base64_encode_config(b"hello", false, false);
    assert!(is_base64(&encoded));
}

#[test]
fn is_base64_invalid() {
    assert!(!is_base64("not!base64"));
    assert!(!is_base64(""));
}

// ── is_base64_code ──

#[test]
fn is_base64_code_chars() {
    assert!(is_base64_code(b'A'));
    assert!(is_base64_code(b'0'));
    assert!(is_base64_code(b'+'));
    assert!(is_base64_code(b'/'));
    assert!(is_base64_code(b'='));
    assert!(!is_base64_code(b'!'));
    assert!(!is_base64_code(b'@'));
}

// ── encoding_for_label ──

#[test]
fn encoding_for_label_various() {
    assert!(encoding_for_label("UTF-8").is_ok());
    assert!(encoding_for_label("utf-8").is_ok());
    assert!(encoding_for_label("GBK").is_ok());
    assert!(encoding_for_label("ISO-8859-1").is_ok());
    assert!(encoding_for_label("unknown-encoding").is_err());
}

// ── base64 text encoding ──

#[test]
fn base64_encode_decode_text_utf8() {
    let encoded = base64_encode_text("Hello, World!", UTF_8, false);
    let decoded = base64_decode_text(&encoded, UTF_8);
    assert_eq!(decoded, "Hello, World!");
}

#[test]
fn base64_encode_decode_text_url_safe() {
    let encoded = base64_encode_text("test+data/here", UTF_8, true);
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('/'));
    let decoded = base64_decode_text(&encoded, UTF_8);
    assert_eq!(decoded, "test+data/here");
}

// ── 扩展 hutool_codec 测试 ──

#[test]
fn base16_to_unicode_hex() {
    let codec = Base16Codec::new(true);
    let hex = codec.to_unicode_hex('A');
    assert!(!hex.is_empty());
    assert!(!hex.is_empty());
}

#[test]
fn base16_append_hex() {
    let codec = Base16Codec::new(true);
    let mut output = String::new();
    codec.append_hex(&mut output, 0xFF);
    assert_eq!(output, "ff");
}

#[test]
fn base64_decode_range_tolerant() {
    let encoded = base64_encode_config(b"Hello, World!", false, false);
    let decoded =
        hutool_core::base64_decode_range_tolerant(encoded.as_bytes(), 0, encoded.len()).unwrap();
    assert_eq!(decoded, b"Hello, World!");
}

#[test]
fn percent_codec_add_remove_safe() {
    let mut codec = PercentCodec::new();
    codec.add_safe('=');
    assert!(codec.encode("a=b", UTF_8, &[]).contains('='));
    codec.remove_safe('=');
    assert!(!codec.encode("a=b", UTF_8, &[]).contains('='));
}

#[test]
fn percent_codec_union() {
    let mut codec1 = PercentCodec::with_safe(vec!['=']);
    let codec2 = PercentCodec::with_safe(vec!['&']);
    codec1.union(&codec2);
    let encoded = codec1.encode("a=b&c", UTF_8, &[]);
    assert!(encoded.contains('='));
    assert!(encoded.contains('&'));
}

#[test]
fn percent_codec_union_new() {
    let codec1 = PercentCodec::with_safe(vec!['=']);
    let codec2 = PercentCodec::with_safe(vec!['&']);
    let combined = codec1.union_new(&codec2);
    let encoded = combined.encode("a=b&c", UTF_8, &[]);
    assert!(encoded.contains('='));
    assert!(encoded.contains('&'));
}
