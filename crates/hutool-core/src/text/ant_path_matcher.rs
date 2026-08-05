//! 对齐: `cn.hutool.core.text.AntPathMatcher`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/AntPathMatcher.java
//!
//! Ant 风格路径匹配（`?` `*` `**` `{var}`）。

use crate::{CoreError, Result};
use indexmap::IndexMap;
use regex::Regex;
use std::sync::Mutex;

/// 对齐 Java: `AntPathMatcher#DEFAULT_PATH_SEPARATOR`
pub const DEFAULT_PATH_SEPARATOR: &str = "/";

/// 对齐 Java: `AntPathMatcher#`
#[derive(Debug)]
pub struct AntPathMatcher {
    path_separator: String,
    case_sensitive: bool,
    trim_tokens: bool,
    cache_patterns: bool,
    cache: Mutex<IndexMap<String, Regex>>,
}

impl Default for AntPathMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for AntPathMatcher {
    fn clone(&self) -> Self {
        Self {
            path_separator: self.path_separator.clone(),
            case_sensitive: self.case_sensitive,
            trim_tokens: self.trim_tokens,
            cache_patterns: self.cache_patterns,
            cache: Mutex::new(IndexMap::new()),
        }
    }
}

impl AntPathMatcher {
    /// 对齐 Java: `AntPathMatcher()`
    pub fn new() -> Self {
        Self::with_separator(DEFAULT_PATH_SEPARATOR)
    }

    /// 对齐 Java: `AntPathMatcher(String pathSeparator)`
    pub fn with_separator(sep: &str) -> Self {
        Self {
            path_separator: sep.to_string(),
            case_sensitive: true,
            trim_tokens: false,
            cache_patterns: true,
            cache: Mutex::new(IndexMap::new()),
        }
    }

    /// 对齐 Java: `AntPathMatcher::setPathSeparator`
    pub fn set_path_separator(&mut self, sep: &str) -> Result<&mut Self> {
        self.path_separator = sep.to_string();
        Ok(self)
    }

    /// 对齐 Java: `AntPathMatcher::setCaseSensitive`
    pub fn set_case_sensitive(&mut self, sensitive: bool) -> Result<&mut Self> {
        self.case_sensitive = sensitive;
        Ok(self)
    }

    /// 对齐 Java: `AntPathMatcher::setTrimTokens`
    pub fn set_trim_tokens(&mut self, trim: bool) -> Result<&mut Self> {
        self.trim_tokens = trim;
        Ok(self)
    }

    /// 对齐 Java: `AntPathMatcher::setCachePatterns`
    pub fn set_cache_patterns(&mut self, cache: bool) -> Result<&mut Self> {
        self.cache_patterns = cache;
        Ok(self)
    }

    /// 对齐 Java: `AntPathMatcher::isPattern`
    pub fn is_pattern(&self, path: &str) -> Result<bool> {
        Ok(path.contains('*') || path.contains('?') || path.contains('{'))
    }

    /// 对齐 Java: `AntPathMatcher::match`
    pub fn match_path(&self, pattern: &str, path: &str) -> Result<bool> {
        self.do_match(pattern, path, true, None)
    }

    /// 对齐 Java: `AntPathMatcher::matchStart`
    pub fn match_start(&self, pattern: &str, path: &str) -> Result<bool> {
        self.do_match(pattern, path, false, None)
    }

    /// 对齐 Java: `AntPathMatcher::extractPathWithinPattern`
    pub fn extract_path_within_pattern(&self, _pattern: &str, path: &str) -> Result<String> {
        Ok(path.trim_start_matches(&self.path_separator).to_string())
    }

    /// 对齐 Java: `AntPathMatcher::extractUriTemplateVariables`
    pub fn extract_uri_template_variables(
        &self,
        pattern: &str,
        path: &str,
    ) -> Result<IndexMap<String, String>> {
        let mut vars = IndexMap::new();
        let matched = self.do_match(pattern, path, true, Some(&mut vars))?;
        if !matched {
            vars.clear();
        }
        Ok(vars)
    }

