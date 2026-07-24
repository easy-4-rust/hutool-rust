//! SSL facade，对齐 hutool 的 `cn.hutool.http.ssl.*`。
//!
//! 提供定制协议的 SSLSocketFactory 抽象 + 信任任意主机名验证器。
//! 具体实现依赖 `javax.net.ssl.SSLSocketFactory`，Rust 用 reqwest 内置 rustls 替代。

use std::any::Any;
use std::io;

/// 信任任意主机名验证器，对齐 `cn.hutool.http.ssl.TrustAnyHostnameVerifier`。
///
/// **安全警告**：此 trait 用于开发/测试，**禁止用于生产环境**。
/// Rust 用户应使用 reqwest 默认的 rustls 验证。
pub trait TrustAnyHostnameVerifier: Send + Sync {
    /// 对齐 `verify(String, SSLSession)`
    ///
    /// 始终返回 `true`，**不安全**。
    fn verify(&self, _hostname: &str, _session: &dyn Any) -> bool {
        true
    }
}
