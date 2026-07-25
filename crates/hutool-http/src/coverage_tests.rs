//! 对齐: 测试文件
//! 来源: hutool-http/src/test/java/cn/hutool/http/
//! 中文说明: HTTP模块集成测试，覆盖主要HTTP功能的端到端测试

use super::*;
use serde::Deserialize;
use std::{
    io::{Read as _, Write as _},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{io::AsyncReadExt as _, net::TcpListener};

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Message {
    message: String,
}

#[derive(Debug, Clone, Copy)]
struct RejectAll;

impl UrlPolicy for RejectAll {
    fn validate(&self, _url: &Url) -> Result<(), UrlPolicyError> {
        Err(UrlPolicyError::DeniedTarget)
    }
}

fn response(status: &str, body: &str, extra_headers: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n{extra_headers}\r\n{body}",
        body.len()
    )
    .into_bytes()
}

async fn async_server(responses: Vec<Vec<u8>>) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        for response in responses {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = vec![0_u8; 8_192];
            let _ = socket.read(&mut request).await.unwrap();
            socket.write_all(&response).await.unwrap();
        }
    });
    (format!("http://{address}"), task)
}

async fn async_capture_server(
    response: Vec<u8>,
) -> (
    String,
    tokio::sync::oneshot::Receiver<String>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = vec![0_u8; 8_192];
        let read = socket.read(&mut request).await.unwrap();
        sender
            .send(String::from_utf8_lossy(&request[..read]).into_owned())
            .unwrap();
        socket.write_all(&response).await.unwrap();
    });
    (format!("http://{address}"), receiver, task)
}

#[cfg(feature = "blocking")]
fn blocking_server(response: Vec<u8>) -> (String, std::thread::JoinHandle<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let task = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut request = [0_u8; 8_192];
        let _ = socket.read(&mut request).unwrap();
        socket.write_all(&response).unwrap();
    });
    (format!("http://{address}"), task)
}

#[cfg(feature = "blocking")]
fn blocking_capture_server(
    response: Vec<u8>,
) -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::thread::JoinHandle<()>,
) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let task = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        let mut request = [0_u8; 8_192];
        let read = socket.read(&mut request).unwrap();
        sender
            .send(String::from_utf8_lossy(&request[..read]).into_owned())
            .unwrap();
        socket.write_all(&response).unwrap();
    });
    (format!("http://{address}"), receiver, task)
}

#[test]
fn url_policies_cover_supported_public_and_denied_targets() {
    let allow = AllowAllUrls;
    assert!(
        allow
            .validate(&Url::parse("file:///tmp/report").unwrap())
            .is_ok()
    );

    let deny = DenyLocalTargets;
    assert_eq!(
        deny.validate(&Url::parse("ftp://example.com/file").unwrap()),
        Err(UrlPolicyError::UnsupportedScheme)
    );
    assert_eq!(
        DenyLocalTargets::validate_parts("http", None),
        Err(UrlPolicyError::MissingHost)
    );
    for url in [
        "http://localhost",
        "http://0.0.0.0",
        "http://169.254.1.1",
        "http://224.0.0.1",
        "http://255.255.255.255",
        "http://192.0.2.1",
        "http://[::1]",
        "http://[::]",
        "http://[ff02::1]",
        "http://[fc00::1]",
        "http://[fe80::1]",
    ] {
        assert_eq!(
            deny.validate(&Url::parse(url).unwrap()),
            Err(UrlPolicyError::DeniedTarget),
            "{url}"
        );
    }
    assert!(
        deny.validate(&Url::parse("https://8.8.8.8/dns-query").unwrap())
            .is_ok()
    );
    assert!(
        deny.validate(&Url::parse("https://[2001:4860:4860::8888]/").unwrap())
            .is_ok()
    );
}

