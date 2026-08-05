//! 对齐: `cn.hutool.http` (响应模块)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpResponse.java
//! 中文说明: HTTP响应模块，包含HttpResponse和HttpCookie实现

mod http_cookie;
mod http_response;

pub use http_cookie::HttpCookie;
pub use http_response::HttpResponse;

fn parse_set_cookie(header: &str) -> Option<HttpCookie> {
    let pair = header.split(';').next()?.trim();
    let (name, value) = pair.split_once('=')?;
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    Some(HttpCookie::new(name, value.trim()))
}

fn filename_from_dispositions(dispositions: &[&str], param_name: &str) -> Option<String> {
    let needle = format!("{param_name}=");
    for disposition in dispositions {
        if let Some(idx) = disposition
            .to_ascii_lowercase()
            .find(&needle.to_ascii_lowercase())
        {
            let rest = disposition[idx + needle.len()..].trim();
            let end = rest.find(';').unwrap_or(rest.len());
            let value = rest[..end].trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn decode_rfc5987(raw: &str) -> String {
    let raw = strip_quotes(raw);
    // charset'lang'value — split on the first two apostrophes
    let parts: Vec<&str> = raw.splitn(3, '\'').collect();
    if parts.len() == 3 {
        return percent_decode_lightweight(parts[2]);
    }
    raw
}

fn strip_quotes(value: &str) -> String {
    value.trim().trim_matches(['\'', '"']).to_string()
}

fn percent_decode_lightweight(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (from_hex(bytes[i + 1]), from_hex(bytes[i + 2])) {
                out.push((hi << 4) | lo);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn from_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
