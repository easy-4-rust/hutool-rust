//! `cn.hutool.crypto` 扩展对比验证测试 —— 对齐 Hutool 146 个 @Test
//! 来源: hutool-crypto/src/test/java/cn/hutool/crypto/
//!
//! hutool-crypto 当前 7 个函数: sha256_hex, hmac_sha256, verify_hmac_sha256,
//! hash_password, verify_password, aes256_gcm_encrypt, aes256_gcm_decrypt
//!
//! 本文件按 Java 测试文件分组,已实现函数用真实断言,
//! 未实现函数标记 ignore 待后续实现后启用。

use hutool_crypto as hc;

// ===== SecureUtilTest 已实现方法 =====

/// 对齐 Java: SecureUtilTest.sha256Test()
#[test]
fn secure_util_sha256_test() {
    let hash = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash.len(), 64, "SHA-256 输出 64 位十六进制");
    // 验证幂等性
    let hash2 = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash, hash2, "SHA-256 幂等性");
}

/// 对齐 Java: SecureUtilTest.hmacSha256Test()
#[test]
fn secure_util_hmac_sha256_test() {
    let key = b"password";
    let msg = "test中文".as_bytes();
    let mac = hc::hmac_sha256(key, msg).unwrap();
    assert_eq!(mac.len(), 32, "HMAC-SHA256 输出 32 字节");
    assert!(hc::verify_hmac_sha256(key, msg, &mac).unwrap(), "验证成功");
}

// ===== DigestTest 已实现方法 =====

/// 对齐 Java: DigestTest.hash256Test()
#[test]
fn digest_hash256_test() {
    let hash = hc::sha256_hex("test中文".as_bytes());
    assert_eq!(hash.len(), 64);
}

// ===== Argon2Test 已实现方法 =====

/// 对齐 Java: Argon2Test.argon2Test()
#[test]
fn argon2_test() {
    let pw = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&pw).unwrap();
    assert!(
        encoded.starts_with("$argon2"),
        "Argon2 输出应以 $argon2 开头"
    );
    assert!(hc::verify_password(&pw, &encoded).unwrap());
}

/// 对齐 Java: Argon2Test.argon2WithSaltTest()
#[test]
fn argon2_with_salt_test() {
    let pw = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&pw).unwrap();
    // Argon2 内部自动加盐,验证每次生成不同
    let encoded2 = hc::hash_password(&pw).unwrap();
    assert_ne!(encoded, encoded2, "两次 hash 应不同(不同盐)");
}

// ===== AESTest 已实现方法 =====

/// 对齐 Java: AESTest.gcmTest()
#[test]
fn aes_gcm_round_trip() {
    let key = [0u8; 32];
    let plaintext = "test中文".as_bytes();
    let ct = hc::aes256_gcm_encrypt(&key, plaintext).unwrap();
    assert!(ct.len() > plaintext.len());
    let pt = hc::aes256_gcm_decrypt(&key, &ct).unwrap();
    assert_eq!(pt, plaintext);
}

/// 对齐 Java: AESTest.gcmTest() — 错误密钥
#[test]
fn aes_gcm_wrong_key() {
    let k1 = [0u8; 32];
    let k2 = [1u8; 32];
    let ct = hc::aes256_gcm_encrypt(&k1, "secret".as_bytes()).unwrap();
    assert!(hc::aes256_gcm_decrypt(&k2, &ct).is_err());
}

/// 对齐 Java: AESTest.gcmTest() — 空明文
#[test]
fn aes_gcm_empty_plaintext() {
    let key = [0u8; 32];
    let ct = hc::aes256_gcm_encrypt(&key, b"").unwrap();
    let pt = hc::aes256_gcm_decrypt(&key, &ct).unwrap();
    assert!(pt.is_empty());
}

// ===== BCryptTest 已实现方法 =====

/// 对齐 Java: BCryptTest.checkpwTest()
#[test]
fn bcrypt_checkpw_test() {
    // Hutool BCryptTest: 错误密码应验证失败
    // Rust argon2 verify_password 使用标准 Argon2id,格式不同于 Java BCrypt
    // 直接测试 Rust argon2 的密码验证逻辑
    let pw = secrecy::SecretString::from("correct_password");
    let encoded = hc::hash_password(&pw).unwrap();
    let wrong_pw = secrecy::SecretString::from("wrong_password");
    assert!(
        !hc::verify_password(&wrong_pw, &encoded).unwrap(),
        "错误密码应验证失败 (对齐 Java BCryptTest)"
    );
}

mod common;

use common::{RSA_PLAINTEXT, load_resource, rsa_pub_enc_priv_dec, sm2_sign_verify_round_trip};

// ===== Hutool parity — previously ignored, now enabled =====

/// 对齐 Java: DigestTest.md5Test()
#[test]
fn digest_md5_test() {
    let test_str = "test中文";
    assert_eq!(
        hc::md5_hex(test_str.as_bytes()),
        "5393554e94bf0eb6436f240a4fd71282"
    );
    assert_eq!(
        hc::md5_hex(test_str.as_bytes()),
        hc::md5_hex(test_str.as_bytes())
    );
}

/// 对齐 Java: DigestTest.sha1Test()
#[test]
fn digest_sha1_test() {
    let test_str = "test中文";
    assert_eq!(
        hc::sha1_hex(test_str.as_bytes()),
        "ecabf586cef0d3b11c56549433ad50b81110a836"
    );
}

/// 对齐 Java: HmacTest.hmacTest() — HmacMD5
#[test]
fn hmac_md5_test() {
    let key = b"password";
    let msg = "test中文".as_bytes();
    assert_eq!(
        hc::hmac_md5_hex(key, msg).expect("hmac"),
        "b977f4b13f93f549e06140971bded384"
    );
}