#[test]
fn retry_policy_validation_delay_and_helpers_are_complete() {
    assert!(matches!(
        RetryPolicy::new(0),
        Err(RetryPolicyError::ZeroAttempts)
    ));
    let policy = RetryPolicy::new(3)
        .unwrap()
        .base_delay(Duration::ZERO)
        .max_delay(Duration::from_secs(2));
    assert_eq!(policy.delay(1, None), Duration::ZERO);
    assert_eq!(
        policy.delay(1, Some(Duration::from_secs(9))),
        Duration::from_secs(2)
    );
    assert!(is_idempotent(&Method::GET));
    assert!(is_idempotent(&Method::HEAD));
    assert!(is_idempotent(&Method::PUT));
    assert!(is_idempotent(&Method::DELETE));
    assert!(is_idempotent(&Method::OPTIONS));
    assert!(is_idempotent(&Method::TRACE));
    assert!(!is_idempotent(&Method::PATCH));
    for status in [408, 429, 500, 502, 503, 504] {
        assert!(is_retryable_status(StatusCode::from_u16(status).unwrap()));
    }
    assert!(!is_retryable_status(StatusCode::BAD_REQUEST));
}

#[test]
fn builders_debug_proxy_and_configuration_paths_are_complete() {
    let config = HttpConfig::default();
    let direct = HttpClient::builder().max_response_size(42).build().unwrap();
    assert!(format!("{direct:?}").contains("max_response_bytes: 42"));
    let built = HttpClientBuilder::from_config(config)
        .connect_timeout(Duration::from_millis(50))
        .timeout(Duration::from_secs(2))
        .max_response_size(64)
        .redirect_limit(1)
        .user_agent("HiTool-Coverage")
        .proxy("http://127.0.0.1:8080")
        .unwrap()
        .url_policy(RejectAll)
        .build()
        .unwrap();
    assert_eq!(built.max_response_bytes, 64);
    assert!(HttpClient::builder().proxy("not a proxy").is_err());
    assert!(
        HttpClient::builder()
            .user_agent("bad\nagent")
            .build()
            .is_err()
    );
    assert!(format!("{built:?}").contains("dyn UrlPolicy"));
}

#[tokio::test]
async fn buffered_text_json_send_and_policy_paths_are_complete() {
    let responses = vec![
        response("200 OK", "hello", ""),
        response(
            "200 OK",
            r#"{"message":"ok"}"#,
            "Content-Type: application/json\r\n",
        ),
        response(
            "200 OK",
            r#"{"message":"sent"}"#,
            "Content-Type: application/json\r\n",
        ),
        response("200 OK", "not-json", ""),
        response("404 Not Found", "missing", ""),
    ];
    let (url, server) = async_server(responses).await;
    let client = HttpClient::builder().build().unwrap();
    assert_eq!(client.get_text(&url).await.unwrap(), "hello");
    assert_eq!(
        client.get_json::<Message>(&url).await.unwrap(),
        Message {
            message: "ok".into()
        }
    );
    assert_eq!(
        client
            .send_json::<Message>(client.request(Method::GET, &url))
            .await
            .unwrap()
            .message,
        "sent"
    );
    assert!(matches!(
        client.get_json::<Message>(&url).await,
        Err(HttpError::Json(_))
    ));
    assert!(matches!(
        client.send(client.request(Method::GET, &url)).await,
        Err(HttpError::Request(_))
    ));
    server.await.unwrap();

    let rejecting = HttpClient::builder().url_policy(RejectAll).build().unwrap();
    assert!(matches!(
        rejecting
            .send(rejecting.request(Method::GET, "https://example.com"))
            .await,
        Err(HttpError::UrlPolicy(UrlPolicyError::DeniedTarget))
    ));
    assert!(matches!(
        client
            .send(client.request(Method::GET, "::bad-url::"))
            .await,
        Err(HttpError::Request(_))
    ));
}

