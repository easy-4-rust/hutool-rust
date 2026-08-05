//! HttpUtil / HttpRequest network facade parity (local mock).
//! 对齐: hutool-http HttpUtil / HttpRequest execute helpers

mod support;

use hutool_http::{AllowAllUrls, HttpRequest, HttpUtil, Method, form_map};
use std::sync::Arc;
use support::mock_server::{http_ok, http_response, request_path, serve_fn, serve_once};

/// 对齐 Java: `HttpUtil.createGet` / `HttpRequest.get` + `execute().body()`
#[tokio::test]
async fn http_util_create_get_execute_body() {
    let (base, task) = serve_once(http_ok("hello-util")).await;
    let text = HttpUtil::create_get(&base)
        .url_policy(AllowAllUrls)
        .timeout(5_000)
        .execute_body()
        .await
        .unwrap();
    assert_eq!(text, "hello-util");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtil.get(String, Map)`
#[tokio::test]
async fn http_util_get_with_form_appends_query() {
    let (base, task) = serve_fn(|req| async move {
        assert!(request_path(&req).contains("a=1"));
        http_ok("q")
    })
    .await;
    let form = form_map(&[("a", "1")]);
    let text = HttpRequest::get(&base)
        .url_policy(AllowAllUrls)
        .form(form)
        .timeout(5_000)
        .execute_body()
        .await
        .unwrap();
    assert_eq!(text, "q");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtil.post(String, String body)`
#[tokio::test]
async fn http_util_post_body_json() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.starts_with("POST "));
        assert!(req.contains("application/json"));
        assert!(req.contains(r#"{"ok":true}"#));
        http_ok("posted")
    })
    .await;
    let text = HttpUtil::create_post(&base)
        .url_policy(AllowAllUrls)
        .body(r#"{"ok":true}"#)
        .timeout(5_000)
        .execute_body()
        .await
        .unwrap();
    assert_eq!(text, "posted");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtil.post(String, Map)` form urlencoded
#[tokio::test]
async fn http_util_post_form() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("application/x-www-form-urlencoded"));
        assert!(req.contains("k=v"));
        http_ok("form-ok")
    })
    .await;
    let form = form_map(&[("k", "v")]);
    let text = HttpRequest::post(&base)
        .url_policy(AllowAllUrls)
        .form(form)
        .timeout(5_000)
        .execute_body()
        .await
        .unwrap();
    assert_eq!(text, "form-ok");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtil.downloadBytes(String)`
#[tokio::test]
async fn http_util_download_bytes() {
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        b"\x00\x01\x02",
    ))
    .await;
    let bytes = HttpRequest::get(&base)
        .url_policy(AllowAllUrls)
        .timeout(5_000)
        .execute_bytes()
        .await
        .unwrap();
    assert_eq!(bytes, vec![0, 1, 2]);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequest.basicAuth` / `HttpUtil.buildBasicAuth`
#[tokio::test]
async fn http_request_basic_auth_header() {
    let (base, task) = serve_fn(|req| async move {
        let expected = HttpUtil::build_basic_auth("u", "p");
        assert!(req.to_ascii_lowercase().contains("authorization:"));
        assert!(req.contains(&expected));
        http_ok("auth")
    })
    .await;
    let text = HttpRequest::get(&base)
        .url_policy(AllowAllUrls)
        .basic_auth("u", "p")
        .timeout(5_000)
        .execute_body()
        .await
        .unwrap();
    assert_eq!(text, "auth");
    task.await.unwrap();
}

/// 对齐 Java: method factories
#[test]
fn http_request_method_factories() {
    assert_eq!(HttpRequest::head("https://e").get_method(), &Method::HEAD);
    assert_eq!(HttpRequest::put("https://e").get_method(), &Method::PUT);
    assert_eq!(HttpRequest::patch("https://e").get_method(), &Method::PATCH);
    assert_eq!(
        HttpRequest::delete("https://e").get_method(),
        &Method::DELETE
    );
    assert_eq!(
        HttpRequest::options("https://e").get_method(),
        &Method::OPTIONS
    );
    assert_eq!(HttpRequest::trace("https://e").get_method(), &Method::TRACE);
    assert_eq!(
        HttpUtil::create_request(Method::PUT, "https://e").get_method(),
        &Method::PUT
    );
}

/// Default policy rejects loopback (SSRF guard).
#[tokio::test]
async fn http_request_denies_local_by_default() {
    let err = HttpRequest::get("http://127.0.0.1:1/")
        .timeout(50)
        .execute()
        .await
        .unwrap_err();
    assert!(matches!(err, hutool_http::HttpError::UrlPolicy(_)));
}

/// Policy override helper used by local parity tests.
#[tokio::test]
async fn http_util_get_with_policy_allows_local() {
    let (base, task) = serve_once(http_ok("policy")).await;
    let text = HttpUtil::get_with_policy(&base, Arc::new(AllowAllUrls))
        .await
        .unwrap();
    assert_eq!(text, "policy");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtil.download` / `downloadFileFromUrl` (local mock + AllowAllUrls)
#[tokio::test]
async fn http_util_download_writer_and_file_from_url() {
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[
            ("Content-Type", "application/octet-stream"),
            ("Content-Disposition", "attachment; filename=\"dl.bin\""),
        ],
        b"XYZ",
    ))
    .await;
    let response = HttpUtil::create_get(&base)
        .url_policy(AllowAllUrls)
        .timeout(5_000)
        .execute()
        .await
        .unwrap();
    let mut buf = Vec::new();
    assert_eq!(response.write_body(&mut buf).unwrap(), 3);
    assert_eq!(buf, b"XYZ");

    let dir = std::env::temp_dir().join(format!(
        "hutool-http-dl-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let path = response.write_body_for_file(&dir).unwrap();
    assert!(path.ends_with("dl.bin"));
    assert_eq!(std::fs::read(&path).unwrap(), b"XYZ");
    let _ = std::fs::remove_dir_all(&dir);
    task.await.unwrap();
}
