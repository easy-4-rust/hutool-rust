//! HTTP 扩展 parity 测试
//! 对齐: hutool-http 多个测试类

use hh::UrlPolicy;
use hutool_http as hh;
use std::time::Duration;

// ── HttpClient Builder (3 tests) ──

#[test]
fn builder_basic() {
    let result = hh::HttpClient::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(10))
        .build();
    assert!(result.is_ok());
}

#[test]
fn builder_max_response_size() {
    let result = hh::HttpClient::builder()
        .max_response_size(8 * 1024 * 1024)
        .build();
    assert!(result.is_ok());
}

#[test]
fn builder_redirect_limit() {
    let result = hh::HttpClient::builder().redirect_limit(5).build();
    assert!(result.is_ok());
}

// ── URL Policy (5 tests) ──

#[test]
fn deny_local_external() {
    let policy = hh::DenyLocalTargets;
    let url = "https://example.com".parse::<hh::Url>().unwrap();
    assert!(policy.validate(&url).is_ok());
}

#[test]
fn deny_local_localhost() {
    let policy = hh::DenyLocalTargets;
    assert!(
        policy
            .validate(&"http://127.0.0.1".parse::<hh::Url>().unwrap())
            .is_err()
    );
    assert!(
        policy
            .validate(&"http://localhost".parse::<hh::Url>().unwrap())
            .is_err()
    );
}

#[test]
fn deny_local_private_ip() {
    let policy = hh::DenyLocalTargets;
    assert!(
        policy
            .validate(&"http://192.168.1.1".parse::<hh::Url>().unwrap())
            .is_err()
    );
    assert!(
        policy
            .validate(&"http://10.0.0.1".parse::<hh::Url>().unwrap())
            .is_err()
    );
}

#[test]
fn allow_all_localhost() {
    let policy = hh::AllowAllUrls;
    assert!(
        policy
            .validate(&"http://127.0.0.1".parse::<hh::Url>().unwrap())
            .is_ok()
    );
}

#[test]
fn allow_all_private() {
    let policy = hh::AllowAllUrls;
    assert!(
        policy
            .validate(&"http://192.168.1.1".parse::<hh::Url>().unwrap())
            .is_ok()
    );
}

// ── ContentType (3 tests) ──

#[test]
fn content_type_json() {
    assert_eq!(hh::ContentType::Json.value(), "application/json");
}

#[test]
fn content_type_xml() {
    assert_eq!(hh::ContentType::Xml.value(), "application/xml");
}

#[test]
fn content_type_text() {
    assert_eq!(hh::ContentType::TextPlain.value(), "text/plain");
}

// ── HttpStatus (5 tests) ──

#[test]
fn status_success() {
    assert_eq!(hh::HttpStatus::HTTP_OK, 200);
    assert_eq!(hh::HttpStatus::HTTP_CREATED, 201);
}

#[test]
fn status_redirection() {
    assert_eq!(hh::HttpStatus::HTTP_MOVED_PERM, 301);
    assert_eq!(hh::HttpStatus::HTTP_NOT_MODIFIED, 304);
}

#[test]
fn status_client_error() {
    assert_eq!(hh::HttpStatus::HTTP_BAD_REQUEST, 400);
    assert_eq!(hh::HttpStatus::HTTP_NOT_FOUND, 404);
}

#[test]
fn status_server_error() {
    assert_eq!(hh::HttpStatus::HTTP_INTERNAL_ERROR, 500);
    assert_eq!(hh::HttpStatus::HTTP_UNAVAILABLE, 503);
}

#[test]
fn status_is_redirected() {
    assert!(hh::HttpStatus::is_redirected(301));
    assert!(hh::HttpStatus::is_redirected(302));
    assert!(!hh::HttpStatus::is_redirected(200));
}

// ── RetryPolicy (2 tests) ──

#[test]
fn retry_policy_valid() {
    let result = hh::RetryPolicy::new(3);
    assert!(result.is_ok());
}

#[test]
fn retry_policy_invalid() {
    let result = hh::RetryPolicy::new(0);
    assert!(result.is_err());
}
