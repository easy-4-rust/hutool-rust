//! 对齐: `cn.hutool.core.lang.Version`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Version.java
//!
//! 移植自 `java.lang.module.ModuleDescriptor.Version` 风格的三段解析：
//! sequence / pre / build，数字按数值比较，字符串按字典序。

#![allow(dead_code)] // 对齐 Java Version，暂未接线，预留

use std::cmp::Ordering;
use std::fmt;

/// 版本号 token：数字或字符串片段。
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Num(i32),
    Str(String),
}

impl Token {
    fn as_cmp_str(&self) -> String {
        match self {
            Token::Num(n) => n.to_string(),
            Token::Str(s) => s.clone(),
        }
    }
}

/// 对齐 Java: `cn.hutool.core.lang.Version`
#[derive(Debug, Clone)]
pub struct Version {
    raw: String,
    sequence: Vec<Token>,
    pre: Vec<Token>,
    build: Vec<Token>,
}

impl Version {
    /// 对齐 Java: `Version.of(String)` / `new Version(String)`
    pub fn of(v: impl Into<String>) -> Self {
        Self::parse(v.into())
    }

    /// 解析版本字符串。
    fn parse(v: String) -> Self {
        let n = v.len();
        let bytes = v.as_bytes();
        let mut sequence = Vec::with_capacity(4);
        let mut pre = Vec::with_capacity(2);
        let mut build = Vec::with_capacity(2);

        if n == 0 {
            return Self {
                raw: v,
                sequence,
                pre,
                build,
            };
        }

        let mut i = 0usize;
        let mut c = bytes[i] as char;

        // 主版本 sequence
        i = take_number(&v, i, &mut sequence);
        while i < n {
            c = bytes[i] as char;
            if c == '.' {
                i += 1;
                continue;
            }
            if c == '-' || c == '+' {
                i += 1;
                break;
            }
            if c.is_ascii_digit() {
                i = take_number(&v, i, &mut sequence);
            } else {
                i = take_string(&v, i, &mut sequence);
            }
        }

        if c == '-' && i >= n {
            return Self {
                raw: v,
                sequence,
                pre,
                build,
            };
        }

        // 次版本 pre
        while i < n {
            c = bytes[i] as char;
            if c.is_ascii_digit() {
                i = take_number(&v, i, &mut pre);
            } else {
                i = take_string(&v, i, &mut pre);
            }
            if i >= n {
                break;
            }
            c = bytes[i] as char;
            if c == '.' || c == '-' {
                i += 1;
                continue;
            }
            if c == '+' {
                i += 1;
                break;
            }
        }

        if c == '+' && i >= n {
            return Self {
                raw: v,
                sequence,
                pre,
                build,
            };
        }

        // build
        while i < n {
            c = bytes[i] as char;
            if c.is_ascii_digit() {
                i = take_number(&v, i, &mut build);
            } else {
                i = take_string(&v, i, &mut build);
            }
            if i >= n {
                break;
            }
            c = bytes[i] as char;
            if c == '.' || c == '-' || c == '+' {
                i += 1;
            }
        }

        Self {
            raw: v,
            sequence,
            pre,
            build,
        }
    }

    /// 对齐 Java: `compareTo`
    pub fn compare_to(&self, other: &Version) -> i32 {
        let c = compare_tokens(&self.sequence, &other.sequence);
        if c != 0 {
            return c;
        }
        match (self.pre.is_empty(), other.pre.is_empty()) {
            (true, false) => return 1,
            (false, true) => return -1,
            _ => {}
        }
        let c = compare_tokens(&self.pre, &other.pre);
        if c != 0 {
            return c;
        }
        compare_tokens(&self.build, &other.build)
    }

    /// 原始版本字符串。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.compare_to(other) == 0
    }
}
impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_to(other).cmp(&0)
    }
}

impl std::hash::Hash for Version {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.raw)
    }
}

/// 从位置 `i` 取连续数字 token。
fn take_number(s: &str, mut i: usize, acc: &mut Vec<Token>) -> usize {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut d = (bytes[i] - b'0') as i32;
    i += 1;
    while i < n {
        let c = bytes[i];
        if c.is_ascii_digit() {
            d = d * 10 + (c - b'0') as i32;
            i += 1;
            continue;
        }
        break;
    }
    acc.push(Token::Num(d));
    i
}

/// 从位置 `i` 取字符串 token（遇 `.`/`-`/`+`/数字结束）。
fn take_string(s: &str, mut i: usize, acc: &mut Vec<Token>) -> usize {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let start = i;
    i += 1;
    while i < n {
        let c = bytes[i] as char;
        if c != '.' && c != '-' && c != '+' && !c.is_ascii_digit() {
            i += 1;
            continue;
        }
        break;
    }
    acc.push(Token::Str(s[start..i].to_string()));
    i
}

/// 比较两组 token（对齐 Java `compareTokens`）。
fn compare_tokens(ts1: &[Token], ts2: &[Token]) -> i32 {
    let n = ts1.len().min(ts2.len());
    for i in 0..n {
        let o1 = &ts1[i];
        let o2 = &ts2[i];
        let c = match (o1, o2) {
            (Token::Num(a), Token::Num(b)) => a.cmp(b),
            (Token::Str(a), Token::Str(b)) => a.cmp(b),
            _ => o1.as_cmp_str().cmp(&o2.as_cmp_str()),
        };
        if c != Ordering::Equal {
            return match c {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            };
        }
    }
    if ts1.len() == ts2.len() {
        return 0;
    }
    let rest = if ts1.len() > ts2.len() {
        &ts1[n..]
    } else {
        &ts2[n..]
    };
    for o in rest {
        if matches!(o, Token::Num(0)) {
            continue;
        }
        return ts1.len() as i32 - ts2.len() as i32;
    }
    0
}

#[cfg(test)]
mod version_idiomatic_parity {
    use super::*;

    /// 对齐 Java Version.of/compareTo/equals 可执行证据。
    #[test]
    fn version_of_compare_and_equals() {
        let a = Version::of("1.2.3");
        let b = Version::of("1.2.4");
        let c = Version::of("1.2.3");
        assert!(a.compare_to(&b) < 0);
        assert!(b.compare_to(&a) > 0);
        assert_eq!(a.compare_to(&c), 0);
        assert_eq!(a, c);
        assert_ne!(a, b);
        assert_eq!(a.to_string(), "1.2.3");
        let pre = Version::of("1.0.0-alpha");
        let release = Version::of("1.0.0");
        assert!(pre.compare_to(&release) < 0);
    }
}
