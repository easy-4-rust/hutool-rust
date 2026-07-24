//! SSL facade，对齐 hutool 的 `cn.hutool.http.ssl.*`。
//!
//! 提供定制协议的 SSLSocketFactory 抽象 + 信任任意主机名验证器。
//! 具体实现依赖 `javax.net.ssl.SSLSocketFactory`，Rust 用 reqwest 内置 rustls 替代。

use std::any::Any;
use std::io;

mod custom_protocols_ssl_factory;
mod custom_protocols_ssl_factory_impl;
mod trust_any_hostname_verifier;
mod trust_any_hostname_verifier_impl;

pub use custom_protocols_ssl_factory::CustomProtocolsSslFactory;
pub use custom_protocols_ssl_factory_impl::CustomProtocolsSslFactoryImpl;
pub use trust_any_hostname_verifier::TrustAnyHostnameVerifier;
pub use trust_any_hostname_verifier_impl::TrustAnyHostnameVerifierImpl;
