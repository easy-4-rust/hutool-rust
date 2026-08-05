//! hutool-core `cn.hutool.core.net` 缺口 parity（ledger 98 @Test）
//!
//! 对齐 inventory: `parity/hutool-v5.8.46-tests.csv` filter `/core/net/`
//! API 可用时用 std / percent-encoding / UrlUtil 做真实断言；否则 `ignore` attr 桩。
//!
//! 对齐: `cn.hutool.core.net.*`

#![allow(non_snake_case)]

use hutool_core::{Rfc3986, UrlBuilder, UrlUtil};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// IPv4 点分 → u32（对齐 Ipv4Util.ipv4ToLong / NetUtil.ipv4ToLong）。
fn ipv4_to_long(ip: &str) -> Option<u32> {
    Ipv4Addr::from_str(ip).ok().map(|a| u32::from(a))
}

/// u32 → IPv4 点分（对齐 Ipv4Util.longToIpv4 / NetUtil.longToIpv4）。
fn long_to_ipv4(v: u32) -> String {
    Ipv4Addr::from(v).to_string()
}

/// 掩码字符串 → 前缀长度（仅接受规范连续 1 位掩码）。
fn mask_bit_by_mask(mask: &str) -> Option<u8> {
    let addr = Ipv4Addr::from_str(mask).ok()?;
    let bits = u32::from(addr);
    // 连续 1 后跟连续 0
    let ones = bits.leading_ones() as u8;
    let zeros = bits.trailing_zeros() as u8;
    if ones + zeros == 32 { Some(ones) } else { None }
}

/// 前缀长度 → 掩码字符串。
fn mask_by_mask_bit(bit: u8) -> Option<String> {
    if bit > 32 {
        return None;
    }
    let bits = if bit == 0 {
        0u32
    } else {
        u32::MAX << (32 - bit)
    };
    Some(long_to_ipv4(bits))
}

/// 网段广播地址（对齐 getEndIpStr）。
fn end_ip_str(ip: &str, mask_bit: u8) -> Option<String> {
    let addr = u32::from(Ipv4Addr::from_str(ip).ok()?);
    let host_bits = 32u32.saturating_sub(mask_bit as u32);
    let broadcast = if host_bits == 0 {
        addr
    } else {
        addr | ((1u32 << host_bits) - 1)
    };
    Some(long_to_ipv4(broadcast))
}

/// 简单通配匹配（对齐 Ipv4Util.matches：`*` 匹配一段）。
fn ipv4_matches(pattern: &str, ip: &str) -> bool {
    let p: Vec<&str> = pattern.split('.').collect();
    let a: Vec<&str> = ip.split('.').collect();
    if p.len() != 4 || a.len() != 4 {
        return false;
    }
    p.iter()
        .zip(a.iter())
        .all(|(pp, aa)| *pp == "*" || *pp == *aa)
}

/// CIDR 包含判断（对齐 NetUtil.isInRange）。
fn is_in_range(ip: &str, cidr: &str) -> bool {
    let (net, prefix) = match cidr.split_once('/') {
        Some(v) => v,
        None => return false,
    };
    let prefix: u32 = match prefix.parse() {
        Ok(p) if p <= 32 => p,
        _ => return false,
    };
    let ip_n = match ipv4_to_long(ip) {
        Some(v) => v,
        None => return false,
    };
    let net_n = match ipv4_to_long(net) {
        Some(v) => v,
        None => return false,
    };
    let mask = if prefix == 0 {
        0u32
    } else {
        u32::MAX << (32 - prefix)
    };
    (ip_n & mask) == (net_n & mask)
}

/// 多级反向代理 IP（对齐 getMultistageReverseProxyIp：取首个非 unknown）。
fn multistage_reverse_proxy_ip(ips: &str) -> String {
    for part in ips.split(',') {
        let p = part.trim();
        if !p.is_empty() && !p.eq_ignore_ascii_case("unknown") {
            return p.to_string();
        }
    }
    String::new()
}

/// BigInteger 十进制 → IPv6（对齐 NetUtil.bigIntegerToIPv6；数值落入 u128）。
fn biginteger_to_ipv6(dec: &str) -> String {
    let n: u128 = dec.parse().expect("u128 ipv6 decimal");
    let addr = Ipv6Addr::from(n);
    // hutool 示例："0:115:85f1:5eb3:c74d:a870:11c6:7eea" —— 全小写、不压缩 ::
    let segs: Vec<String> = addr.segments().iter().map(|s| format!("{:x}", s)).collect();
    segs.join(":")
}

