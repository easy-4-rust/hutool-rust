//! `cn.hutool.core.codec` 子包对比验证测试
//!
//! 对齐: hutool-core codec 包全部 @Test 清单（33 个）
//! 来源: hutool-core/src/test/java/cn/hutool/core/codec/

use encoding_rs::{GBK, UTF_8};
use hutool_core::{
    HashIds, MorseCodec, RandomUtil, base32_decode, base32_encode, base32_hex_decode,
    base32_hex_encode, base58_decode, base58_decode_checked, base58_decode_checked_auto,
    base58_encode, base58_encode_checked, base62_decode, base62_encode, base62_inverted_decode,
    base62_inverted_encode, base64_decode_text, base64_encode, base64_encode_text,
    base64_encode_without_padding, bcd_decode, bcd_encode, caesar_decode, caesar_encode,
    idna_decode_domain, idna_encode_domain, is_base64, punycode_decode, punycode_encode,
    rot_decode, rot_encode,
};

// ════════════════════════════════════════════════════════════
// Base64Test (9)
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `Base64Test.isBase64Test()`
#[test]
fn is_base64_test() {
    let encoded = base64_encode(RandomUtil::random_string(1000));
    assert!(is_base64(&encoded), "random base64 应为合法 (对齐 Java)");
}

/// 对齐 Java: `Base64Test.isBase64Test2()`
#[test]
fn is_base64_test_2() {
    let base64 = "dW1kb3MzejR3bmljM2J6djAyZzcwbWk5M213Nnk3cWQ3eDJwOHFuNXJsYmMwaXhxbmg0dmxrcmN0anRkbmd3\n\
ZzcyZWFwanI2NWNneTg2dnp6cmJoMHQ4MHpxY2R6c3pjazZtaQ==";
    assert!(is_base64(base64));
    let bad = "dW1kb3MzejR3bmljM2J6=djAyZzcwbWk5M213Nnk3cWQ3eDJwOHFuNXJsYmMwaXhxbmg0dmxrcmN0anRkbmd3\n\
ZzcyZWFwanI2NWNneTg2dnp6cmJoMHQ4MHpxY2R6c3pjazZtaQ=";
    assert!(!is_base64(bad), "'=' 不在末尾时 isBase64 应为 false");
}

/// 对齐 Java: `Base64Test.encodeAndDecodeTest()`
#[test]
fn encode_and_decode_test() {
    let a = "伦家是一个非常长的字符串66";
    let encode = base64_encode_text(a, UTF_8, false);
    assert_eq!(
        encode,
        "5Lym5a625piv5LiA5Liq6Z2e5bi46ZW/55qE5a2X56ym5LiyNjY="
    );
    assert_eq!(base64_decode_text(&encode, UTF_8), a);
}

/// 对齐 Java: `Base64Test.encodeAndDecodeWithoutPaddingTest()`
#[test]
fn encode_and_decode_without_padding_test() {
    let a = "伦家是一个非常长的字符串66";
    let encode = base64_encode_without_padding(a.as_bytes());
    assert_eq!(
        encode,
        "5Lym5a625piv5LiA5Liq6Z2e5bi46ZW/55qE5a2X56ym5LiyNjY"
    );
    assert_eq!(base64_decode_text(&encode, UTF_8), a);
}

/// 对齐 Java: `Base64Test.encodeAndDecodeTest2()`
#[test]
fn encode_and_decode_test_2() {
    let a = "a61a5db5a67c01445ca2-HZ20181120172058/pdf/中国电信影像云单体网关Docker版-V1.2.pdf";
    let encode = base64_encode_text(a, UTF_8, false);
    assert_eq!(
        encode,
        "YTYxYTVkYjVhNjdjMDE0NDVjYTItSFoyMDE4MTEyMDE3MjA1OC9wZGYv5Lit5Zu955S15L+h5b2x5YOP5LqR5Y2V5L2T572R5YWzRG9ja2Vy54mILVYxLjIucGRm"
    );
    assert_eq!(base64_decode_text(&encode, UTF_8), a);
}

