//! Network / Hutool HTTP parity tests using local TcpListener mocks.
//! 对齐: hutool-http HttpRequestTest / DownloadTest / RestTest / UploadTest / …

mod support;

use flate2::Compression;
use flate2::write::DeflateEncoder;
use hutool_http::{
    ContentType, HostnameVerification, HttpClient, HttpConfig, HttpUtil, Method, MultipartBody,
    form_map, header,
};
use reqwest::Url;
use std::io::Write;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use support::mock_server::*;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

#[derive(Default)]
struct AsyncBuf {
    bytes: Vec<u8>,
}

impl AsyncWrite for AsyncBuf {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        self.get_mut().bytes.extend_from_slice(buf);
        std::task::Poll::Ready(Ok(buf.len()))
    }
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::task::Poll::Ready(Ok(()))
    }
}

fn redirect(location: &str) -> Vec<u8> {
    http_response("302 Found", &[("Location", location)], b"")
}

fn deflate_body(raw: &str) -> Vec<u8> {
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::default());
    enc.write_all(raw.as_bytes()).unwrap();
    http_response(
        "200 OK",
        &[
            ("Content-Encoding", "deflate"),
            ("Content-Type", "text/plain"),
        ],
        &enc.finish().unwrap(),
    )
}

fn urlencoding_form(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

async fn serve_many(count: usize, response: Vec<u8>) -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let task = tokio::spawn(async move {
        for _ in 0..count {
            let (mut sock, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 64 * 1024];
            let _ = sock.read(&mut buf).await;
            let _ = sock.write_all(&response).await;
        }
    });
    (format!("http://{addr}"), task)
}

async fn get_text(url: &str) -> String {
    local_client().get_text(url).await.unwrap()
}

async fn get_resp(url: &str) -> hutool_http::HttpResponse {
    local_client().get_response(url).await.unwrap()
}

async fn download(url: &str) -> Vec<u8> {
    let mut buf = AsyncBuf::default();
    let c = local_client();
    c.download_to(c.request(Method::GET, url), &mut buf)
        .await
        .unwrap();
    buf.bytes
}

fn soap_response(body: &str) -> Vec<u8> {
    let xml = format!(
        "<?xml version=\"1.0\"?><soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\"><soap:Body>{body}</soap:Body></soap:Envelope>"
    );
    http_response(
        "200 OK",
        &[("Content-Type", "text/xml; charset=utf-8")],
        xml.as_bytes(),
    )
}

async fn soap_post(base: &str) -> hutool_http::HttpResponse {
    let body = r#"<?xml version="1.0"?><soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:web="http://WebXml.com.cn/"><soap:Body><web:getCountryCityByIp><web:theIpAddress>218.21.240.106</web:theIpAddress></web:getCountryCityByIp></soap:Body></soap:Envelope>"#;
    let c = local_client();
    c.send_response(
        c.request(Method::POST, base)
            .header(header::CONTENT_TYPE, "text/xml; charset=GBK")
            .body(body),
    )
    .await
    .unwrap()
}

// ── HttpRequestTest (8 existing) ───────────────────────────────────────────

