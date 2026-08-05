//! `cn.hutool.core.exceptions` 缺口 parity
//!
//! 对齐: `cn.hutool.core.exceptions.*` 未覆盖 @Test

use hutool_core::convert::Convert;
use hutool_core::convert::ConvertValue;
use hutool_core::exceptions::{CheckedUtil, ExceptionUtil, WrappedError, sleep_checked};
use std::error::Error;
use std::io;
use std::panic::{AssertUnwindSafe, catch_unwind};

#[derive(Debug)]
struct IoErr(String);

impl std::fmt::Display for IoErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IoErr {}

// ── CheckedUtilTest ──

/// 对齐 Java: `CheckedUtilTest.sleepTest()`
#[test]
fn checked_util_sleep_test() {
    let func = CheckedUtil::uncheck_void(|| {
        sleep_checked(1).expect("sleep");
    });
    func.call_with_runtime_exception();
}

/// 对齐 Java: `CheckedUtilTest.supplierTest()`
#[test]
fn checked_util_supplier_test() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        CheckedUtil::uncheck_result0(|| std::fs::File::open("./no-file").map(|_| ())).call();
    }));
    assert!(result.is_err());
}

/// 对齐 Java: `CheckedUtilTest.functionTest()`
#[test]
fn checked_util_function_test() {
    let result = catch_unwind(AssertUnwindSafe(|| {
        CheckedUtil::uncheck_result1(|param: String| {
            if param.len() > 5 {
                Err::<String, _>("checked".to_string())
            } else {
                Ok(param.to_uppercase())
            }
        })
        .call("hello world".to_string());
    }));
    assert!(result.is_err());
}

// ── ExceptionUtilTest ──

/// 对齐 Java: `ExceptionUtilTest.wrapTest()`
#[test]
fn exception_util_wrap_test() {
    let io = io::Error::new(io::ErrorKind::Other, "io");
    let wrapped = ExceptionUtil::wrap_io(io, |e| WrappedError {
        message: e.to_string(),
        source: None,
    });
    assert!(format!("{wrapped:?}").contains("io"));
}

/// 对齐 Java: `ExceptionUtilTest.getRootTest()`
#[test]
fn exception_util_get_root_test() {
    let frames = ExceptionUtil::get_stack_elements();
    let root = ExceptionUtil::get_root_stack_element();
    assert_eq!(Some(&root), frames.last());
}

/// 对齐 Java: `ExceptionUtilTest.convertTest()`
#[test]
fn exception_util_convert_test() {
    let io = IoErr("io".into());
    let arg = ExceptionUtil::chain("arg", Box::new(io));
    assert!(arg.source().is_some());
}

/// 对齐 Java: `ExceptionUtilTest.bytesIntConvertTest()`
#[test]
fn exception_util_bytes_int_convert_test() {
    let s = Convert::to_str(&ConvertValue::from(12_i32)).expect("str");
    let integer = Convert::to_int(&ConvertValue::Str(s)).expect("int");
    assert_eq!(integer, 12);

    let bytes = Convert::int_to_bytes(12);
    let i = Convert::bytes_to_int(&bytes);
    assert_eq!(i, 12);
}

use hutool_core::exceptions::{
    DependencyException, InvocationTargetRuntimeException, NotInitedException, StatefulException,
    UtilException, ValidateException,
};

/// Wave2: exception constructors + ExceptionUtil stacktrace helpers
#[test]
fn exceptions_types_and_stacktrace_wave2_test() {
    let s = StatefulException::with_status(503, "down");
    assert_eq!(s.get_status(), 503);
    let v = ValidateException::with_status(400, "bad");
    assert_eq!(v.get_status(), 400);
    let _ = UtilException::with_template("x={}", &[&7]);
    let _ = DependencyException::new("dep");
    let _ = NotInitedException::new("init");
    let _ = InvocationTargetRuntimeException::new("invoke");

    let err = std::io::Error::new(std::io::ErrorKind::Other, "boom\nline");
    let one = ExceptionUtil::stacktrace_to_one_line_string(&err);
    assert!(!one.contains('\n'));
    let multi = ExceptionUtil::stacktrace_to_string(&err);
    assert!(multi.contains("boom"));
    assert!(!ExceptionUtil::get_stack_elements().is_empty());
}
