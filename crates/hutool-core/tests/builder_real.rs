//! `builder` 子包真实功能测试
//! 对齐: hutool-core/builder CompareToBuilder/EqualsBuilder/HashCodeBuilder
//! 基于 hutool-core builder/mod.rs 真实实现

use hutool_core::{CompareToBuilder, EqualsBuilder, GenericBuilder, HashCodeBuilder, IdKey};

// ── GenericBuilder ──

#[test]
fn generic_builder_of_and_build() {
    let mut gb = GenericBuilder::of(|| vec![1, 2, 3]);
    let result = gb.build();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn generic_builder_with_modifier() {
    let mut gb = GenericBuilder::of(|| vec![1, 2, 3]).with(|v| v.push(4));
    let result = gb.build();
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn generic_builder_multiple_modifiers() {
    let mut gb = GenericBuilder::of(|| vec![1, 2, 3])
        .with(|v| v.push(4))
        .with(|v| v.push(5));
    assert_eq!(gb.pending_modifiers(), 2);
    let result = gb.build();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn generic_builder_of1() {
    let mut gb = GenericBuilder::of1(|n: i32| vec![0; n as usize], 3);
    assert_eq!(gb.build(), vec![0, 0, 0]);
}

#[test]
fn generic_builder_of2() {
    let mut gb = GenericBuilder::of2(|a: i32, b: i32| a + b, 10, 20);
    assert_eq!(gb.build(), 30);
}

#[test]
fn generic_builder_with1() {
    let mut gb = GenericBuilder::of(|| String::from("hello")).with1(
        |s: &mut String, suffix: &String| s.push_str(suffix),
        " world".to_string(),
    );
    assert_eq!(gb.build(), "hello world");
}

// ── CompareToBuilder ──

#[test]
fn compare_to_builder_equal() {
    let result = CompareToBuilder::new()
        .append(&1, &1)
        .append(&"abc", &"abc")
        .build();
    assert_eq!(result, 0);
}

#[test]
fn compare_to_builder_less() {
    let result = CompareToBuilder::new().append(&1, &2).build();
    assert!(result < 0);
}

#[test]
fn compare_to_builder_greater() {
    let result = CompareToBuilder::new().append(&5, &3).build();
    assert!(result > 0);
}

#[test]
fn compare_to_builder_super() {
    let result = CompareToBuilder::new()
        .append_super(-1)
        .append(&1, &1)
        .build();
    // append_super(-1) means already less; short-circuit
    assert!(result < 0);
}

#[test]
fn compare_to_builder_option_some_vs_none() {
    let result = CompareToBuilder::new()
        .append_option(Some(&1), None::<&i32>)
        .build();
    assert!(result > 0);
}

#[test]
fn compare_to_builder_f32() {
    let result = CompareToBuilder::new().append_f32(1.0, 2.0).build();
    assert!(result < 0);
}

#[test]
fn compare_to_builder_f64() {
    let result = CompareToBuilder::new().append_f64(3.14, 3.14).build();
    assert_eq!(result, 0);
}

#[test]
fn compare_to_builder_slice() {
    let result = CompareToBuilder::new()
        .append_slice(&[1, 2, 3], &[1, 2, 3])
        .build();
    assert_eq!(result, 0);
}

// ── EqualsBuilder ──

#[test]
fn equals_builder_equal() {
    let result = EqualsBuilder::new()
        .append(&1, &1)
        .append(&"abc", &"abc")
        .build();
    assert!(result);
}

#[test]
fn equals_builder_not_equal() {
    let result = EqualsBuilder::new()
        .append(&1, &2)
        .append(&"abc", &"abc")
        .build();
    assert!(!result);
}

#[test]
fn equals_builder_f32() {
    let result = EqualsBuilder::new().append_f32(1.0, 1.0).build();
    assert!(result);
}

#[test]
fn equals_builder_f64() {
    let result = EqualsBuilder::new().append_f64(3.14, 3.15).build();
    assert!(!result);
}

#[test]
fn equals_builder_reset() {
    let mut eb = EqualsBuilder::new();
    eb.append(&1, &2);
    eb.reset();
    let result = eb.build();
    assert!(result, "after reset should be true");
}

#[test]
fn equals_builder_append_super() {
    let result = EqualsBuilder::new()
        .append_super(false)
        .append(&1, &1)
        .build();
    assert!(!result, "append_super(false) short-circuits");
}

// ── HashCodeBuilder ──

#[test]
fn hash_code_builder_deterministic() {
    let h1 = HashCodeBuilder::new()
        .append_i32(42)
        .append_str("hello")
        .build();
    let h2 = HashCodeBuilder::new()
        .append_i32(42)
        .append_str("hello")
        .build();
    assert_eq!(h1, h2, "same inputs should produce same hash");
}

#[test]
fn hash_code_builder_different_values() {
    let h1 = HashCodeBuilder::new().append_i32(1).build();
    let h2 = HashCodeBuilder::new().append_i32(2).build();
    assert_ne!(h1, h2);
}

#[test]
fn hash_code_builder_bool() {
    let h1 = HashCodeBuilder::new().append_bool(true).build();
    let h2 = HashCodeBuilder::new().append_bool(false).build();
    assert_ne!(h1, h2);
}

#[test]
fn hash_code_builder_i64() {
    let h = HashCodeBuilder::new()
        .append_i64(123456789012345i64)
        .build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_f32() {
    let h = HashCodeBuilder::new().append_f32(3.14).build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_f64() {
    let h = HashCodeBuilder::new().append_f64(2.71828).build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_with_constants() {
    let h = HashCodeBuilder::with_constants(17, 37)
        .unwrap()
        .append_i32(42)
        .build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_with_constants_invalid() {
    let result = HashCodeBuilder::with_constants(0, 37);
    assert!(result.is_err(), "initial must be non-zero");
}

#[test]
fn hash_code_builder_char() {
    let h1 = HashCodeBuilder::new().append_char('A' as u16).build();
    let h2 = HashCodeBuilder::new().append_char('B' as u16).build();
    assert_ne!(h1, h2);
}

#[test]
fn hash_code_builder_i8_i16() {
    let h = HashCodeBuilder::new()
        .append_i8(127)
        .append_i16(32767)
        .build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_append_hash_code() {
    let inner = HashCodeBuilder::new().append_i32(1).build();
    let h = HashCodeBuilder::new().append_hash_code(inner).build();
    assert_ne!(h, 0);
}

#[test]
fn hash_code_builder_reflection() {
    let h = HashCodeBuilder::reflection_hash_code(&42i32);
    assert!(h.is_ok());
    assert_ne!(h.unwrap(), 0);
}

// ── IdKey ──

#[test]
fn id_key_hash_code() {
    let val = 42i32;
    let key = IdKey::new(&val);
    let h = key.hash_code();
    assert_ne!(h, 0);
}

#[test]
fn id_key_two_instances_same_ref() {
    let val = 42i32;
    let k1 = IdKey::new(&val);
    let k2 = IdKey::new(&val);
    assert_eq!(k1.hash_code(), k2.hash_code());
}