#[tokio::test]
async fn response_facade_preserves_non_success_status_and_bounded_body() {
    let (url, server) = async_server(vec![
        response(
            "404 Not Found",
            "missing",
            "Content-Type: text/plain; charset=utf-8\r\nSet-Cookie: sid=one\r\n",
        ),
        response("201 Created", "created", "X-Origin: runtime\r\n"),
        response("200 OK", "too-large", ""),
    ])
    .await;
    let client = HttpClient::builder().max_response_size(8).build().unwrap();

    let missing = client.get_response(&url).await.unwrap();
    assert_eq!(missing.get_status(), 404);
    assert!(!missing.is_ok());
    assert_eq!(missing.body(), "missing");
    assert_eq!(missing.get_cookie_str(), Some("sid=one"));

    let created = client
        .send_response(client.request(Method::POST, &url).body("request"))
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::CREATED);
    assert_eq!(created.header("x-origin"), Some("runtime"));
    assert_eq!(created.body_bytes(), b"created");

    assert!(matches!(
        client.get_response(&url).await,
        Err(HttpError::ResponseTooLarge {
            limit: 8,
            actual: 9
        })
    ));
    server.await.unwrap();

    assert!(matches!(
        client
            .send_response(client.request(Method::GET, "::bad-url::"))
            .await,
        Err(HttpError::Request(_))
    ));
}

#[tokio::test]
async fn checked_buffering_internal_path_covers_every_exit() {
    let (url, server) = async_server(vec![
        response("200 OK", "ok", ""),
        response("404 Not Found", "missing", ""),
        response("200 OK", "too-large", ""),
    ])
    .await;
    let client = HttpClient::builder().max_response_size(8).build().unwrap();
    assert_eq!(
        client
            .send_response_checked(client.request(Method::GET, &url))
            .await
            .unwrap()
            .body(),
        "ok"
    );
    assert!(matches!(
        client
            .send_response_checked(client.request(Method::GET, &url))
            .await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client
            .send_response_checked(client.request(Method::GET, &url))
            .await,
        Err(HttpError::ResponseTooLarge { .. })
    ));
    server.await.unwrap();
    assert!(matches!(
        client
            .send_response_checked(client.request(Method::GET, "::bad-url::"))
            .await,
        Err(HttpError::Request(_))
    ));
}

#[tokio::test]
async fn runtime_config_changes_real_requests_responses_and_errors() {
    let (url, captured, server) =
        async_capture_server(response("200 OK", "configured", "X-Origin: server\r\n")).await;
    let changed_url = Url::parse(&format!("{url}/changed")).unwrap();
    let mut config = HttpConfig::default();
    config.disable_cache();
    config
        .add_request_interceptor(move |context| {
            context
                .set_method(Method::POST)
                .set_url(changed_url.clone());
            context.headers_mut().insert(
                "x-hutool-request",
                header::HeaderValue::from_static("intercepted"),
            );
            Ok(())
        })
        .add_response_interceptor(|context| {
            assert_eq!(context.status(), StatusCode::OK);
            assert_eq!(context.headers()["x-origin"], "server");
            context.headers_mut().insert(
                "x-hutool-response",
                header::HeaderValue::from_static("intercepted"),
            );
            Ok(())
        });
    let client = HttpClient::new(&config).unwrap();
    let actual_response = client
        .send(client.request(Method::GET, format!("{url}/original")))
        .await
        .unwrap();
    assert_eq!(
        actual_response.headers()["x-hutool-response"],
        "intercepted"
    );
    let request = captured.await.unwrap().to_ascii_lowercase();
    assert!(request.starts_with("post /changed http/1.1"));
    assert!(request.contains("x-hutool-request: intercepted"));
    assert!(request.contains("cache-control: no-cache, no-store"));
    assert!(request.contains("pragma: no-cache"));
    server.await.unwrap();

    let mut rejecting_request = HttpConfig::default();
    rejecting_request.add_request_interceptor(|_| Err(HttpInterceptorError::new("request denied")));
    let client = HttpClient::new(&rejecting_request).unwrap();
    assert!(matches!(
        client
            .send(client.request(Method::GET, "https://example.com"))
            .await,
        Err(HttpError::Interceptor(error)) if error.message() == "request denied"
    ));

    let (url, server) = async_server(vec![response("200 OK", "denied", "")]).await;
    let mut rejecting_response = HttpConfig::default();
    rejecting_response
        .add_response_interceptor(|_| Err(HttpInterceptorError::new("response denied")));
    let client = HttpClient::new(&rejecting_response).unwrap();
    assert!(matches!(
        client.send(client.request(Method::GET, url)).await,
        Err(HttpError::Interceptor(error)) if error.message() == "response denied"
    ));
    server.await.unwrap();
}

