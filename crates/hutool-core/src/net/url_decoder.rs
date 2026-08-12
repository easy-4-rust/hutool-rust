//! 对齐: `cn.hutool.core.net.URLDecoder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/URLDecoder.java

use encoding_rs::UTF_8;

use crate::char_util::CharUtil;

/// 对齐 Java 类: `cn.hutool.core.net.URLDecoder`
#[derive(Debug, Clone, Copy, Default)]
pub struct UrlDecoder;

impl UrlDecoder {
    /// 对齐 Java: `URLDecoder.decode(String, Charset)`
    ///
    /// 规则见: https://url.spec.whatwg.org/#urlencoded-parsing
    pub fn decode(content: &str) -> String {
        Self::decode_with_plus(content, true)
    }

    /// 对齐 Java: `URLDecoder.decode(String, Charset, boolean isPlusToSpace)`
    pub fn decode_with_plus(content: &str, is_plus_to_space: bool) -> String {
        if content.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(content.len());
        let mut begin = 0usize;
        let chars: Vec<char> = content.chars().collect();
        let length = chars.len();

        for (index, ch) in chars.iter().copied().enumerate() {
            if ch == '%' || CharUtil::is_hex_char(ch) {
                continue;
            }

            if index > begin {
                result.push_str(&Self::decode_sub(&chars[begin..index], is_plus_to_space));
            }

            let mut output = ch;
            if output == '+' && is_plus_to_space {
                output = ' ';
            }
            result.push(output);
            begin = index + 1;
        }

        if begin < length {
            result.push_str(&Self::decode_sub(&chars[begin..length], is_plus_to_space));
        }

        result
    }

    /// 对齐 Java: `URLDecoder.decodeForPath(String, Charset)`
    pub fn decode_for_path(content: &str) -> String {
        Self::decode_with_plus(content, false)
    }

    /// 对齐 Java: `URLDecoder.decodeSub(...)`
    fn decode_sub(segment: &[char], is_plus_to_space: bool) -> String {
        let bytes: Vec<u8> = segment
            .iter()
            .map(|ch| u8::try_from(*ch as u32).unwrap_or(b'?'))
            .collect();
        let decoded = Self::decode_bytes(&bytes, is_plus_to_space);
        UTF_8.decode(&decoded).0.into_owned()
    }

    /// 对齐 Java: `URLDecoder.decode(byte[], boolean isPlusToSpace)`
    fn decode_bytes(bytes: &[u8], is_plus_to_space: bool) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(bytes.len());
        let mut index = 0usize;
        while index < bytes.len() {
            let byte = bytes[index];
            if byte == b'+' {
                buffer.push(if is_plus_to_space { b' ' } else { b'+' });
            } else if byte == b'%' {
                if index + 1 < bytes.len() {
                    let upper = CharUtil::digit16(u32::from(bytes[index + 1]));
                    if upper >= 0 && index + 2 < bytes.len() {
                        let lower = CharUtil::digit16(u32::from(bytes[index + 2]));
                        if lower >= 0 {
                            buffer.push(((upper << 4) + lower) as u8);
                            index += 3;
                            continue;
                        }
                    }
                }
                buffer.push(byte);
            } else {
                buffer.push(byte);
            }
            index += 1;
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_plus_and_percent() {
        assert_eq!(UrlDecoder::decode("hello+world"), "hello world");
        assert_eq!(UrlDecoder::decode("a%20b"), "a b");
    }
}
