//! Legacy symmetric algorithms aligned with Hutool parity tests.

use crate::CryptoError;

/// RC4 stream cipher (Hutool `RC4`).
pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    /// Builds RC4 from a UTF-8 key string.
    pub fn new(key: impl AsRef<[u8]>) -> Self {
        let key = key.as_ref();
        let mut s = [0u8; 256];
        for (i, slot) in s.iter_mut().enumerate() {
            *slot = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(s[i]).wrapping_add(key[i % key.len()]);
            s.swap(i, j as usize);
        }
        Self { s, i: 0, j: 0 }
    }

    /// Encrypts or decrypts in place.
    pub fn apply_keystream(&mut self, data: &mut [u8]) {
        for byte in data {
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            self.s.swap(self.i as usize, self.j as usize);
            let k = self.s[(self.s[self.i as usize].wrapping_add(self.s[self.j as usize])) as usize];
            *byte ^= k;
        }
    }

    /// Returns encrypted bytes.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut out = plaintext.to_vec();
        self.apply_keystream(&mut out);
        out
    }

    /// Returns decrypted bytes.
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        self.encrypt(ciphertext)
    }
}

use super::{DesEcbDec, DesEcbEnc, pkcs7_pad, pkcs7_unpad, read_u32_be, read_u32_key, sm4_ecb, tea_core};
use super::{vigenere_map, write_u32_be};
