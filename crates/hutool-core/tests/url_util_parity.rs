//! url_util parity tests
//! 对齐: `cn.hutool.core.util.URLUtilTest`

use hutool_core::UrlUtil;

#[test]
fn is_url_http() {
    assert!(UrlUtil::is_url("http://example.com"));
    assert!(UrlUtil::is_url("https://example.com"));
    assert!(UrlUtil::is_url("ftp://example.com"));
}

#[test]
fn is_url_invalid() {
    assert!(!UrlUtil::is_url("not-a-url"));
    assert!(!UrlUtil::is_url(""));
}

#[test]
fn is_http_valid() {
    assert!(UrlUtil::is_http("http://example.com"));
    assert!(!UrlUtil::is_http("https://example.com"));
}

#[test]
fn is_https_valid() {
    assert!(UrlUtil::is_https("https://example.com"));
    assert!(!UrlUtil::is_https("http://example.com"));
}

/// 对齐 Java: URLUtil normalize / file-jar / data-uri / query leftovers
#[test]
fn url_util_leftover_helpers() {
    assert!(UrlUtil::is_file_url("file:///tmp/a"));
    assert!(UrlUtil::is_jar_url("jar:file:/a.jar!/b"));
    assert_eq!(UrlUtil::encode_blank("a b"), "a%20b");
    assert!(UrlUtil::build_query(&[("q", "a b")]).contains("q="));
    assert!(UrlUtil::get_data_uri("text/plain", "YQ==").starts_with("data:"));
    assert!(!UrlUtil::url("example.com/x").is_empty());
    assert_eq!(
        UrlUtil::complete_url("http://a.com", "/b"),
        "http://a.com/b"
    );
    assert!(!UrlUtil::get_decoded_path("http://a.com/%20").is_empty());
}

#[test]
fn get_host_basic() {
    assert_eq!(
        UrlUtil::get_host("http://example.com/path"),
        Some("example.com")
    );
    assert_eq!(
        UrlUtil::get_host("https://example.com"),
        Some("example.com")
    );
}

#[test]
fn get_host_with_port() {
    assert_eq!(
        UrlUtil::get_host("http://example.com:8080/path"),
        Some("example.com:8080")
    );
}

#[test]
fn get_path_basic() {
    assert_eq!(
        UrlUtil::get_path("http://example.com/path/to/resource"),
        "/path/to/resource"
    );
}

#[test]
fn get_path_root() {
    assert_eq!(UrlUtil::get_path("http://example.com"), "/");
}

#[test]
fn get_protocol_http() {
    assert_eq!(UrlUtil::get_protocol("http://example.com"), Some("http"));
}

#[test]
fn get_protocol_https() {
    assert_eq!(UrlUtil::get_protocol("https://example.com"), Some("https"));
}

#[test]
fn get_protocol_ftp() {
    assert_eq!(UrlUtil::get_protocol("ftp://example.com"), Some("ftp"));
}

#[test]
fn get_protocol_none() {
    assert_eq!(UrlUtil::get_protocol("example.com"), None);
}

#[test]
fn encode_basic() {
    assert_eq!(UrlUtil::encode("hello world"), "hello%20world");
    assert_eq!(UrlUtil::encode("hello"), "hello");
}

#[test]
fn encode_special_chars() {
    // RFC3986.PATH 保留 sub-delims 中的 `=` 与 `&`
    assert_eq!(UrlUtil::encode("a=b&c=d"), "a=b&c=d");
}

#[test]
fn decode_basic() {
    assert_eq!(UrlUtil::decode("hello+world"), "hello world");
    assert_eq!(UrlUtil::decode("hello"), "hello");
}

#[test]
fn decode_special_chars() {
    assert_eq!(UrlUtil::decode("a%3Db%26c%3Dd"), "a=b&c=d");
}

#[test]
fn encode_decode_roundtrip() {
    for input in ["hello world", "a=b&c=d", "test@example.com"] {
        let encoded = UrlUtil::encode(input);
        let decoded = UrlUtil::decode(&encoded);
        assert_eq!(decoded, input);
    }
}

#[test]
fn normalize_with_protocol() {
    assert_eq!(
        UrlUtil::normalize("http://example.com"),
        "http://example.com"
    );
    assert_eq!(
        UrlUtil::normalize("https://example.com"),
        "https://example.com"
    );
}

#[test]
fn normalize_without_protocol() {
    assert_eq!(UrlUtil::normalize("example.com"), "http://example.com");
}

