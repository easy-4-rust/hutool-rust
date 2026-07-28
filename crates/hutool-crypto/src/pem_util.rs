//! 对齐: `cn.hutool.crypto.PemUtil`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/PemUtil.java
//! 中文说明: PEM 格式密钥读写工具，对齐 Hutool PemUtil

use crate::CryptoError;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use rsa::{RsaPrivateKey, RsaPublicKey};

/// Parsed PEM object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PemKind {
    /// PKCS#1 / PKCS#8 RSA private key.
    RsaPrivate,
    /// SPKI RSA public key.
    RsaPublic,
    /// X.509 certificate public key material.
    Certificate,
    /// SEC1 EC private key.
    EcPrivate,
}

/// Reads a PEM RSA private key (`PemUtil.readPemPrivateKey`).
pub fn read_pem_private_key(pem: &str) -> Result<RsaPrivateKey, CryptoError> {
    crate::rsa_private_key_from_pem(pem)
}

/// Reads a PEM RSA public key or extracts it from a certificate.
pub fn read_pem_public_key(pem: &str) -> Result<RsaPublicKey, CryptoError> {
    if pem.contains("CERTIFICATE") {
        let der = pem_block_data(pem)?;
        return rsa_public_key_from_cert_der(&der);
    }
    crate::rsa_public_key_from_pem(pem)
}

/// Reads either RSA private or public key (`PemUtil.readPemKey`).
pub fn read_pem_key(pem: &str) -> Result<PemKind, CryptoError> {
    if pem.contains("EC PRIVATE KEY") {
        return Ok(PemKind::EcPrivate);
    }
    if pem.contains("PRIVATE KEY") {
        read_pem_private_key(pem)?;
        return Ok(PemKind::RsaPrivate);
    }
    if pem.contains("PUBLIC KEY") || pem.contains("CERTIFICATE") {
        read_pem_public_key(pem)?;
        return Ok(if pem.contains("CERTIFICATE") {
            PemKind::Certificate
        } else {
            PemKind::RsaPublic
        });
    }
    Err(CryptoError::InvalidPem)
}

/// RSA encrypt/decrypt round-trip (`PemUtil.validateKey`).
pub fn rsa_validate_key_pem(private_pem: &str, public_pem: &str, message: &str) -> Result<bool, CryptoError> {
    let private = read_pem_private_key(private_pem)?;
    let public = read_pem_public_key(public_pem)?;
    let enc = crate::rsa_encrypt_pkcs1v15(&public, message.as_bytes())?;
    let dec = crate::rsa_decrypt_pkcs1v15(&private, &enc)?;
    Ok(dec == message.as_bytes())
}

fn pem_block_data(pem: &str) -> Result<Vec<u8>, CryptoError> {
    let mut lines = pem.lines();
    let _begin = lines.next();
    let mut b64 = String::new();
    for line in lines {
        if line.starts_with("-----END") {
            break;
        }
        b64.push_str(line.trim());
    }
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|_| CryptoError::InvalidPem)
}

/// Writes PKCS#8 PEM for tests/debug.
pub fn write_pkcs8_private_pem(key: &RsaPrivateKey) -> Result<String, CryptoError> {
    key.to_pkcs8_pem(LineEnding::LF)
        .map(|s| s.to_string())
        .map_err(|_| CryptoError::RsaKey)
}

/// Extracts RSA public key from X.509 certificate DER.
pub fn rsa_public_key_from_cert_der(der: &[u8]) -> Result<RsaPublicKey, CryptoError> {
    use rsa::pkcs8::DecodePublicKey;
    use x509_cert::der::Decode;
    use der::Encode;
    let cert = x509_cert::certificate::Certificate::from_der(der).map_err(|_| CryptoError::InvalidPem)?;
    let spki = cert.tbs_certificate().subject_public_key_info();
    let der = spki.to_der().map_err(|_| CryptoError::InvalidPem)?;
    RsaPublicKey::from_public_key_der(&der).map_err(|_| CryptoError::InvalidPem)
}

/// Reads EC SEC1 private key PEM and returns raw scalar bytes (`PemUtil.readECPrivateKeyTest`).
pub fn read_ec_private_key_pem(pem: &str) -> Result<Vec<u8>, CryptoError> {
    use p256::pkcs8::DecodePrivateKey as _;
    let key = p256::SecretKey::from_sec1_pem(pem).map_err(|_| CryptoError::InvalidPem)?;
    Ok(key.to_bytes().to_vec())
}