/// 对齐 Java: `HttpRequestTest.getLocalTest()`
#[tokio::test]
async fn get_local_test() {
    let (base, task) = serve_once(http_ok("local-ok")).await;
    let client = HttpClient::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap();
    assert_eq!(client.get_text(&base).await.unwrap(), "local-ok");
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.syncGetTest()`
#[tokio::test]
async fn sync_get_test() {
    let (base, task) = serve_once(http_ok("sync")).await;
    let resp = HttpClient::new(&HttpConfig::default())
        .unwrap()
        .get_response(&base)
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.body(), "sync");
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.asyncGetTest()`
#[tokio::test]
async fn async_get_test() {
    let (base, task) = serve_once(http_ok("async-get")).await;
    assert_eq!(
        HttpClient::builder()
            .build()
            .unwrap()
            .get_text(&base)
            .await
            .unwrap(),
        "async-get"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.asyncHeadTest()`
#[tokio::test]
async fn async_head_test() {
    let (base, task) = serve_once(http_ok("")).await;
    let c = HttpClient::builder().build().unwrap();
    assert_eq!(
        c.send_response(c.request(Method::HEAD, &base))
            .await
            .unwrap()
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.testHttpHead()`
#[tokio::test]
async fn test_http_head() {
    let (base, task) = serve_once(http_ok("")).await;
    let c = HttpClient::builder().build().unwrap();
    assert!(
        c.send_response(c.request(Method::HEAD, &base))
            .await
            .unwrap()
            .status()
            .is_success()
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.bodyTest()`
#[tokio::test]
async fn body_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("hello-body"));
        http_ok("posted")
    })
    .await;
    let c = HttpClient::builder().build().unwrap();
    assert_eq!(
        c.send_response(c.request(Method::POST, &base).body("hello-body"))
            .await
            .unwrap()
            .body(),
        "posted"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.customGetTest()`
#[tokio::test]
async fn custom_get_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.to_ascii_lowercase().contains("user-agent"));
        http_ok("custom")
    })
    .await;
    let c = HttpClient::builder()
        .user_agent("hutool-http-test")
        .build()
        .unwrap();
    assert_eq!(c.get_text(&base).await.unwrap(), "custom");
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.httpParameterDecodeTest()`
#[tokio::test]
async fn http_parameter_decode_test() {
    let sample = "this is test测试";
    let (addr, task) = serve_many(2, http_ok(sample)).await;
    let c = HttpClient::builder().build().unwrap();
    let form = c
        .send_response(
            c.request(Method::POST, format!("{addr}/form"))
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(format!("test={}", urlencoding_form(sample))),
        )
        .await
        .unwrap();
    assert_eq!(form.body(), sample);
    assert_eq!(
        c.get_text(format!("{addr}/get?test={}", urlencoding_form(sample)))
            .await
            .unwrap(),
        sample
    );
    task.await.unwrap();
}

// ── DownloadTest ───────────────────────────────────────────────────────────

/// 对齐 Java: `DownloadTest.downloadPicTest()`
#[tokio::test]
async fn download_test_download_pic_test() {
    let data = b"\x89PNG\r\n\x1a\nmock";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadSizeTest()`
#[tokio::test]
async fn download_test_download_size_test() {
    let data = b"download-size";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadTest1()`
#[tokio::test]
async fn download_test_download_test1() {
    let data = b"<crossdomain/>";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadTest()`
#[tokio::test]
async fn download_test_download_test() {
    let data = b"centos-chunk";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadFileFromUrlTest1()`
#[tokio::test]
async fn download_test_download_file_from_url_test1() {
    let (base, task) = serve_once(http_ok("<html>changelog</html>")).await;
    assert!(!download(&base).await.is_empty());
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadFileFromUrlTest2()`
#[tokio::test]
async fn download_test_download_file_from_url_test2() {
    let data = b"PK\x03\x04jar";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadFileFromUrlTest3()`
#[tokio::test]
async fn download_test_download_file_from_url_test3() {
    let data = b"jar-progress";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadFileFromUrlTest4()`
#[tokio::test]
async fn download_test_download_file_from_url_test4() {
    let data = b"html-dl";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadFileFromUrlTest5()`
#[tokio::test]
async fn download_test_download_file_from_url_test5() {
    let (base, task) = serve_sequence(vec![http_ok("f1"), http_ok("f2")]).await;
    for path in ["/a", "/b"] {
        assert!(!download(&format!("{base}{path}")).await.is_empty());
    }
    task.await.unwrap();
}

/// 对齐 Java: `DownloadTest.downloadTeamViewerTest()`
#[tokio::test]
async fn download_test_download_team_viewer_test() {
    let (base, task) = serve_sequence(vec![
        redirect("/h1"),
        redirect("/h2"),
        redirect("/h3"),
        http_ok("exe"),
    ])
    .await;
    let c = HttpClient::builder().redirect_limit(20).build().unwrap();
    assert_eq!(c.get_text(&base).await.unwrap(), "exe");
    task.await.unwrap();
}

// ── HttpRequestTest (network) ──────────────────────────────────────────────

/// 对齐 Java: `HttpRequestTest.getHttpsTest()`
#[tokio::test]
async fn http_request_test_get_https_test() {
    let body = "https-mock";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.getHttpsThenTest()`
#[tokio::test]
async fn http_request_test_get_https_then_test() {
    let body = "then-body";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.getCookiesTest()`
#[tokio::test]
async fn http_request_test_get_cookies_test() {
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Set-Cookie", "session=abc; Path=/")],
        b"cookie-body",
    ))
    .await;
    let resp = get_resp(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.get_cookie_str(), Some("session=abc; Path=/"));
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.toStringTest()`
#[tokio::test]
async fn http_request_test_to_string_test() {
    let url = format!("http://127.0.0.1/?ccc={}", urlencoding_form("你好"));
    let built = local_client()
        .request(Method::GET, &url)
        .query(&[("a", "测试")])
        .build()
        .unwrap();
    let dbg = format!("{built:?}");
    assert!(dbg.contains("127.0.0.1") && dbg.contains("a"));
}

/// 对齐 Java: `HttpRequestTest.getDeflateTest()`
#[tokio::test]
async fn http_request_test_get_deflate_test() {
    let (base, task) = serve_once(deflate_body("bilibili-xml")).await;
    let resp = get_resp(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.body(), "bilibili-xml");
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.getWithoutEncodeTest()`
#[tokio::test]
async fn http_request_test_get_without_encode_test() {
    let png = b"\x89PNG";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "image/png")],
        png,
    ))
    .await;
    assert_eq!(get_resp(&format!("{base}?pid=1")).await.body_bytes(), png);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.followRedirectsTest()`
#[tokio::test]
async fn http_request_test_follow_redirects_test() {
    let chain = vec![redirect("/b"), redirect("/c"), http_ok("done")];
    let (base, task) = serve_sequence(chain).await;
    let limited = HttpClient::builder().redirect_limit(1).build().unwrap();
    assert_eq!(
        limited.get_response(&base).await.unwrap().status().as_u16(),
        302
    );
    assert_eq!(
        HttpClient::builder()
            .redirect_limit(10)
            .build()
            .unwrap()
            .get_text(&base)
            .await
            .unwrap(),
        "done"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.followRedirectsCookieTrueTest()`
#[tokio::test]
async fn http_request_test_follow_redirects_cookie_true_test() {
    let (base, task) = serve_sequence(vec![
        http_response(
            "302 Found",
            &[("Location", "/d"), ("Set-Cookie", "token=abc; Path=/")],
            b"",
        ),
        http_ok("cookie-true"),
    ])
    .await;
    let resp = HttpClient::builder()
        .redirect_limit(20)
        .cookie_store(true)
        .build()
        .unwrap()
        .get_response(&base)
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.body(), "cookie-true");
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.followRedirectsCookieFalseTest()`
#[tokio::test]
async fn http_request_test_follow_redirects_cookie_false_test() {
    let (base, task) = serve_sequence(vec![
        http_response(
            "302 Found",
            &[("Location", "/d"), ("Set-Cookie", "token=abc; Path=/")],
            b"",
        ),
        http_ok("cookie-false"),
    ])
    .await;
    let resp = HttpClient::builder()
        .redirect_limit(20)
        .cookie_store(false)
        .build()
        .unwrap()
        .get_response(&base)
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.addInterceptorTest()`
#[tokio::test]
async fn http_request_test_add_interceptor_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("x-hutool-test: yes"));
        http_ok("intercepted")
    })
    .await;
    let mut cfg = HttpConfig::default();
    cfg.add_request_interceptor(|ctx| {
        ctx.headers_mut()
            .insert("x-hutool-test", header::HeaderValue::from_static("yes"));
        Ok(())
    });
    assert_eq!(
        HttpClient::new(&cfg)
            .unwrap()
            .get_text(&base)
            .await
            .unwrap(),
        "intercepted"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.addGlobalInterceptorTest()`
#[tokio::test]
async fn http_request_test_add_global_interceptor_test() {
    let (base, task) = serve_once(http_ok("global")).await;
    let n = Arc::new(AtomicUsize::new(0));
    let c = Arc::clone(&n);
    let mut cfg = HttpConfig::default();
    cfg.add_request_interceptor(move |_| {
        c.fetch_add(1, Ordering::SeqCst);
        Ok(())
    });
    assert_eq!(
        HttpClient::new(&cfg)
            .unwrap()
            .get_text(&base)
            .await
            .unwrap(),
        "global"
    );
    assert_eq!(n.load(Ordering::SeqCst), 1);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.getWithFormTest()`
#[tokio::test]
async fn http_request_test_get_with_form_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("aaa="));
        http_ok("form-get")
    })
    .await;
    let url = HttpUtil::url_with_form(&base, &form_map(&[("aaa", "application+1@qqq.com")]), false);
    assert_eq!(get_text(&url).await, "form-get");
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.urlWithParamIfGetTest()`
#[tokio::test]
async fn http_request_test_url_with_param_if_get_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.starts_with("GET /"));
        http_ok("ok")
    })
    .await;
    assert_eq!(
        get_resp(Url::parse(&base).unwrap().as_str())
            .await
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.issueI5Y68WTest()`
#[tokio::test]
async fn http_request_test_issue_i5_y68_w_test() {
    let body = r#"{"address":"mock"}"#;
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.issueIAAE88Test()`
#[tokio::test]
async fn http_request_test_issue_iaae88_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(request_path(&req).contains("com.rnd.aiq:message"));
        http_ok("colon-path")
    })
    .await;
    assert_eq!(
        get_text(&format!("{base}/com.rnd.aiq:message/message/getName/15")).await,
        "colon-path"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.testHttpResource()`
#[tokio::test]
async fn http_request_test_test_http_resource() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("passport=12456") && req.contains("张三"));
        http_ok("resource-ok")
    })
    .await;
    let mp = MultipartBody::create(
        [
            ("passport".into(), "12456".into()),
            ("user".into(), r#"{"name":"张三"}"#.into()),
        ]
        .into(),
        "UTF-8",
    );
    let c = local_client();
    let resp = c
        .send_response(
            c.request(Method::POST, &base)
                .header(header::CONTENT_TYPE, mp.content_type())
                .body(mp.to_bytes()),
        )
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpRequestTest.issueIAAOC1Test()`
#[tokio::test]
async fn http_request_test_issue_iaaoc1_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("goodsName="));
        http_ok("report")
    })
    .await;
    assert_eq!(
        get_text(&format!("{base}/get?goodsName=工业硫酸98%")).await,
        "report"
    );
    task.await.unwrap();
}

// ── HttpUtilTest (network) ─────────────────────────────────────────────────

/// 对齐 Java: `HttpUtilTest.postTest()`
#[tokio::test]
async fn http_util_test_post_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.starts_with("POST"));
        http_ok("post")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(c.request(Method::POST, &base).body("{}"))
            .await
            .unwrap()
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.postTest2()`
#[tokio::test]
async fn http_util_test_post_test2() {
    let (base, task) = serve_fn(|req| async move {
        assert_eq!(request_header(&req, "accept"), Some("*/*"));
        http_ok("ok")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(c.request(Method::POST, &base).header(header::ACCEPT, "*/*"))
            .await
            .unwrap()
            .body(),
        "ok"
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getTest()`
#[tokio::test]
async fn http_util_test_get_test() {
    let body = "qzone";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getTest2()`
#[tokio::test]
async fn http_util_test_get_test2() {
    let body = "oauth";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getTest3()`
#[tokio::test]
async fn http_util_test_get_test3() {
    let body = "space";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.get12306Test()`
#[tokio::test]
async fn http_util_test_get12306_test() {
    let body = "12306";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.downloadStringTest()`
#[tokio::test]
async fn http_util_test_download_string_test() {
    let body = "baidu";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.oschinaTest()`
#[tokio::test]
async fn http_util_test_oschina_test() {
    let body = "oschina";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getWeixinTest()`
#[tokio::test]
async fn http_util_test_get_weixin_test() {
    let body = "weixin";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getNocovTest()`
#[tokio::test]
async fn http_util_test_get_nocov_test() {
    let body = "nocov";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.sinajsTest()`
#[tokio::test]
async fn http_util_test_sinajs_test() {
    let body = "var s;";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.gimg2Test()`
#[tokio::test]
async fn http_util_test_gimg2_test() {
    let body = "gimg2";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.acplayTest()`
#[tokio::test]
async fn http_util_test_acplay_test() {
    let body = "acplay";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getTest4()`
#[tokio::test]
async fn http_util_test_get_test4() {
    let img = b"\x89PNG";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "image/jpeg")],
        img,
    ))
    .await;
    assert_eq!(get_resp(&base).await.body_bytes(), img);
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getTest5()`
#[tokio::test]
async fn http_util_test_get_test5() {
    let pdf = b"%PDF";
    let resp = http_response("200 OK", &[("Content-Type", "application/pdf")], pdf);
    let (base, task) = serve_sequence(vec![resp.clone(), resp]).await;
    for p in ["/a.pdf", "/b.pdf"] {
        assert_eq!(download(&format!("{base}{p}")).await, pdf);
    }
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.patchTest()`
#[tokio::test]
async fn http_util_test_patch_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.starts_with("PATCH"));
        http_ok("patch")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(c.request(Method::PATCH, &base))
            .await
            .unwrap()
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `HttpUtilTest.getPicTest()`
#[tokio::test]
async fn http_util_test_get_pic_test() {
    let data = b"\x89PNG-pic";
    let (base, task) = serve_once(http_response(
        "200 OK",
        &[("Content-Type", "application/octet-stream")],
        data,
    ))
    .await;
    assert_eq!(download(&base).await, data);
    task.await.unwrap();
}

// ── HttpsTest / Issue* ───────────────────────────────────────────────────

/// 对齐 Java: `HttpsTest.getTest()`
#[tokio::test]
async fn https_test_get_test() {
    let (base, task) = serve_many(20, http_ok("tls")).await;
    let n = Arc::new(AtomicUsize::new(0));
    let mut hs = Vec::new();
    for _ in 0..20 {
        let url = base.clone();
        let n = Arc::clone(&n);
        hs.push(tokio::spawn(async move {
            assert_eq!(get_text(&url).await, "tls");
            n.fetch_add(1, Ordering::SeqCst);
        }));
    }
    for h in hs {
        h.await.unwrap();
    }
    assert_eq!(n.load(Ordering::SeqCst), 20);
    task.await.unwrap();
}

/// 对齐 Java: `Issue2658Test.getWithCookieTest()`
#[tokio::test]
async fn issue2658_test_get_with_cookie_test() {
    let (base, task) = serve_sequence(vec![
        http_response("200 OK", &[("Set-Cookie", "BDORZ=1; Path=/")], b"1"),
        http_ok("2"),
    ])
    .await;
    let c = HttpClient::builder().cookie_store(true).build().unwrap();
    assert_eq!(c.get_text(&base).await.unwrap(), "1");
    assert_eq!(c.get_text(&base).await.unwrap(), "2");
    task.await.unwrap();
}

/// 对齐 Java: `Issue3074Test.bodyTest()`
#[tokio::test]
async fn issue3074_test_body_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("aaa"));
        http_ok("json")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header(header::CONTENT_TYPE, ContentType::Json.value())
                .body("aaa")
        )
        .await
        .unwrap()
        .status()
        .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `Issue3197Test.getTest()`
#[tokio::test]
async fn issue3197_test_get_test() {
    let body = "index";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

/// 对齐 Java: `Issue3314Test.postTest()`
#[tokio::test]
async fn issue3314_test_post_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("PK"));
        http_ok("list")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(c.request(Method::GET, &base).body(b"PK\x03\x04".to_vec()))
            .await
            .unwrap()
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `IssueI5TPSYTest.redirectTest()`
#[tokio::test]
async fn issue_i5_tpsy_test_redirect_test() {
    let (base, task) = serve_sequence(vec![redirect("/final"), http_ok("redir")]).await;
    let c = local_client();
    let resp = c
        .send_response(
            c.request(Method::GET, &base)
                .header(header::COOKIE, "iPlanetDirectoryPro=123"),
        )
        .await
        .unwrap();
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.body(), "redir");
    task.await.unwrap();
}

/// 对齐 Java: `IssueI6RE7JTest.getTest()`
#[tokio::test]
async fn issue_i6_re7_j_test_get_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("TXSM"));
        http_ok("arc")
    })
    .await;
    let url = HttpUtil::url_with_form(
        &format!("{base}/ArcValue"),
        &form_map(&[
            ("tag", "TXSM_SC2202-苯乙烯%"),
            ("time", "2023/03/28 08:00:00"),
        ]),
        false,
    );
    assert_eq!(get_text(&url).await, "arc");
    task.await.unwrap();
}

/// 对齐 Java: `IssueI7EHSETest.encodePathTest()`
#[tokio::test]
async fn issue_i7_ehse_test_encode_path_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(request_header(&req, "referer").is_some());
        http_ok("var s;")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::GET, format!("{base}/list=s_sh600090"))
                .header("Referer", "http://finance.sina.com.cn")
        )
        .await
        .unwrap()
        .status()
        .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `IssueI7WZEOTest.postTest()`
#[tokio::test]
async fn issue_i7_wzeo_test_post_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("url="));
        http_ok(r#"{"video":"mock"}"#)
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body("url=https://v.douyin.com/x")
        )
        .await
        .unwrap()
        .body(),
        r#"{"video":"mock"}"#
    );
    task.await.unwrap();
}

/// 对齐 Java: `IssueI7ZRJUTest.getTest()`
#[tokio::test]
async fn issue_i7_zrju_test_get_test() {
    let (base, task) = serve_once(http_ok("badssl")).await;
    let mut cfg = HttpConfig::default();
    cfg.set_hostname_verifier(HostnameVerification::DangerousAcceptInvalid);
    assert_eq!(
        HttpClient::new(&cfg)
            .unwrap()
            .get_text(&base)
            .await
            .unwrap(),
        "badssl"
    );
    task.await.unwrap();
}

/// 对齐 Java: `IssueIB7REWTest.getTest()`
#[tokio::test]
async fn issue_ib7_rew_test_get_test() {
    let body = "boc";
    let (base, task) = serve_many(2, http_ok(body)).await;
    assert_eq!(get_text(&base).await, body);
    assert_eq!(get_resp(&base).await.status().as_u16(), 200);
    task.await.unwrap();
}

// ── RestTest ───────────────────────────────────────────────────────────────

/// 对齐 Java: `RestTest.postTest()`
#[tokio::test]
async fn rest_test_post_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("aaaValue"));
        http_ok("rest")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header(header::CONTENT_TYPE, "application/json;charset=UTF-8")
                .body(r#"{"aaa":"aaaValue"}"#)
        )
        .await
        .unwrap()
        .status()
        .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `RestTest.postTest2()`
#[tokio::test]
async fn rest_test_post_test2() {
    let (base, task) = serve_once(http_ok("util-post")).await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header(header::CONTENT_TYPE, "application/json")
                .body("{}")
        )
        .await
        .unwrap()
        .body(),
        "util-post"
    );
    task.await.unwrap();
}

/// 对齐 Java: `RestTest.getWithBodyTest()`
#[tokio::test]
async fn rest_test_get_with_body_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.starts_with("GET") && req.contains("aaaValue"));
        http_ok("gb")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::GET, &base)
                .header(header::CONTENT_TYPE, "application/json")
                .body(r#"{"aaa":"aaaValue"}"#)
        )
        .await
        .unwrap()
        .body(),
        "gb"
    );
    task.await.unwrap();
}

/// 对齐 Java: `RestTest.getWithBodyTest2()`
#[tokio::test]
async fn rest_test_get_with_body_test2() {
    let (base, task) = serve_fn(|req| async move {
        assert!(request_header(&req, "access-token").is_some());
        http_ok("adv")
    })
    .await;
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::GET, &base)
                .header("Access-Token", "t")
                .body(r#"{"advertiser_ids":[1]}"#)
        )
        .await
        .unwrap()
        .status()
        .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `RestTest.getTest()`
#[tokio::test]
async fn rest_test_get_test() {
    let (base, task) = serve_once(http_response("200 OK", &[("X-Rest", "yes")], b"rest-get")).await;
    let resp = get_resp(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert_eq!(resp.header("x-rest"), Some("yes"));
    task.await.unwrap();
}

// ── UploadTest ─────────────────────────────────────────────────────────────

/// 对齐 Java: `UploadTest.uploadFilesTest()`
#[tokio::test]
async fn upload_test_upload_files_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("fileType") && req.contains("a.png"));
        http_ok("multi")
    })
    .await;
    let form = reqwest::multipart::Form::new()
        .text("fileType", "图片")
        .part(
            "file",
            reqwest::multipart::Part::bytes(b"1")
                .file_name("a.png")
                .mime_str("image/png")
                .unwrap(),
        )
        .part(
            "file",
            reqwest::multipart::Part::bytes(b"2")
                .file_name("b.png")
                .mime_str("image/png")
                .unwrap(),
        );
    let c = local_client();
    assert_eq!(
        c.send_response(c.request(Method::POST, &base).multipart(form))
            .await
            .unwrap()
            .body(),
        "multi"
    );
    task.await.unwrap();
}

/// 对齐 Java: `UploadTest.uploadFileTest()`
#[tokio::test]
async fn upload_test_upload_file_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("city") && req.contains("face.jpg"));
        http_ok("weather")
    })
    .await;
    let form = reqwest::multipart::Form::new().text("city", "北京").part(
        "file",
        reqwest::multipart::Part::bytes(b"j")
            .file_name("face.jpg")
            .mime_str("image/jpeg")
            .unwrap(),
    );
    assert_eq!(
        local_client()
            .send_response(local_client().request(Method::POST, &base).multipart(form))
            .await
            .unwrap()
            .status()
            .as_u16(),
        200
    );
    task.await.unwrap();
}

/// 对齐 Java: `UploadTest.uploadTest2()`
#[tokio::test]
async fn upload_test_upload_test2() {
    let (base, task) = serve_fn(|req| async move {
        assert!(request_header(&req, "md5").is_some());
        http_ok("chunk")
    })
    .await;
    let form = reqwest::multipart::Form::new()
        .text("fileName", "x.xlsx")
        .part(
            "file",
            reqwest::multipart::Part::bytes(b"x")
                .file_name("x.xlsx")
                .mime_str("application/octet-stream")
                .unwrap(),
        );
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header("md5", "aaaaaaaa")
                .multipart(form)
        )
        .await
        .unwrap()
        .body(),
        "chunk"
    );
    task.await.unwrap();
}

/// 对齐 Java: `UploadTest.smmsTest()`
#[tokio::test]
async fn upload_test_smms_test() {
    let (base, task) = serve_fn(|req| async move {
        assert!(req.contains("smfile"));
        http_ok(r#"{"success":true}"#)
    })
    .await;
    let form = reqwest::multipart::Form::new().part(
        "smfile",
        reqwest::multipart::Part::bytes(b"q")
            .file_name("q.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let c = local_client();
    assert_eq!(
        c.send_response(
            c.request(Method::POST, &base)
                .header(header::USER_AGENT, "PostmanRuntime/7.28.4")
                .header(header::AUTHORIZATION, "Bearer test")
                .multipart(form)
        )
        .await
        .unwrap()
        .body(),
        r#"{"success":true}"#
    );
    task.await.unwrap();
}

// ── SOAP parity via HttpClient POST ────────────────────────────────────────

/// 对齐 Java: `JakartaSoapClientTest.requestTest()`
#[tokio::test]
async fn jakarta_soap_client_test_request_test() {
    let (base, task) = serve_once(soap_response("黑龙江 哈尔滨")).await;
    let resp = soap_post(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert!(resp.body().contains("黑龙江"));
    task.await.unwrap();
}

/// 对齐 Java: `JakartaSoapClientTest.requestForMessageTest()`
#[tokio::test]
async fn jakarta_soap_client_test_request_for_message_test() {
    let (base, task) = serve_once(soap_response("黑龙江 哈尔滨")).await;
    let resp = soap_post(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert!(resp.body().contains("哈尔滨"));
    task.await.unwrap();
}

/// 对齐 Java: `SoapClientTest.requestTest()`
#[tokio::test]
async fn soap_client_test_request_test() {
    let (base, task) = serve_once(soap_response("黑龙江 哈尔滨")).await;
    let resp = soap_post(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert!(resp.body().contains("黑龙江"));
    task.await.unwrap();
}

/// 对齐 Java: `SoapClientTest.requestForMessageTest()`
#[tokio::test]
async fn soap_client_test_request_for_message_test() {
    let (base, task) = serve_once(soap_response("黑龙江 哈尔滨")).await;
    let resp = soap_post(&base).await;
    assert_eq!(resp.status().as_u16(), 200);
    assert!(resp.body().contains("哈尔滨"));
    task.await.unwrap();
}
