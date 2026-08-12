//! 对齐: `cn.hutool.jwt.JWTHeader`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/JWTHeader.java`
//! 中文说明: Hutool JWT 头部对象的 Rust 兼容实现，负责管理 `alg`、`typ`、`cty`、`kid`
//! 等受保护头字段，并复用 `Claims` 承载动态扩展头。

use serde_json::{Map, Value};

use super::Claims;

/// JWT protected header.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JWTHeader(Claims);

impl JWTHeader {
    /// `alg` header name.
    pub const ALGORITHM: &'static str = "alg";
    /// `typ` header name.
    pub const TYPE: &'static str = "typ";
    /// `cty` header name.
    pub const CONTENT_TYPE: &'static str = "cty";
    /// `kid` header name.
    pub const KEY_ID: &'static str = "kid";

    /// 基于已解析 claims 构造头部对象。
    pub(super) const fn from_claims(claims: Claims) -> Self {
        Self(claims)
    }

    /// Sets `alg`.
    pub fn set_algorithm(&mut self, value: impl Into<String>) -> &mut Self {
        self.0
            .set_claim(Self::ALGORITHM, Value::String(value.into()));
        self
    }

    /// Sets `typ`.
    pub fn set_type(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.set_claim(Self::TYPE, Value::String(value.into()));
        self
    }

    /// Sets `cty`.
    pub fn set_content_type(&mut self, value: impl Into<String>) -> &mut Self {
        self.0
            .set_claim(Self::CONTENT_TYPE, Value::String(value.into()));
        self
    }

    /// Sets `kid`.
    pub fn set_key_id(&mut self, value: impl Into<String>) -> &mut Self {
        self.0.set_claim(Self::KEY_ID, Value::String(value.into()));
        self
    }

    /// Adds protected headers.
    pub fn add_headers(&mut self, values: Map<String, Value>) -> &mut Self {
        self.0.put_all(values);
        self
    }

    /// Returns the underlying claims.
    #[must_use]
    pub const fn claims(&self) -> &Claims {
        &self.0
    }

    /// Returns mutable underlying claims for internal composition.
    pub(super) fn claims_mut(&mut self) -> &mut Claims {
        &mut self.0
    }
}
