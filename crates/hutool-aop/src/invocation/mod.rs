//! 对齐: `java.lang.reflect` 及 `cn.hutool.aop` 包中的调用处理
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: 方法元数据与显式调用处理器模块，提供 Method、InvocationHandler 和 HandlerProxy。

use std::{borrow::Cow, fmt};

mod method;
mod invocation_handler;
mod handler_proxy;

pub use method::Method;
pub use invocation_handler::InvocationHandler;
pub use handler_proxy::HandlerProxy;
