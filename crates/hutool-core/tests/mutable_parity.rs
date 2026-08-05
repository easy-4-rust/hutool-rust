//! mutable module parity tests
//! 对齐: hutool-core MutableTest

use hutool_core::{
    MutableBool, MutableByte, MutableDouble, MutableFloat, MutableInt, MutableLong, MutableObj,
    MutablePair, MutableShort,
};

// ── MutableInt ──

#[test]
fn mutable_int_new_and_get() {
    let mut m = MutableInt::new(42);
    assert_eq!(m.get(), 42);
    m.set(100);
    assert_eq!(m.get(), 100);
}

#[test]
fn mutable_int_increment() {
    let mut m = MutableInt::new(0);
    m.increment();
    assert_eq!(m.get(), 1);
    m.increment();
    assert_eq!(m.get(), 2);
}

#[test]
fn mutable_int_decrement() {
    let mut m = MutableInt::new(5);
    m.decrement();
    assert_eq!(m.get(), 4);
}

#[test]
fn mutable_int_add() {
    let mut m = MutableInt::new(10);
    m.add(5);
    assert_eq!(m.get(), 15);
}

#[test]
fn mutable_int_subtract() {
    let mut m = MutableInt::new(10);
    m.subtract(3);
    assert_eq!(m.get(), 7);
}

#[test]
fn mutable_int_java_hash_code() {
    let m = MutableInt::new(42);
    assert_eq!(m.java_hash_code(), 42);
}

// ── MutableLong ──

#[test]
fn mutable_long_new_and_get() {
    let mut m = MutableLong::new(123456789012345i64);
    assert_eq!(m.get(), 123456789012345i64);
    m.set(999);
    assert_eq!(m.get(), 999);
}

#[test]
fn mutable_long_increment() {
    let mut m = MutableLong::new(0);
    m.increment();
    assert_eq!(m.get(), 1);
}

#[test]
fn mutable_long_add() {
    let mut m = MutableLong::new(100);
    m.add(50);
    assert_eq!(m.get(), 150);
}

#[test]
fn mutable_long_int_value() {
    let m = MutableLong::new(42);
    assert_eq!(m.int_value(), 42);
}

#[test]
fn mutable_long_long_value() {
    let m = MutableLong::new(123456789012345i64);
    assert_eq!(m.long_value(), 123456789012345i64);
}

#[test]
fn mutable_long_float_value() {
    let m = MutableLong::new(42);
    assert_eq!(m.float_value(), 42.0);
}

#[test]
fn mutable_long_double_value() {
    let m = MutableLong::new(42);
    assert_eq!(m.double_value(), 42.0);
}

// ── MutableBool ──

#[test]
fn mutable_bool_new_and_get() {
    let mut m = MutableBool::new(true);
    assert!(m.get());
    m.set(false);
    assert!(!m.get());
}

#[test]
fn mutable_bool_toggle() {
    let mut m = MutableBool::new(true);
    m.set(false);
    assert!(!m.get());
    m.set(true);
    assert!(m.get());
}

// ── MutableDouble ──

#[test]
fn mutable_double_new_and_get() {
    let mut m = MutableDouble::new(3.14);
    assert_eq!(m.get(), 3.14);
    m.set(2.71);
    assert_eq!(m.get(), 2.71);
}

#[test]
fn mutable_double_add() {
    let mut m = MutableDouble::new(1.0);
    m.add(2.5);
    assert_eq!(m.get(), 3.5);
}

#[test]
fn mutable_double_subtract() {
    let mut m = MutableDouble::new(10.0);
    m.subtract(3.0);
    assert_eq!(m.get(), 7.0);
}

// ── MutableFloat ──

#[test]
fn mutable_float_new_and_get() {
    let mut m = MutableFloat::new(1.5f32);
    assert_eq!(m.get(), 1.5f32);
    m.set(2.5f32);
    assert_eq!(m.get(), 2.5f32);
}

// ── MutableShort ──

#[test]
fn mutable_short_new_and_get() {
    let mut m = MutableShort::new(100i16);
    assert_eq!(m.get(), 100i16);
    m.set(200i16);
    assert_eq!(m.get(), 200i16);
}

#[test]
fn mutable_short_increment() {
    let mut m = MutableShort::new(0);
    m.increment();
    assert_eq!(m.get(), 1);
}

// ── MutableByte ──

#[test]
fn mutable_byte_new_and_get() {
    let mut m = MutableByte::new(127i8);
    assert_eq!(m.get(), 127i8);
    m.set(0i8);
    assert_eq!(m.get(), 0i8);
}

#[test]
fn mutable_byte_increment() {
    let mut m = MutableByte::new(0);
    m.increment();
    assert_eq!(m.get(), 1);
}

// ── MutablePair ──

#[test]
fn mutable_pair_new_and_get() {
    let m = MutablePair::new(1, "hello");
    assert_eq!(*m.key(), 1);
    assert_eq!(*m.value(), "hello");
}

// ── MutableObj ──

#[test]
fn mutable_obj_new_and_get() {
    let mut m = MutableObj::new(vec![1, 2, 3]);
    assert_eq!(m.get(), &vec![1, 2, 3]);
    m.set(vec![4, 5]);
    assert_eq!(m.get(), &vec![4, 5]);
}
