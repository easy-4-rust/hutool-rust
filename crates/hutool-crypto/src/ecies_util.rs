//! 对齐: `cn.hutool.crypto.asymmetric.ECIES`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/asymmetric/ECIES.java
//! 中文说明: ECIES 混合加密（P-256 ECDH + AES-256-GCM），对齐 Hutool ECIES

use crate::{CryptoError, aes256_gcm_decrypt, aes256_gcm_encrypt};
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::{EncodedPoint, PublicKey, SecretKey};
use sha2::{Digest, Sha256};

const EPHEMERAL_LEN: usize = 65;

/// ECIES encryptor/decryptor holding optional P-256 key material.
#[derive(Debug, Clone)]
pub struct Ecies {
    /// Recipient/own private key (required for decrypt).
    private_key: Option<SecretKey>,
    /// Peer public key (required for encrypt).
    public_key: Option<PublicKey>,
}

impl Ecies {
    /// Creates a fresh key pair (Hutool `ECIES()`).
    pub fn new() -> Result<Self, CryptoError> {
        let (secret, public) = crate::generate_ec_keypair()?;
        Ok(Self {
            private_key: Some(secret),
            public_key: Some(public),
        })
    }

    /// Creates from algorithm name; algorithm string is accepted then ignored
    /// (Hutool `ECIES(String)` — BC algorithm variants are not mirrored).
    pub fn with_algorithm(_ecies_algorithm: &str) -> Result<Self, CryptoError> {
        Self::new()
    }

    /// Creates from PKCS#8 private / SPKI or SEC1 public key bytes.
    pub fn from_key_bytes(
        private_key: Option<&[u8]>,
        public_key: Option<&[u8]>,
    ) -> Result<Self, CryptoError> {
        let private_key = match private_key {
            Some(bytes) if !bytes.is_empty() => Some(crate::ec_private_from_pkcs8(bytes)?),
            _ => None,
        };
        let public_key = match public_key {
            Some(bytes) if !bytes.is_empty() => Some(decode_public(bytes)?),
            _ => None,
        };
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Creates from hex/base64 private and public key strings (Hutool string ctor).
    pub fn from_key_strs(
        private_key_str: Option<&str>,
        public_key_str: Option<&str>,
    ) -> Result<Self, CryptoError> {
        let private_key = match private_key_str {
            Some(s) if !s.is_empty() => {
                let bytes =
                    crate::asymmetric::decode(s).map_err(|_| CryptoError::InvalidEncoding)?;
                Some(crate::ec_private_from_pkcs8(&bytes)?)
            }
            _ => None,
        };
        let public_key = match public_key_str {
            Some(s) if !s.is_empty() => {
                let bytes =
                    crate::asymmetric::decode(s).map_err(|_| CryptoError::InvalidEncoding)?;
                Some(decode_public(&bytes)?)
            }
            _ => None,
        };
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Creates from existing P-256 keys.
    #[must_use]
    pub fn from_keys(private_key: Option<SecretKey>, public_key: Option<PublicKey>) -> Self {
        Self {
            private_key,
            public_key,
        }
    }

    /// Returns the private key when present.
    #[must_use]
    pub fn private_key(&self) -> Option<&SecretKey> {
        self.private_key.as_ref()
    }

    /// Returns the public key when present.
    #[must_use]
    pub fn public_key(&self) -> Option<&PublicKey> {
        self.public_key.as_ref()
    }

    /// Encrypts to the configured public key (Hutool `encrypt`).
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let public = self.public_key.as_ref().ok_or(CryptoError::InvalidPem)?;
        encrypt_ecies(public, plaintext)
    }

    /// Decrypts with the configured private key (Hutool `decrypt`).
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let secret = self.private_key.as_ref().ok_or(CryptoError::InvalidPem)?;
        decrypt_ecies(secret, ciphertext)
    }
}

/// Encrypts `plaintext` for `recipient` public key.
pub fn encrypt_ecies(recipient: &PublicKey, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let ephemeral = p256::ecdh::EphemeralSecret::random(&mut rand_core06::OsRng);
    let shared = ephemeral.diffie_hellman(recipient);
    let key = Sha256::digest(shared.raw_secret_bytes());
    let body = aes256_gcm_encrypt(&key, plaintext)?;
    let eph_bytes = ephemeral.public_key().as_affine().to_encoded_point(false);
    let mut out = Vec::with_capacity(EPHEMERAL_LEN + body.len());
    out.extend_from_slice(eph_bytes.as_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

/// Decrypts ECIES ciphertext produced by [`encrypt_ecies`].
pub fn decrypt_ecies(secret: &SecretKey, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if ciphertext.len() <= EPHEMERAL_LEN {
        return Err(CryptoError::InvalidCiphertext);
    }
    let (eph_bytes, body) = ciphertext.split_at(EPHEMERAL_LEN);
    let point = EncodedPoint::from_bytes(eph_bytes).map_err(|_| CryptoError::InvalidCiphertext)?;
    let eph_pub = Option::<PublicKey>::from(PublicKey::from_encoded_point(&point))
        .ok_or(CryptoError::InvalidCiphertext)?;
    let shared = p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), eph_pub.as_affine());
    let key = Sha256::digest(shared.raw_secret_bytes());
    aes256_gcm_decrypt(&key, body)
}

fn decode_public(bytes: &[u8]) -> Result<PublicKey, CryptoError> {
    if let Ok(point) = EncodedPoint::from_bytes(bytes) {
        if let Some(pk) = Option::<PublicKey>::from(PublicKey::from_encoded_point(&point)) {
            return Ok(pk);
        }
    }
    use p256::pkcs8::DecodePublicKey;
    PublicKey::from_public_key_der(bytes).map_err(|_| CryptoError::InvalidPem)
}
