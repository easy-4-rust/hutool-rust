//! 对齐: `cn.hutool.core.exceptions.ExceptionUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/ExceptionUtil.java
//!
//! 异常链处理与包装工具。


mod exception_util;
mod wrapped_error;
mod stack_frame;

pub use exception_util::ExceptionUtil;
pub use wrapped_error::WrappedError;
pub use stack_frame::StackFrame;
