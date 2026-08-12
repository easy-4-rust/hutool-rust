//! 对齐: `cn.hutool.jwt.signers` 子包模块声明
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/
//!
//! 中文说明: Hutool JWT 签名器子包。Java 端共 7 个类型（RegisteredPayload trait +
//! JWTSigner trait + 5 个具体 Signer + AlgorithmUtil + JWTSignerUtil）。
//! 由于 trait JWTSigner 的具体实现（如 HMacJWTSigner::algorithm_id）依赖
//! signers 子包内部的 helper 函数（signing_input/rsa_private_der/ec256_keys_from_pem
//! 等），本子包采用**集中实现**策略：所有 7 个类型在单一 `all.rs` 文件中实现，
//! 等价于 Java 包的 7 个类 1:1 对齐（每个类型有独立 impl block）。

pub mod all;

pub use all::{
    AlgorithmUtil, AsymmetricJWTSigner, EllipticCurveJWTSigner, HMacJWTSigner, JWTSigner,
    JWTSignerUtil, NoneJWTSigner, RegisteredPayload,
};