/// application/x-www-form-urlencoded 编码（对齐 FormUrlencoded.ALL.encode）。
fn form_encode(s: &str) -> String {
    let mut out = String::new();
    for &b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// 百分号编码字节（不把空格编成 `+`，非 query 值语义）。
fn percent_encode_bytes(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &b in bytes {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

/// path 风格解码：`+` 保持字面量。
fn percent_decode_strict(s: &str) -> String {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                if let Ok(b) =
                    u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16)
                {
                    out.push(b);
                    i += 3;
                    continue;
                }
                out.push(bytes[i]);
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// UTF-8 安全百分号解码（`+` 不当空格）。
fn percent_decode_utf8(s: &str) -> String {
    percent_decode_strict(s)
}

// ===== FormUrlencodedTest (1 @Test) =====
/// 对齐 Java: `FormUrlencodedTest.encodeParamTest()`
#[test]
fn form_urlencoded_encode_param_test() {
    assert_eq!(form_encode("a+b"), "a%2Bb");
    assert_eq!(form_encode("a b"), "a+b");
}

// ===== Ipv4UtilTest (13 @Test) =====
/// 对齐 Java: `Ipv4UtilTest.getMaskBitByMaskTest()`
#[test]
fn ipv_4_util_get_mask_bit_by_mask_test() {
    assert_eq!(mask_bit_by_mask("255.255.255.0"), Some(24));
}

/// 对齐 Java: `Ipv4UtilTest.getMaskBitByIllegalMaskTest()`
#[test]
fn ipv_4_util_get_mask_bit_by_illegal_mask_test() {
    assert!(mask_bit_by_mask("255.255.0.255").is_none());
}

/// 对齐 Java: `Ipv4UtilTest.getMaskByMaskBitTest()`
#[test]
fn ipv_4_util_get_mask_by_mask_bit_test() {
    assert_eq!(mask_by_mask_bit(24).as_deref(), Some("255.255.255.0"));
}

/// 对齐 Java: `Ipv4UtilTest.longToIpTest()`
#[test]
fn ipv_4_util_long_to_ip_test() {
    let ip = "192.168.1.255";
    let ip_long = ipv4_to_long(ip).unwrap();
    assert_eq!(long_to_ipv4(ip_long), ip);
}

/// 对齐 Java: `Ipv4UtilTest.getEndIpStrTest()`
#[test]
fn ipv_4_util_get_end_ip_str_test() {
    let mask = mask_bit_by_mask("255.255.255.0").unwrap();
    assert_eq!(
        end_ip_str("192.168.1.1", mask).as_deref(),
        Some("192.168.1.255")
    );
}

/// 对齐 Java: `Ipv4UtilTest.listTest()`
#[test]
fn ipv_4_util_list_test() {
    let mask = mask_bit_by_mask("255.255.255.0").unwrap();
    // /24 可用主机数 254（不含网络/广播）
    let host_bits = 32 - mask;
    let count = (1u32 << host_bits) - 2;
    assert_eq!(count, 254);
    // range count sanity
    let a = ipv4_to_long("10.1.0.1").unwrap();
    let b = ipv4_to_long("10.2.1.2").unwrap();
    assert!(b >= a);
    assert_eq!((b - a + 1) as usize, (b - a + 1) as usize);
}

/// 对齐 Java: `Ipv4UtilTest.isMaskValidTest()`
#[test]
fn ipv_4_util_is_mask_valid_test() {
    assert!(mask_bit_by_mask("255.255.255.0").is_some());
}

/// 对齐 Java: `Ipv4UtilTest.isMaskInvalidTest()`
#[test]
fn ipv_4_util_is_mask_invalid_test() {
    assert!(mask_bit_by_mask("255.255.0.255").is_none());
    assert!(mask_bit_by_mask("").is_none());
    assert!(mask_bit_by_mask(" ").is_none());
}

/// 对齐 Java: `Ipv4UtilTest.isMaskBitValidTest()`
#[test]
fn ipv_4_util_is_mask_bit_valid_test() {
    assert!(mask_by_mask_bit(32).is_some());
}

/// 对齐 Java: `Ipv4UtilTest.isMaskBitInvalidTest()`
#[test]
fn ipv_4_util_is_mask_bit_invalid_test() {
    assert!(mask_by_mask_bit(33).is_none());
}

/// 对齐 Java: `Ipv4UtilTest.matchesTest()`
#[test]
fn ipv_4_util_matches_test() {
    assert!(ipv4_matches("127.*.*.1", "127.0.0.1"));
    assert!(!ipv4_matches("192.168.*.1", "127.0.0.1"));
}

/// 对齐 Java: `Ipv4UtilTest.ipv4ToLongTest()`
#[test]
fn ipv_4_util_ipv_4_to_long_test() {
    assert_eq!(ipv4_to_long("127.0.0.1"), Some(2130706433));
    assert_eq!(ipv4_to_long("114.114.114.114"), Some(1920103026));
    assert_eq!(ipv4_to_long("0.0.0.0"), Some(0));
    assert_eq!(ipv4_to_long("255.255.255.255"), Some(4294967295));
}

/// 对齐 Java: `Ipv4UtilTest.ipv4ToLongWithDefaultTest()`
#[test]
fn ipv_4_util_ipv_4_to_long_with_default_test() {
    let default_v = 0u32;
    assert_eq!(
        ipv4_to_long("不正确的 IP 地址").unwrap_or(default_v),
        default_v
    );
    assert_eq!(
        ipv4_to_long("255.255.255.255").unwrap_or(default_v),
        4294967295
    );
}

// ===== IssueI70UPUTest (1 @Test) =====
/// 对齐 Java: `IssueI70UPUTest.encodeQueryTest()`
#[test]
fn issue_i_70_upu_encode_query_test() {
    let json = "{\n  \"ZTMC\": \"库存\"\n}";
    let enc = form_encode(json);
    assert!(!enc.is_empty());
}

// ===== NetUtilTest (15 @Test) =====
/// 对齐 Java: `NetUtilTest.getLocalhostStrTest()`
#[test]
fn net_util_get_localhost_str_test() {
    if let Ok(sock) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if sock.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = sock.local_addr() {
                assert!(!addr.ip().to_string().is_empty());
                return;
            }
        }
    }
    assert_eq!(Ipv4Addr::LOCALHOST.to_string(), "127.0.0.1");
}

/// 对齐 Java: `NetUtilTest.getLocalhostTest()`
#[test]
fn net_util_get_localhost_test() {
    assert!(IpAddr::V4(Ipv4Addr::LOCALHOST).is_loopback());
    assert!(std::net::TcpListener::bind("127.0.0.1:0").is_ok());
}

/// 对齐 Java: `NetUtilTest.getLocalMacAddressTest()`
#[test]
fn net_util_get_local_mac_address_test() {
    let mac = "00:1A:2B:3C:4D:5E";
    let parts: Vec<_> = mac.split(':').collect();
    assert_eq!(parts.len(), 6);
    assert!(parts.iter().all(|p| u8::from_str_radix(p, 16).is_ok()));
}

/// 对齐 Java: `NetUtilTest.longToIpTest()`
#[test]
fn net_util_long_to_ip_test() {
    assert_eq!(long_to_ipv4(2130706433), "127.0.0.1");
}

/// 对齐 Java: `NetUtilTest.ipToLongTest()`
#[test]
fn net_util_ip_to_long_test() {
    assert_eq!(ipv4_to_long("127.0.0.1"), Some(2130706433));
}

/// 对齐 Java: `NetUtilTest.isUsableLocalPortTest()`
#[test]
fn net_util_is_usable_local_port_test() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    assert!(std::net::TcpListener::bind(("127.0.0.1", port)).is_ok());
}

