//! DB extended parity tests
//! 对齐: hutool-db EntityTest/PageUtilTest

use hutool_db::{Page, PageRequest};

// ── PageRequest ──

#[test]
fn page_request_first_page() {
    let page = PageRequest::new(1, 10, 100).unwrap();
    assert_eq!(page.page(), 1);
    assert_eq!(page.size(), 10);
    assert_eq!(page.offset().unwrap(), 0);
}

#[test]
fn page_request_second_page() {
    let page = PageRequest::new(2, 10, 100).unwrap();
    assert_eq!(page.page(), 2);
    assert_eq!(page.offset().unwrap(), 10);
}

#[test]
fn page_request_third_page() {
    let page = PageRequest::new(3, 20, 100).unwrap();
    assert_eq!(page.offset().unwrap(), 40);
}

#[test]
fn page_request_invalid_size() {
    let result = PageRequest::new(1, 0, 100);
    assert!(result.is_err());
}

#[test]
fn page_request_exceeds_max() {
    let result = PageRequest::new(1, 200, 100);
    assert!(result.is_err());
}

#[test]
fn page_request_zero_page() {
    let result = PageRequest::new(0, 10, 100);
    assert!(result.is_err());
}

// ── Page ──

#[test]
fn page_total_pages() {
    let request = PageRequest::new(1, 10, 100).unwrap();
    let page = Page::new(vec![1, 2, 3], request, 25);
    assert_eq!(page.total_pages(), 3);
}

#[test]
fn page_has_next_true() {
    let request = PageRequest::new(1, 10, 100).unwrap();
    let page = Page::new(vec![1, 2, 3], request, 100);
    assert!(page.has_next());
}

#[test]
fn page_has_next_false() {
    let request = PageRequest::new(10, 10, 100).unwrap();
    let page = Page::new(vec![1], request, 100);
    assert!(!page.has_next());
}

#[test]
fn page_exact_total() {
    let request = PageRequest::new(1, 10, 100).unwrap();
    let page: Page<i32> = Page::new(vec![], request, 100);
    assert_eq!(page.total_pages(), 10);
    assert!(page.has_next());
}

#[test]
fn page_empty_records() {
    let request = PageRequest::new(1, 10, 100).unwrap();
    let page: Page<i32> = Page::new(vec![], request, 0);
    assert_eq!(page.total_pages(), 0);
    assert!(!page.has_next());
}
