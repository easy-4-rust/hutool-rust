//! Typed JSON Web Token encoding and validation.

#![forbid(unsafe_code)]

pub use jsonwebtoken::{Algorithm, Header, TokenData, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey, decode, encode};
use serde::{Serialize, de::DeserializeOwned};

mod compat;
pub mod prelude;

pub use compat::{
    AlgorithmUtil, AsymmetricJWTSigner, Claims, EllipticCurveJWTSigner, HMacJWTSigner, JWT,
    JWTException, JWTHeader, JWTPayload, JWTSigner, JWTSignerUtil, JWTUtil, JWTValidator,
    NoneJWTSigner, RegisteredPayload,
};

/// Hutool-aligned signer namespace.
pub mod signers {
    pub use crate::{
        AlgorithmUtil, AsymmetricJWTSigner, EllipticCurveJWTSigner, HMacJWTSigner, JWTSigner,
        JWTSignerUtil, NoneJWTSigner,
    };
}

/// Explicit validation requirements for issued tokens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwtValidationPolicy {
    /// Required issuer (`iss`).
    pub issuer: String,
    /// Required audience (`aud`).
    pub audience: String,
    /// Allowed clock skew in seconds.
    pub leeway_seconds: u64,
    /// Whether an expiration claim is required and validated.
    pub require_expiration: bool,
}

impl JwtValidationPolicy {
    /// Creates a scoped validation policy.
    #[must_use]
    pub fn new(
        issuer: impl Into<String>,
        audience: impl Into<String>,
        leeway_seconds: u64,
        require_expiration: bool,
    ) -> Self {
        Self {
            issuer: issuer.into(),
            audience: audience.into(),
            leeway_seconds,
            require_expiration,
        }
    }
}

/// An HMAC-SHA256 JWT codec with validation enabled by default.
#[derive(Clone)]
pub struct JwtHs256 {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtHs256 {
    /// Creates a codec from a shared secret and explicit validation policy.
    #[must_use]
    pub fn new(secret: &[u8], policy: &JwtValidationPolicy) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = policy.require_expiration;
        validation.set_required_spec_claims(if policy.require_expiration {
            &["exp", "iss", "aud"]
        } else {
            &["iss", "aud"]
        });
        validation.set_issuer(&[policy.issuer.as_str()]);
        validation.set_audience(&[policy.audience.as_str()]);
        validation.leeway = policy.leeway_seconds;
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
            validation,
        }
    }

    /// Encodes serializable claims with an HS256 header.
    pub fn encode<T: Serialize>(&self, claims: &T) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(Algorithm::HS256), claims, &self.encoding_key)
    }

    /// Decodes and validates typed claims.
    pub fn decode<T: DeserializeOwned + Clone>(
        &self,
        token: &str,
    ) -> Result<TokenData<T>, jsonwebtoken::errors::Error> {
        decode(token, &self.decoding_key, &self.validation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Claims {
        sub: String,
        iss: String,
        aud: String,
        exp: usize,
    }

    #[test]
    fn encodes_and_validates_claims() {
        let policy = JwtValidationPolicy::new("hutool", "hutool-tests", 30, true);
        let codec = JwtHs256::new(b"a sufficiently long test secret", &policy);
        let claims = Claims {
            sub: "user-1".into(),
            iss: "hutool".into(),
            aud: "hutool-tests".into(),
            exp: 4_102_444_800,
        };
        let token = codec.encode(&claims).unwrap();
        assert_eq!(codec.decode::<Claims>(&token).unwrap().claims, claims);
        assert!(
            JwtHs256::new(b"different secret", &policy)
                .decode::<Claims>(&token)
                .is_err()
        );
        let wrong_audience = JwtValidationPolicy::new("hutool", "other", 30, true);
        assert!(
            JwtHs256::new(b"a sufficiently long test secret", &wrong_audience)
                .decode::<Claims>(&token)
                .is_err()
        );

        let policy = JwtValidationPolicy::new("hutool", "hutool-tests", 0, false);
        let codec = JwtHs256::new(b"a sufficiently long test secret", &policy);
        let claims = serde_json::json!({"iss":"hutool", "aud":"hutool-tests"});
        let token = codec.encode(&claims).unwrap();
        assert!(codec.decode::<serde_json::Value>(&token).is_ok());
    }

    #[test]
    fn rejects_malformed_standard_claim_types() {
        let secret = b"a sufficiently long test secret";
        let malformed = serde_json::json!({
            "sub": "user-1",
            "nbf": "99999999999",
            "exp": 4_102_444_800_u64,
        });
        let token = encode(
            &Header::new(Algorithm::HS256),
            &malformed,
            &EncodingKey::from_secret(secret),
        )
        .unwrap();
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_nbf = true;

        let decoded =
            decode::<serde_json::Value>(&token, &DecodingKey::from_secret(secret), &validation);
        assert!(decoded.is_err());
    }
}
