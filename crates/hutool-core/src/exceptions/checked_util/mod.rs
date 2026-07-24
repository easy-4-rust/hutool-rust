//! 对齐: `cn.hutool.core.exceptions.CheckedUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/CheckedUtil.java
//!
//! 将可能失败的表达式包装为运行时错误，避免显式 try/catch。

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread;
use std::time::Duration;

mod checked_util;
mod unchecked_fn0;
mod unchecked_fn1;
mod unchecked_void_fn0;
mod wrapped_runtime;

pub use checked_util::CheckedUtil;
pub use unchecked_fn0::UncheckedFn0;
pub use unchecked_fn1::UncheckedFn1;
pub use unchecked_void_fn0::UncheckedVoidFn0;
pub use wrapped_runtime::WrappedRuntime;

pub fn sleep_checked(millis: u64) -> Result<(), std::io::Error> {
    thread::sleep(Duration::from_millis(millis));
    Ok(())
}
