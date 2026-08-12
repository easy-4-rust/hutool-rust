//! 对齐: `cn.hutool.jwt.JWTValidator`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/JWTValidator.java`
//! 中文说明: Hutool JWT 数据校验器的 Rust 兼容实现，负责算法一致性、签名以及注册时间字段校验。

use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use super::{JWT, JWTException, JWTPayload, JWTSigner};

/// JWT 算法与注册时间字段校验器。
pub struct JWTValidator {
    jwt: JWT,
}

impl JWTValidator {
    /// 基于 JWT 对象创建验证器。
    #[must_use]
    pub fn new(jwt: &JWT) -> Self {
        Self { jwt: jwt.clone() }
    }

    /// 基于紧凑格式 token 创建验证器。
    pub fn of_token(token: &str) -> Result<Self, JWTException> {
        Ok(Self {
            jwt: JWT::of(token)?,
        })
    }

    /// 基于已拥有所有权的 JWT 对象创建验证器。
    #[must_use]
    pub const fn of_jwt(jwt: JWT) -> Self {
        Self { jwt }
    }

    /// 校验 JWT 自带签名器的算法与签名。
    pub fn validate_algorithm(&self) -> Result<&Self, JWTException> {
        let signer = self
            .jwt
            .signer()
            .ok_or_else(|| JWTException::new("no signer provided"))?;
        self.validate_algorithm_with(signer)
    }

    /// 使用显式签名器校验头部算法与签名。
    pub fn validate_algorithm_with(&self, signer: &dyn JWTSigner) -> Result<&Self, JWTException> {
        if self.jwt.verify_with(signer)? {
            Ok(self)
        } else {
            Err(JWTException::new("signature verification failed"))
        }
    }

    /// 按显式纪元秒校验 `nbf`、`exp` 与 `iat`，并支持 leeway。
    pub fn validate_date_at(&self, now: u64, leeway: u64) -> Result<&Self, JWTException> {
        let claims = self.jwt.payload.claims();
        let number = |name| {
            claims
                .get_claim(name)
                .map(Value::as_u64)
                .transpose_value(name)
        };
        if let Some(nbf) = number(JWTPayload::NOT_BEFORE)? {
            if nbf > now.saturating_add(leeway) {
                return Err(JWTException::new("token is not active yet"));
            }
        }
        if let Some(exp) = number(JWTPayload::EXPIRES_AT)? {
            if exp.saturating_add(leeway) < now {
                return Err(JWTException::new("token has expired"));
            }
        }
        if let Some(iat) = number(JWTPayload::ISSUED_AT)? {
            if iat > now.saturating_add(leeway) {
                return Err(JWTException::new("token was issued in the future"));
            }
        }
        Ok(self)
    }

    /// 基于当前 UTC 秒时间校验注册日期字段。
    pub fn validate_date(&self) -> Result<&Self, JWTException> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.validate_date_at(now, 0)
    }
}

trait TransposeValue {
    fn transpose_value(self, name: &str) -> Result<Option<u64>, JWTException>;
}

impl TransposeValue for Option<Option<u64>> {
    fn transpose_value(self, name: &str) -> Result<Option<u64>, JWTException> {
        match self {
            None => Ok(None),
            Some(Some(value)) => Ok(Some(value)),
            Some(None) => Err(JWTException::formatted(
                "registered claim {} must be an unsigned integer",
                &[&name],
            )),
        }
    }
}