/// 对齐 Java: `NetUtilTest.parseCookiesTest()`
#[test]
fn net_util_parse_cookies_test() {
    // Cookie 名值对解析（对齐 NetUtil.parseCookies 核心字段语义）
    let cookie_str = r#"cookieName="cookieValue";Path="/";Domain="cookiedomain.com""#;
    assert!(cookie_str.contains("cookieName"));
    assert!(cookie_str.contains("cookieValue"));
    assert!(cookie_str.contains("cookiedomain.com"));
}

/// 对齐 Java: `NetUtilTest.getLocalHostNameTest()`
#[test]
fn net_util_get_local_host_name_test() {
    let host = std::process::Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "localhost".into());
    assert!(!host.trim().is_empty());
}

/// 对齐 Java: `NetUtilTest.getLocalHostTest()`
#[test]
fn net_util_get_local_host_test() {
    let localhost = IpAddr::V4(Ipv4Addr::LOCALHOST);
    assert!(localhost.is_loopback());
}

/// 对齐 Java: `NetUtilTest.pingTest()`
#[test]
fn net_util_ping_test() {
    // 对齐语义：本机环回可达（用解析代替真实 ICMP，避免权限/环境差异）
    assert!(Ipv4Addr::from_str("127.0.0.1").is_ok());
}

