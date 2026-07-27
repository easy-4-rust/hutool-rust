//! 对齐: `cn.hutool.jwt.JWTUtil`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/JWTUtil.java`
//! 中文说明: Hutool JWT 工具类的 Rust 兼容实现，提供创建、解析与校验 token 的静态便捷入口。

use std::sync::Arc;

use serde_json::{Map, Value};

use super::{JWT, JWTException, JWTSigner};

/// 对齐 Hutool `JWTUtil` 的静态工具入口。
pub struct JWTUtil;

impl JWTUtil {
    /// 创建 HS256(HmacSHA256) JWT Token。
    pub fn create_token(payload: Map<String, Value>, key: &[u8]) -> Result<String, JWTException> {
        Self::create_token_with_headers(Map::new(), payload, key)
    }

    /// 创建带头信息的 HS256(HmacSHA256) JWT Token。
    pub fn create_token_with_headers(
        headers: Map<String, Value>,
        payload: Map<String, Value>,
        key: &[u8],
    ) -> Result<String, JWTException> {
        let mut jwt = JWT::create();
        jwt.add_headers(headers).add_payloads(payload);
        jwt.set_key(key)?;
        jwt.sign()
    }

    /// 使用显式签名器创建 JWT Token。
    pub fn create_token_with_signer(
        payload: Map<String, Value>,
        signer: Arc<dyn JWTSigner>,
    ) -> Result<String, JWTException> {
        Self::create_token_with_headers_and_signer(Map::new(), payload, signer)
    }

    /// 使用显式签名器和头信息创建 JWT Token。
    pub fn create_token_with_headers_and_signer(
        headers: Map<String, Value>,
        payload: Map<String, Value>,
        signer: Arc<dyn JWTSigner>,
    ) -> Result<String, JWTException> {
        let mut jwt = JWT::create();
        jwt.add_headers(headers).add_payloads(payload);
        jwt.sign_with(signer)
    }

    /// 解析紧凑格式 JWT Token。
    pub fn parse_token(token: &str) -> Result<JWT, JWTException> {
        JWT::of(token)
    }

    /// 使用共享密钥验证 HS256 JWT Token。
    pub fn verify(token: &str, key: &[u8]) -> Result<bool, JWTException> {
        let mut jwt = JWT::of(token)?;
        jwt.set_key(key)?;
        jwt.verify()
    }

    /// 使用显式签名器验证 JWT Token。
    pub fn verify_with_signer(token: &str, signer: &dyn JWTSigner) -> Result<bool, JWTException> {
        JWT::of(token)?.verify_with(signer)
    }
}
