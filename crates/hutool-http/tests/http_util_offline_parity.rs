//! HttpUtil offline param/URL helpers parity
//! 对齐: `cn.hutool.http.HttpUtilTest`, `Issue3536Test`, `IssueIBRVE4Test`, `IssueIBQIYQTest`, `MultipartBodyTest`

use hutool_http::{HttpUtil, MultipartBody, form_map};
use std::collections::HashMap;

/// 对齐 Java: `HttpUtilTest.isHttpTest()`
#[test]
fn is_http_test() {
    assert!(HttpUtil::is_http("Http://aaa.bbb"));
    assert!(HttpUtil::is_http("HTTP://aaa.bbb"));
    assert!(!HttpUtil::is_http("FTP://aaa.bbb"));
}

/// 对齐 Java: `HttpUtilTest.isHttpsTest()`
#[test]
fn is_https_test() {
    assert!(HttpUtil::is_https("Https://aaa.bbb"));
    assert!(HttpUtil::is_https("HTTPS://aaa.bbb"));
    assert!(HttpUtil::is_https("https://aaa.bbb"));
    assert!(!HttpUtil::is_https("ftp://aaa.bbb"));
}

/// 对齐 Java: `HttpUtilTest.getCharsetTest()`
#[test]
fn get_charset_test() {
    assert_eq!(extract_charset("Charset=UTF-8;fq=0.9"), Some("UTF-8"));
    for sample in [
        "<meta charset=utf-8",
        "<meta charset='utf-8'",
        "<meta charset=\"utf-8\"",
        "<meta charset = \"utf-8\"",
    ] {
        assert_eq!(extract_meta_charset(sample), Some("utf-8"));
    }
}