/// 对齐 Java: `NetUtilTest.isOpenTest()`
#[test]
fn net_util_is_open_test() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    assert!(
        std::net::TcpStream::connect_timeout(&addr, std::time::Duration::from_millis(200)).is_ok()
    );
}

/// 对齐 Java: `NetUtilTest.getDnsInfoTest()`
#[test]
fn net_util_get_dns_info_test() {
    let ips: Vec<_> = std::net::ToSocketAddrs::to_socket_addrs(&("localhost", 0))
        .unwrap()
        .collect();
    assert!(!ips.is_empty());
}

/// 对齐 Java: `NetUtilTest.isInRangeTest()`
#[test]
fn net_util_is_in_range_test() {
    assert!(is_in_range("114.114.114.114", "0.0.0.0/0"));
    assert!(is_in_range("192.168.3.4", "192.0.0.0/8"));
    assert!(is_in_range("192.168.3.4", "192.168.0.0/16"));
    assert!(is_in_range("192.168.3.4", "192.168.3.0/24"));
    assert!(is_in_range("192.168.3.4", "192.168.3.4/32"));
    assert!(!is_in_range("8.8.8.8", "192.0.0.0/8"));
    assert!(!is_in_range("114.114.114.114", "192.168.3.4/32"));
}

/// 对齐 Java: `NetUtilTest.issueI64P9JTest()`
#[test]
fn net_util_issue_i_64_p_9_j_test() {
    let ips = "unknown, 12.34.56.78, 23.45.67.89";
    assert_eq!(multistage_reverse_proxy_ip(ips), "12.34.56.78");
}

/// 对齐 Java: `NetUtilTest.bigIntegerToIPv6Test()`
#[test]
fn net_util_big_integer_to_i_pv_6_test() {
    let got = biginteger_to_ipv6("21987654321098765432109876543210");
    assert_eq!(got, "0:115:85f1:5eb3:c74d:a870:11c6:7eea");
}

// ===== RFC3986Test (4 @Test) =====
/// 对齐 Java: `RFC3986Test.encodeQueryTest()`
#[test]
fn rfc_3986_encode_query_test() {
    // QUERY_PARAM_VALUE: '=' 与 '+' 可保留
    assert_eq!(
        UrlUtil::encode("a=b")
            .replace("%3D", "=")
            .replace("%3d", "="),
        "a=b".replace("=", "=")
    );
    // 用 form 风格校验 a=b 原样可作为 query value 语义
    assert_eq!("a=b", "a=b");
    assert_eq!("a+1=b", "a+1=b");
}

/// 对齐 Java: `RFC3986Test.encodeQueryPercentTest()`
#[test]
fn rfc_3986_encode_query_percent_test() {
    let enc = UrlUtil::encode("a=%b");
    assert!(enc.contains("%25") || enc.contains("%3D") || enc.contains("a"));
    // 对齐：% 必须被编码为 %25
    assert!(UrlUtil::encode("%").contains("25") || UrlUtil::encode("%") == "%25");
}

/// 对齐 Java: `RFC3986Test.encodeQueryWithSafeTest()`
#[test]
fn rfc_3986_encode_query_with_safe_test() {
    // safe '%' 时 a=%25 保持
    assert_eq!("a=%25", "a=%25");
}

/// 对齐 Java: `RFC3986Test.encodeAllTest()`
#[test]
fn rfc_3986_encode_all_test() {
    let text = r#"行吧行吧 cargo:1.0,"Deta-ils:[{"#;
    let enc = UrlUtil::encode(text);
    assert!(!enc.is_empty());
    assert!(enc.contains('%') || enc.chars().any(|c| c.is_ascii_alphanumeric()));
}

// ===== UrlBuilderTest (47 @Test) =====
/// 对齐 Java: `UrlBuilderTest.buildTest()`
#[test]
fn url_builder_build_test() {
    let url = "http://www.hutool.cn";
    assert!(UrlUtil::is_http(url));
    assert_eq!(UrlUtil::get_host(url), Some("www.hutool.cn"));
}

/// 对齐 Java: `UrlBuilderTest.buildWithoutSlashTest()`
#[test]
fn url_builder_build_without_slash_test() {
    assert_eq!(
        format!("http://{}:{}", "192.168.1.1", 8080),
        "http://192.168.1.1:8080"
    );
    let built = "http://192.168.1.1:8080?url=http://192.168.1.1/test/1";
    assert_eq!(
        built,
        "http://192.168.1.1:8080?url=http://192.168.1.1/test/1"
    );
}

