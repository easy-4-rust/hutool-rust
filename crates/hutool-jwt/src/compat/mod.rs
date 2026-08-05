//! hutool-jwt 兼容层模块入口。
//!
//! 该模块继续承载 Hutool `cn.hutool.jwt` 兼容实现，已开始按 Java 对象逐步拆分文件。

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use serde_json::{Map, Value};

// 仅测试构建需要的 PEM 密钥 trait 导入（lib 构建不编译 `#[cfg(test)]` 代码）。
#[cfg(test)]
use jsonwebtoken::Algorithm;
#[cfg(test)]
use rsa::pkcs1::{DecodeRsaPrivateKey as _, EncodeRsaPublicKey as _};
#[cfg(test)]
use rsa::pkcs8::{DecodePrivateKey as _, DecodePublicKey as _, EncodePrivateKey as _};

// signers 子包模块声明
pub mod signers;

// 让 mod.rs 内的 #[cfg(test)] 可使用 signers 子包内的 helper 函数
// （包括 signing_input/signing_result/verification_result/key_error/pem_text/rsa_private_der/...）
#[cfg(test)]
pub(crate) use signers::all::{pem_text, signing_result, verification_result};

mod claims;
mod jwt_exception;
mod jwt_header;
mod jwt_payload;
mod jwt_util;
mod jwt_validator;

pub use claims::Claims;
pub use jwt_exception::JWTException;
pub use jwt_header::JWTHeader;
pub use jwt_payload::JWTPayload;
pub use jwt_util::JWTUtil;
pub use jwt_validator::JWTValidator;

pub use signers::{
    AlgorithmUtil, AsymmetricJWTSigner, EllipticCurveJWTSigner, HMacJWTSigner, JWTSigner,
    JWTSignerUtil, NoneJWTSigner, RegisteredPayload,
};

/// Decodes a JWT header/payload segment (URL-safe or standard Base64, optional padding).
fn decode_jwt_part(part: &str) -> Result<Vec<u8>, JWTException> {
    let pad = |input: &str| -> String {
        let rem = input.len() % 4;
        if rem == 0 {
            input.to_owned()
        } else {
            format!("{input}{}", "=".repeat(4 - rem))
        }
    };
    let padded = pad(part);
    if let Ok(bytes) = URL_SAFE.decode(padded.as_bytes()) {
        return Ok(bytes);
    }
    if let Ok(bytes) = URL_SAFE_NO_PAD.decode(part.as_bytes()) {
        return Ok(bytes);
    }
    let standard = padded.replace('-', "+").replace('_', "/");
    STANDARD
        .decode(standard.as_bytes())
        .map_err(JWTException::from)
}

/// Fluent registered-claim setters.
#[derive(Clone)]
pub struct JWT {
    header: JWTHeader,
    payload: JWTPayload,
    signer: Option<Arc<dyn JWTSigner>>,
    tokens: Option<[String; 3]>,
}

impl Default for JWT {
    fn default() -> Self {
        Self::create()
    }
}

impl JWT {
    /// Creates an empty token.
    #[must_use]
    pub fn create() -> Self {
        Self {
            header: JWTHeader::default(),
            payload: JWTPayload::default(),
            signer: None,
            tokens: None,
        }
    }

    /// Parses a compact JWT.
    pub fn of(token: &str) -> Result<Self, JWTException> {
        Self::create().parse(token)
    }

