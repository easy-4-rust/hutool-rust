//! 对齐: `cn.hutool.core.exceptions.CheckedUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/CheckedUtil.java
//!
//! 将可能失败的表达式包装为运行时错误，避免显式 try/catch。


/// 运行时包装错误。
#[derive(Debug, Clone)]
pub struct WrappedRuntime {
    /// 消息。
    pub message: String,
}

impl WrappedRuntime {
    /// 构造。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for WrappedRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