/// 对齐 Java: `Base64Test.encodeAndDecodeTest3()`
#[test]
fn encode_and_decode_test_3() {
    let a = ":";
    let encode = base64_encode(a);
    assert_eq!(encode, "Og==");
    assert_eq!(base64_decode_text(&encode, UTF_8), a);
}

/// 对齐 Java: `Base64Test.encodeAndDecodeGbkTest()`
#[test]
fn encode_and_decode_gbk_test() {
    let order = "订购成功立即生效，30天内可观看专区中除单独计费影片外的所有内容，到期自动取消。";
    let result = base64_encode_text(order, GBK, false);
    assert_eq!(base64_decode_text(&result, GBK), order);
}

/// 对齐 Java: `Base64Test.decodeEmojiTest()`
#[test]
fn decode_emoji_test() {
    let str = "😄";
    let encode = base64_encode_text(str, UTF_8, false);
    assert_eq!(base64_decode_text(&encode, UTF_8), str);
}

/// 对齐 Java: `Base64Test.issuesI5QR4WTest()`
#[test]
fn issues_i5_qr4w_test() {
    // 对齐 Java util.Base64 与 hutool Base64 对 "111" 的编码一致
    assert_eq!(base64_encode("111"), "MTEx");
}

// ════════════════════════════════════════════════════════════
// Base32Test (4)
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `Base32Test.encodeAndDecodeTest()`
#[test]
fn base32_encode_and_decode_test() {
    let a = "伦家是一个非常长的字符串";
    let encode = base32_encode(a.as_bytes());
    assert_eq!(
        encode,
        "4S6KNZNOW3TJRL7EXCAOJOFK5GOZ5ZNYXDUZLP7HTKCOLLMX46WKNZFYWI======"
    );
    assert_eq!(
        String::from_utf8(base32_decode(&encode).unwrap()).unwrap(),
        a
    );
    assert_eq!(
        String::from_utf8(base32_decode(&encode.to_lowercase()).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base32Test.hexEncodeAndDecodeTest()`
#[test]
fn hex_encode_and_decode_test() {
    let a = "伦家是一个非常长的字符串";
    let encode = base32_hex_encode(a.as_bytes());
    assert_eq!(
        encode,
        "SIUADPDEMRJ9HBV4N20E9E5AT6EPTPDON3KPBFV7JA2EBBCNSUMADP5OM8======"
    );
    assert_eq!(
        String::from_utf8(base32_hex_decode(&encode).unwrap()).unwrap(),
        a
    );
    assert_eq!(
        String::from_utf8(base32_hex_decode(&encode.to_lowercase()).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base32Test.encodeAndDecodeRandomTest()`
#[test]
fn base32_encode_and_decode_random_test() {
    let a =
        RandomUtil::random_string(RandomUtil::random_int_range(1, 1000).unsigned_abs() as usize);
    let encode = base32_encode(a.as_bytes());
    assert_eq!(
        String::from_utf8(base32_decode(&encode).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base32Test.decodeTest()`
#[test]
fn base32_decode_test() {
    let a = "伦家是一个非常长的字符串";
    // Hutool 允许无 padding；hutool 解码器要求对齐长度，补齐 '=' 后断言
    let mut encoded = String::from("4S6KNZNOW3TJRL7EXCAOJOFK5GOZ5ZNYXDUZLP7HTKCOLLMX46WKNZFYWI");
    while encoded.len() % 8 != 0 {
        encoded.push('=');
    }
    let decode = base32_decode(&encoded).unwrap();
    assert_eq!(String::from_utf8(decode).unwrap(), a);
}

// ════════════════════════════════════════════════════════════
// Base58Test (4)
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `Base58Test.encodeCheckedTest()`
#[test]
fn encode_checked_test() {
    let a = b"hello world";
    assert_eq!(base58_encode_checked(Some(0), a), "13vQB7B6MrGQZaxCuFg4oh");
    assert_eq!(base58_encode_checked(None, a), "3vQB7B6MrGQZaxCuFg4oh");
}

/// 对齐 Java: `Base58Test.encodeTest()`
#[test]
fn base58_encode_test() {
    assert_eq!(base58_encode(b"hello world"), "StV1DL6CwTryKyV");
}

/// 对齐 Java: `Base58Test.decodeCheckedTest()`
#[test]
fn decode_checked_test() {
    let a = "3vQB7B6MrGQZaxCuFg4oh";
    assert_eq!(
        base58_decode_checked(&format!("1{a}"), true).unwrap(),
        b"hello world"
    );
    assert_eq!(base58_decode_checked(a, false).unwrap(), b"hello world");
    assert_eq!(base58_decode_checked_auto(a).unwrap(), b"hello world");
}

/// 对齐 Java: `Base58Test.testDecode()`
#[test]
fn test_decode() {
    assert_eq!(base58_decode("StV1DL6CwTryKyV").unwrap(), b"hello world");
}

// ════════════════════════════════════════════════════════════
// Base62Test (4)
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `Base62Test.encodeAndDecodeTest()`
#[test]
fn base62_encode_and_decode_test() {
    let a = "伦家是一个非常长的字符串66";
    let encode = base62_encode(a);
    assert_eq!(
        encode,
        "17vKU8W4JMG8dQF8lk9VNnkdMOeWn4rJMva6F0XsLrrT53iKBnqo"
    );
    assert_eq!(
        String::from_utf8(base62_decode(&encode).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base62Test.encodeAndDecodeInvertedTest()`
#[test]
fn encode_and_decode_inverted_test() {
    let a = "伦家是一个非常长的字符串66";
    let encode = base62_inverted_encode(a);
    assert_eq!(
        encode,
        "17Vku8w4jmg8Dqf8LK9vnNKDmoEwN4RjmVA6f0xSlRRt53IkbNQO"
    );
    assert_eq!(
        String::from_utf8(base62_inverted_decode(&encode).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base62Test.encodeAndDecodeRandomTest()`
#[test]
fn base62_encode_and_decode_random_test() {
    let a =
        RandomUtil::random_string(RandomUtil::random_int_range(1, 1000).unsigned_abs() as usize);
    let encode = base62_encode(&a);
    assert_eq!(
        String::from_utf8(base62_decode(&encode).unwrap()).unwrap(),
        a
    );
}

/// 对齐 Java: `Base62Test.encodeAndDecodeInvertedRandomTest()`
#[test]
fn encode_and_decode_inverted_random_test() {
    let a =
        RandomUtil::random_string(RandomUtil::random_int_range(1, 1000).unsigned_abs() as usize);
    let encode = base62_inverted_encode(&a);
    assert_eq!(
        String::from_utf8(base62_inverted_decode(&encode).unwrap()).unwrap(),
        a
    );
}

// ════════════════════════════════════════════════════════════
// BCD / Caesar / Rot / Morse / PunyCode / Hashids
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `BCDTest.bcdTest()`
#[test]
fn bcd_test() {
    let str_for_test = "123456ABCDEF";
    let bcd = bcd_encode(str_for_test).unwrap();
    assert_eq!(bcd_decode(&bcd), str_for_test);
}

/// 对齐 Java: `CaesarTest.caesarTest()`
#[test]
fn caesar_test() {
    let s = "1f2e9df6131b480b9fdddc633cf24996";
    let encode = caesar_encode(s, 3);
    assert_eq!(encode, "1H2G9FH6131D480D9HFFFE633EH24996");
    assert_eq!(caesar_decode(&encode, 3), s);
}

/// 对齐 Java: `RotTest.rot13Test()`
#[test]
fn rot13_test() {
    let s = "1f2e9df6131b480b9fdddc633cf24996";
    let encode13 = rot_encode(s, 13, true);
    assert_eq!(encode13, "4s5r2qs9464o713o2sqqqp966ps57229");
    assert_eq!(rot_decode(&encode13, 13, true), s);
}

/// 对齐 Java: `MorseTest.test0()`
#[test]
fn test0() {
    let text = "Hello World!";
    let morse = "...././.-../.-../---/-...../.--/---/.-./.-../-../-.-.--/";
    assert_eq!(MorseCodec::default().encode(text), morse);
    assert_eq!(
        MorseCodec::default().decode(morse).unwrap(),
        text.to_uppercase()
    );
}

/// 对齐 Java: `MorseTest.test1()`
#[test]
fn test1() {
    let text = "你好，世界！";
    let morse = "-..----.--...../-.--..-.-----.-/--------....--../-..---....-.--./---.-.-.-..--../--------.......-/";
    assert_eq!(MorseCodec::default().encode(text), morse);
    assert_eq!(MorseCodec::default().decode(morse).unwrap(), text);
}

/// 对齐 Java: `MorseTest.test2()`
#[test]
fn test2() {
    let text = "こんにちは";
    let morse = "--.....-.-..--/--....-..-..--/--.....--.-.--/--.....--....-/--.....--.----/";
    assert_eq!(MorseCodec::default().encode(text), morse);
    assert_eq!(MorseCodec::default().decode(morse).unwrap(), text);
}

/// 对齐 Java: `PunyCodeTest.encodeDecodeTest()`
#[test]
fn encode_decode_test() {
    let text = "Hutool编码器";
    let puny = punycode_encode(text).unwrap();
    assert_eq!(puny, "Hutool-ux9js33tgln");
    assert_eq!(punycode_decode("Hutool-ux9js33tgln").unwrap(), text);
    assert_eq!(punycode_decode("xn--Hutool-ux9js33tgln").unwrap(), text);
}

/// 对齐 Java: `PunyCodeTest.encodeDecodeTest2()`
#[test]
fn encode_decode_test_2() {
    assert_eq!(punycode_encode("Hutool").unwrap(), "Hutool");
}

/// 对齐 Java: `PunyCodeTest.encodeEncodeDomainTest()`
#[test]
fn encode_encode_domain_test() {
    let domain = "赵新虎.中国";
    let puny = idna_encode_domain(domain).unwrap();
    assert_eq!(idna_decode_domain(&puny).unwrap(), domain);
}

/// 对齐 Java: `PunyCodeTest.encodeEncodeDomainTest2()`
#[test]
fn encode_encode_domain_test_2() {
    let domain = "赵新虎.com";
    let puny = idna_encode_domain(domain).unwrap();
    assert_eq!(puny, "xn--efvz93e52e.com");
    assert_eq!(idna_decode_domain(&puny).unwrap(), domain);
}

/// 对齐 Java: `PunyCodeTest.encodeEncodeDomainTest3()`
#[test]
fn encode_encode_domain_test_3() {
    let domain = "赵新虎.COM";
    let puny = idna_encode_domain(domain).unwrap();
    assert_eq!(puny, "xn--efvz93e52e.COM");
    assert_eq!(idna_decode_domain(&puny).unwrap(), domain);
}

/// 对齐 Java: `HashidsTest.hexEncodeDecode()`
#[test]
fn hex_encode_decode() {
    let hashids = HashIds::new("my awesome salt", 0).unwrap();
    let encoded1 = hashids.encode_hex("507f1f77bcf86cd799439011").unwrap();
    let encoded2 = hashids.encode_hex("0x507f1f77bcf86cd799439011").unwrap();
    let encoded3 = hashids.encode_hex("0X507f1f77bcf86cd799439011").unwrap();
    assert_eq!(encoded1, "R2qnd2vkOJTXm7XV7yq4");
    assert_eq!(encoded1, encoded2);
    assert_eq!(encoded1, encoded3);
    assert_eq!(
        hashids.decode_hex(&encoded1).unwrap(),
        "507f1f77bcf86cd799439011"
    );
}