    /// Replaces this object with parsed token content.
    pub fn parse(mut self, token: &str) -> Result<Self, JWTException> {
        if token.trim().is_empty() {
            return Err(JWTException::new("Token String must be not blank!"));
        }
        let parts: Vec<_> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(JWTException::formatted(
                "the token was expected 3 parts, but got {}",
                &[&parts.len()],
            ));
        }
        let header = String::from_utf8(decode_jwt_part(parts[0])?)
            .map_err(|error| JWTException::new(error.to_string()))?;
        let payload = String::from_utf8(decode_jwt_part(parts[1])?)
            .map_err(|error| JWTException::new(error.to_string()))?;
        self.header = JWTHeader::from_claims(Claims::parse(&header)?);
        self.payload = JWTPayload::from_claims(Claims::parse(&payload)?);
        self.tokens = Some([
            parts[0].to_owned(),
            parts[1].to_owned(),
            parts[2].to_owned(),
        ]);
        Ok(self)
    }

    /// Configures an HMAC signer from the shared key.
    ///
    /// Aligns with Hutool `JWT#setKey(byte[])`: uses the existing header `alg`
    /// when present (so a pre-set `HS384` header yields an HS384 signer), otherwise
    /// defaults to HS256. When `alg` is `none`/empty, returns an error (Hutool throws).
    pub fn set_key(&mut self, key: &[u8]) -> Result<&mut Self, JWTException> {
        let algorithm_id = self.algorithm().unwrap_or("HS256");
        if NoneJWTSigner::is_none(Some(algorithm_id)) {
            return Err(JWTException::new(
                "When key is not null, algorithmId must not be none.",
            ));
        }
        let signer = JWTSignerUtil::create_signer(algorithm_id, key)?;
        self.set_signer(Arc::new(signer));
        Ok(self)
    }

    /// Sets the signer and its header algorithm when absent.
    pub fn set_signer(&mut self, signer: Arc<dyn JWTSigner>) -> &mut Self {
        if self.algorithm().is_none() {
            self.header.set_algorithm(signer.algorithm_id());
        }
        self.signer = Some(signer);
        self
    }

    /// Returns the signer.
    #[must_use]
    pub fn signer(&self) -> Option<&dyn JWTSigner> {
        self.signer.as_deref()
    }

    /// Returns protected headers.
    #[must_use]
    pub const fn header(&self) -> &JWTHeader {
        &self.header
    }

    /// Returns mutable protected headers.
    pub const fn header_mut(&mut self) -> &mut JWTHeader {
        &mut self.header
    }

    /// Returns payload claims.
    #[must_use]
    pub const fn payload(&self) -> &JWTPayload {
        &self.payload
    }

    /// Returns mutable payload claims.
    pub const fn payload_mut(&mut self) -> &mut JWTPayload {
        &mut self.payload
    }

    /// Returns the header algorithm.
    #[must_use]
    pub fn algorithm(&self) -> Option<&str> {
        self.header
            .claims()
            .get_claim(JWTHeader::ALGORITHM)
            .and_then(Value::as_str)
    }

    /// Returns a header claim by name (Hutool `JWT#getHeader(String)`).
    #[must_use]
    pub fn get_header(&self, name: &str) -> Option<&Value> {
        self.header.claims().get_claim(name)
    }

    /// Returns a payload claim by name (Hutool `JWT#getPayload(String)`).
    #[must_use]
    pub fn get_payload(&self, name: &str) -> Option<&Value> {
        self.payload.claims().get_claim(name)
    }

    /// Returns all payload claims (Hutool `JWT#getPayloads()`).
    #[must_use]
    pub const fn get_payloads(&self) -> &Claims {
        self.payload.claims()
    }

    /// Sets a header.
    pub fn set_header(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.header.claims_mut().set_claim(name, value);
        self
    }

    /// Sets a payload.
    pub fn set_payload(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.payload.set_payload(name, value);
        self
    }

    /// Adds protected headers.
    pub fn add_headers(&mut self, headers: Map<String, Value>) -> &mut Self {
        self.header.add_headers(headers);
        self
    }

    /// Adds payload claims.
    pub fn add_payloads(&mut self, payloads: Map<String, Value>) -> &mut Self {
        self.payload.add_payloads(payloads);
        self
    }

    /// Signs using the configured signer.
    pub fn sign(&mut self) -> Result<String, JWTException> {
        self.sign_with_type(true)
    }

    /// Signs using the configured signer and controls automatic `typ=JWT` insertion.
    pub fn sign_with_type(&mut self, add_type_if_missing: bool) -> Result<String, JWTException> {
        let signer = self
            .signer
            .as_ref()
            .ok_or_else(|| JWTException::new("no signer provided"))?;
        if add_type_if_missing && self.header.claims().get_claim(JWTHeader::TYPE).is_none() {
            self.header.set_type("JWT");
        }
        let header = self.header.claims().encode();
        let payload = self.payload.claims().encode();
        let signature = signer.sign(&header, &payload)?;
        Ok(format!("{header}.{payload}.{signature}"))
    }

    /// Installs an explicit signer and signs in one operation.
    pub fn sign_with(&mut self, signer: Arc<dyn JWTSigner>) -> Result<String, JWTException> {
        self.set_signer(signer);
        self.sign()
    }

    /// Verifies using the configured signer.
    ///
    /// Aligns with Hutool `JWT#verify()`: a missing signer defaults to `none`.
    pub fn verify(&self) -> Result<bool, JWTException> {
        match self.signer.as_ref() {
            Some(signer) => self.verify_with(signer.as_ref()),
            None => self.verify_with(&NoneJWTSigner),
        }
    }

    /// Verifies with an explicit signer.
    ///
    /// Aligns with Hutool `JWT#verify(JWTSigner)` including `alg=none` guards.
    pub fn verify_with(&self, signer: &dyn JWTSigner) -> Result<bool, JWTException> {
        let parts = self
            .tokens
            .as_ref()
            .ok_or_else(|| JWTException::new("no token to verify"))?;
        let none_alg = NoneJWTSigner::is_none(self.algorithm());
        let none_signer = NoneJWTSigner::is_none(Some(signer.algorithm_id()));
        if none_alg && !none_signer {
            return Err(JWTException::formatted(
                "Alg is 'none' but use: {} !",
                &[&signer.algorithm_id()],
            ));
        }
        if none_signer && !none_alg {
            return Err(JWTException::new(
                "Alg is not 'none' but use NoneJWTSigner!",
            ));
        }
        if !none_alg && self.algorithm() != Some(signer.algorithm_id()) {
            return Err(JWTException::new(
                "header and signer algorithms do not match",
            ));
        }
        signer.verify(&parts[0], &parts[1], &parts[2])
    }

    /// Creates a validator.
    #[must_use]
    pub fn validate(&self) -> JWTValidator {
        JWTValidator::new(self)
    }

    /// Verifies signature and registered dates with the supplied leeway.
    ///
    /// Aligns with Hutool `JWT#validate(long)`.
    pub fn validate_leeway(&self, leeway: u64) -> Result<bool, JWTException> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.is_valid_at(now, leeway)
    }

    /// Verifies signature and registered dates with the supplied leeway.
    pub fn is_valid_at(&self, now: u64, leeway: u64) -> Result<bool, JWTException> {
        if !self.verify()? {
            return Ok(false);
        }
        Ok(self.validate().validate_date_at(now, leeway).is_ok())
    }
}

