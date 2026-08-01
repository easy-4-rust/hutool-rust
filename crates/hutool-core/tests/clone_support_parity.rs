//! clone_support module parity tests
//! 对齐: hutool-core CloneTest

use hutool_core::{CloneSupport, CloneRuntimeException};

// ── CloneSupport ──

#[test]
fn clone_support_new_and_inner() {
    let cs = CloneSupport::new(42);
    assert_eq!(cs.into_inner(), 42);
}

#[test]
fn clone_support_string() {
    let cs = CloneSupport::new("hello".to_string());
    assert_eq!(cs.into_inner(), "hello");
}

// ── DefaultCloneable ──

#[test]
fn default_cloneable_vec() {
    let v = vec![1, 2, 3];
    let cloned = v.clone();
    assert_eq!(v, cloned);
}

// ── CloneRuntimeException ──

#[test]
fn clone_runtime_exception_new() {
    let e = CloneRuntimeException::new("clone failed");
    assert!(e.to_string().contains("clone failed"));
}

#[test]
fn clone_runtime_exception_formatted() {
    let e = CloneRuntimeException::formatted("error at {}:{}", &[&"line", &"42"]);
    assert!(e.to_string().contains("line"));
}
