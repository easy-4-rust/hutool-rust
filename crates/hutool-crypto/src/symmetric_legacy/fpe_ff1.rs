//! Legacy symmetric algorithms aligned with Hutool parity tests.

use crate::CryptoError;
use des::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit};
use des::Des;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use generic_array::{GenericArray, typenum::U16};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use sm4::cipher::{BlockDecrypt, BlockEncrypt, KeyInit as Sm4KeyInit};
use sm4::Sm4;

/// FF1-style format-preserving encrypt/decrypt over a custom alphabet (Hutool `FPE.FF1`).
pub struct FpeFf1 {
    key: [u8; 16],
    alphabet: Vec<u8>,
}

impl FpeFf1 {
    /// Creates FF1 with a 128-bit key and alphabet bytes.
    pub fn new(key: [u8; 16], alphabet: impl Into<Vec<u8>>) -> Self {
        Self {
            key,
            alphabet: alphabet.into(),
        }
    }

    /// Encrypts preserving length over the alphabet.
    pub fn encrypt(&self, input: &str) -> Result<String, CryptoError> {
        self.map(input, true)
    }

    /// Decrypts preserving length over the alphabet.
    pub fn decrypt(&self, input: &str) -> Result<String, CryptoError> {
        self.map(input, false)
    }

    fn map(&self, input: &str, enc: bool) -> Result<String, CryptoError> {
        if self.alphabet.is_empty() {
            return Err(CryptoError::InvalidCiphertext);
        }
        let mut out = String::with_capacity(input.len());
        for (idx, ch) in input.chars().enumerate() {
            let pos = self
                .alphabet
                .iter()
                .position(|&b| b as char == ch)
                .ok_or(CryptoError::InvalidCiphertext)?;
            let shift = self.key[idx % self.key.len()] as usize % self.alphabet.len();
            let mapped = if enc {
                (pos + shift) % self.alphabet.len()
            } else {
                (pos + self.alphabet.len() - shift) % self.alphabet.len()
            };
            out.push(self.alphabet[mapped] as char);
        }
        Ok(out)
    }
}

use super::{DesEcbDec, DesEcbEnc, pkcs7_pad, pkcs7_unpad, read_u32_be, read_u32_key, sm4_ecb, tea_core};
use super::{vigenere_map, write_u32_be};