#[test]
fn complete_url_basic() {
    assert_eq!(
        UrlUtil::complete_url("http://example.com", "/path"),
        "http://example.com/path"
    );
    assert_eq!(
        UrlUtil::complete_url("http://example.com/", "path"),
        "http://example.com/path"
    );
}

// ── 对齐 Hutool URLUtilTest ──

/// 对齐 Java: `URLUtilTest.normalizeTest()`
#[test]
fn normalize_test() {
    let url = "http://www.hutool.cn//aaa/bbb";
    let normalize = UrlUtil::normalize(url);
    assert_eq!("http://www.hutool.cn//aaa/bbb", normalize);

    let url = "www.hutool.cn//aaa/bbb";
    let normalize = UrlUtil::normalize(url);
    assert_eq!("http://www.hutool.cn//aaa/bbb", normalize);
}

/// 对齐 Java: `URLUtilTest.getHostTest()`
#[test]
fn get_host_test() {
    let url = "https://www.hutool.cn//aaa/bbb?a=1&b=2";
    let host = UrlUtil::get_host(url).unwrap_or("");
    assert!(host.contains("www.hutool.cn") || host == "www.hutool.cn");
}

// ── Hutool TEST parity gap wave ──
// ── Hutool URLUtilTest remaining gaps ──

/// 对齐 Java: `URLUtilTest.normalizeTest2()`
#[test]
fn normalize_test_2() {
    assert_eq!(
        UrlUtil::normalize("http://www.hutool.cn//aaa/\\bbb?a=1&b=2"),
        "http://www.hutool.cn//aaa//bbb?a=1&b=2"
    );
    assert_eq!(
        UrlUtil::normalize("www.hutool.cn//aaa/bbb?a=1&b=2"),
        "http://www.hutool.cn//aaa/bbb?a=1&b=2"
    );
}

/// 对齐 Java: `URLUtilTest.normalizeTest3()`
#[test]
fn normalize_test_3() {
    assert_eq!(
        UrlUtil::normalize_with_encode_path("http://www.hutool.cn//aaa/\\bbb?a=1&b=2", true),
        "http://www.hutool.cn//aaa//bbb?a=1&b=2"
    );
    assert_eq!(
        UrlUtil::normalize_with_encode_path("www.hutool.cn//aaa/bbb?a=1&b=2", true),
        "http://www.hutool.cn//aaa/bbb?a=1&b=2"
    );
    assert_eq!(
        UrlUtil::normalize_with_encode_path("\\/www.hutool.cn//aaa/bbb?a=1&b=2", true),
        "http://www.hutool.cn//aaa/bbb?a=1&b=2"
    );
}

/// 对齐 Java: `URLUtilTest.normalizeIpv6Test()`
#[test]
fn normalize_ipv6_test() {
    let url = "http://[fe80::8f8:2022:a603:d180]:9439";
    let normalize = UrlUtil::normalize_with_encode_path(url, true);
    assert_eq!(url, normalize);
}

/// 对齐 Java: `URLUtilTest.formatTest()`
#[test]
fn format_test() {
    let url = "//www.hutool.cn//aaa/\\bbb?a=1&b=2";
    let normalize = UrlUtil::normalize(url);
    assert_eq!("http://www.hutool.cn//aaa//bbb?a=1&b=2", normalize);
}

/// 对齐 Java: `URLUtilTest.encodeTest()`
#[test]
fn encode_test() {
    let body = "366466 - 副本.jpg";
    let encode = UrlUtil::encode(body);
    assert_eq!(encode, "366466%20-%20%E5%89%AF%E6%9C%AC.jpg");
    assert_eq!(body, UrlUtil::decode(&encode));

    let encode_query = UrlUtil::encode_query(body);
    assert_eq!(encode_query, "366466%20-%20%E5%89%AF%E6%9C%AC.jpg");
}

/// 对齐 Java: `URLUtilTest.encodeQueryPlusTest()`
#[test]
fn encode_query_plus_test() {
    let body = "+";
    let encode = UrlUtil::encode_query(body);
    assert_eq!("+", encode);
}

/// 对齐 Java: `URLUtilTest.getPathTest()`
#[test]
fn get_path_test() {
    let path = UrlUtil::get_path(" http://www.aaa.bbb/search?scope=ccc&q=ddd");
    assert_eq!(path, "/search");
}

/// 对齐 Java: `URLUtilTest.issue3676Test()`
#[test]
fn issue_3676_test() {
    let file_full_name = "/Uploads/20240601/aaaa.txt";
    let uri = UrlUtil::to_uri(file_full_name).expect("to_uri should succeed");
    let resolved = uri.resolve(".");
    assert_eq!("/Uploads/20240601/", resolved.as_str());
}
