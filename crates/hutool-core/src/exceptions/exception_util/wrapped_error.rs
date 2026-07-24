//! 对齐: `cn.hutool.core.exceptions.ExceptionUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/ExceptionUtil.java
//!
//! 异常链处理与包装工具。

use std::error::Error;
use std::fmt;
use std::panic::Location;

/// 可携带 cause 的运行时错误，对齐 Java checked → runtime 包装场景。
#[derive(Debug)]
pub struct WrappedError {
    /// 展示消息。
    pub message: String,
    /// 根因。
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for WrappedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for WrappedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}
