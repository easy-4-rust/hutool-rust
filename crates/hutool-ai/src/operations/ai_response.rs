//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use serde_json::Value;

/// Normalized raw provider response.
#[derive(Debug, Clone, PartialEq)]
pub enum AIResponse {
    /// JSON payload.
    Json(Value),
    /// Binary media payload.
    Bytes(Vec<u8>),
}

impl AIResponse {
    /// Serializes JSON or returns a lossy textual representation of bytes.
    #[must_use]
    pub fn into_text(self) -> String {
        match self {
            Self::Json(value) => value.to_string(),
            Self::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    /// Extracts binary media, serializing JSON when necessary.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Json(value) => value.to_string().into_bytes(),
            Self::Bytes(bytes) => bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_response_into_text_and_bytes() {
        let response = AIResponse::Json(json!({"answer": 42}));
        assert_eq!(response.clone().into_text(), r#"{"answer":42}"#);
        assert_eq!(response.into_bytes(), br#"{"answer":42}"#.to_vec());
    }

    #[test]
    fn bytes_response_into_text_and_bytes() {
        let bytes = AIResponse::Bytes(b"hello".to_vec());
        assert_eq!(bytes.clone().into_text(), "hello");
        assert_eq!(bytes.into_bytes(), b"hello".to_vec());
    }
}
