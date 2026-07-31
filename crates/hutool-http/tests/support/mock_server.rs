//! Local HTTP mock helpers for Hutool network parity tests.

use std::future::Future;
use std::time::Duration;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpListener;

/// Builds a minimal HTTP/1.1 200 response body.
pub fn http_ok(body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
    .into_bytes()
}

/// Builds a response with arbitrary status, headers, and body.
pub fn http_response(status: &str, headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
    let mut out = format!("HTTP/1.1 {status}\r\n");
    for (k, v) in headers {
        out.push_str(&format!("{k}: {v}\r\n"));
    }
    out.push_str(&format!(
        "Content-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    ));
    let mut bytes = out.into_bytes();
    bytes.extend_from_slice(body);
    bytes
}

/// Serves one request with a fixed response.
pub async fn serve_once(response: Vec<u8>) -> (String, tokio::task::JoinHandle<()>) {
    serve_fn(move |_req| {
        let response = response.clone();
        async move { response }
    })
    .await
}

/// Serves one request using an async handler.
pub async fn serve_fn<F, Fut>(handler: F) -> (String, tokio::task::JoinHandle<()>)
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Vec<u8>> + Send + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        let (mut sock, _) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 64 * 1024];
        let n = sock.read(&mut buf).await.unwrap_or(0);
        let req = String::from_utf8_lossy(&buf[..n]).into_owned();
        let resp = handler(req).await;
        let _ = sock.write_all(&resp).await;
    });
    (format!("http://{addr}"), task)
}

/// Serves multiple sequential requests on one listener.
///
/// 对齐测试脚手架中连续响应场景的辅助函数，当前未被测试直接调用。
#[allow(dead_code)]
pub async fn serve_sequence(responses: Vec<Vec<u8>>) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        for response in responses {
            let (mut sock, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 64 * 1024];
            let _ = sock.read(&mut buf).await;
            let _ = sock.write_all(&response).await;
        }
    });
    (format!("http://{addr}"), task)
}

/// Returns a client configured for local mock servers.
///
/// 对齐测试脚手架中共享客户端配置的辅助函数，当前未被测试直接调用。
#[allow(dead_code)]
pub fn local_client() -> hutool_http::HttpClient {
    hutool_http::HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .redirect_limit(10)
        .cookie_store(true)
        .build()
        .unwrap()
}

/// Reads the request path from a raw HTTP request.
pub fn request_path(req: &str) -> &str {
    req.lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
}

/// Reads a header value from a raw HTTP request.
///
/// 对齐测试脚手架中原始请求头解析的辅助函数，当前未被测试直接调用。
#[allow(dead_code)]
pub fn request_header<'a>(req: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("{}:", name.to_ascii_lowercase());
    req.lines().find_map(|line| {
        if line.len() < needle.len() {
            return None;
        }
        if line[..needle.len()].eq_ignore_ascii_case(&needle) {
            Some(line[needle.len()..].trim())
        } else {
            None
        }
    })
}
