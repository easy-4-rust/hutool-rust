//! 对齐: `cn.hutool.jwt.JWTPayload`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/JWTPayload.java`
//! 中文说明: Hutool JWT 载荷对象的 Rust 兼容实现，负责管理注册 claims 与普通业务 claims，
//! 并复用 `Claims` 承载动态字段集合。

use serde_json::{Map, Value};

use super::Claims;

/// JWT payload claims.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JWTPayload(Claims);

impl JWTPayload {
    /// Issuer claim.
    pub const ISSUER: &'static str = "iss";
    /// Subject claim.
    pub const SUBJECT: &'static str = "sub";
    /// Audience claim.
    pub const AUDIENCE: &'static str = "aud";
    /// Expiration claim.
    pub const EXPIRES_AT: &'static str = "exp";
    /// Not-before claim.
    pub const NOT_BEFORE: &'static str = "nbf";
    /// Issued-at claim.
    pub const ISSUED_AT: &'static str = "iat";
    /// JWT ID claim.
    pub const JWT_ID: &'static str = "jti";

    /// 基于已解析 claims 构造载荷对象。
    pub(super) const fn from_claims(claims: Claims) -> Self {
        Self(claims)
    }

    /// Sets a payload claim.
    pub fn set_payload(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        self.0.set_claim(name, value);
        self
    }

    /// Adds payload claims.
    pub fn add_payloads(&mut self, values: Map<String, Value>) -> &mut Self {
        self.0.put_all(values);
        self
    }

    /// Returns the underlying claims.
    #[must_use]
    pub const fn claims(&self) -> &Claims {
        &self.0
    }
}
