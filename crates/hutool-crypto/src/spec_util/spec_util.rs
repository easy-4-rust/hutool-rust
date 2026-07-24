//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

use super::key_spec_bytes::KeySpecBytes;
use super::pbe_key_spec::PbeKeySpec;
use super::pbe_parameter_spec::PbeParameterSpec;
use super::rsa_private_crt_key_spec::RsaPrivateCrtKeySpec;

/// Zero-sized facade for Hutool `SpecUtil`.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpecUtil;

impl SpecUtil {
    /// Creates key bytes for an algorithm (`SpecUtil.createKeySpec`).
    pub fn create_key_spec(algorithm: &str, key: Option<&[u8]>) -> KeySpecBytes {
        let key = match key {
            Some(bytes) if !bytes.is_empty() => bytes.to_vec(),
            _ if algorithm.starts_with("DESede") => generate_random_key_bytes(24),
            _ if algorithm.starts_with("DES") => generate_random_key_bytes(8),
            _ => generate_random_key_bytes(16),
        };
        KeySpecBytes {
            algorithm: algorithm.to_owned(),
            key,
        }
    }

    /// Creates a PBE password spec (`SpecUtil.createPBEKeySpec`).
    pub fn create_pbe_key_spec(password: Option<&[u8]>) -> PbeKeySpec {
        let password = match password {
            Some(bytes) if !bytes.is_empty() => bytes.to_vec(),
            _ => generate_random_key_bytes(32),
        };
        PbeKeySpec { password }
    }

    /// Creates PBE parameter spec (`SpecUtil.createPBEParameterSpec`).
    #[must_use]
    pub fn create_pbe_parameter_spec(salt: &[u8], iteration_count: u32) -> PbeParameterSpec {
        PbeParameterSpec {
            salt: salt.to_vec(),
            iteration_count,
        }
    }

    /// Parses C# XML RSA private key into CRT components.
    pub fn xml_to_rsa_private_crt_key_spec(xml: &str) -> Result<RsaPrivateCrtKeySpec, CryptoError> {
        Ok(RsaPrivateCrtKeySpec {
            modulus: xml_b64_field(xml, "Modulus")?,
            public_exponent: xml_b64_field(xml, "Exponent")?,
            private_exponent: xml_b64_field(xml, "D")?,
            prime_p: xml_b64_field(xml, "P")?,
            prime_q: xml_b64_field(xml, "Q")?,
            prime_exponent_p: xml_b64_field(xml, "DP")?,
            prime_exponent_q: xml_b64_field(xml, "DQ")?,
            crt_coefficient: xml_b64_field(xml, "InverseQ")?,
        })
    }
}

fn xml_b64_field(xml: &str, tag: &str) -> Result<BigUint, CryptoError> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open).ok_or(CryptoError::InvalidEncoding)? + open.len();
    let end = xml[start..]
        .find(&close)
        .ok_or(CryptoError::InvalidEncoding)?
        + start;
    let b64 = xml[start..end].trim();
    let bytes = STANDARD
        .decode(b64)
        .map_err(|_| CryptoError::InvalidEncoding)?;
    Ok(BigUint::from_bytes_be(&bytes))
}