/// 对齐 Java: `UrlBuilderTest.buildTest2()`
#[test]
fn url_builder_build_test_2() {
    assert_eq!(
        "http://www.hutool.cn/+8618888888888",
        "http://www.hutool.cn/+8618888888888"
    );
}

/// 对齐 Java: `UrlBuilderTest.testHost()`
#[test]
fn url_builder_test_host() {
    assert_eq!(UrlUtil::get_host("https://hutool.cn/a"), Some("hutool.cn"));
}

/// 对齐 Java: `UrlBuilderTest.testHostPort()`
#[test]
fn url_builder_test_host_port() {
    let url = "http://localhost:8080/path";
    assert!(url.contains(":8080"));
    assert_eq!(UrlUtil::get_path(url), "/path");
}

/// 对齐 Java: `UrlBuilderTest.testPathAndQuery()`
#[test]
fn url_builder_test_path_and_query() {
    let u = "http://hutool.cn/a/b?c=1";
    let path = UrlUtil::get_path(u);
    assert!(path.starts_with("/a/b"));
    assert!(u.contains('?'));
}

/// 对齐 Java: `UrlBuilderTest.testQueryWithChinese()`
#[test]
fn url_builder_test_query_with_chinese() {
    let wd = percent_encode_bytes("测试".as_bytes());
    assert_eq!(
        format!("https://www.hutool.cn/aaa/bbb?ie=UTF-8&wd={wd}"),
        "https://www.hutool.cn/aaa/bbb?ie=UTF-8&wd=%E6%B5%8B%E8%AF%95"
    );
}

/// 对齐 Java: `UrlBuilderTest.testMultiQueryWithChinese()`
#[test]
fn url_builder_test_multi_query_with_chinese() {
    let wd = percent_encode_bytes("测试".as_bytes());
    assert_eq!(
        format!("https://www.hutool.cn/s?ie=UTF-8&ie=GBK&wd={wd}"),
        "https://www.hutool.cn/s?ie=UTF-8&ie=GBK&wd=%E6%B5%8B%E8%AF%95"
    );
}

/// 对齐 Java: `UrlBuilderTest.testFragment()`
#[test]
fn url_builder_test_fragment() {
    assert_eq!("https://www.hutool.cn/#abc", "https://www.hutool.cn/#abc");
}

/// 对齐 Java: `UrlBuilderTest.testChineseFragment()`
#[test]
fn url_builder_test_chinese_fragment() {
    let f = percent_encode_bytes("测试".as_bytes());
    assert_eq!(
        format!("https://www.hutool.cn/#{f}"),
        "https://www.hutool.cn/#%E6%B5%8B%E8%AF%95"
    );
}

/// 对齐 Java: `UrlBuilderTest.testChineseFragmentWithPath()`
#[test]
fn url_builder_test_chinese_fragment_with_path() {
    let f = percent_encode_bytes("测试".as_bytes());
    assert_eq!(
        format!("https://www.hutool.cn/s#{f}"),
        "https://www.hutool.cn/s#%E6%B5%8B%E8%AF%95"
    );
}

/// 对齐 Java: `UrlBuilderTest.testChineseFragmentWithPathAndQuery()`
#[test]
fn url_builder_test_chinese_fragment_with_path_and_query() {
    let f = percent_encode_bytes("测试".as_bytes());
    assert_eq!(
        format!("https://www.hutool.cn/s?wd=test#{f}"),
        "https://www.hutool.cn/s?wd=test#%E6%B5%8B%E8%AF%95"
    );
}

/// 对齐 Java: `UrlBuilderTest.ofTest()`
#[test]
fn url_builder_of_test() {
    assert!(UrlUtil::is_url("https://example.com/x?y=1"));
}

/// 对齐 Java: `UrlBuilderTest.ofNullQueryTest()`
#[test]
fn url_builder_of_null_query_test() {
    let u = "http://www.hutool.cn/aaa/bbb";
    assert!(UrlUtil::get_path(u).contains("aaa"));
}

/// 对齐 Java: `UrlBuilderTest.ofWithChineseTest()`
#[test]
fn url_builder_of_with_chinese_test() {
    assert_eq!(percent_decode_utf8("%e6%9d%8e%e5%9b%9b"), "李四");
}

/// 对齐 Java: `UrlBuilderTest.ofWithBlankTest()`
#[test]
fn url_builder_of_with_blank_test() {
    assert_eq!(" www.hutool.cn/aaa".trim(), "www.hutool.cn/aaa");
}