/// 对齐 Java: HmacTest.hmacSha1Test()
#[test]
fn hmac_sha1_test() {
    let key = b"password";
    let msg = "test中文".as_bytes();
    assert_eq!(
        hc::hmac_sha1_hex(key, msg).expect("hmac"),
        "1dd68d2f119d5640f0d416e99d3f42408b88d511"
    );
}

/// 对齐 Java: AESTest.encryptCBCTest()
#[test]
fn aes_cbc_test() {
    let key = b"1234567890123456";
    let iv = b"1234567890123456";
    assert_eq!(
        hc::aes128_cbc_encrypt_hex(key, iv, b"123456").expect("cbc"),
        "d637735ae9e21ba50cb686b74fab8d2c"
    );
}

/// 对齐 Java: AESTest.encryptCTSTest()
#[test]
fn aes_cts_test() {
    let key = b"0CoJUm6Qyw8W8jue";
    let iv = b"0102030405060708";
    assert_eq!(
        hc::aes128_cts_encrypt_hex(key, iv, "test中文".as_bytes()).expect("cts"),
        "8dc9de7f050e86ca2c8261dde56dfec9"
    );
}

/// 对齐 Java: RSATest.rsaTest()
#[test]
fn rsa_test() {
    let pair = hc::generate_rsa_keypair().expect("keygen");
    assert!(
        hc::rsa_private_key_to_pem(&pair.private_key)
            .expect("pem")
            .contains("PRIVATE KEY")
    );
    assert!(
        hc::rsa_public_key_to_pem(&pair.public_key)
            .expect("pem")
            .contains("PUBLIC KEY")
    );
    rsa_pub_enc_priv_dec(RSA_PLAINTEXT);
}

/// 对齐 Java: SymmetricTest.desTest()
#[test]
fn symmetric_des_test() {
    assert!(matches!(
        hc::des_encrypt(b"12345678", b"test"),
        Err(hc::CryptoError::LegacyRejected(_))
    ));
}

/// 对齐 Java: SM2Test.sm2Test()
#[test]
fn sm2_test() {
    let (secret, _public) = hc::generate_sm2_keypair().expect("sm2 keygen");
    assert_eq!(hc::sm2_private_hex_len(&secret), 64);
    sm2_sign_verify_round_trip(RSA_PLAINTEXT.as_bytes());
}

/// 对齐 Java: PemUtilTest.readPrivateKeyTest()
#[test]
fn pem_read_private_key_test() {
    let pem = load_resource("test_private_key.pem");
    assert!(hc::read_pem_private_key(&pem).is_ok());
}

/// 对齐 Java: KeyUtilTest.generateECIESKeyTest()
#[test]
fn key_util_generate_ecies_key_test() {
    let (secret, public) = hc::generate_ec_keypair().expect("ec keygen");
    assert!(hc::ec_private_pkcs8_round_trip(&secret).expect("roundtrip"));
    assert_eq!(secret.public_key(), public);
}

/// 对齐 Java: SignTest.signAndVerifyTest()
#[test]
fn sign_and_verify_test() {
    sm2_sign_verify_round_trip(b"\xe6\x88\x91\xe6\x98\xafHanley.");
}

/// 对齐 Java: OTPTest.genKeyTest()
#[test]
fn otp_gen_key_test() {
    let key = hc::generate_totp_secret_key(8).expect("secret");
    let decoded = data_encoding::BASE32_NOPAD
        .decode(key.as_bytes())
        .expect("b32");
    assert_eq!(decoded.len(), 8);
}

/// 对齐 Java: RC4Test.testCryptMessage()
#[test]
fn rc4_crypt_test() {
    assert!(matches!(
        hc::rc4_crypt(b"key", b"test"),
        Err(hc::CryptoError::LegacyRejected(_))
    ));
}

/// 对齐 Java: `TEATest.teaTest()`
#[test]
fn tea_test() {
    let key = b"MyPassword123456";
    let pt = "测试的加密数据 by Hutool".as_bytes();
    let ct = hc::tea_encrypt(key, pt).expect("tea enc");
    let dec = hc::tea_decrypt(key, &ct).expect("tea dec");
    assert_eq!(dec, pt);
}

/// 对齐 Java: SmTest.sm3Test()
#[test]
fn sm3_test() {
    assert_eq!(
        hc::sm3_hex(b"aaaaa"),
        "136ce3c86e4ed909b76082055a61586af20b4dab674732ebd4b599eef080c9be"
    );
}

/// 对齐 Java: ChaCha20Test.encryptAndDecryptTest()
#[test]
fn chacha20_encrypt_decrypt_test() {
    let key = [9u8; 32];
    let iv = [1u8; 12];
    let pt = "test中文".as_bytes();
    let ct = hc::chacha20_encrypt(&key, &iv, pt).expect("enc");
    let dec = hc::chacha20_decrypt(&key, &iv, &ct).expect("dec");
    assert_eq!(dec, pt);
}

/// 对齐 Java: PBKDF2Test.encryptTest()
#[test]
fn pbkdf2_encrypt_test() {
    let salt = [0u8; 16];
    let out = hc::pbkdf2_hex(b"123456", &salt).expect("pbkdf2");
    assert_eq!(out.len(), 128);
}

/// 对齐 Java: FPETest.ff1Test()
#[test]
fn fpe_ff1_test() {
    let fpe = hc::FpeFf1::new([1u8; 16], b"0123456789");
    let enc = fpe.encrypt("1234567890123456").expect("fpe enc");
    let dec = fpe.decrypt(&enc).expect("fpe dec");
    assert_eq!(dec, "1234567890123456");
}
