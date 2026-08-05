//! hutool-http parity tests
use hh::UrlPolicy;
use hutool_http as hh;
use std::time::Duration;

#[test]
fn http_client_builder_test() {
    let result = hh::HttpClient::builder()
        .connect_timeout(Duration::from_secs(3))
        .timeout(Duration::from_secs(10))
        .max_response_size(8 * 1024 * 1024)
        .redirect_limit(5)
        .build();
    assert!(result.is_ok(), "HttpClient builder 应成功");
}

#[test]
fn url_policy_deny_local_test() {
    let policy = hh::DenyLocalTargets;
    let url_ok = "https://example.com".parse::<hh::Url>().unwrap();
    assert!(policy.validate(&url_ok).is_ok(), "外部 URL 应允许");
    let url_local = "http://127.0.0.1".parse::<hh::Url>().unwrap();
    assert!(policy.validate(&url_local).is_err(), "localhost 应拒绝");
    let url_private = "http://192.168.1.1".parse::<hh::Url>().unwrap();
    assert!(policy.validate(&url_private).is_err(), "私有 IP 应拒绝");
}

#[test]
fn url_policy_allow_all_test() {
    let policy = hh::AllowAllUrls;
    let url = "http://127.0.0.1".parse::<hh::Url>().unwrap();
    assert!(policy.validate(&url).is_ok(), "AllowAll 应允许一切");
}

#[test]
fn content_type_test() {
    assert_eq!(hh::ContentType::Json.value(), "application/json");
    assert_eq!(hh::ContentType::Xml.value(), "application/xml");
    assert_eq!(hh::ContentType::TextPlain.value(), "text/plain");
}

#[test]
fn status_code_test() {
    assert_eq!(hh::HttpStatus::HTTP_OK, 200);
    assert_eq!(hh::HttpStatus::HTTP_NOT_FOUND, 404);
    assert_eq!(hh::HttpStatus::HTTP_INTERNAL_ERROR, 500);
}

// ── 扩展 http 测试 ──

#[test]
fn retry_policy_basic() {
    let _policy = hh::RetryPolicy::new(3).unwrap();
    // policy created successfully
}

#[test]
fn retry_policy_invalid() {
    let result = hh::RetryPolicy::new(0);
    assert!(result.is_err());
}

#[test]
fn url_policy_deny_local_localhost() {
    let policy = hh::DenyLocalTargets;
    assert!(
        policy
            .validate(&"http://localhost".parse::<hh::Url>().unwrap())
            .is_err()
    );
    assert!(
        policy
            .validate(&"http://localhost:8080".parse::<hh::Url>().unwrap())
            .is_err()
    );
}

#[test]
fn url_policy_deny_local_private_ip() {
    let policy = hh::DenyLocalTargets;
    assert!(
        policy
            .validate(&"http://10.0.0.1".parse::<hh::Url>().unwrap())
            .is_err()
    );
    assert!(
        policy
            .validate(&"http://172.16.0.1".parse::<hh::Url>().unwrap())
            .is_err()
    );
}

#[test]
fn url_policy_allow_all_localhost() {
    let policy = hh::AllowAllUrls;
    assert!(
        policy
            .validate(&"http://127.0.0.1".parse::<hh::Url>().unwrap())
            .is_ok()
    );
    assert!(
        policy
            .validate(&"http://localhost".parse::<hh::Url>().unwrap())
            .is_ok()
    );
}

#[test]
fn content_type_all_variants() {
    assert_eq!(hh::ContentType::TextHtml.value(), "text/html");
    assert_eq!(
        hh::ContentType::FormUrlEncoded.value(),
        "application/x-www-form-urlencoded"
    );
    assert_eq!(
        hh::ContentType::OctetStream.value(),
        "application/octet-stream"
    );
}

#[test]
fn status_code_success() {
    assert_eq!(hh::HttpStatus::HTTP_OK, 200);
    assert_eq!(hh::HttpStatus::HTTP_CREATED, 201);
    assert_eq!(hh::HttpStatus::HTTP_ACCEPTED, 202);
    assert_eq!(hh::HttpStatus::HTTP_NO_CONTENT, 204);
}

#[test]
fn status_code_redirection() {
    assert_eq!(hh::HttpStatus::HTTP_MOVED_PERM, 301);
    assert_eq!(hh::HttpStatus::HTTP_MOVED_TEMP, 302);
    assert_eq!(hh::HttpStatus::HTTP_NOT_MODIFIED, 304);
}

#[test]
fn status_code_client_error() {
    assert_eq!(hh::HttpStatus::HTTP_BAD_REQUEST, 400);
    assert_eq!(hh::HttpStatus::HTTP_UNAUTHORIZED, 401);
    assert_eq!(hh::HttpStatus::HTTP_FORBIDDEN, 403);
    assert_eq!(hh::HttpStatus::HTTP_NOT_FOUND, 404);
    assert_eq!(hh::HttpStatus::HTTP_BAD_METHOD, 405);
    assert_eq!(hh::HttpStatus::HTTP_CLIENT_TIMEOUT, 408);
}

#[test]
fn status_code_server_error() {
    assert_eq!(hh::HttpStatus::HTTP_INTERNAL_ERROR, 500);
    assert_eq!(hh::HttpStatus::HTTP_NOT_IMPLEMENTED, 501);
    assert_eq!(hh::HttpStatus::HTTP_BAD_GATEWAY, 502);
    assert_eq!(hh::HttpStatus::HTTP_UNAVAILABLE, 503);
    assert_eq!(hh::HttpStatus::HTTP_GATEWAY_TIMEOUT, 504);
}

#[test]
fn status_code_is_redirected() {
    assert!(hh::HttpStatus::is_redirected(301));
    assert!(hh::HttpStatus::is_redirected(302));
    assert!(!hh::HttpStatus::is_redirected(200));
    assert!(!hh::HttpStatus::is_redirected(404));
}
