//! 对齐: `cn.hutool.crypto.asymmetric.ASN1Util`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/asymmetric/ASN1Util.java
//! 中文说明: ASN.1 DER 编码工具，对齐 Hutool ASN1Util

use crate::CryptoError;
use der::{Decode, Encode};

/// Zero-sized facade for Hutool `ASN1Util`.
#[derive(Debug, Clone, Copy, Default)]
pub struct Asn1Util;

impl Asn1Util {
    /// Encodes octet payloads as a DER SEQUENCE of OCTET STRINGs (`ASN1Util.encodeDer`).
    pub fn encode_der(elements: &[&[u8]]) -> Result<Vec<u8>, CryptoError> {
        Self::encode("DER", elements)
    }

    /// Encodes with Hutool encoding name (`DER` / `BER` / `DL`); BER/DL collapse to DER.
    pub fn encode(asn1_encoding: &str, elements: &[&[u8]]) -> Result<Vec<u8>, CryptoError> {
        if !asn1_encoding.eq_ignore_ascii_case("DER")
            && !asn1_encoding.eq_ignore_ascii_case("BER")
            && !asn1_encoding.eq_ignore_ascii_case("DL")
        {
            return Err(CryptoError::InvalidEncoding);
        }
        encode_octet_sequence(elements)
    }

    /// Writes encoded bytes to `out` (`ASN1Util.encodeTo`).
    pub fn encode_to(
        asn1_encoding: &str,
        out: &mut Vec<u8>,
        elements: &[&[u8]],
    ) -> Result<(), CryptoError> {
        let encoded = Self::encode(asn1_encoding, elements)?;
        out.extend_from_slice(&encoded);
        Ok(())
    }

    /// Decodes a DER blob and returns the raw bytes (`ASN1Util.decode` stand-in).
    pub fn decode(input: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let _ = der::Any::from_der(input).map_err(|_| CryptoError::InvalidEncoding)?;
        Ok(input.to_vec())
    }

    /// Debug dump of DER bytes as lowercase hex (`ASN1Util.getDumpStr`).
    #[must_use]
    pub fn get_dump_str(input: &[u8]) -> String {
        hex::encode(input)
    }
}

/// Builds `SEQUENCE { OCTET STRING, ... }` DER.
fn encode_octet_sequence(elements: &[&[u8]]) -> Result<Vec<u8>, CryptoError> {
    use der::asn1::OctetStringRef;
    let mut body = Vec::new();
    for element in elements {
        let os = OctetStringRef::new(element).map_err(|_| CryptoError::InvalidEncoding)?;
        body.extend_from_slice(&os.to_der().map_err(|_| CryptoError::InvalidEncoding)?);
    }
    let mut out = Vec::new();
    out.push(0x30);
    write_der_length(&mut out, body.len());
    out.extend_from_slice(&body);
    Ok(out)
}

fn write_der_length(out: &mut Vec<u8>, len: usize) {
    if len < 0x80 {
        out.push(len as u8);
        return;
    }
    let bytes = len.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len() - 1);
    let significant = &bytes[start..];
    out.push(0x80 | significant.len() as u8);
    out.extend_from_slice(significant);
}
