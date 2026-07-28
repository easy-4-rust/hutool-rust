//! Legacy symmetric algorithms aligned with Hutool parity tests.

use crate::CryptoError;
use des::cipher::{BlockModeDecrypt, BlockModeEncrypt, KeyInit};
use des::Des;
use ecb::{Decryptor as EcbDecryptor, Encryptor as EcbEncryptor};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use sm4::cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit as Sm4KeyInit};
use sm4::Sm4;

mod rc4;
mod fpe_ff1;

pub use rc4::Rc4;
pub use fpe_ff1::FpeFf1;

type DesEcbEnc = EcbEncryptor<Des>;

type DesEcbDec = EcbDecryptor<Des>;

pub fn tea_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    tea_core(key, plaintext, false)
}

pub fn tea_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    tea_core(key, ciphertext, true)
}

fn tea_core(key: &[u8], data: &[u8], decrypt: bool) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 16 {
        return Err(CryptoError::InvalidAesKey);
    }
    // Hutool SymmetricCrypto uses zero-padding for TEA (not PKCS7).
    let mut buf = data.to_vec();
    if !decrypt {
        let remain = buf.len() % 8;
        if remain > 0 {
            buf.resize(buf.len() + 8 - remain, 0);
        }
    }
    let k = read_u32_key(key);
    let mut out = Vec::with_capacity(buf.len());
    for chunk in buf.chunks(8) {
        let mut v0 = read_u32_be(&chunk[0..4]);
        let mut v1 = read_u32_be(&chunk[4..8]);
        let mut sum: u32 = if decrypt { 0xC6EF_3720 } else { 0 };
        let delta = 0x9E37_79B9;
        for _ in 0..32 {
            if decrypt {
                v1 = v1.wrapping_sub(
                    (((v0 << 4).wrapping_add(k[2])) ^ (v0.wrapping_add(sum)).wrapping_add((v0 >> 5).wrapping_add(k[3]))),
                );
                v0 = v0.wrapping_sub(
                    (((v1 << 4).wrapping_add(k[0])) ^ (v1.wrapping_add(sum)).wrapping_add((v1 >> 5).wrapping_add(k[1]))),
                );
                sum = sum.wrapping_sub(delta);
            } else {
                sum = sum.wrapping_add(delta);
                v0 = v0.wrapping_add(
                    (((v1 << 4).wrapping_add(k[0])) ^ (v1.wrapping_add(sum)).wrapping_add((v1 >> 5).wrapping_add(k[1]))),
                );
                v1 = v1.wrapping_add(
                    (((v0 << 4).wrapping_add(k[2])) ^ (v0.wrapping_add(sum)).wrapping_add((v0 >> 5).wrapping_add(k[3]))),
                );
            }
        }
        write_u32_be(&mut out, v0);
        write_u32_be(&mut out, v1);
    }
    if !decrypt {
        return Ok(out);
    }
    let mut end = out.len();
    while end > 0 && out[end - 1] == 0 {
        end -= 1;
    }
    out.truncate(end);
    Ok(out)
}

fn read_u32_key(key: &[u8]) -> [u32; 4] {
    [
        u32::from_be_bytes(key[0..4].try_into().unwrap()),
        u32::from_be_bytes(key[4..8].try_into().unwrap()),
        u32::from_be_bytes(key[8..12].try_into().unwrap()),
        u32::from_be_bytes(key[12..16].try_into().unwrap()),
    ]
}

fn read_u32_be(chunk: &[u8]) -> u32 {
    u32::from_be_bytes(chunk.try_into().unwrap())
}

fn write_u32_be(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_be_bytes());
}

pub fn des_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 8 {
        return Err(CryptoError::InvalidAesKey);
    }
    let mut buf = vec![0u8; plaintext.len() + 8];
    buf[..plaintext.len()].copy_from_slice(plaintext);
    let mut cipher = DesEcbEnc::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let written = cipher
        .encrypt_padded::<aes::cipher::block_padding::Pkcs7>(&mut buf, plaintext.len())
        .map_err(|_| CryptoError::Aead)?;
    Ok(written.to_vec())
}

