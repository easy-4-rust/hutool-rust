//! 对齐: `cn.hutool.http.HTMLFilter`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HTMLFilter.java
//! 中文说明: XSS过滤器，通过标签白名单、属性过滤和实体验证防止跨站脚本攻击

use fancy_regex::{Captures, Regex};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Builds a compiled regex with optional case-insensitivity and dot-all mode.
fn regex_build(pattern: &str, case_insensitive: bool, dotall: bool) -> Regex {
    let mut flags = String::new();
    if case_insensitive {
        flags.push('i');
    }
    if dotall {
        flags.push('s');
    }
    let wrapped = if flags.is_empty() {
        pattern.to_string()
    } else {
        format!("(?{flags}){pattern}")
    };
    Regex::new(&wrapped).unwrap_or_else(|err| panic!("invalid HTML filter regex: {err}"))
}

static P_COMMENTS: OnceLock<Regex> = OnceLock::new();
static P_COMMENT: OnceLock<Regex> = OnceLock::new();
static P_TAGS: OnceLock<Regex> = OnceLock::new();
static P_END_TAG: OnceLock<Regex> = OnceLock::new();
static P_START_TAG: OnceLock<Regex> = OnceLock::new();
static P_QUOTED_ATTRIBUTES: OnceLock<Regex> = OnceLock::new();
static P_UNQUOTED_ATTRIBUTES: OnceLock<Regex> = OnceLock::new();
static P_PROTOCOL: OnceLock<Regex> = OnceLock::new();
static P_ENTITY: OnceLock<Regex> = OnceLock::new();
static P_ENTITY_UNICODE: OnceLock<Regex> = OnceLock::new();
static P_ENCODE: OnceLock<Regex> = OnceLock::new();
static P_VALID_ENTITIES: OnceLock<Regex> = OnceLock::new();
static P_VALID_QUOTES: OnceLock<Regex> = OnceLock::new();
static P_END_ARROW: OnceLock<Regex> = OnceLock::new();
static P_BODY_TO_END: OnceLock<Regex> = OnceLock::new();
static P_XML_CONTENT: OnceLock<Regex> = OnceLock::new();
static P_STRAY_LEFT_ARROW: OnceLock<Regex> = OnceLock::new();
static P_STRAY_RIGHT_ARROW: OnceLock<Regex> = OnceLock::new();
static P_AMP: OnceLock<Regex> = OnceLock::new();
static P_QUOTE: OnceLock<Regex> = OnceLock::new();
static P_LEFT_ARROW: OnceLock<Regex> = OnceLock::new();
static P_RIGHT_ARROW: OnceLock<Regex> = OnceLock::new();
static P_BOTH_ARROWS: OnceLock<Regex> = OnceLock::new();
static P_REMOVE_PAIR_BLANKS: OnceLock<Mutex<HashMap<String, Regex>>> = OnceLock::new();
static P_REMOVE_SELF_BLANKS: OnceLock<Mutex<HashMap<String, Regex>>> = OnceLock::new();

