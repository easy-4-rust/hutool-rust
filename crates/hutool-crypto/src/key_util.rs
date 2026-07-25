//! 对齐: `cn.hutool.crypto.KeyUtil`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/KeyUtil.java
//! 中文说明: 密钥生成与转换工具（RSA/EC），对齐 Hutool KeyUtil

use crate::CryptoError;
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use p256::SecretKey;
use rand_core06::RngCore;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;

/// Generates an RSA key pair (`KeyUtil.generateKeyPair("RSA")`).
pub fn generate_rsa_keypair_simple() -> Result<(RsaPrivateKey, RsaPublicKey), CryptoError> {
    let pair = crate::generate_rsa_keypair()?;
    Ok((pair.private_key, pair.public_key))
}

/// Derives RSA public key from private (`KeyUtil.getRSAPublicKey`).
pub fn rsa_public_from_private_key(private: &RsaPrivateKey) -> RsaPublicKey {
    crate::rsa_public_from_private(private)
}

/// Alias for Hutool `KeyUtil.getRSAPublicKey`.
pub fn get_rsa_public_key(private: &RsaPrivateKey) -> RsaPublicKey {
    rsa_public_from_private_key(private)
}

/// Generates an EC key pair for ECIES parity (`KeyUtil.generateKeyPair("ECIES")`).
pub fn generate_ec_keypair() -> Result<(SecretKey, p256::PublicKey), CryptoError> {
    let secret = SecretKey::random(&mut rand_core06::OsRng);
    Ok((secret.clone(), secret.public_key()))
}

/// Reconstructs EC private key from PKCS#8 bytes (`KeyUtil.generatePrivateKey("EC", bytes)`).
pub fn ec_private_from_pkcs8(bytes: &[u8]) -> Result<SecretKey, CryptoError> {
    SecretKey::from_pkcs8_der(bytes).map_err(|_| CryptoError::InvalidPem)
}

/// Encodes EC private key as PKCS#8 DER.
pub fn ec_private_to_pkcs8(secret: &SecretKey) -> Result<Vec<u8>, CryptoError> {
    secret
        .to_pkcs8_der()
        .map(|doc| doc.as_bytes().to_vec())
        .map_err(|_| CryptoError::InvalidPem)
}

/// Generates random key bytes (`KeyUtil.generateKey` length checks).
pub fn generate_random_key_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    rand_core06::OsRng.fill_bytes(&mut buf);
    buf
}

/// PKCS#8 round-trip for EC private key (DH/ECIES parity).
pub fn ec_private_pkcs8_round_trip(secret: &SecretKey) -> Result<bool, CryptoError> {
    let der = ec_private_to_pkcs8(secret)?;
    let restored = ec_private_from_pkcs8(&der)?;
    Ok(restored.to_bytes() == secret.to_bytes())
}