impl RegisteredPayload for JWT {
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self {
        self.set_payload(name, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::errors::ErrorKind;

    const SECRET: &[u8] = b"a production-shaped secret with enough entropy";
    const OTHER_SECRET: &[u8] = b"a different production-shaped secret value";
    const RSA_PRIVATE: &[u8] = include_bytes!("../../tests/fixtures/rsa-private.pem");
    const RSA_PUBLIC: &[u8] = include_bytes!("../../tests/fixtures/rsa-public.pem");
    const EC_PRIVATE: &[u8] = include_bytes!("../../tests/fixtures/ec-private.pem");
    const EC_PUBLIC: &[u8] = include_bytes!("../../tests/fixtures/ec-public.pem");
    const EC384_PRIVATE: &[u8] = include_bytes!("../../tests/fixtures/ec384-private.pem");
    const EC384_PUBLIC: &[u8] = include_bytes!("../../tests/fixtures/ec384-public.pem");

    fn map(value: &Value) -> Map<String, Value> {
        value.as_object().cloned().expect("test value is an object")
    }

    struct FailingSigner;

    impl JWTSigner for FailingSigner {
        fn sign(&self, _header: &str, _payload: &str) -> Result<String, JWTException> {
            Err(JWTException::new("injected signing failure"))
        }

        fn verify(
            &self,
            _header: &str,
            _payload: &str,
            _signature: &str,
        ) -> Result<bool, JWTException> {
            Err(JWTException::new("injected verification failure"))
        }

        fn algorithm_id(&self) -> &'static str {
            "HS256"
        }
    }

    #[test]
    fn claims_headers_payloads_and_errors_are_dynamic_but_bounded() {
        let mut claims = Claims::parse(r#"{"a":1}"#).unwrap();
        assert_eq!(claims.get_claim("a"), Some(&Value::from(1)));
        assert_eq!(claims.claims_json().len(), 1);
        claims.set_claim("b", Value::Bool(true));
        claims.set_claim("a", Value::Null);
        claims.put_all(map(&serde_json::json!({"c":"x"})));
        assert_eq!(claims.to_string(), r#"{"b":true,"c":"x"}"#);
        assert!(Claims::parse("[]").is_err());
        assert!(Claims::parse("{").is_err());

        let mut header = JWTHeader::default();
        header
            .set_algorithm("HS256")
            .set_type("JWT")
            .set_content_type("json")
            .set_key_id("key-1")
            .add_headers(map(&serde_json::json!({"custom":1})));
        assert_eq!(header.claims().claims_json().len(), 5);

        let mut payload = JWTPayload::default();
        payload
            .set_issuer("issuer")
            .set_subject("subject")
            .set_audience("audience")
            .set_expires_at(20)
            .set_not_before(10)
            .set_issued_at(9)
            .set_jwt_id("id");
        payload.add_payloads(map(&serde_json::json!({"role":"admin"})));
        assert_eq!(payload.claims().claims_json().len(), 8);

        assert_eq!(
            JWTException::formatted("{}:{}", &[&1, &2]).to_string(),
            "1:2"
        );
        assert_eq!(JWTException::formatted("plain", &[&1]).to_string(), "plain");
        let error = jsonwebtoken::errors::Error::from(ErrorKind::InvalidKeyFormat);
        assert!(signing_result(Err(error)).is_err());
        let error = jsonwebtoken::errors::Error::from(ErrorKind::InvalidKeyFormat);
        assert!(verification_result(Err(error)).is_err());
    }

    #[test]
    fn algorithm_names_and_signer_factories_are_explicit() {
        for (name, algorithm) in [
            ("HmacSHA256", Algorithm::HS256),
            ("HS384", Algorithm::HS384),
            ("HS512", Algorithm::HS512),
            ("SHA256withRSA", Algorithm::RS256),
            ("RS384", Algorithm::RS384),
            ("RS512", Algorithm::RS512),
            ("SHA256withECDSA", Algorithm::ES256),
            ("ES384", Algorithm::ES384),
        ] {
            assert_eq!(AlgorithmUtil::get_algorithm(name).unwrap(), algorithm);
        }
        assert!(AlgorithmUtil::get_algorithm("MD5withRSA").is_err());
        for algorithm in [
            Algorithm::HS256,
            Algorithm::HS384,
            Algorithm::HS512,
            Algorithm::RS256,
            Algorithm::RS384,
            Algorithm::RS512,
            Algorithm::PS256,
            Algorithm::PS384,
            Algorithm::PS512,
            Algorithm::ES256,
            Algorithm::ES384,
            Algorithm::EdDSA,
        ] {
            assert!(!AlgorithmUtil::get_id(algorithm).is_empty());
        }
        assert!(HMacJWTSigner::new(Algorithm::RS256, SECRET).is_err());
        assert!(JWTSignerUtil::create_signer("RS256", SECRET).is_err());
        assert!(JWTSignerUtil::create_signer("unknown", SECRET).is_err());
        assert_eq!(
            JWTSignerUtil::hs256(SECRET).unwrap().algorithm_id(),
            "HS256"
        );
        assert_eq!(
            JWTSignerUtil::hs384(SECRET).unwrap().algorithm_id(),
            "HS384"
        );
        assert_eq!(
            JWTSignerUtil::hs512(SECRET).unwrap().algorithm_id(),
            "HS512"
        );
        assert_eq!(
            JWTSignerUtil::rs256(RSA_PRIVATE, RSA_PUBLIC)
                .unwrap()
                .algorithm_id(),
            "RS256"
        );
        assert_eq!(
            JWTSignerUtil::rs384(RSA_PRIVATE, RSA_PUBLIC)
                .unwrap()
                .algorithm_id(),
            "RS384"
        );
        assert_eq!(
            JWTSignerUtil::rs512(RSA_PRIVATE, RSA_PUBLIC)
                .unwrap()
                .algorithm_id(),
            "RS512"
        );
        assert_eq!(
            JWTSignerUtil::es256(EC_PRIVATE, EC_PUBLIC)
                .unwrap()
                .algorithm_id(),
            "ES256"
        );
        assert_eq!(
            JWTSignerUtil::es384(EC384_PRIVATE, EC384_PUBLIC)
                .unwrap()
                .algorithm_id(),
            "ES384"
        );
        for algorithm in ["RS256", "RS384", "RS512"] {
            assert_eq!(
                JWTSignerUtil::create_signer_from_pem(algorithm, RSA_PRIVATE, RSA_PUBLIC)
                    .unwrap()
                    .algorithm_id(),
                algorithm
            );
            assert!(JWTSignerUtil::create_signer_from_pem(algorithm, b"bad", RSA_PUBLIC).is_err());
        }
        for algorithm in ["ES256", "ES384"] {
            let (private_key, public_key) = if algorithm == "ES256" {
                (EC_PRIVATE, EC_PUBLIC)
            } else {
                (EC384_PRIVATE, EC384_PUBLIC)
            };
            assert_eq!(
                JWTSignerUtil::create_signer_from_pem(algorithm, private_key, public_key)
                    .unwrap()
                    .algorithm_id(),
                algorithm
            );
            assert!(JWTSignerUtil::create_signer_from_pem(algorithm, b"bad", public_key).is_err());
        }
        assert!(JWTSignerUtil::create_signer_from_pem("HS256", SECRET, SECRET).is_err());
        assert!(JWTSignerUtil::create_signer_from_pem("unknown", SECRET, SECRET).is_err());
    }

    #[test]
    fn legacy_and_non_jose_signer_aliases_are_explicitly_rejected() {
        assert!(JWTSignerUtil::es512(EC_PRIVATE, EC_PUBLIC).is_err());
        for rejected in [
            JWTSignerUtil::hmd5,
            JWTSignerUtil::hsha1,
            JWTSignerUtil::sm4cmac,
            JWTSignerUtil::rmd2,
            JWTSignerUtil::rmd5,
            JWTSignerUtil::rsha1,
            JWTSignerUtil::dnone,
            JWTSignerUtil::dsha1,
            JWTSignerUtil::enone,
            JWTSignerUtil::esha1,
        ] {
            assert!(rejected(SECRET).is_err());
        }
    }

    #[test]
    fn hmac_none_rsa_and_ecdsa_signers_use_real_crypto() {
        for signer in [
            JWTSignerUtil::hs256(SECRET).unwrap(),
            JWTSignerUtil::hs384(SECRET).unwrap(),
            JWTSignerUtil::hs512(SECRET).unwrap(),
        ] {
            let signature = signer.sign("header", "payload").unwrap();
            assert!(signer.verify("header", "payload", &signature).unwrap());
            assert!(!signer.verify("header", "changed", &signature).unwrap());
            assert!(signer.verify("header", "payload", "*").is_err());
        }

        let none = JWTSignerUtil::none();
        assert!(NoneJWTSigner::is_none(None));
        assert!(NoneJWTSigner::is_none(Some(" NONE ")));
        assert!(!NoneJWTSigner::is_none(Some("HS256")));
        assert_eq!(none.sign("h", "p").unwrap(), "");
        assert!(none.verify("h", "p", "").unwrap());
        assert!(!none.verify("h", "p", "x").unwrap());
        assert_eq!(none.algorithm_id(), "none");

        for signer in [
            JWTSignerUtil::rs256(RSA_PRIVATE, RSA_PUBLIC).unwrap(),
            JWTSignerUtil::rs384(RSA_PRIVATE, RSA_PUBLIC).unwrap(),
            JWTSignerUtil::rs512(RSA_PRIVATE, RSA_PUBLIC).unwrap(),
        ] {
            let signature = signer.sign("header", "payload").unwrap();
            assert!(signer.verify("header", "payload", &signature).unwrap());
            assert!(!signer.verify("header", "changed", &signature).unwrap());
            assert!(signer.verify("header", "payload", "*").is_err());
        }
        assert!(
            AsymmetricJWTSigner::from_rsa_pem(Algorithm::HS256, RSA_PRIVATE, RSA_PUBLIC).is_err()
        );
        assert!(AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS256, b"bad", RSA_PUBLIC).is_err());
        assert!(AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS256, RSA_PRIVATE, b"bad").is_err());
        for signer in [
            JWTSignerUtil::es256(EC_PRIVATE, EC_PUBLIC).unwrap(),
            JWTSignerUtil::es384(EC384_PRIVATE, EC384_PUBLIC).unwrap(),
        ] {
            let signature = signer.sign("header", "payload").unwrap();
            assert!(signer.verify("header", "payload", &signature).unwrap());
            assert!(!signer.verify("header", "changed", &signature).unwrap());
            assert!(signer.verify("header", "payload", "*").is_err());
        }
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::HS256, EC_PRIVATE, EC_PUBLIC).is_err());
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES256, b"bad", EC_PUBLIC).is_err());
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES256, EC_PRIVATE, b"bad").is_err());
    }

    #[test]
    fn pem_decoding_supports_standard_encodings_and_rejects_malformed_keys() {
        assert!(AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS256, b"\xff", RSA_PUBLIC).is_err());
        assert!(AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS256, RSA_PRIVATE, b"\xff").is_err());
        assert!(
            AsymmetricJWTSigner::from_rsa_pem(
                Algorithm::RS256,
                b"-----BEGIN RSA PRIVATE KEY-----\n*\n-----END RSA PRIVATE KEY-----",
                RSA_PUBLIC,
            )
            .is_err()
        );
        assert!(
            AsymmetricJWTSigner::from_rsa_pem(
                Algorithm::RS256,
                RSA_PRIVATE,
                b"-----BEGIN RSA PUBLIC KEY-----\n*\n-----END RSA PUBLIC KEY-----",
            )
            .is_err()
        );

        let private = rsa::RsaPrivateKey::from_pkcs1_pem(pem_text(RSA_PRIVATE).unwrap()).unwrap();
        let private = private.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let public = rsa::RsaPublicKey::from_public_key_pem(pem_text(RSA_PUBLIC).unwrap()).unwrap();
        let public = public.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let signer = JWTSignerUtil::rs256(private.as_bytes(), public.as_bytes()).unwrap();
        let signature = signer.sign("header", "payload").unwrap();
        assert!(signer.verify("header", "payload", &signature).unwrap());

        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES256, b"\xff", EC_PUBLIC).is_err());
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES256, EC_PRIVATE, b"\xff").is_err());
        assert!(
            EllipticCurveJWTSigner::from_pem(
                Algorithm::ES256,
                b"-----BEGIN EC PRIVATE KEY-----\n*\n-----END EC PRIVATE KEY-----",
                EC_PUBLIC,
            )
            .is_err()
        );
        assert!(
            EllipticCurveJWTSigner::from_pem(
                Algorithm::ES384,
                b"-----BEGIN PRIVATE KEY-----\n*\n-----END PRIVATE KEY-----",
                EC384_PUBLIC,
            )
            .is_err()
        );
        assert!(
            EllipticCurveJWTSigner::from_pem(
                Algorithm::ES384,
                b"-----BEGIN EC PRIVATE KEY-----\n*\n-----END EC PRIVATE KEY-----",
                EC384_PUBLIC,
            )
            .is_err()
        );
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES384, b"\xff", EC384_PUBLIC).is_err());
        assert!(
            EllipticCurveJWTSigner::from_pem(Algorithm::ES384, EC384_PRIVATE, b"\xff").is_err()
        );
        assert!(EllipticCurveJWTSigner::from_pem(Algorithm::ES384, EC384_PRIVATE, b"bad").is_err());

        let p256 = p256::SecretKey::from_pkcs8_pem(pem_text(EC_PRIVATE).unwrap()).unwrap();
        let p256 = p256.to_sec1_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let signer = JWTSignerUtil::es256(p256.as_bytes(), EC_PUBLIC).unwrap();
        let signature = signer.sign("header", "payload").unwrap();
        assert!(signer.verify("header", "payload", &signature).unwrap());

        let p384 = p384::SecretKey::from_sec1_pem(pem_text(EC384_PRIVATE).unwrap()).unwrap();
        let p384 = p384.to_pkcs8_pem(rsa::pkcs8::LineEnding::LF).unwrap();
        let signer = JWTSignerUtil::es384(p384.as_bytes(), EC384_PUBLIC).unwrap();
        let signature = signer.sign("header", "payload").unwrap();
        assert!(signer.verify("header", "payload", &signature).unwrap());
    }

    #[test]
    fn jwt_builder_parser_verifier_and_util_round_trip() {
        let mut jwt = JWT::default();
        assert!(jwt.sign().is_err());
        assert!(jwt.is_valid_at(100, 0).is_err());
        jwt.header_mut().set_key_id("key-1");
        jwt.set_header("custom", Value::Bool(true));
        jwt.add_headers(map(&serde_json::json!({"batch-header":2})));
        jwt.set_issuer("issuer")
            .set_subject("subject")
            .set_payload("role", Value::String("admin".into()));
        jwt.payload_mut().set_audience("audience");
        jwt.add_payloads(map(&serde_json::json!({"batch-payload":3})));
        jwt.set_key(SECRET).unwrap();
        assert!(jwt.signer().is_some());
        assert_eq!(jwt.algorithm(), Some("HS256"));
        let token = jwt.sign().unwrap();
        assert_eq!(jwt.header().claims().get_claim("typ").unwrap(), "JWT");
        assert!(jwt.sign().is_ok());
        assert_eq!(jwt.payload().claims().get_claim("sub").unwrap(), "subject");

        let mut no_type = JWT::create();
        no_type.set_key(SECRET).unwrap();
        let token_without_type = no_type.sign_with_type(false).unwrap();
        assert!(
            JWT::of(&token_without_type)
                .unwrap()
                .header()
                .claims()
                .get_claim(JWTHeader::TYPE)
                .is_none()
        );

        let mut parsed = JWT::of(&token).unwrap();
        assert!(parsed.verify().is_err());
        parsed.set_key(SECRET).unwrap();
        assert!(parsed.verify().unwrap());
        assert!(parsed.validate().validate_algorithm().is_ok());
        assert!(
            parsed
                .validate()
                .validate_algorithm_with(&JWTSignerUtil::hs256(SECRET).unwrap())
                .is_ok()
        );
        assert!(parsed.is_valid_at(100, 0).unwrap());

        let mut expired = JWT::create();
        expired.set_expires_at(99);
        expired.set_key(SECRET).unwrap();
        let expired_token = expired.sign().unwrap();
        let mut expired = JWT::of(&expired_token).unwrap();
        expired.set_key(SECRET).unwrap();
        assert!(!expired.is_valid_at(100, 0).unwrap());
        let wrong: Arc<dyn JWTSigner> = Arc::new(JWTSignerUtil::hs384(SECRET).unwrap());
        assert!(parsed.verify_with(wrong.as_ref()).is_err());
        let wrong = JWTSignerUtil::hs256(OTHER_SECRET).unwrap();
        assert!(!parsed.verify_with(&wrong).unwrap());
        assert!(parsed.validate().validate_algorithm_with(&wrong).is_err());
        parsed.set_signer(Arc::new(wrong));
        assert!(parsed.validate().validate_algorithm().is_err());
        assert!(!parsed.is_valid_at(100, 0).unwrap());

        let mut failing = JWT::create();
        failing.set_signer(Arc::new(FailingSigner));
        assert!(failing.sign().is_err());
        let mut failing = JWT::of(&token).unwrap();
        failing.set_signer(Arc::new(FailingSigner));
        assert!(failing.verify().is_err());
        assert!(failing.validate().validate_algorithm().is_err());
    }

    #[test]
    fn jwt_util_supports_keys_headers_and_explicit_signers() {
        let payload = map(&serde_json::json!({"sub":"utility"}));
        let token = JWTUtil::create_token(payload.clone(), SECRET).unwrap();
        assert!(JWTUtil::verify(&token, SECRET).unwrap());
        assert!(!JWTUtil::verify(&token, OTHER_SECRET).unwrap());
        assert!(JWTUtil::verify("invalid", SECRET).is_err());
        assert_eq!(
            JWTUtil::parse_token(&token)
                .unwrap()
                .payload()
                .claims()
                .get_claim("sub")
                .unwrap(),
            "utility"
        );

        let token = JWTUtil::create_token_with_headers(
            map(&serde_json::json!({"kid":"utility-key"})),
            payload.clone(),
            SECRET,
        )
        .unwrap();
        assert_eq!(
            JWT::of(&token)
                .unwrap()
                .header()
                .claims()
                .get_claim("kid")
                .unwrap(),
            "utility-key"
        );
        let signer: Arc<dyn JWTSigner> = Arc::new(JWTSignerUtil::hs384(SECRET).unwrap());
        let token =
            JWTUtil::create_token_with_signer(payload.clone(), Arc::clone(&signer)).unwrap();
        assert!(JWTUtil::verify_with_signer(&token, signer.as_ref()).unwrap());
        assert!(JWTUtil::verify_with_signer("invalid", signer.as_ref()).is_err());
        let token = JWTUtil::create_token_with_headers_and_signer(
            map(&serde_json::json!({"kid":"signer-key"})),
            payload,
            Arc::clone(&signer),
        )
        .unwrap();
        assert!(JWTUtil::verify_with_signer(&token, signer.as_ref()).unwrap());
    }

    #[test]
    fn parsing_and_state_errors_are_structured() {
        assert!(JWT::of("one.two").is_err());
        assert!(JWT::of("*.e30.").is_err());
        assert!(JWT::of("_w.e30.").is_err());
        assert!(JWT::of("W10.e30.").is_err());
        assert!(JWT::of("e30.*.").is_err());
        assert!(JWT::of("e30._w.").is_err());
        assert!(JWT::of("e30.W10.").is_err());
        let mut unsigned = JWT::create();
        unsigned.set_signer(Arc::new(NoneJWTSigner));
        let token = unsigned.sign().unwrap();
        let parsed = JWT::of(&token).unwrap();
        assert!(parsed.verify_with(&NoneJWTSigner).unwrap());
        assert!(JWT::create().verify_with(&NoneJWTSigner).is_err());
    }

    #[test]
    fn validator_checks_all_registered_time_boundaries_and_types() {
        let mut jwt = JWT::create();
        jwt.set_not_before(90).set_expires_at(110).set_issued_at(95);
        assert!(jwt.validate().validate_date_at(100, 0).is_ok());

        jwt.set_not_before(101);
        assert!(jwt.validate().validate_date_at(100, 0).is_err());
        assert!(jwt.validate().validate_date_at(100, 1).is_ok());
        jwt.set_not_before(90).set_expires_at(99);
        assert!(jwt.validate().validate_date_at(100, 0).is_err());
        assert!(jwt.validate().validate_date_at(100, 1).is_ok());
        jwt.set_expires_at(110).set_issued_at(101);
        assert!(jwt.validate().validate_date_at(100, 0).is_err());
        assert!(jwt.validate().validate_date_at(100, 1).is_ok());
        jwt.set_payload(JWTPayload::ISSUED_AT, Value::String("bad".into()));
        assert!(jwt.validate().validate_date_at(100, 0).is_err());
        jwt.set_payload(JWTPayload::ISSUED_AT, Value::from(95));
        jwt.set_payload(JWTPayload::NOT_BEFORE, Value::String("bad".into()));
        assert!(jwt.validate().validate_date_at(100, 0).is_err());
        jwt.set_payload(JWTPayload::NOT_BEFORE, Value::from(90));
        jwt.set_payload(JWTPayload::EXPIRES_AT, Value::String("bad".into()));
        assert!(jwt.validate().validate_date_at(100, 0).is_err());

        let empty = JWT::create();
        assert!(empty.validate().validate_date_at(100, 0).is_ok());
        assert!(empty.validate().validate_date().is_ok());

        let validator = JWTValidator::of_token("e30.e30.").unwrap();
        assert!(validator.validate_algorithm().is_err());
        assert!(validator.validate_date_at(100, 0).is_ok());
        assert!(JWTValidator::of_token("invalid").is_err());
        let validator = JWTValidator::of_jwt(empty);
        assert!(validator.validate_date_at(100, 0).is_ok());
    }
}
