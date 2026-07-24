//! Proxy factories and the Hutool-aligned `ProxyUtil` facade.

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

/// Available explicit proxy strategies.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ProxyBackend {
    /// JDK callback ordering.
    #[default]
    Jdk,
    /// CGLIB callback ordering.
    Cglib,
    /// Spring's repackaged CGLIB callback ordering.
    SpringCglib,
}