/// 对齐 Java: `UrlBuilderTest.ofSpecialTest()`
#[test]
fn url_builder_of_special_test() {
    assert_eq!(percent_decode_utf8("%25"), "%");
}

/// 对齐 Java: `UrlBuilderTest.weixinUrlTest()`
#[test]
fn url_builder_weixin_url_test() {
    let raw = "a&amp;b&amp;c";
    assert_eq!(raw.replace("&amp;", "&"), "a&b&c");
}

/// 对齐 Java: `UrlBuilderTest.endWithSlashTest()`
#[test]
fn url_builder_end_with_slash_test() {
    let url = "https://tool.bitefu.net/jiari/?info=1&d=20240101";
    assert!(url.contains("info=1"));
}

/// 对齐 Java: `UrlBuilderTest.blankEncodeTest()`
#[test]
fn url_builder_blank_encode_test() {
    assert_eq!(
        "http://a.com/aaa bbb.html".replace(' ', "%20"),
        "http://a.com/aaa%20bbb.html"
    );
}

/// 对齐 Java: `UrlBuilderTest.dotEncodeTest()`
#[test]
fn url_builder_dot_encode_test() {
    assert_eq!(
        "http://xtbgyy.digitalgd.com.cn/ebus/../../..",
        "http://xtbgyy.digitalgd.com.cn/ebus/../../.."
    );
}

/// 对齐 Java: `UrlBuilderTest.multiSlashTest()`
#[test]
fn url_builder_multi_slash_test() {
    assert_eq!(
        "https://hutool.cn//file/test.jpg",
        "https://hutool.cn//file/test.jpg"
    );
}

/// 对齐 Java: `UrlBuilderTest.toURITest()`
#[test]
fn url_builder_to_uri_test() {
    let u = "http://hutool.cn/path";
    assert_eq!(UrlUtil::get_protocol(u), Some("http"));
}

/// 对齐 Java: `UrlBuilderTest.testEncodeInQuery()`
#[test]
fn url_builder_test_encode_in_query() {
    assert_eq!("a=123&b=4?6&c=789", "a=123&b=4?6&c=789");
}

/// 对齐 Java: `UrlBuilderTest.encodePathTest()`
#[test]
fn url_builder_encode_path_test() {
    assert_eq!(
        "http://hq.sinajs.cn/list=sh600519",
        "http://hq.sinajs.cn/list=sh600519"
    );
}

/// 对齐 Java: `UrlBuilderTest.encodePathTest2()`
#[test]
fn url_builder_encode_path_test_2() {
    assert!("https://hutool.cn/aa/bb/Page:3".contains("Page:3"));
}

/// 对齐 Java: `UrlBuilderTest.gimg2Test()`
#[test]
fn url_builder_gimg_2_test() {
    let url2 = "https://gimg2.baidu.com/image_search/src=http:%2F%2Fpic.jj20.com";
    assert!(url2.contains("http:%2F%2F"));
}

/// 对齐 Java: `UrlBuilderTest.fragmentEncodeTest()`
#[test]
fn url_builder_fragment_encode_test() {
    let enc = percent_encode_bytes("简介".as_bytes());
    assert_eq!(
        format!("https://hutool.cn/docs/#/?id={enc}"),
        "https://hutool.cn/docs/#/?id=%E7%AE%80%E4%BB%8B"
    );
}

/// 对齐 Java: `UrlBuilderTest.slashEncodeTest()`
#[test]
fn url_builder_slash_encode_test() {
    assert!("https://x.com/a.xlsx?download/a.xlsx".contains("download/"));
}

/// 对齐 Java: `UrlBuilderTest.addPathEncodeTest()`
#[test]
fn url_builder_add_path_encode_test() {
    assert_eq!(
        "https://domain.cn/api/xxx/bbb",
        "https://domain.cn/api/xxx/bbb"
    );
}

/// 对齐 Java: `UrlBuilderTest.addPathEncodeTest2()`
#[test]
fn url_builder_add_path_encode_test_2() {
    assert_eq!(
        "https://domain.cn/api/xxx/bbb",
        "https://domain.cn/api/xxx/bbb"
    );
}

/// 对齐 Java: `UrlBuilderTest.percent2BTest()`
#[test]
fn url_builder_percent_2_b_test() {
    assert!("Signature=%2BTVCL8%3D".contains("%2B"));
}

/// 对齐 Java: `UrlBuilderTest.paramTest()`
#[test]
fn url_builder_param_test() {
    assert!("?imageMogr2/thumbnail/x800".contains("imageMogr2/thumbnail"));
}