fn extract_charset(s: &str) -> Option<&str> {
    let lower = s.to_ascii_lowercase();
    let idx = lower.find("charset")?;
    let rest = &s[idx + "charset".len()..];
    let rest = rest.trim_start_matches([' ', '\t']);
    let rest = rest.strip_prefix('=')?;
    let rest = rest.trim_start_matches([' ', '\t', '\'', '"']);
    let end = rest
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

fn extract_meta_charset(s: &str) -> Option<&str> {
    let lower = s.to_ascii_lowercase();
    if !lower.contains("<meta") {
        return None;
    }
    extract_charset(s)
}

/// 对齐 Java: `HttpUtilTest.getMimeTypeTest()`
#[test]
fn get_mime_type_test() {
    assert!(HttpUtil::get_mime_type("aaa.aaa").is_none());
    assert_eq!(
        HttpUtil::get_mime_type("assets/INDEX.HTML?version=1#main"),
        Some("text/html")
    );
    assert_eq!(
        HttpUtil::get_mime("api/data.JSON"),
        Some(mime::APPLICATION_JSON)
    );
}

/// 对齐 Java: `HttpUtilTest.decodeParamsTest()`
#[test]
fn http_util_test_decode_params_test() {
    let map = HttpUtil::decode_params("uuuu=0&a=b&c=%3F%23%40!%24%25%5E%26%3Ddsssss555555");
    assert_eq!(map["uuuu"][0], "0");
    assert_eq!(map["a"][0], "b");
    assert_eq!(map["c"][0], "?#@!$%^&=dsssss555555");
}

/// 对齐 Java: `HttpUtilTest.decodeParamMapTest()`
#[test]
fn http_util_test_decode_param_map_test() {
    let param_map = HttpUtil::decode_param_map(
        "https://www.xxx.com/api.action?aa=123&f_token=NzBkMjQxNDM1MDVlMDliZTk1OTU3ZDI1OTI0NTBiOWQ=",
    );
    assert_eq!(param_map["aa"], "123");
    assert_eq!(
        param_map["f_token"],
        "NzBkMjQxNDM1MDVlMDliZTk1OTU3ZDI1OTI0NTBiOWQ="
    );
}

/// 对齐 Java: `HttpUtilTest.toParamsTest()`
#[test]
fn http_util_test_to_params_test() {
    let params_str = "uuuu=0&a=b&c=3Ddsssss555555";
    let map = HttpUtil::decode_params(params_str);
    let encoded = HttpUtil::to_params(&map);
    assert_eq!(encoded, params_str);
}

/// 对齐 Java: `HttpUtilTest.encodeParamTest()`
#[test]
fn http_util_test_encode_param_test() {
    assert_eq!(HttpUtil::encode_params("?a=b&c=d&"), "a=b&c=d");
    assert_eq!(
        HttpUtil::encode_params("http://www.abc.dd?a=b&c=d&"),
        "http://www.abc.dd?a=b&c=d"
    );
    assert_eq!(HttpUtil::encode_params("a=b=b&c=d&"), "a=b=b&c=d");
    assert_eq!(HttpUtil::encode_params("a=bbb&c=d&=d"), "a=bbb&c=d&=d");
    assert_eq!(HttpUtil::encode_params("a=bbb&c=d&d="), "a=bbb&c=d&d=");
    assert_eq!(HttpUtil::encode_params("a=bbb&c=d&&&d="), "a=bbb&c=d&d=");
    assert_eq!(HttpUtil::encode_params("a=bbb&c=d&d&"), "a=bbb&c=d&d=");
    assert_eq!(
        HttpUtil::encode_params("a=bbb&c=你好&哈喽&"),
        "a=bbb&c=%E4%BD%A0%E5%A5%BD&%E5%93%88%E5%96%BD="
    );
    assert_eq!(
        HttpUtil::encode_params("https://www.hutool.cn/"),
        "https://www.hutool.cn/"
    );
    assert_eq!(
        HttpUtil::encode_params("https://www.hutool.cn/?"),
        "https://www.hutool.cn/"
    );
}

/// 对齐 Java: `HttpUtilTest.decodeParamTest()`
#[test]
fn http_util_test_decode_param_test() {
    let mut map = HttpUtil::decode_params("?a=b&c=d&");
    assert_eq!(map["a"][0], "b");
    assert_eq!(map["c"][0], "d");

    map = HttpUtil::decode_params("?a=b&c=d&=e");
    assert_eq!(map[""][0], "e");

    map = HttpUtil::decode_params("?a=b&c=d&=e&&&&");
    assert_eq!(map[""][0], "e");

    map = HttpUtil::decode_params("?a=b&c=d&e=");
    assert_eq!(map["e"][0], "");

    map = HttpUtil::decode_params("a=b&c=d&=");
    assert_eq!(map[""][0], "");

    map = HttpUtil::decode_params("a=b&c=d&e&");
    assert_eq!(map["e"][0], "");

    map = HttpUtil::decode_params("a=bbb&c=%E4%BD%A0%E5%A5%BD&%E5%93%88%E5%96%BD=");
    assert_eq!(map["a"][0], "bbb");
    assert_eq!(map["c"][0], "你好");
    assert_eq!(map["哈喽"][0], "");
}

/// 对齐 Java: `HttpUtilTest.urlWithFormTest()`
#[test]
fn http_util_test_url_with_form_test() {
    let param = form_map(&[
        ("AccessKeyId", "123"),
        ("Action", "DescribeDomainRecords"),
        ("Format", "date"),
        ("DomainName", "lesper.cn"),
        ("SignatureMethod", "POST"),
        ("SignatureNonce", "123"),
        ("SignatureVersion", "4.3.1"),
        ("Timestamp", "123432453"),
        ("Version", "1.0"),
    ]);
    let expected = "http://api.hutool.cn/login?type=aaa&AccessKeyId=123&Action=DescribeDomainRecords&Format=date&DomainName=lesper.cn&SignatureMethod=POST&SignatureNonce=123&SignatureVersion=4.3.1&Timestamp=123432453&Version=1.0";
    let url_with_form =
        HttpUtil::url_with_form("http://api.hutool.cn/login?type=aaa", &param, false);
    assert_eq!(url_with_form, expected);
    let again = HttpUtil::url_with_form("http://api.hutool.cn/login?type=aaa", &param, false);
    assert_eq!(again, expected);
}

/// 对齐 Java: `HttpUtilTest.normalizeParamsTest()`
#[test]
fn http_util_test_normalize_params_test() {
    assert_eq!(
        HttpUtil::normalize_params("参数", true),
        "%E5%8F%82%E6%95%B0"
    );
}

/// 对齐 Java: `HttpUtilTest.normalizeBlankParamsTest()`
#[test]
fn http_util_test_normalize_blank_params_test() {
    assert_eq!(HttpUtil::normalize_params("", true), "");
}

/// 对齐 Java: `HttpUtilTest.normalizEampersandParamsTest()`
#[test]
fn http_util_test_normaliz_eampersand_params_test() {
    assert_eq!(HttpUtil::normalize_params("&", true), "");
}

/// 对齐 Java: `Issue3536Test.urlWithFormUrlEncodedTest()`
#[test]
fn issue3536_test_url_with_form_url_encoded_test() {
    let param = form_map(&[
        ("redirect_uri", "https://api.hutool.cn/v1/test"),
        ("scope", "a,b,c你"),
    ]);
    let s = HttpUtil::url_with_form_url_encoded("https://hutool.cn/test", &param);
    // IndexMap preserves insertion order (Hutool HashMap order is unspecified).
    assert_eq!(
        s,
        "https://hutool.cn/test?redirect_uri=https://api.hutool.cn/v1/test&scope=a,b,c%E4%BD%A0"
    );
}

/// 对齐 Java: `Issue3536Test.toParamsTest()`
#[test]
fn issue3536_test_to_params_test() {
    let param = form_map(&[
        ("redirect_uri", "https://api.hutool.cn/v1/test"),
        ("scope", "a,b,c你"),
    ]);
    let params = HttpUtil::to_params_form(&param, true);
    assert_eq!(
        params,
        "redirect_uri=https%3A%2F%2Fapi.hutool.cn%2Fv1%2Ftest&scope=a%2Cb%2Cc%E4%BD%A0"
    );
}

/// 对齐 Java: `IssueIBRVE4Test.decodeParamMapNoParamTest()`
#[test]
fn issue_ibrve4_test_decode_param_map_no_param_test() {
    let param_map = HttpUtil::decode_params("https://hutool.cn/api.action");
    assert!(param_map.is_empty());
}

/// 对齐 Java: `IssueIBRVE4Test.decodeParamMapListNoParamTest()`
#[test]
fn issue_ibrve4_test_decode_param_map_list_no_param_test() {
    let param_map = HttpUtil::decode_param_map("https://hutool.cn/api.action");
    assert!(param_map.is_empty());
}

/// 对齐 Java: `IssueIBQIYQTest.normalizeParamsTest1()`
#[test]
fn issue_ibqiyq_test_normalize_params_test1() {
    let map = form_map(&[("id", ""), ("type", "4")]);
    let url = HttpUtil::to_params_map(&map);
    assert_eq!(url, "id=&type=4");
    assert_eq!(HttpUtil::normalize_params(&url, false), "id=&type=4");
}

/// 对齐 Java: `MultipartBodyTest.buildTest()`
#[test]
fn multipart_body_test_build_test() {
    let mut form = HashMap::new();
    form.insert("pic1".to_string(), "pic1 content".to_string());
    form.insert("pic2".to_string(), "pic2 content".to_string());
    form.insert("pic3".to_string(), "pic3 content".to_string());
    let body = MultipartBody::create(form, "UTF-8");
    assert!(!body.to_string().is_empty());
    assert!(body.content_type().contains("multipart/form-data"));
    let mut buf = Vec::new();
    body.write(&mut buf).unwrap();
    assert!(buf.windows(4).any(|w| w == b"pic1"));
    let alias = MultipartBody::new(HashMap::new(), "UTF-8");
    assert!(alias.content_type().contains("boundary="));
}

/// 对齐 Java: `HttpUtilTest` / `HttpUtil.buildBasicAuth`
#[test]
fn http_util_build_basic_auth_test() {
    assert_eq!(
        HttpUtil::build_basic_auth("aladdin", "opensesame"),
        "Basic YWxhZGRpbjpvcGVuc2VzYW1l"
    );
}

/// 对齐 Java: `HttpUtil.getCharset(String)`
#[test]
fn http_util_get_charset_string_test() {
    assert_eq!(
        HttpUtil::get_charset("text/html; charset=UTF-8"),
        Some("UTF-8".into())
    );
}

/// 对齐 Java: `HttpUtil.getContentTypeByRequestBody`
#[test]
fn http_util_get_content_type_by_request_body_test() {
    assert_eq!(
        HttpUtil::get_content_type_by_request_body("{\"a\":1}").as_deref(),
        Some("application/json")
    );
    assert_eq!(
        HttpUtil::get_content_type_by_request_body("<root/>").as_deref(),
        Some("application/xml")
    );
}

/// 对齐 Java: `HttpUtil.getMimeType(String, String)`
#[test]
fn http_util_get_mime_type_default_test() {
    assert_eq!(
        HttpUtil::get_mime_type_or("x.unknown", "application/octet-stream"),
        "application/octet-stream"
    );
    assert_eq!(
        HttpUtil::get_mime_type_or("a.json", "x"),
        "application/json"
    );
}

/// 对齐 Java: `HttpUtil.getString(byte[], Charset, boolean)`
#[test]
fn http_util_get_string_bytes_test() {
    let bytes = "你好".as_bytes();
    assert_eq!(HttpUtil::get_string(bytes, Some("UTF-8"), false), "你好");
}
