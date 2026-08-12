//! page_util module parity tests
//! 对齐: hutool-core PageUtilTest

use hutool_core::PageUtil;

// ── total_page (static) ──

#[test]
fn total_page_exact() {
    assert_eq!(PageUtil::total_page_i32(100, 10).unwrap(), 10);
}

#[test]
fn total_page_remainder() {
    assert_eq!(PageUtil::total_page_i32(101, 10).unwrap(), 11);
}

#[test]
fn total_page_zero() {
    assert_eq!(PageUtil::total_page_i32(0, 10).unwrap(), 0);
}

#[test]
fn total_page_i64() {
    assert_eq!(
        PageUtil::total_page_i64(1000000000000, 1000).unwrap(),
        1000000000
    );
}

// ── segment (instance method) ──

#[test]
fn segment_first_page() {
    let pu = PageUtil::new(0);
    let range = pu.segment(0, 10);
    assert_eq!(range.start, 0);
    assert_eq!(range.end, 10);
}

#[test]
fn segment_second_page() {
    let pu = PageUtil::new(0);
    let range = pu.segment(1, 10);
    assert_eq!(range.start, 10);
    assert_eq!(range.end, 20);
}

// ── rainbow (static) ──

#[test]
fn rainbow_basic() {
    let result = PageUtil::rainbow(5, 10, 3).unwrap();
    assert!(!result.is_empty());
    assert!(result.contains(&5));
}

#[test]
fn rainbow_default_basic() {
    let result = PageUtil::rainbow_default(5, 10).unwrap();
    assert!(!result.is_empty());
}

// ── PageUtil::new ──

#[test]
fn page_util_new() {
    let pu = PageUtil::new(0);
    assert_eq!(pu.first_page_no(), 0);
    let pu1 = PageUtil::new(1);
    assert_eq!(pu1.first_page_no(), 1);
}
