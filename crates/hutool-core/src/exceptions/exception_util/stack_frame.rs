//! 对齐: `cn.hutool.core.exceptions.ExceptionUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/ExceptionUtil.java
//!
//! 异常链处理与包装工具。

use std::error::Error;
use std::fmt;
use std::panic::Location;

use super::exception_util::ExceptionUtil;
use super::wrapped_error::WrappedError;

/// 栈帧信息，对齐 Java `StackTraceElement` 子集。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StackFrame {
    /// 函数名。
    pub method_name: String,
    /// 文件路径。
    pub file: Option<String>,
    /// 行号。
    pub line: Option<u32>,
}

impl ExceptionUtil {
    /// 对齐 Java: `getMessage(Throwable)`
    pub fn get_message(err: Option<&dyn Error>) -> String {
        match err {
            None => "null".to_string(),
            Some(e) => format!("{}: {}", type_name(e), e),
        }
    }

    /// 对齐 Java: `getSimpleMessage(Throwable)`
    pub fn get_simple_message(err: Option<&dyn Error>) -> String {
        err.map(|e| e.to_string())
            .unwrap_or_else(|| "null".to_string())
    }

    /// 对齐 Java: `wrapRuntime(Throwable)`
    pub fn wrap_runtime(err: Box<dyn Error + Send + Sync>) -> WrappedError {
        WrappedError {
            message: err.to_string(),
            source: Some(err),
        }
    }

    /// 对齐 Java: `wrap(Throwable, Class<T>)` —— Rust 版按目标构造器映射。
    pub fn wrap_io<E, T>(err: E, map: impl FnOnce(E) -> T) -> T {
        map(err)
    }

    /// 对齐 Java: `unwrap(Throwable)` —— 剥离一层包装。
    pub fn unwrap(err: &dyn Error) -> &dyn Error {
        err.source().unwrap_or(err)
    }

    /// 对齐 Java: `getThrowableList(Throwable)`
    pub fn get_throwable_list(err: &dyn Error) -> Vec<&dyn Error> {
        let mut list: Vec<&dyn Error> = Vec::new();
        let mut current: &dyn Error = err;
        loop {
            if list
                .iter()
                .any(|e| std::ptr::eq(*e as *const _, current as *const _))
            {
                break;
            }
            list.push(current);
            match current.source() {
                Some(next) => current = next,
                None => break,
            }
        }
        list
    }

    /// 对齐 Java: `getRootCause(Throwable)`
    pub fn get_root_cause(err: Option<&dyn Error>) -> Option<&dyn Error> {
        err.and_then(|e| Self::get_throwable_list(e).last().copied())
    }

    /// 对齐 Java: `getRootCauseMessage(Throwable)`
    pub fn get_root_cause_message(err: Option<&dyn Error>) -> String {
        Self::get_message(Self::get_root_cause(err))
    }

    /// 对齐 Java: `convertFromOrSuppressedThrowable(..., checkCause)`
    pub fn convert_from_or_suppressed<E: Error + 'static>(
        err: &(dyn Error + 'static),
        _target: std::any::TypeId,
        check_cause: bool,
    ) -> bool {
        if err.is::<E>() {
            return true;
        }
        if check_cause {
            return err
                .source()
                .map(|source| source.is::<E>())
                .unwrap_or(false);
        }
        false
    }

    /// 对齐 Java: `getRootStackElement()` —— stable Rust 下返回占位帧。
    pub fn get_root_stack_element() -> StackFrame {
        StackFrame {
            method_name: "main".to_string(),
            file: None,
            line: None,
        }
    }

    /// 对齐 Java: `getStackElements()`
    pub fn get_stack_elements() -> Vec<StackFrame> {
        vec![Self::get_root_stack_element()]
    }

    /// 对齐 Java: `getStackElement(int i)`
    pub fn get_stack_element(i: usize) -> Option<StackFrame> {
        Self::get_stack_elements().into_iter().nth(i)
    }