pub fn des_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 8 {
        return Err(CryptoError::InvalidAesKey);
    }
    let mut buf = ciphertext.to_vec();
    let mut cipher = DesEcbDec::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let plain = cipher
        .decrypt_padded::<aes::cipher::block_padding::Pkcs7>(&mut buf)
        .map_err(|_| CryptoError::Aead)?;
    Ok(plain.to_vec())
}

pub fn pbkdf2_sha1_hex(password: &[u8], salt: &[u8]) -> String {
    let mut out = [0u8; 64];
    pbkdf2_hmac::<Sha1>(password, salt, 1000, &mut out);
    hex::encode(out)
}

pub fn sm4_ecb_encrypt_hex(key: &[u8], plaintext: &[u8]) -> Result<String, CryptoError> {
    Ok(hex::encode(sm4_ecb_encrypt(key, plaintext)?))
}

pub fn sm4_ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sm4_ecb(key, plaintext, true)
}

pub fn sm4_ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    sm4_ecb(key, ciphertext, false)
}

fn sm4_ecb(key: &[u8], data: &[u8], encrypt: bool) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 16 {
        return Err(CryptoError::InvalidAesKey);
    }
    let cipher = Sm4::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let input = if encrypt {
        pkcs7_pad(data, 16)
    } else {
        data.to_vec()
    };
    let mut out = Vec::with_capacity(input.len());
    for chunk in input.chunks(16) {
        let mut block = sm4::cipher::Block::<Sm4>::default();
        block.copy_from_slice(chunk);
        if encrypt {
            cipher.encrypt_block(&mut block);
        } else {
            cipher.decrypt_block(&mut block);
        }
        out.extend_from_slice(&block);
    }
    if encrypt {
        Ok(out)
    } else {
        pkcs7_unpad(&out).map(|v| v.to_vec())
    }
}

fn pkcs7_pad(data: &[u8], block: usize) -> Vec<u8> {
    let pad = block - (data.len() % block);
    let mut out = data.to_vec();
    out.extend(std::iter::repeat(pad as u8).take(pad));
    out
}

fn pkcs7_unpad(data: &[u8]) -> Result<&[u8], CryptoError> {
    let pad = *data.last().ok_or(CryptoError::InvalidCiphertext)? as usize;
    if pad == 0 || pad > 16 || pad > data.len() {
        return Err(CryptoError::InvalidCiphertext);
    }
    Ok(&data[..data.len() - pad])
}

pub fn generate_sm4_key(bits: usize) -> Result<Vec<u8>, CryptoError> {
    let len = bits / 8;
    if len != 16 && len != 32 {
        return Err(CryptoError::InvalidAesKey);
    }
    use rand_core06::RngCore;
    let mut key = vec![0u8; len];
    rand_core06::OsRng.fill_bytes(&mut key);
    Ok(key)
}

pub fn vigenere_encrypt(content: &str, key: &str) -> String {
    vigenere_map(content, key, true)
}

pub fn vigenere_decrypt(content: &str, key: &str) -> String {
    vigenere_map(content, key, false)
}

fn vigenere_map(content: &str, key: &str, enc: bool) -> String {
    let data: Vec<char> = content.chars().collect();
    let key_chars: Vec<char> = key.chars().collect();
    let data_len = data.len();
    let key_len = key_chars.len().max(1);
    let mut out = vec!['\0'; data_len];
    for i in 0..data_len / key_len + 1 {
        for t in 0..key_len {
            let idx = t + i * key_len;
            if idx >= data_len {
                continue;
            }
            out[idx] = if enc {
                char::from_u32(((data[idx] as u32 + key_chars[t] as u32 - 64) % 95) + 32).unwrap()
            } else if data[idx] as i32 - key_chars[t] as i32 >= 0 {
                char::from_u32(((data[idx] as u32 - key_chars[t] as u32) % 95) + 32).unwrap()
            } else {
                char::from_u32((data[idx] as u32 - key_chars[t] as u32 + 95) % 95 + 32).unwrap()
            };
        }
    }
    out.into_iter().collect()
}
