//! radix_codec module parity tests
//! 对齐: hutool-core Base32/Base58/Base62 codec tests

use hutool_core::{
    Base32Decoder, Base32Encoder, Base58Decoder, Base58Encoder, Base62Decoder, Base62Encoder,
};

// ── Base32 Encoder/Decoder ──

#[test]
fn base32_encoder_encode() {
    let encoder = Base32Encoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Some('=')).unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    assert!(!encoded.is_empty());
}

#[test]
fn base32_roundtrip() {
    let encoder = Base32Encoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Some('=')).unwrap();
    let decoder = Base32Decoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567").unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, b"hello world");
}

// ── Base58 Encoder/Decoder ──

#[test]
fn base58_encoder_encode() {
    let encoder =
        Base58Encoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    assert!(!encoded.is_empty());
}

#[test]
fn base58_roundtrip() {
    let encoder =
        Base58Encoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let decoder =
        Base58Decoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    let decoded = decoder.decode_text(&encoded).unwrap();
    assert_eq!(decoded, b"hello world");
}

// ── Base62 Encoder/Decoder ──

#[test]
fn base62_encoder_encode() {
    let encoder =
        Base62Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    assert!(!encoded.is_empty());
}

#[test]
fn base62_roundtrip() {
    let encoder =
        Base62Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let decoder =
        Base62Decoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let encoded = encoder.encode_bytes(b"hello world");
    let decoded = decoder.decode_bytes(&encoded).unwrap();
    assert_eq!(decoded, b"hello world");
}

// ── 扩展 radix_codec 测试 ──

#[test]
fn base32_standard_roundtrip() {
    let encoder = Base32Encoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", Some('=')).unwrap();
    let decoder = Base32Decoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567").unwrap();
    let inputs = vec![
        "", "a", "ab", "abc", "abcd", "abcde", "abcdef", "abcdefg", "abcdefgh",
    ];
    for input in inputs {
        let encoded = encoder.encode_bytes(input.as_bytes());
        let decoded = decoder.decode_text(&encoded);
        assert_eq!(
            decoded,
            input.as_bytes(),
            "roundtrip failed for '{}'",
            input
        );
    }
}

#[test]
fn base32_hex_roundtrip() {
    let encoder = Base32Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", Some('=')).unwrap();
    let decoder = Base32Decoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUV").unwrap();
    let input = b"Hello, World!";
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn base32_no_padding() {
    let encoder = Base32Encoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567", None).unwrap();
    let decoder = Base32Decoder::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ234567").unwrap();
    let input = b"test";
    let encoded = encoder.encode_bytes(input);
    assert!(!encoded.contains('='));
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn base58_roundtrip_various_lengths() {
    let encoder =
        Base58Encoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let decoder =
        Base58Decoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let inputs = vec![
        b"hello".as_ref(),
        b"world",
        b"test123",
        b"Hello, World!",
        &[0u8],
        &[255u8],
    ];
    for input in inputs {
        let encoded = encoder.encode_bytes(input);
        let decoded = decoder.decode_text(&encoded).unwrap();
        assert_eq!(decoded, input, "roundtrip failed");
    }
}

#[test]
fn base62_roundtrip_various_lengths() {
    let encoder =
        Base62Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let decoder =
        Base62Decoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let inputs = vec![
        b"hello".as_ref(),
        b"world",
        b"test123",
        b"Hello, World!",
        &[0u8],
        &[255u8],
    ];
    for input in inputs {
        let encoded = encoder.encode_bytes(input);
        let decoded = decoder.decode_bytes(&encoded).unwrap();
        assert_eq!(decoded, input, "roundtrip failed");
    }
}

#[test]
fn base62_inverted_roundtrip() {
    let encoder =
        Base62Encoder::new("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .unwrap();
    let decoder =
        Base62Decoder::new("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")
            .unwrap();
    let input = b"Hello, World!";
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_bytes(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn base32_standard_static() {
    let encoder = Base32Encoder::standard();
    let decoder = Base32Decoder::standard();
    let input = b"test data";
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn base32_extended_hex_static() {
    let encoder = Base32Encoder::extended_hex();
    let decoder = Base32Decoder::extended_hex();
    let input = b"test data";
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn base32_empty_input() {
    let encoder = Base32Encoder::standard();
    let decoder = Base32Decoder::standard();
    let encoded = encoder.encode_bytes(b"");
    assert!(encoded.is_empty());
    let decoded = decoder.decode_text(&encoded);
    assert!(decoded.is_empty());
}

#[test]
fn base58_empty_input() {
    let encoder =
        Base58Encoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let decoder =
        Base58Decoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let encoded = encoder.encode_bytes(b"");
    let decoded = decoder.decode_text(&encoded).unwrap();
    assert!(decoded.is_empty());
}

#[test]
fn base62_empty_input() {
    let encoder =
        Base62Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let decoder =
        Base62Decoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let encoded = encoder.encode_bytes(b"");
    let decoded = decoder.decode_bytes(&encoded).unwrap();
    assert!(decoded.is_empty());
}

#[test]
fn base32_unicode_roundtrip() {
    let encoder = Base32Encoder::standard();
    let decoder = Base32Decoder::standard();
    let input = "你好世界".as_bytes();
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}

#[test]
fn base58_unicode_roundtrip() {
    let encoder =
        Base58Encoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let decoder =
        Base58Decoder::new("123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz").unwrap();
    let input = "你好世界".as_bytes();
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_text(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn base62_unicode_roundtrip() {
    let encoder =
        Base62Encoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let decoder =
        Base62Decoder::new("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
            .unwrap();
    let input = "你好世界".as_bytes();
    let encoded = encoder.encode_bytes(input);
    let decoded = decoder.decode_bytes(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn base32_binary_data() {
    let encoder = Base32Encoder::standard();
    let decoder = Base32Decoder::standard();
    let input: Vec<u8> = (0..256).map(|i| i as u8).collect();
    let encoded = encoder.encode_bytes(&input);
    let decoded = decoder.decode_text(&encoded);
    assert_eq!(decoded, input);
}