    /// 对齐 Java: `AntPathMatcher::combine`
    pub fn combine(&self, pattern1: &str, pattern2: &str) -> Result<String> {
        if pattern1.is_empty() {
            return Ok(pattern2.to_string());
        }
        if pattern2.is_empty() {
            return Ok(pattern1.to_string());
        }
        Ok(format!(
            "{}{}{}",
            pattern1.trim_end_matches('/'),
            self.path_separator,
            pattern2.trim_start_matches('/')
        ))
    }

    fn do_match(
        &self,
        pattern: &str,
        path: &str,
        _full_match: bool,
        vars: Option<&mut IndexMap<String, String>>,
    ) -> Result<bool> {
        if pattern == path {
            return Ok(true);
        }
        let re = self.compile(pattern)?;
        let Some(caps) = re.captures(path) else {
            return Ok(false);
        };
        if caps.get(0).map(|m| m.as_str()) != Some(path) {
            return Ok(false);
        }
        if let Some(out) = vars {
            for name in re.capture_names().flatten() {
                if let Some(m) = caps.name(name) {
                    out.insert(name.to_string(), m.as_str().to_string());
                }
            }
        }
        Ok(true)
    }

    fn compile(&self, pattern: &str) -> Result<Regex> {
        if self.cache_patterns {
            if let Ok(cache) = self.cache.lock() {
                if let Some(re) = cache.get(pattern) {
                    return Ok(re.clone());
                }
            }
        }
        let src = ant_pattern_to_regex(pattern, &self.path_separator, self.case_sensitive);
        let re = Regex::new(&src).map_err(|e| CoreError::Codec(format!("ant pattern: {e}")))?;
        if self.cache_patterns {
            if let Ok(mut cache) = self.cache.lock() {
                cache.insert(pattern.to_string(), re.clone());
            }
        }
        Ok(re)
    }
}

fn ant_pattern_to_regex(pattern: &str, sep: &str, case_sensitive: bool) -> String {
    let mut out = String::from("^");
    if !case_sensitive {
        out.push_str("(?i)");
    }
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '*' {
            if i + 1 < chars.len() && chars[i + 1] == '*' {
                i += 1; // consume second *
                // If previous emitted char was separator, make "sep + **" optional empty
                // Pattern like /foo/bar/** should match /foo/bar
                let preceded_by_sep = out.ends_with(&regex_escape(sep));
                if preceded_by_sep {
                    // remove trailing sep from out and emit (?:/.*)?
                    let esc = regex_escape(sep);
                    out.truncate(out.len() - esc.len());
                    out.push_str(&format!("(?:{}.*)?", esc));
                } else if i + 1 < chars.len() && chars[i + 1].to_string() == sep {
                    i += 1;
                    out.push_str("(?:.*)?");
                } else {
                    out.push_str(".*");
                }
            } else {
                out.push_str(&format!("[^{}]*", regex_escape(sep)));
            }
        } else if c == '?' {
            out.push_str(&format!("[^{}]", regex_escape(sep)));
        } else if c == '{' {
            let mut body = String::new();
            i += 1;
            while i < chars.len() && chars[i] != '}' {
                body.push(chars[i]);
                i += 1;
            }
            let (name, custom) = match body.split_once(':') {
                Some((n, r)) => (sanitize_group(n), Some(r.to_string())),
                None => (sanitize_group(&body), None),
            };
            if let Some(r) = custom {
                out.push_str(&format!("(?P<{name}>{r})"));
            } else {
                out.push_str(&format!("(?P<{name}>[^{}]+?)", regex_escape(sep)));
            }
        } else {
            out.push_str(&regex_escape(&c.to_string()));
        }
        i += 1;
    }
    out.push('$');
    out
}

fn regex_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if matches!(
            c,
            '.' | '+' | '*' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' | '^' | '$'
        ) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn sanitize_group(name: &str) -> String {
    let mut s: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if s.is_empty() || s.chars().next().unwrap().is_ascii_digit() {
        s = format!("g_{s}");
    }
    s
}
