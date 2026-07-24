//! Typed method metadata and explicit invocation handlers.

use std::{borrow::Cow, fmt};

mod method;
mod invocation_handler;
mod handler_proxy;

pub use method::Method;
pub use invocation_handler::InvocationHandler;
pub use handler_proxy::HandlerProxy;
