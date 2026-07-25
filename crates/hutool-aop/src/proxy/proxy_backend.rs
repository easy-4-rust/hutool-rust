//! 对齐: `cn.hutool.aop.proxy` 包中的代理策略枚举
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/proxy/
//! 中文说明: 代理后端策略枚举，定义可用的代理实现方式。

use crate::{
    HandlerProxy, Method,
    aspects::Aspect,
    interceptor::{CglibInterceptor, JdkInterceptor, SpringCglibInterceptor},
};
use std::{fmt, sync::Arc};

/// 对齐: Hutool 代理策略选择
/// 中文说明: 可用的显式代理策略，支持 JDK、CGLIB 和 Spring CGLIB 三种方式。
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