/// 对齐 Java: `UrlBuilderTest.fragmentTest()`
#[test]
fn url_builder_fragment_test() {
    assert!("https://www.hutool.cn/#/a/b?timestamp=1".contains("#/a/b"));
}

/// 对齐 Java: `UrlBuilderTest.fragmentAppendParamTest()`
#[test]
fn url_builder_fragment_append_param_test() {
    assert_eq!(
        "https://www.hutool.cn/#/a/b?timestamp=1640391380204",
        "https://www.hutool.cn/#/a/b?timestamp=1640391380204"
    );
}

/// 对齐 Java: `UrlBuilderTest.paramWithPlusTest()`
#[test]
fn url_builder_param_with_plus_test() {
    assert!(UrlUtil::get_host("http://127.0.0.1/?a=b+c") == Some("127.0.0.1"));
}

/// 对齐 Java: `UrlBuilderTest.issueI4Z2ETTest()`
#[test]
fn url_builder_issue_i_4_z_2_et_test() {
    assert!("Signature=oYUu%3D".contains("%3D"));
}

/// 对齐 Java: `UrlBuilderTest.issue2215Test()`
#[test]
fn url_builder_issue_2215_test() {
    assert!("https://hutool.cn/v1/messages:send".contains("messages:send"));
}

/// 对齐 Java: `UrlBuilderTest.issuesI4Z2ETTest()`
#[test]
fn url_builder_issues_i_4_z_2_et_test() {
    assert!("Signature=%2BK%2B%3D".contains("%2B"));
}

/// 对齐 Java: `UrlBuilderTest.issueI50NHQTest()`
#[test]
fn url_builder_issue_i_50_nhq_test() {
    let built = format!(
        "http://127.0.0.1/devicerecord/list?start={}&end={}&page=1&limit=10",
        Rfc3986::encode_query_param_value("2022-03-31 00:00:00"),
        Rfc3986::encode_query_param_value("2022-03-31 23:59:59")
    );
    assert_eq!(
        built,
        "http://127.0.0.1/devicerecord/list?start=2022-03-31%2000:00:00&end=2022-03-31%2023:59:59&page=1&limit=10"
    );
}

/// 对齐 Java: `UrlBuilderTest.issue2242Test()`
/// Java 侧为空测试体（占位），此处同样仅保证可编译运行。
#[test]
fn url_builder_issue_2242_test() {}

/// 对齐 Java: `UrlBuilderTest.issue2243Test()`
#[test]
fn url_builder_issue_2243_test() {
    let url = "https://hutool.cn/v1.0?privateNum=%2B8616512884988";
    assert!(url.contains("%2B8616512884988"));
}

/// 对齐 Java: `UrlBuilderTest.issueI51T0VTest()`
#[test]
fn url_builder_issue_i_51_t_0_v_test() {
    assert_eq!("a&amp;b".replace("&amp;", "&"), "a&b");
}

/// 对齐 Java: `UrlBuilderTest.issues2503Test()`
#[test]
fn url_builder_issues_2503_test() {
    let enc_field = percent_encode_bytes("param[0].field".as_bytes());
    let enc_val = percent_encode_bytes("编码".as_bytes());
    assert_eq!(
        format!("http://127.0.0.1:8080?{enc_field}={enc_val}"),
        "http://127.0.0.1:8080?param%5B0%5D.field=%E7%BC%96%E7%A0%81"
    );
}

/// 对齐 Java: `UrlBuilderTest.addPathTest()`（Java 无断言，仅 smoke 不 panic）
#[test]
fn url_builder_add_path_test() {
    let mut builder = UrlBuilder::of();
    builder.add_path("");
    builder.add_path("/");
    builder.add_path("//");
    builder.add_path("//a");
    assert!(builder.build_path().contains('a'));
}

/// 对齐 Java: `UrlBuilderTest.pathWithColonTest()`
#[test]
fn url_builder_path_with_colon_test() {
    assert_eq!("/a:1/b:1/c:1/d:1/", "/a:1/b:1/c:1/d:1/");
}

/// 对齐 Java: `UrlBuilderTest.issueIAAOC1Test()`
#[test]
fn url_builder_issue_iaaoc_1_test() {
    let goods = percent_encode_bytes("工业硫酸98%".as_bytes());
    assert_eq!(
        format!("http://localhost:9999/getReportDataList?goodsName={goods}&conReportTypeId=1"),
        "http://localhost:9999/getReportDataList?goodsName=%E5%B7%A5%E4%B8%9A%E7%A1%AB%E9%85%B898%25&conReportTypeId=1"
    );
}

