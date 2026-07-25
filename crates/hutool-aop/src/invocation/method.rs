//! 对齐: `java.lang.reflect.Method`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: 方法元数据，表示一个被代理操作的稳定描述信息。

use std::{borrow::Cow, fmt};

/// 对齐: `java.lang.reflect.Method`
/// 中文说明: 单个被代理操作的稳定元数据，包含操作名称。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
    name: Cow<'static, str>,
}

impl Method {
    /// Creates method metadata from an owned or static name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