/// Initializes lazily-compiled static regex patterns.
fn init_patterns() {
    P_COMMENTS.get_or_init(|| regex_build(r"<!--(.*?)-->", false, true));
    P_COMMENT.get_or_init(|| regex_build(r"^!--(.*)--$", true, true));
    P_TAGS.get_or_init(|| regex_build(r"<(.*?)>", false, true));
    P_END_TAG.get_or_init(|| regex_build(r"^/([a-z0-9]+)", true, true));
    P_START_TAG.get_or_init(|| regex_build(r"^([a-z0-9]+)(.*?)(/?)$", true, true));
    P_QUOTED_ATTRIBUTES.get_or_init(|| regex_build(r#"([a-z0-9]+)=(["'])(.*?)\2"#, true, true));
    P_UNQUOTED_ATTRIBUTES.get_or_init(|| regex_build(r#"([a-z0-9]+)(=)([^"\s']+)"#, true, true));
    P_PROTOCOL.get_or_init(|| regex_build(r"^([^:]+):", true, true));
    P_ENTITY.get_or_init(|| Regex::new(r"&#(\d+);?").expect("entity"));
    P_ENTITY_UNICODE.get_or_init(|| regex_build(r"&#x([0-9a-f]+);?", true, true));
    P_ENCODE.get_or_init(|| regex_build(r"%([0-9a-f]{2});?", true, true));
    P_VALID_ENTITIES.get_or_init(|| Regex::new(r"&([^&;]*)(?=(;|&|$))").expect("valid entities"));
    P_VALID_QUOTES.get_or_init(|| regex_build(r"(>|^)([^<]+?)(<|$)", false, true));
    P_END_ARROW.get_or_init(|| Regex::new(r"^>").expect("end arrow"));
    P_BODY_TO_END.get_or_init(|| Regex::new(r"<([^>]*?)(?=<|$)").expect("body to end"));
    P_XML_CONTENT.get_or_init(|| Regex::new(r"(^|>)([^<]*?)(?=>)").expect("xml content"));
    P_STRAY_LEFT_ARROW.get_or_init(|| Regex::new(r"<([^>]*?)(?=<|$)").expect("stray left"));
    P_STRAY_RIGHT_ARROW.get_or_init(|| Regex::new(r"(^|>)([^<]*?)(?=>)").expect("stray right"));
    P_AMP.get_or_init(|| Regex::new(r"&").expect("amp"));
    P_QUOTE.get_or_init(|| Regex::new(r#"""#).expect("quote"));
    P_LEFT_ARROW.get_or_init(|| Regex::new(r"<").expect("left"));
    P_RIGHT_ARROW.get_or_init(|| Regex::new(r">").expect("right"));
    P_BOTH_ARROWS.get_or_init(|| Regex::new(r"<>").expect("both"));
    P_REMOVE_PAIR_BLANKS.get_or_init(|| Mutex::new(HashMap::new()));
    P_REMOVE_SELF_BLANKS.get_or_init(|| Mutex::new(HashMap::new()));
}

/// HTML whitelist filter for mitigating XSS in user-supplied markup.
pub struct HtmlFilter {
    allowed: HashMap<String, Vec<String>>,
    self_closing_tags: Vec<&'static str>,
    need_closing_tags: Vec<&'static str>,
    disallowed: Vec<&'static str>,
    protocol_atts: Vec<&'static str>,
    allowed_protocols: Vec<&'static str>,
    remove_blanks: Vec<&'static str>,
    allowed_entities: Vec<&'static str>,
    strip_comment: bool,
    encode_quotes: bool,
    always_make_tags: bool,
}

impl Default for HtmlFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlFilter {
    /// Creates a filter with Hutool's default tag and attribute whitelist.
    ///
    /// Java: `new HTMLFilter()` / `new HTMLFilter(boolean debug)`
    pub fn new() -> Self {
        init_patterns();
        let mut allowed = HashMap::new();
        allowed.insert(
            "a".to_string(),
            vec!["href".to_string(), "target".to_string()],
        );
        allowed.insert(
            "img".to_string(),
            vec![
                "src".to_string(),
                "width".to_string(),
                "height".to_string(),
                "alt".to_string(),
            ],
        );
        allowed.insert("b".to_string(), vec![]);
        allowed.insert("strong".to_string(), vec![]);
        allowed.insert("i".to_string(), vec![]);
        allowed.insert("em".to_string(), vec![]);
        allowed.insert("p".to_string(), vec![]);

        Self {
            allowed,
            self_closing_tags: vec!["img"],
            need_closing_tags: vec!["a", "b", "strong", "i", "em"],
            disallowed: vec![],
            protocol_atts: vec!["src", "href"],
            allowed_protocols: vec!["http", "mailto", "https"],
            remove_blanks: vec!["a", "b", "strong", "i", "em"],
            allowed_entities: vec!["amp", "gt", "lt", "quot"],
            strip_comment: true,
            encode_quotes: true,
            always_make_tags: true,
        }
    }

    /// Creates a filter; `debug` is accepted for Hutool signature parity.
    ///
    /// Java: `new HTMLFilter(final boolean debug)`
    #[must_use]
    pub fn with_debug(_debug: bool) -> Self {
        Self::new()
    }

    /// Creates a filter from a configuration map (unknown keys ignored).
    ///
    /// Java: `new HTMLFilter(final Map<String, Object> conf)`
    #[must_use]
    pub fn with_conf(conf: &HashMap<String, String>) -> Self {
        let mut filter = Self::new();
        if let Some(value) = conf.get("stripComment").or_else(|| conf.get("strip_comment")) {
            filter.strip_comment = value.eq_ignore_ascii_case("true") || value == "1";
        }
        if let Some(value) = conf
            .get("alwaysMakeTags")
            .or_else(|| conf.get("always_make_tags"))
        {
            filter.always_make_tags = value.eq_ignore_ascii_case("true") || value == "1";
        }
        filter
    }

    /// Filters user input, returning markup that only contains whitelisted HTML.
    ///
    /// Java: `HTMLFilter.filter(String input)`
    pub fn filter(&self, input: &str) -> String {
        let s = self.escape_comments(input);
        let s = self.balance_html(&s);
        let s = self.check_tags(&s);
        let s = self.process_remove_blanks(&s);
        self.validate_entities(&s)
    }

    /// Reports whether unbalanced markup is forcibly tag-balanced.
    ///
    /// Java: `HTMLFilter.isAlwaysMakeTags()`
    #[must_use]
    pub fn is_always_make_tags(&self) -> bool {
        self.always_make_tags
    }

    /// Reports whether HTML comments are stripped.
    ///
    /// Java: `HTMLFilter.isStripComments()`
    #[must_use]
    pub fn is_strip_comments(&self) -> bool {
        self.strip_comment
    }

    /// Converts a decimal code point to a single-character string.
    ///
    /// Java: `HTMLFilter.chr(final int decimal)`
    #[must_use]
    pub fn chr(decimal: u32) -> String {
        char::from_u32(decimal)
            .map(|c| c.to_string())
            .unwrap_or_default()
    }

    /// Escapes `& " < >` for safe HTML embedding.
    ///
    /// Java: `HTMLFilter.htmlSpecialChars(final String s)`
    #[must_use]
    pub fn html_special_chars(s: &str) -> String {
        init_patterns();
        let mut result = s.to_string();
        result = regex_replace(P_AMP.get().unwrap(), "&amp;", &result);
        result = regex_replace(P_QUOTE.get().unwrap(), "&quot;", &result);
        result = regex_replace(P_LEFT_ARROW.get().unwrap(), "&lt;", &result);
        result = regex_replace(P_RIGHT_ARROW.get().unwrap(), "&gt;", &result);
        result
    }

    /// Escapes the inner text of the first HTML comment (Java parity).
    fn escape_comments(&self, s: &str) -> String {
        let re = P_COMMENTS.get().unwrap();
        if let Ok(Some(caps)) = re.captures(s) {
            let m = caps.get(0).unwrap();
            let inner = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let replacement = format!("<!--{}-->", Self::html_special_chars(inner));
            let mut out = String::with_capacity(s.len());
            out.push_str(&s[..m.start()]);
            out.push_str(&replacement);
            out.push_str(&s[m.end()..]);
            out
        } else {
            s.to_string()
        }
    }

    /// Balances stray angle brackets or escapes them depending on configuration.
    fn balance_html(&self, s: &str) -> String {
        if self.always_make_tags {
            let mut result = regex_replace(P_END_ARROW.get().unwrap(), "", s);
            result = regex_replace(P_BODY_TO_END.get().unwrap(), "<$1>", &result);
            regex_replace(P_XML_CONTENT.get().unwrap(), "$1<$2", &result)
        } else {
            let mut result =
                regex_replace(P_STRAY_LEFT_ARROW.get().unwrap(), "&lt;$1", s);
            result = regex_replace(P_STRAY_RIGHT_ARROW.get().unwrap(), "$1$2&gt;<", &result);
            regex_replace(P_BOTH_ARROWS.get().unwrap(), "", &result)
        }
    }

    /// Processes each tag match and appends auto-closed tags for unbalanced markup.
    fn check_tags(&self, s: &str) -> String {
        let re = P_TAGS.get().unwrap();
        let mut tag_counts: HashMap<String, i32> = HashMap::new();
        let mut result = String::new();
        let mut last = 0;
        for caps in re.captures_iter(s).flatten() {
            let m = caps.get(0).unwrap();
            result.push_str(&s[last..m.start()]);
            let inner = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            result.push_str(&self.process_tag(inner, &mut tag_counts));
            last = m.end();
        }
        result.push_str(&s[last..]);

        for (key, count) in tag_counts {
            for _ in 0..count {
                result.push_str("</");
                result.push_str(&key);
                result.push('>');
            }
        }
        result
    }

    /// Removes empty paired and self-closing tags configured in `remove_blanks`.
    fn process_remove_blanks(&self, s: &str) -> String {
        let mut result = s.to_string();
        for tag in &self.remove_blanks {
            let pair_re = blank_pattern(&P_REMOVE_PAIR_BLANKS, tag, true);
            result = regex_replace(&pair_re, "", &result);
            let self_re = blank_pattern(&P_REMOVE_SELF_BLANKS, tag, false);
            result = regex_replace(&self_re, "", &result);
        }
        result
    }

    /// Validates entity references and optionally encodes stray quotes in text nodes.
    fn validate_entities(&self, s: &str) -> String {
        let re = P_VALID_ENTITIES.get().unwrap();
        let mut buf = String::with_capacity(s.len());
        let mut last = 0;
        for caps in re.captures_iter(s).flatten() {
            let m = caps.get(0).unwrap();
            buf.push_str(&s[last..m.start()]);
            let one = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let two = caps.get(2).map(|c| c.as_str()).unwrap_or("");
            buf.push_str(&self.check_entity(one, two));
            last = m.end();
        }
        buf.push_str(&s[last..]);
        self.encode_quotes(&buf)
    }

    /// Encodes double quotes in plain text segments outside tags.
    fn encode_quotes(&self, s: &str) -> String {
        if !self.encode_quotes {
            return s.to_string();
        }
        let re = P_VALID_QUOTES.get().unwrap();
        let mut out = String::with_capacity(s.len());
        let mut last = 0;
        for caps in re.captures_iter(s).flatten() {
            let m = caps.get(0).unwrap();
            out.push_str(&s[last..m.start()]);
            let one = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let two = caps.get(2).map(|c| c.as_str()).unwrap_or("");
            let three = caps.get(3).map(|c| c.as_str()).unwrap_or("");
            out.push_str(one);
            out.push_str(&regex_replace(P_QUOTE.get().unwrap(), "&quot;", two));
            out.push_str(three);
            last = m.end();
        }
        out.push_str(&s[last..]);
        out
    }

    /// Returns a validated entity or escapes a malformed ampersand sequence.
    fn check_entity(&self, preamble: &str, term: &str) -> String {
        if term == ";" && self.is_valid_entity(preamble) {
            format!("&{preamble}")
        } else {
            format!("&amp;{preamble}")
        }
    }

    /// Returns whether the named entity is on the allow-list.
    fn is_valid_entity(&self, entity: &str) -> bool {
        Self::in_array(entity, &self.allowed_entities)
    }

    /// Processes a single tag body (content inside angle brackets).
    fn process_tag(&self, s: &str, tag_counts: &mut HashMap<String, i32>) -> String {
        let end_re = P_END_TAG.get().unwrap();
        if let Ok(Some(caps)) = end_re.captures(s) {
            let name = caps.get(1).map(|c| c.as_str()).unwrap_or("").to_lowercase();
            if self.allowed(&name)
                && !Self::in_array(&name, &self.self_closing_tags)
                && tag_counts.get(&name).copied().unwrap_or(0) > 0
            {
                *tag_counts.entry(name.clone()).or_insert(0) -= 1;
                return format!("</{name}>");
            }
        }

        let start_re = P_START_TAG.get().unwrap();
        if let Ok(Some(caps)) = start_re.captures(s) {
            let name = caps.get(1).map(|c| c.as_str()).unwrap_or("").to_lowercase();
            let body = caps.get(2).map(|c| c.as_str()).unwrap_or("");
            let mut ending = caps.get(3).map(|c| c.as_str()).unwrap_or("").to_string();

            if self.allowed(&name) {
                let mut params = String::new();
                let mut param_names = Vec::new();
                let mut param_values = Vec::new();

                let quoted = P_QUOTED_ATTRIBUTES.get().unwrap();
                for cap in quoted.captures_iter(body).flatten() {
                    param_names.push(
                        cap.get(1)
                            .map(|c| c.as_str().to_lowercase())
                            .unwrap_or_default(),
                    );
                    param_values.push(cap.get(3).map(|c| c.as_str()).unwrap_or("").to_string());
                }
                let unquoted = P_UNQUOTED_ATTRIBUTES.get().unwrap();
                for cap in unquoted.captures_iter(body).flatten() {
                    param_names.push(
                        cap.get(1)
                            .map(|c| c.as_str().to_lowercase())
                            .unwrap_or_default(),
                    );
                    param_values.push(cap.get(3).map(|c| c.as_str()).unwrap_or("").to_string());
                }

                for (param_name, mut param_value) in
                    param_names.into_iter().zip(param_values.into_iter())
                {
                    if self.allowed_attribute(&name, &param_name) {
                        if Self::in_array(&param_name, &self.protocol_atts) {
                            param_value = self.process_param_protocol(&param_value);
                        }
                        params.push(' ');
                        params.push_str(&param_name);
                        params.push_str("=\"");
                        params.push_str(&param_value);
                        params.push('"');
                    }
                }

                if Self::in_array(&name, &self.self_closing_tags) {
                    ending = " /".to_string();
                }
                if Self::in_array(&name, &self.need_closing_tags) {
                    ending.clear();
                }

                if ending.is_empty() {
                    *tag_counts.entry(name.clone()).or_insert(0) += 1;
                } else {
                    ending = " /".to_string();
                }
                return format!("<{name}{params}{ending}>");
            }
            return String::new();
        }

        let comment_re = P_COMMENT.get().unwrap();
        if !self.strip_comment && comment_re.is_match(s).unwrap_or(false) {
            return format!("<{s}>");
        }

        String::new()
    }

    /// Validates URL protocols on `href` / `src` attributes.
    fn process_param_protocol(&self, s: &str) -> String {
        let mut value = self.decode_entities(s);
        let re = P_PROTOCOL.get().unwrap();
        if let Ok(Some(caps)) = re.captures(&value) {
            let protocol = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            if !Self::in_array(protocol, &self.allowed_protocols) {
                value = format!("#{}", &value[protocol.len() + 1..]);
                if let Some(rest) = value.strip_prefix("#//") {
                    value = format!("#{rest}");
                }
            }
        }
        value
    }

    /// Decodes numeric and percent-encoded entities in attribute values.
    fn decode_entities(&self, s: &str) -> String {
        let mut result = replace_append(P_ENTITY.get().unwrap(), s, |caps| {
            let decimal: u32 = caps
                .get(1)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap_or(0);
            Self::chr(decimal)
        });
        result = replace_append(P_ENTITY_UNICODE.get().unwrap(), &result, |caps| {
            let hex = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let decimal = u32::from_str_radix(hex, 16).unwrap_or(0);
            Self::chr(decimal)
        });
        result = replace_append(P_ENCODE.get().unwrap(), &result, |caps| {
            let hex = caps.get(1).map(|c| c.as_str()).unwrap_or("");
            let decimal = u32::from_str_radix(hex, 16).unwrap_or(0);
            Self::chr(decimal)
        });
        self.validate_entities(&result)
    }

    /// Returns whether a tag name is allowed and not explicitly disallowed.
    fn allowed(&self, name: &str) -> bool {
        (self.allowed.is_empty() || self.allowed.contains_key(name))
            && !Self::in_array(name, &self.disallowed)
    }

    /// Returns whether an attribute is allowed for the given tag.
    fn allowed_attribute(&self, name: &str, param_name: &str) -> bool {
        self.allowed(name)
            && (self.allowed.is_empty()
                || self
                    .allowed
                    .get(name)
                    .is_some_and(|attrs| attrs.iter().any(|a| a == param_name)))
    }

    /// Linear search helper matching Java `inArray`.
    fn in_array(needle: &str, haystack: &[&str]) -> bool {
        haystack.iter().any(|item| item == &needle)
    }
}

/// Global replace using a plain replacement string.
fn regex_replace(re: &Regex, replacement: &str, s: &str) -> String {
    re.replace_all(s, replacement).into_owned()
}

/// Global replace with a per-match callback (Java `appendReplacement` parity).
fn replace_append<F>(re: &Regex, s: &str, mut f: F) -> String
where
    F: FnMut(&Captures<'_>) -> String,
{
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    for caps in re.captures_iter(s).flatten() {
        let m = caps.get(0).unwrap();
        out.push_str(&s[last..m.start()]);
        out.push_str(&f(&caps));
        last = m.end();
    }
    out.push_str(&s[last..]);
    out
}

/// Lazily builds and caches blank-tag removal regexes per tag name.
fn blank_pattern(
    cache: &OnceLock<Mutex<HashMap<String, Regex>>>,
    tag: &str,
    paired: bool,
) -> Regex {
    let map = cache.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = map.lock().expect("blank pattern cache");
    if let Some(re) = guard.get(tag) {
        return re.clone();
    }
    let pattern = if paired {
        format!(r"<{tag}(\s[^>]*)?></{tag}>")
    } else {
        format!(r"<{tag}(\s[^>]*)?/>")
    };
    let re = Regex::new(&pattern).expect("blank tag regex");
    guard.insert(tag.to_string(), re.clone());
    re
}

#[cfg(test)]
mod tests {
    use super::HtmlFilter;

    #[test]
    fn filter_removes_disallowed_tags() {
        assert_eq!(HtmlFilter::new().filter("<alert></alert>"), "");
    }

    #[test]
    fn filter_strips_disallowed_attributes() {
        assert_eq!(
            HtmlFilter::new().filter(r#"<p onclick="bbbb">a</p>"#),
            "<p>a</p>"
        );
    }

    #[test]
    fn filter_keeps_allowed_paragraph() {
        assert_eq!(HtmlFilter::new().filter("<p>a</p>"), "<p>a</p>");
    }
}
