//! advanced_codec parity tests
//! 对齐: hutool-core Base32Test/Base58Test/Base62Test/BCDTest/CaesarTest/MorseTest/PunyCodeTest/RotTest

use hutool_core::{
    HashIds, MorseCodec, base32_decode, base32_encode, base32_hex_decode, base32_hex_encode,
    base58_decode, base58_encode, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode, bcd_decode, bcd_encode, caesar_decode, caesar_encode, punycode_decode,
    punycode_encode, rot_decode, rot_encode,
};

// ── Base32 ──

#[test]
fn base32_encode_decode() {
    let encoded = base32_encode("hello world");
    let decoded = base32_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

#[test]
fn base32_hex_encode_decode() {
    let encoded = base32_hex_encode("hello world");
    let decoded = base32_hex_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

// ── Base58 ──

#[test]
fn base58_encode_decode() {
    let encoded = base58_encode("hello world");
    let decoded = base58_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

// ── Base62 ──

#[test]
fn base62_encode_decode() {
    let encoded = base62_encode("hello world");
    let decoded = base62_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

#[test]
fn base62_inverted_encode_decode() {
    let encoded = base62_inverted_encode("hello world");
    let decoded = base62_inverted_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decoded).unwrap(), "hello world");
}

// ── BCD ──

#[test]
fn bcd_encode_decode() {
    let encoded = bcd_encode("12345").unwrap();
    let decoded = bcd_decode(&encoded);
    assert_eq!(decoded, "012345"); // BCD pads odd-length with leading 0
}

// ── Caesar ──

#[test]
fn caesar_encode_decode() {
    let encoded = caesar_encode("hello", 3);
    let decoded = caesar_decode(&encoded, 3);
    assert_eq!(decoded, "hello");
}

// ── ROT ──

#[test]
fn rot13_encode_decode() {
    let encoded = rot_encode("hello", 13, false);
    let decoded = rot_decode(&encoded, 13, false);
    assert_eq!(decoded, "hello");
}

// ── Punycode ──

#[test]
fn punycode_encode_decode() {
    let encoded = punycode_encode("münchen").unwrap();
    let decoded = punycode_decode(&encoded).unwrap();
    assert_eq!(decoded, "münchen");
}

// ── MorseCodec ──

#[test]
fn morse_codec_encode_decode() {
    let codec = MorseCodec::new('.', '-', ' ').unwrap();
    let encoded = codec.encode("SOS");
    let decoded = codec.decode(&encoded).unwrap();
    assert_eq!(decoded, "SOS");
}

// ── HashIds ──

#[test]
fn hashids_encode_decode() {
    let hashids = HashIds::new("salt", 0).unwrap();
    let encoded = hashids.encode(&[12345]);
    let decoded = hashids.decode(&encoded).unwrap();
    assert_eq!(decoded, vec![12345]);
}
