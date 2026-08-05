//! hutool-db parity tests
use hutool_db as hd;
#[test]
fn page_request_test() {
    let page = hd::PageRequest::new(1, 10, 100).unwrap();
    assert_eq!(page.page(), 1);
    assert_eq!(page.size(), 10);
    assert_eq!(page.offset().unwrap(), 0);
}
#[test]
fn page_request_invalid_size_test() {
    let result = hd::PageRequest::new(1, 0, 100);
    assert!(result.is_err());
}
#[test]
fn page_test() {
    let request = hd::PageRequest::new(2, 10, 100).unwrap();
    let page = hd::Page::new(vec![1, 2, 3], request, 30);
    assert_eq!(page.total_pages(), 3);
    assert!(page.has_next());
}
