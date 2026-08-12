//! DB 扩展 parity 测试 5
//! 对齐: hutool-db 多个测试类

use hutool_db::{Page, PageRequest};

// ── PageRequest 综合测试 (8 tests) ──

#[test]
fn pr_first_page() {
    let p = PageRequest::new(1, 10, 100).unwrap();
    assert_eq!(p.page(), 1);
    assert_eq!(p.size(), 10);
    assert_eq!(p.offset().unwrap(), 0);
}

#[test]
fn pr_second_page() {
    let p = PageRequest::new(2, 10, 100).unwrap();
    assert_eq!(p.offset().unwrap(), 10);
}

#[test]
fn pr_third_page() {
    let p = PageRequest::new(3, 20, 100).unwrap();
    assert_eq!(p.offset().unwrap(), 40);
}

#[test]
fn pr_large_page() {
    let p = PageRequest::new(100, 10, 1000).unwrap();
    assert_eq!(p.offset().unwrap(), 990);
}

#[test]
fn pr_invalid_zero_size() {
    assert!(PageRequest::new(1, 0, 100).is_err());
}

#[test]
fn pr_invalid_exceeds_max() {
    assert!(PageRequest::new(1, 200, 100).is_err());
}

#[test]
fn pr_invalid_zero_page() {
    assert!(PageRequest::new(0, 10, 100).is_err());
}

#[test]
fn pr_max_size() {
    let p = PageRequest::new(1, 100, 100).unwrap();
    assert_eq!(p.size(), 100);
}

// ── Page 综合测试 (8 tests) ──

#[test]
fn pg_total_pages() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![1, 2, 3], r, 25);
    assert_eq!(p.total_pages(), 3);
}

#[test]
fn pg_has_next_yes() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![1, 2, 3], r, 100);
    assert!(p.has_next());
}

#[test]
fn pg_has_next_no() {
    let r = PageRequest::new(10, 10, 100).unwrap();
    let p = Page::new(vec![1], r, 100);
    assert!(!p.has_next());
}

#[test]
fn pg_exact_total() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p: Page<i32> = Page::new(vec![], r, 100);
    assert_eq!(p.total_pages(), 10);
    assert!(p.has_next());
}

#[test]
fn pg_empty() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p: Page<i32> = Page::new(vec![], r, 0);
    assert_eq!(p.total_pages(), 0);
    assert!(!p.has_next());
}

#[test]
fn pg_single_record() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let p = Page::new(vec![42], r, 1);
    assert_eq!(p.total_pages(), 1);
    assert!(!p.has_next());
}

#[test]
fn pg_many_records() {
    let r = PageRequest::new(1, 10, 100).unwrap();
    let records: Vec<i32> = (1..=10).collect();
    let p = Page::new(records, r, 100);
    assert_eq!(p.total_pages(), 10);
    assert!(p.has_next());
}

#[test]
fn pg_last_page() {
    let r = PageRequest::new(10, 10, 100).unwrap();
    let p = Page::new(vec![91, 92, 93, 94, 95, 96, 97, 98, 99, 100], r, 100);
    assert_eq!(p.total_pages(), 10);
    assert!(!p.has_next());
}
