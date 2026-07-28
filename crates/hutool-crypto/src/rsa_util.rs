//! 对齐: `cn.hutool.crypto.asymmetric.RSA`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/asymmetric/RSA.java
//! 中文说明: RSA 非对称加密/解密与签名工具，对齐 Hutool RSA

use crate::CryptoError;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::sha2::Sha256;
use rsa::{Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

/// RSA key pair (private + public).
pub struct RsaKeyPair {
    /// PKCS#8 private key.
    pub private_key: RsaPrivateKey,
    /// SPKI public key.
    pub public_key: RsaPublicKey,
}

/// Generates a 2048-bit RSA key pair (Hutool `KeyUtil.generateKeyPair("RSA")`).
pub fn generate_rsa_keypair() -> Result<RsaKeyPair, CryptoError> {
    use rsa::rand_core::OsRng;
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).map_err(|_| CryptoError::RsaKey)?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok(RsaKeyPair {
        private_key,
        public_key,
    })
}

/// Derives the RSA public key from a private key (Hutool `KeyUtil.getRSAPublicKey`).
#[must_use]
pub fn rsa_public_from_private(private_key: &RsaPrivateKey) -> RsaPublicKey {
    RsaPublicKey::from(private_key)
}

/// Parses RSA private key from PKCS#1/ PKCS#8 PEM (Hutool `PemUtil.readPemPrivateKey`).
pub fn rsa_private_key_from_pem(pem: &str) -> Result<RsaPrivateKey, CryptoError> {
    RsaPrivateKey::from_pkcs1_pem(pem)
        .or_else(|_| RsaPrivateKey::from_pkcs8_pem(pem))
        .map_err(|_| CryptoError::InvalidPem)
}

/// Parses RSA public key from SPKI PEM.
pub fn rsa_public_key_from_pem(pem: &str) -> Result<RsaPublicKey, CryptoError> {
    RsaPublicKey::from_public_key_pem(pem).map_err(|_| CryptoError::InvalidPem)
}

/// Parses RSA private key from PKCS#8 DER Base64 (Hutool `RSA(String, null)`).
pub fn rsa_private_key_from_pkcs8_base64(b64: &str) -> Result<RsaPrivateKey, CryptoError> {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    let der = STANDARD.decode(b64).map_err(|_| CryptoError::InvalidPem)?;
    RsaPrivateKey::from_pkcs8_der(&der).map_err(|_| CryptoError::InvalidPem)
}

/// Parses RSA public key from SPKI DER Base64 (Hutool `KeyUtil.generateRSAPublicKey`).
pub fn rsa_public_key_from_spki_base64(b64: &str) -> Result<RsaPublicKey, CryptoError> {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    let der = STANDARD.decode(b64).map_err(|_| CryptoError::InvalidPem)?;
    RsaPublicKey::from_public_key_der(&der).map_err(|_| CryptoError::InvalidPem)
}

/// RSA/ECB/NoPadding public-key encrypt (Hutool `RSA("RSA/ECB/NoPadding")`).
pub fn rsa_encrypt_nopadding(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    use rsa::traits::PublicKeyParts;
    let k = public_key.size();
    if plaintext.len() > k {
        return Err(CryptoError::RsaOperation);
    }
    let mut em = vec![0u8; k];
    em[k - plaintext.len()..].copy_from_slice(plaintext);
    let m = rsa::BigUint::from_bytes_be(&em);
    let c = m
        .modpow(public_key.e(), public_key.n())
        .to_bytes_be();
    let mut out = vec![0u8; k];
    out[k - c.len()..].copy_from_slice(&c);
    Ok(out)
}

pub fn rsa_encrypt_pkcs1v15(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let padding = Pkcs1v15Encrypt;
    public_key
        .encrypt(&mut rsa::rand_core::OsRng, padding, plaintext)
        .map_err(|_| CryptoError::RsaOperation)
}

/// PKCS#1 v1.5 private-key decrypt.
pub fn rsa_decrypt_pkcs1v15(
    private_key: &RsaPrivateKey,
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let padding = Pkcs1v15Encrypt;
    private_key
        .decrypt(padding, ciphertext)
        .map_err(|_| CryptoError::RsaOperation)
}

/// OAEP-SHA256 public-key encrypt (Hutool `RSA/ECB/OAEPWithSHA-1AndMGF1Padding` proxy uses OAEP).
pub fn rsa_encrypt_oaep(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let padding = Oaep::new::<Sha256>();
    public_key
        .encrypt(&mut rsa::rand_core::OsRng, padding, plaintext)
        .map_err(|_| CryptoError::RsaOperation)
}

/// OAEP-SHA256 private-key decrypt.
pub fn rsa_decrypt_oaep(
    private_key: &RsaPrivateKey,
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let padding = Oaep::new::<Sha256>();
    private_key
        .decrypt(padding, ciphertext)
        .map_err(|_| CryptoError::RsaOperation)
}

/// Encrypts and returns Base64 (Hutool `RSA.encryptBase64`).
pub fn rsa_encrypt_base64(
    public_key: &RsaPublicKey,
    plaintext: &[u8],
) -> Result<String, CryptoError> {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    Ok(STANDARD.encode(rsa_encrypt_pkcs1v15(public_key, plaintext)?))
}

/// Decrypts Base64 ciphertext (Hutool `RSA.decryptStr`).
pub fn rsa_decrypt_base64(
    private_key: &RsaPrivateKey,
    ciphertext_b64: &str,
) -> Result<Vec<u8>, CryptoError> {
    use base64::Engine as _;
    use base64::engine::general_purpose::STANDARD;
    let ct = STANDARD.decode(ciphertext_b64).map_err(|_| CryptoError::InvalidCiphertext)?;
    rsa_decrypt_pkcs1v15(private_key, &ct)
}

/// Decrypts hex ciphertext (Hutool `RSA.decrypt` with hex input).
pub fn rsa_decrypt_hex(
    private_key: &RsaPrivateKey,
    ciphertext_hex: &str,
) -> Result<Vec<u8>, CryptoError> {
    let ct = hex::decode(ciphertext_hex).map_err(|_| CryptoError::InvalidCiphertext)?;
    rsa_decrypt_pkcs1v15(private_key, &ct)
}

/// Builds RSA from hex modulus + public exponent (Hutool `RSA(BigInteger, null, BigInteger)`).
pub fn rsa_public_from_hex_modulus(
    modulus_hex: &str,
    exponent: u64,
) -> Result<RsaPublicKey, CryptoError> {
    use num_bigint::BigUint;
    let n = BigUint::parse_bytes(modulus_hex.as_bytes(), 16).ok_or(CryptoError::RsaKey)?;
    let n = rsa::BigUint::from_bytes_be(&n.to_bytes_be());
    let e = rsa::BigUint::from(exponent);
    RsaPublicKey::new(n, e).map_err(|_| CryptoError::RsaKey)
}

/// Exports private key as PKCS#8 PEM.
pub fn rsa_private_key_to_pem(key: &RsaPrivateKey) -> Result<String, CryptoError> {
    key.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map(|s| s.to_string())
        .map_err(|_| CryptoError::RsaKey)
}

/// Exports public key as SPKI PEM.
pub fn rsa_public_key_to_pem(key: &RsaPublicKey) -> Result<String, CryptoError> {
    key.to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map(|s| s.to_string())
        .map_err(|_| CryptoError::RsaKey)
}
