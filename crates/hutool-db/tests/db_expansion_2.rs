//! DB 扩展 parity 测试 2
//! 对齐: hutool-db 多个测试类

use hutool_db::{Page, PageRequest};

// ── PageRequest 扩展测试 (6 tests) ──

#[test]
fn page_request_first() {
    let p = PageRequest::new(1, 10, 100).unwrap();
    assert_eq!(p.page(), 1);
    assert_eq!(p.size(), 10);
    assert_eq!(p.offset().unwrap(), 0);
}

#[test]
fn page_request_second() {
    let p = PageRequest::new(2, 10, 100).unwrap();
    assert_eq!(p.offset().unwrap(), 10);
}

#[test]
fn page_request_large_page() {
    let p = PageRequest::new(100, 10, 1000).unwrap();
    assert_eq!(p.offset().unwrap(), 990);
}

#[test]
fn page_request_invalid_zero_size() {
    assert!(PageRequest::new(1, 0, 100).is_err());
}

#[test]
fn page_request_invalid_exceeds_max() {
    assert!(PageRequest::new(1, 200, 100).is_err());
}

#[test]
fn page_request_invalid_zero_page() {
    assert!(PageRequest::new(0, 10, 100).is_err());
}

// ── Page 扩展测试 (6 tests) ──

#[test]
fn page_total() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![1, 2, 3], r, 25);
    assert_eq!(p.total_pages(), 3);
}

#[test]
fn page_has_next_yes() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![1, 2, 3], r, 100);
    assert!(p.has_next());
}

#[test]
fn page_has_next_no() {
    let r = PageRequest::new(10, 10, 100).unwrap();
    let p = Page::new(vec![1], r, 100);
    assert!(!p.has_next());
}

#[test]
fn page_exact_total() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p: Page<i32> = Page::new(vec![], r, 100);
    assert_eq!(p.total_pages(), 10);
    assert!(p.has_next());
}

#[test]
fn page_empty() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p: Page<i32> = Page::new(vec![], r, 0);
    assert_eq!(p.total_pages(), 0);
    assert!(!p.has_next());
}

#[test]
fn page_single_record() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![42], r, 1);
    assert_eq!(p.total_pages(), 1);
    assert!(!p.has_next());
}