#[tokio::test]
async fn configured_proxy_is_the_real_transport_destination() {
    let (proxy_url, captured, server) =
        async_capture_server(response("200 OK", "proxied", "")).await;
    let client = HttpClient::builder()
        .proxy(&proxy_url)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        client
            .get_text("http://example.com/through-proxy")
            .await
            .unwrap(),
        "proxied"
    );
    let request = captured.await.unwrap();
    assert!(request.starts_with("GET http://example.com/through-proxy HTTP/1.1"));
    server.await.unwrap();
}

#[tokio::test]
async fn retry_path_propagates_response_interceptor_errors_for_every_status_family() {
    let mut rejecting_request = HttpConfig::default();
    rejecting_request
        .add_request_interceptor(|_| Err(HttpInterceptorError::new("retry request denied")));
    let client = HttpClient::new(&rejecting_request).unwrap();
    assert!(matches!(
        client
            .send_idempotent(
                client.request(Method::GET, "https://example.com"),
                RetryPolicy::new(1).unwrap(),
            )
            .await,
        Err(HttpError::Interceptor(error)) if error.message() == "retry request denied"
    ));

    for status in ["200 OK", "500 Internal Server Error"] {
        let (url, server) = async_server(vec![response(status, "intercepted", "")]).await;
        let mut config = HttpConfig::default();
        config
            .add_response_interceptor(|_| Err(HttpInterceptorError::new("retry response denied")));
        let client = HttpClient::new(&config).unwrap();
        assert!(matches!(
            client
                .send_idempotent(
                    client.request(Method::GET, url),
                    RetryPolicy::new(1).unwrap(),
                )
                .await,
            Err(HttpError::Interceptor(error)) if error.message() == "retry response denied"
        ));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn buffered_helpers_propagate_status_transport_body_size_and_json_errors() {
    let incomplete =
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\nhi".to_vec();
    let (url, server) = async_server(vec![
        response("404 Not Found", "missing", ""),
        incomplete.clone(),
        response("200 OK", "too-large", ""),
        response("404 Not Found", "missing", ""),
        incomplete,
        response("200 OK", r#"{"message":"too-large"}"#, ""),
        response("200 OK", "not-json", ""),
    ])
    .await;
    let client = HttpClient::builder().max_response_size(8).build().unwrap();
    assert!(matches!(
        client.get_text(&url).await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client.get_text(&url).await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client.get_text(&url).await,
        Err(HttpError::ResponseTooLarge { .. })
    ));
    assert!(matches!(
        client.get_json::<Message>(&url).await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client
            .send_json::<Message>(client.request(Method::GET, &url))
            .await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client.get_json::<Message>(&url).await,
        Err(HttpError::ResponseTooLarge { .. })
    ));
    assert!(matches!(
        client.get_json::<Message>(&url).await,
        Err(HttpError::Json(_))
    ));
    server.await.unwrap();

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let closed = listener.local_addr().unwrap();
    drop(listener);
    assert!(matches!(
        client
            .send(client.request(Method::GET, format!("http://{closed}")))
            .await,
        Err(HttpError::Request(_))
    ));
}

#[tokio::test]
async fn retry_status_transport_exhaustion_and_stream_body_paths_are_complete() {
    let (url, server) = async_server(vec![
        response("503 Service Unavailable", "", "Retry-After: 0\r\n"),
        response("200 OK", "ok", ""),
        response("400 Bad Request", "bad", ""),
        response("503 Service Unavailable", "", ""),
        response("503 Service Unavailable", "", ""),
    ])
    .await;
    let client = HttpClient::builder().build().unwrap();
    assert!(matches!(
        client
            .send_idempotent(
                client.request(Method::POST, "https://example.com"),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::NonIdempotentRetry(Method::POST))
    ));
    let policy = RetryPolicy::new(2)
        .unwrap()
        .base_delay(Duration::ZERO)
        .max_delay(Duration::ZERO);
    assert_eq!(
        client
            .send_idempotent(client.request(Method::GET, &url), policy)
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );
    assert!(matches!(
        client
            .send_idempotent(client.request(Method::GET, &url), policy)
            .await,
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client
            .send_idempotent(client.request(Method::GET, &url), policy)
            .await,
        Err(HttpError::RetriesExhausted { attempts: 2, .. })
    ));
    server.await.unwrap();

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let closed = listener.local_addr().unwrap();
    drop(listener);
    assert!(matches!(
        client
            .send_idempotent(
                client.request(Method::GET, format!("http://{closed}")),
                RetryPolicy::new(2)
                    .unwrap()
                    .base_delay(Duration::ZERO)
                    .max_delay(Duration::ZERO)
            )
            .await,
        Err(HttpError::RetriesExhausted { attempts: 2, .. })
    ));
    assert!(matches!(
        client
            .send_idempotent(
                client.request(Method::GET, format!("http://{closed}")),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::Request(_))
    ));

    let body = reqwest::Body::wrap_stream(async_stream::stream! {
        yield Ok::<_, std::io::Error>(b"stream".as_slice());
    });
    assert!(matches!(
        client
            .send_idempotent(
                client
                    .request(Method::PUT, "https://example.com")
                    .body(body),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::UncloneableRetryRequest)
    ));
}

#[tokio::test]
async fn retry_rejects_build_policy_timeout_and_invalid_retry_after_inputs() {
    let client = HttpClient::builder().build().unwrap();
    assert!(matches!(
        client
            .send_idempotent(
                client.request(Method::GET, "::bad-url::"),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::Request(_))
    ));
    let rejecting = HttpClient::builder().url_policy(RejectAll).build().unwrap();
    assert!(matches!(
        rejecting
            .send_idempotent(
                rejecting.request(Method::GET, "https://example.com"),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::UrlPolicy(_))
    ));

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let timeout_server = tokio::spawn(async move {
        let (_socket, _) = listener.accept().await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
    });
    let impatient = HttpClient::builder()
        .timeout(Duration::from_millis(10))
        .build()
        .unwrap();
    assert!(matches!(
        impatient
            .send_idempotent(
                impatient.request(Method::GET, format!("http://{address}")),
                RetryPolicy::new(1).unwrap()
            )
            .await,
        Err(HttpError::Request(_))
    ));
    timeout_server.await.unwrap();

    let invalid_utf8 = b"HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nRetry-After: \xff\r\nConnection: close\r\n\r\n".to_vec();
    let (url, server) = async_server(vec![
        response("503 Service Unavailable", "", "Retry-After: soon\r\n"),
        invalid_utf8,
    ])
    .await;
    for _ in 0..2 {
        assert!(matches!(
            client
                .send_idempotent(
                    client.request(Method::GET, &url),
                    RetryPolicy::new(1).unwrap()
                )
                .await,
            Err(HttpError::Request(_))
        ));
    }
    server.await.unwrap();
}

#[derive(Default)]
struct AsyncSink {
    bytes: Vec<u8>,
    fail_write: bool,
    fail_flush: bool,
}

impl AsyncWrite for AsyncSink {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        if self.fail_write {
            return Poll::Ready(Err(std::io::Error::other("write failed")));
        }
        self.bytes.extend_from_slice(buffer);
        Poll::Ready(Ok(buffer.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        if self.fail_flush {
            Poll::Ready(Err(std::io::Error::other("flush failed")))
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        _context: &mut Context<'_>,
    ) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

#[tokio::test]
async fn asynchronous_download_covers_success_size_write_and_flush_failures() {
    let (url, server) = async_server(vec![
        response("200 OK", "download", ""),
        response("200 OK", "large", ""),
        response("200 OK", "write", ""),
        response("200 OK", "flush", ""),
    ])
    .await;
    let client = HttpClient::builder().max_response_size(16).build().unwrap();
    let mut success = AsyncSink::default();
    assert_eq!(
        client
            .download_to(client.request(Method::GET, &url), &mut success)
            .await
            .unwrap(),
        8
    );
    assert_eq!(success.bytes, b"download");

    let limited = HttpClient::builder().max_response_size(4).build().unwrap();
    assert!(matches!(
        limited
            .download_to(
                limited.request(Method::GET, &url),
                &mut AsyncSink::default()
            )
            .await,
        Err(HttpError::ResponseTooLarge {
            limit: 4,
            actual: 5
        })
    ));
    assert!(matches!(
        client
            .download_to(
                client.request(Method::GET, &url),
                &mut AsyncSink {
                    fail_write: true,
                    ..AsyncSink::default()
                }
            )
            .await,
        Err(HttpError::Io(_))
    ));
    assert!(matches!(
        client
            .download_to(
                client.request(Method::GET, &url),
                &mut AsyncSink {
                    fail_flush: true,
                    ..AsyncSink::default()
                }
            )
            .await,
        Err(HttpError::Io(_))
    ));
    server.await.unwrap();
}

#[tokio::test]
async fn asynchronous_download_propagates_send_and_chunk_failures() {
    let rejecting = HttpClient::builder().url_policy(RejectAll).build().unwrap();
    assert!(matches!(
        rejecting
            .download_to(
                rejecting.request(Method::GET, "https://example.com"),
                &mut AsyncSink::default()
            )
            .await,
        Err(HttpError::UrlPolicy(_))
    ));

    let incomplete =
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\nhi".to_vec();
    let (url, server) = async_server(vec![incomplete]).await;
    let client = HttpClient::builder().build().unwrap();
    assert!(matches!(
        client
            .download_to(client.request(Method::GET, &url), &mut AsyncSink::default())
            .await,
        Err(HttpError::Request(_))
    ));
    server.await.unwrap();
}

#[cfg(feature = "blocking")]
#[derive(Default)]
struct BlockingSink {
    bytes: Vec<u8>,
    fail_write: bool,
    fail_flush: bool,
}

#[cfg(feature = "blocking")]
impl std::io::Write for BlockingSink {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        if self.fail_write {
            return Err(std::io::Error::other("write failed"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.fail_flush {
            Err(std::io::Error::other("flush failed"))
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_runtime_config_changes_real_requests_responses_and_errors() {
    use super::blocking;

    let (url, captured, server) =
        blocking_capture_server(response("200 OK", "configured", "X-Origin: server\r\n"));
    let changed_url = Url::parse(&format!("{url}/changed")).unwrap();
    let mut config = HttpConfig::default();
    config.disable_cache();
    config
        .add_request_interceptor(move |context| {
            context.set_method(Method::PUT).set_url(changed_url.clone());
            context.headers_mut().insert(
                "x-hutool-request",
                header::HeaderValue::from_static("blocking"),
            );
            Ok(())
        })
        .add_response_interceptor(|context| {
            assert_eq!(context.status(), StatusCode::OK);
            context.headers_mut().insert(
                "x-hutool-response",
                header::HeaderValue::from_static("blocking"),
            );
            Ok(())
        });
    let client = blocking::HttpClient::new(&config).unwrap();
    let requests = reqwest::blocking::Client::new();
    let actual_response = client.send(requests.get(&url)).unwrap();
    assert_eq!(actual_response.headers()["x-hutool-response"], "blocking");
    let request = captured.recv().unwrap().to_ascii_lowercase();
    assert!(request.starts_with("put /changed http/1.1"));
    assert!(request.contains("x-hutool-request: blocking"));
    assert!(request.contains("cache-control: no-cache, no-store"));
    server.join().unwrap();

    let mut rejecting_request = HttpConfig::default();
    rejecting_request.add_request_interceptor(|_| Err(HttpInterceptorError::new("request denied")));
    let client = blocking::HttpClient::new(&rejecting_request).unwrap();
    assert!(matches!(
        client.send(requests.get("https://example.com")),
        Err(HttpError::Interceptor(error)) if error.message() == "request denied"
    ));

    let (url, server) = blocking_server(response("200 OK", "denied", ""));
    let mut rejecting_response = HttpConfig::default();
    rejecting_response
        .add_response_interceptor(|_| Err(HttpInterceptorError::new("response denied")));
    let client = blocking::HttpClient::new(&rejecting_response).unwrap();
    assert!(matches!(
        client.send(requests.get(url)),
        Err(HttpError::Interceptor(error)) if error.message() == "response denied"
    ));
    server.join().unwrap();
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_response_facade_preserves_error_responses_and_enforces_limit() {
    use super::blocking;

    let (url, server) = blocking_server(response(
        "418 I'm a teapot",
        "teapot",
        "Content-Type: text/plain\r\n",
    ));
    let client = blocking::HttpClient::new(&HttpConfig {
        max_response_bytes: 8,
        ..HttpConfig::default()
    })
    .unwrap();
    let actual = client.get_response(&url).unwrap();
    assert_eq!(actual.get_status(), 418);
    assert!(!actual.is_ok());
    assert_eq!(actual.body(), "teapot");
    server.join().unwrap();

    let (url, server) = blocking_server(response("200 OK", "too-large", ""));
    assert!(matches!(
        client.get_response(url),
        Err(HttpError::ResponseTooLarge {
            limit: 8,
            actual: 9
        })
    ));
    server.join().unwrap();
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_client_covers_text_json_send_policy_and_download_paths() {
    use super::blocking;

    let config = HttpConfig {
        max_response_bytes: 16,
        ..HttpConfig::default()
    };
    let client = blocking::HttpClient::new(&config).unwrap();
    assert!(format!("{client:?}").contains("max_response_bytes: 16"));

    let (url, server) = blocking_server(response("200 OK", "hello", ""));
    assert_eq!(client.get_text(&url).unwrap(), "hello");
    server.join().unwrap();

    let (url, server) = blocking_server(response(
        "200 OK",
        r#"{"message":"ok"}"#,
        "Content-Type: application/json\r\n",
    ));
    assert_eq!(client.get_json::<Message>(&url).unwrap().message, "ok");
    server.join().unwrap();

    let rejecting = blocking::HttpClient::new_with_policy(&config, RejectAll).unwrap();
    let requests = reqwest::blocking::Client::new();
    assert!(matches!(
        rejecting.send(requests.get("https://example.com")),
        Err(HttpError::UrlPolicy(_))
    ));

    let (url, server) = blocking_server(response("404 Not Found", "missing", ""));
    assert!(matches!(
        client.send(requests.get(&url)),
        Err(HttpError::Request(_))
    ));
    server.join().unwrap();

    let (url, server) = blocking_server(response("200 OK", "download", ""));
    let mut output = BlockingSink::default();
    assert_eq!(
        client.download_to(requests.get(&url), &mut output).unwrap(),
        8
    );
    assert_eq!(output.bytes, b"download");
    server.join().unwrap();

    let limited_config = HttpConfig {
        max_response_bytes: 4,
        ..HttpConfig::default()
    };
    let limited = blocking::HttpClient::new(&limited_config).unwrap();
    let (url, server) = blocking_server(response("200 OK", "large", ""));
    assert!(matches!(
        limited.download_to(requests.get(&url), &mut BlockingSink::default()),
        Err(HttpError::ResponseTooLarge {
            limit: 4,
            actual: 5
        })
    ));
    server.join().unwrap();
}

#[cfg(feature = "blocking")]
#[test]
#[allow(clippy::too_many_lines)]
fn blocking_client_propagates_every_builder_body_and_writer_failure() {
    use super::blocking;

    let invalid = HttpConfig {
        user_agent: "bad\nagent".into(),
        ..HttpConfig::default()
    };
    assert!(blocking::HttpClient::new(&invalid).is_err());

    let config = HttpConfig {
        max_response_bytes: 8,
        ..HttpConfig::default()
    };
    let client = blocking::HttpClient::new(&config).unwrap();
    let requests = reqwest::blocking::Client::new();
    assert!(matches!(
        client.send(requests.get("::bad-url::")),
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client.send_response(requests.get("::bad-url::")),
        Err(HttpError::Request(_))
    ));
    assert!(matches!(
        client.get_text("::bad-url::"),
        Err(HttpError::Request(_))
    ));

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let closed = listener.local_addr().unwrap();
    drop(listener);
    assert!(matches!(
        client.send(requests.get(format!("http://{closed}"))),
        Err(HttpError::Request(_))
    ));

    let incomplete =
        b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\nhi".to_vec();
    let (url, server) = blocking_server(incomplete.clone());
    assert!(matches!(client.get_text(&url), Err(HttpError::Io(_))));
    server.join().unwrap();
    let (url, server) = blocking_server(incomplete.clone());
    assert!(matches!(
        client.get_json::<Message>(&url),
        Err(HttpError::Io(_))
    ));
    server.join().unwrap();

    let (url, server) = blocking_server(response("404 Not Found", "missing", ""));
    assert!(matches!(client.get_text(&url), Err(HttpError::Request(_))));
    server.join().unwrap();
    let (url, server) = blocking_server(response("404 Not Found", "missing", ""));
    assert!(matches!(
        client.get_json::<Message>(&url),
        Err(HttpError::Request(_))
    ));
    server.join().unwrap();

    let (url, server) = blocking_server(response("200 OK", "too-large", ""));
    assert!(matches!(
        client.get_text(&url),
        Err(HttpError::ResponseTooLarge { .. })
    ));
    server.join().unwrap();
    let (url, server) = blocking_server(response("200 OK", "too-large", ""));
    assert!(matches!(
        client.get_json::<Message>(&url),
        Err(HttpError::ResponseTooLarge { .. })
    ));
    server.join().unwrap();
    let (url, server) = blocking_server(response("200 OK", "not-json", ""));
    assert!(matches!(
        client.get_json::<Message>(&url),
        Err(HttpError::Json(_))
    ));
    server.join().unwrap();

    let rejecting = blocking::HttpClient::new_with_policy(&config, RejectAll).unwrap();
    assert!(matches!(
        rejecting.download_to(
            requests.get("https://example.com"),
            &mut BlockingSink::default()
        ),
        Err(HttpError::UrlPolicy(_))
    ));
    let (url, server) = blocking_server(incomplete);
    assert!(matches!(
        client.download_to(requests.get(&url), &mut BlockingSink::default()),
        Err(HttpError::Io(_))
    ));
    server.join().unwrap();

    for sink in [
        BlockingSink {
            fail_write: true,
            ..BlockingSink::default()
        },
        BlockingSink {
            fail_flush: true,
            ..BlockingSink::default()
        },
    ] {
        let (url, server) = blocking_server(response("200 OK", "writer", ""));
        let mut sink = sink;
        assert!(matches!(
            client.download_to(requests.get(&url), &mut sink),
            Err(HttpError::Io(_))
        ));
        server.join().unwrap();
    }
}