    /// 对齐 Java: `getStackElement(String fqcn, int i)` —— 按文件名过滤后取第 i 帧。
    pub fn get_stack_element_by_fqcn(fqcn: &str, i: usize) -> Option<StackFrame> {
        Self::get_stack_elements()
            .into_iter()
            .filter(|f| f.file.as_deref().is_some_and(|p| p.contains(fqcn)))
            .nth(i)
    }

    /// 对齐 Java: `wrapRuntime(String message)`
    pub fn wrap_runtime_message(message: impl Into<String>) -> WrappedError {
        WrappedError {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `wrapAndThrow(Throwable)` —— Rust 侧 panic。
    pub fn wrap_and_throw(err: Box<dyn Error + Send + Sync>) -> ! {
        panic!("{}", err)
    }

    /// 对齐 Java: `wrapRuntimeAndThrow(String)` —— Rust 侧 panic。
    pub fn wrap_runtime_and_throw(message: impl AsRef<str>) -> ! {
        panic!("{}", message.as_ref())
    }

    /// 对齐 Java: `stacktraceToOneLineString(Throwable)`
    pub fn stacktrace_to_one_line_string(err: &dyn Error) -> String {
        Self::stacktrace_to_one_line_string_limit(err, 3000)
    }

    /// 对齐 Java: `stacktraceToOneLineString(Throwable, int limit)`
    pub fn stacktrace_to_one_line_string_limit(err: &dyn Error, limit: usize) -> String {
        let mut s = Self::stacktrace_to_string_limit(err, limit);
        s = s.replace(['\r', '\n', '\t'], " ");
        if s.len() > limit {
            s.truncate(limit);
        }
        s
    }

    /// 对齐 Java: `stacktraceToString(Throwable)`
    pub fn stacktrace_to_string(err: &dyn Error) -> String {
        Self::stacktrace_to_string_limit(err, usize::MAX)
    }

    /// 对齐 Java: `stacktraceToString(Throwable, int limit)`
    pub fn stacktrace_to_string_limit(err: &dyn Error, limit: usize) -> String {
        let mut out = String::new();
        for (i, item) in Self::get_throwable_list(err).into_iter().enumerate() {
            if i > 0 {
                out.push_str("\nCaused by: ");
            }
            out.push_str(&format!("{}: {}", type_name(item), item));
            if out.len() >= limit {
                out.truncate(limit);
                break;
            }
        }
        out
    }

    /// 对齐 Java: `stacktraceToString(Throwable, int, Map)` —— 简单字符替换。
    pub fn stacktrace_to_string_replace(
        err: &dyn Error,
        limit: usize,
        replace: &[(char, &str)],
    ) -> String {
        let mut s = Self::stacktrace_to_string_limit(err, limit);
        for (ch, to) in replace {
            s = s.replace(*ch, to);
        }
        s
    }

    /// 对齐 Java: `isCausedBy(Throwable, Class...)`
    pub fn is_caused_by(err: &dyn Error, matcher: impl Fn(&dyn Error) -> bool) -> bool {
        Self::get_caused_by(err, matcher).is_some()
    }

    /// 对齐 Java: `getCausedBy(Throwable, Class...)`
    pub fn get_caused_by<'a>(
        err: &'a dyn Error,
        matcher: impl Fn(&dyn Error) -> bool,
    ) -> Option<&'a dyn Error> {
        for item in Self::get_throwable_list(err) {
            if matcher(item) {
                return Some(item);
            }
        }
        None
    }

    /// 测试辅助：构造带 cause 的链。
    pub fn chain(message: &str, cause: Box<dyn Error + Send + Sync>) -> WrappedError {
        WrappedError {
            message: message.to_string(),
            source: Some(cause),
        }
    }

    /// 记录调用位置（测试用）。
    pub fn here() -> StackFrame {
        let loc = Location::caller();
        StackFrame {
            method_name: loc.file().to_string(),
            file: Some(loc.file().to_string()),
            line: Some(loc.line()),
        }
    }
}

fn type_name(err: &dyn Error) -> String {
    std::any::type_name_of_val(err)
        .rsplit("::")
        .next()
        .unwrap_or("Error")
        .to_string()
}