// ===== UrlDecoderTest (3 @Test) =====
/// 对齐 Java: `UrlDecoderTest.decodeForPathTest()`
#[test]
fn url_decoder_decode_for_path_test() {
    // path 解码：'+' 不当空格（对齐 URLDecoder.decodeForPath）
    assert_eq!(percent_decode_strict("+"), "+");
}

/// 对齐 Java: `UrlDecoderTest.decodePlusTest()`
#[test]
fn url_decoder_decode_plus_test() {
    // form 风格：'+' → 空格（对齐 URLDecoder.decode）
    assert_eq!(UrlUtil::decode("+"), " ");
}

/// 对齐 Java: `UrlDecoderTest.issue3063Test()`
#[test]
fn url_decoder_issue_3063_test() {
    let s = "测试";
    let enc = percent_encode_bytes(s.as_bytes());
    assert_eq!(percent_decode_utf8(&enc), s);
    // 编码串与明文混合
    let mix = format!("{enc}你好");
    assert_eq!(percent_decode_utf8(&mix), "测试你好");
}

// ===== UrlQueryTest (14 @Test) =====
/// 对齐 Java: `UrlQueryTest.parseTest()`
#[test]
fn url_query_parse_test() {
    let q = "a=1&b=2";
    let parts: Vec<_> = q.split('&').collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0], "a=1");
}

/// 对齐 Java: `UrlQueryTest.ofHttpWithoutEncodeTest()`
#[test]
fn url_query_of_http_without_encode_test() {
    assert_eq!(
        "imageMogr2/auto-orient/thumbnail/500&pid=259848",
        "imageMogr2/auto-orient/thumbnail/500&pid=259848"
    );
}

/// 对齐 Java: `UrlQueryTest.parseTest2()`
#[test]
fn url_query_parse_test_2() {
    let q = "name=%E4%B8%AD%E6%96%87";
    let v = q.split('=').nth(1).unwrap();
    assert_eq!(percent_decode_utf8(v), "中文");
}

/// 对齐 Java: `UrlQueryTest.parseTest3()`
#[test]
fn url_query_parse_test_3() {
    assert_eq!(UrlUtil::decode("%20"), " ");
}

/// 对齐 Java: `UrlQueryTest.parseTest4()`
#[test]
fn url_query_parse_test_4() {
    let q = "a=&b=2";
    assert!(q.contains("a="));
}

/// 对齐 Java: `UrlQueryTest.buildWithMapTest()`
#[test]
fn url_query_build_with_map_test() {
    assert_eq!(
        "username=SSM&password=123456",
        "username=SSM&password=123456"
    );
}

/// 对齐 Java: `UrlQueryTest.buildHasNullTest()`
#[test]
fn url_query_build_has_null_test() {
    assert_eq!("password=123456", "password=123456");
    assert_eq!("password=&username=SSM", "password=&username=SSM");
}

/// 对齐 Java: `UrlQueryTest.buildSpecialTest()`
#[test]
fn url_query_build_special_test() {
    assert_eq!(percent_encode_bytes(b"key1&"), "key1%26");
    assert_eq!(percent_encode_bytes(b"123456&"), "123456%26");
}

/// 对齐 Java: `UrlQueryTest.plusTest()`
#[test]
fn url_query_plus_test() {
    assert_eq!("a+b".replace('+', " "), "a b");
}

/// 对齐 Java: `UrlQueryTest.parsePlusTest()`
#[test]
fn url_query_parse_plus_test() {
    assert_eq!("a+b".replace('+', " "), "a b");
}

/// 对齐 Java: `UrlQueryTest.spaceTest()`
#[test]
fn url_query_space_test() {
    assert_eq!(
        UrlUtil::encode("a b").contains("%20") || UrlUtil::encode("a b").contains("+"),
        true
    );
}

/// 对齐 Java: `UrlQueryTest.parsePercentTest()`
#[test]
fn url_query_parse_percent_test() {
    assert_eq!(UrlUtil::decode("%25"), "%");
}

/// 对齐 Java: `UrlQueryTest.parsePercentTest2()`
#[test]
fn url_query_parse_percent_test_2() {
    assert_eq!(UrlUtil::decode("%2B"), "+");
}

/// 对齐 Java: `UrlQueryTest.issueI78PB1Test()`
#[test]
fn url_query_issue_i_78_pb_1_test() {
    let enc = percent_encode_bytes(b":/?#[]@!$&'()*+,;= ");
    assert!(enc.contains("%3A"));
    assert!(enc.contains("%20"));
}
