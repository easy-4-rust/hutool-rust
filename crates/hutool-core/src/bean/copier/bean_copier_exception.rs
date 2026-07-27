//! 对齐: `cn.hutool.core.bean.copier.BeanCopierException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/BeanCopierException.java
//!
//! 中文说明: BeanCopier 拷贝过程中抛出的异常类型。
//! Java 侧继承 `RuntimeException`;Rust 中用 `thiserror::Error` 表示。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java exception: `cn.hutool.core.bean.copier.BeanCopierException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct BeanCopierException {
    /// 错误消息。
    pub message: String,
}

impl BeanCopierException {
    /// 创建新的异常实例。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<&str> for BeanCopierException {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for BeanCopierException {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<crate::bean::bean_exception::BeanException> for BeanCopierException {
    fn from(e: crate::bean::bean_exception::BeanException) -> Self {
        Self::new(e.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bean_copier_exception_constructors() {
        let e1 = BeanCopierException::new("oops");
        assert_eq!(e1.message, "oops");
        let e2: BeanCopierException = "bad".into();
        assert_eq!(e2.message, "bad");
        let e3: BeanCopierException = String::from("also bad").into();
        assert_eq!(e3.message, "also bad");
        let be = crate::bean::bean_exception::BeanException::new("from bean");
        let e4: BeanCopierException = be.into();
        assert_eq!(e4.message, "from bean");
    }
}