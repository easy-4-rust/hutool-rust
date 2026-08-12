//! string module parity tests
//! 对齐: hutool-core StrUtilTest CharSequenceUtilTest

use hutool_core::{
    clean_blank, contains, contains_ignore_case, end_with, equals, equals_ignore_case,
    format_template, index_of_ignore_case, is_blank, last_index_of, last_index_of_ignore_case,
    length, lower_first, remove_all, remove_chars, repeat, replace, reverse, split, start_with,
    str_or_empty, strip, strip_ignore_case, trim, upper_first,
};

// ── is_blank ──

#[test]
fn is_blank_empty() {
    assert!(is_blank(""));
    assert!(is_blank("   "));
    assert!(is_blank("\t\n"));
}

#[test]
fn is_blank_non_empty() {
    assert!(!is_blank("hello"));
    assert!(!is_blank(" hello "));
}

// ── trim ──

#[test]
fn trim_whitespace() {
    assert_eq!(trim("  hello  "), "hello");
    assert_eq!(trim("hello"), "hello");
    assert_eq!(trim(""), "");
}

// ── clean_blank ──

#[test]
fn clean_blank_removes_all_whitespace() {
    assert_eq!(clean_blank("h e l l o"), "hello");
    assert_eq!(clean_blank("  hello  world  "), "helloworld");
}

// ── strip ──

#[test]
fn strip_chars() {
    assert_eq!(strip("abcd123", "ab23"), "cd1");
    assert_eq!(strip("abcd123", "ab"), "cd123");
}

#[test]
fn strip_ignore_case_test() {
    assert_eq!(strip_ignore_case("abcd123", "Ab23"), "cd1");
    assert_eq!(strip_ignore_case("abcd123", "AB"), "cd123");
}

// ── start_with / end_with ──

#[test]
fn start_with_test() {
    assert!(start_with("hello", "hel"));
    assert!(!start_with("hello", "world"));
}

#[test]
fn end_with_test() {
    assert!(end_with("hello", "llo"));
    assert!(!end_with("hello", "world"));
}

// ── contains ──

#[test]
fn contains_test() {
    assert!(contains("hello world", "world"));
    assert!(!contains("hello world", "xyz"));
}

#[test]
fn contains_ignore_case_test() {
    assert!(contains_ignore_case("Hello World", "hello"));
    assert!(!contains_ignore_case("Hello World", "xyz"));
}

// ── equals ──

#[test]
fn equals_test() {
    assert!(equals("hello", "hello"));
    assert!(!equals("hello", "world"));
}

#[test]
fn equals_ignore_case_test() {
    assert!(equals_ignore_case("Hello", "hello"));
    assert!(!equals_ignore_case("Hello", "world"));
}

// ── index_of_ignore_case ──

#[test]
fn index_of_ignore_case_test() {
    assert_eq!(index_of_ignore_case("aabaabaa", "B"), Some(2));
    assert_eq!(index_of_ignore_case("aabaabaa", "AB"), Some(1));
}

// ── last_index_of / last_index_of_ignore_case ──

#[test]
fn last_index_of_test() {
    assert_eq!(last_index_of("hello world", "o"), Some(7));
    assert_eq!(last_index_of("hello world", "z"), None);
}

#[test]
fn last_index_of_ignore_case_test() {
    assert_eq!(last_index_of_ignore_case("Hello World", "hello"), Some(0));
    assert_eq!(last_index_of_ignore_case("Hello World", "WORLD"), Some(6));
}

// ── replace ──

#[test]
fn replace_test() {
    assert_eq!(replace("hello world", "world", "rust"), "hello rust");
    assert_eq!(replace("aaa", "a", "b"), "bbb");
}

// ── reverse ──

#[test]
fn reverse_test() {
    assert_eq!(reverse("hello"), "olleh");
    assert_eq!(reverse("abc"), "cba");
}

// ── repeat ──

#[test]
fn repeat_test() {
    assert_eq!(repeat("ab", 3), "ababab");
    assert_eq!(repeat("x", 5), "xxxxx");
}

// ── length ──

#[test]
fn length_test() {
    assert_eq!(length(Some("hello")), 5);
    assert_eq!(length(Some("")), 0);
    assert_eq!(length(None), 0);
}

// ── upper_first / lower_first ──

#[test]
fn upper_first_test() {
    assert_eq!(upper_first("hello"), "Hello");
    assert_eq!(upper_first("Hello"), "Hello");
}

#[test]
fn lower_first_test() {
    assert_eq!(lower_first("Hello"), "hello");
    assert_eq!(lower_first("hello"), "hello");
}

// ── split ──

#[test]
fn split_test() {
    let parts: Vec<&str> = split("a,b,c", ',', false, false);
    assert_eq!(parts, vec!["a", "b", "c"]);
}

#[test]
fn split_trim_empty() {
    let parts: Vec<&str> = split(" a , b , c ", ',', true, true);
    assert_eq!(parts, vec!["a", "b", "c"]);
}

// ── remove_all / remove_chars ──

#[test]
fn remove_all_test() {
    assert_eq!(remove_all("hello world", "o"), "hell wrld");
}

#[test]
fn remove_chars_test() {
    assert_eq!(remove_chars("hello world", &['l', 'o']), "he wrd");
}

// ── str_or_empty ──

#[test]
fn str_or_empty_test() {
    assert_eq!(str_or_empty(Some("hello")), "hello");
    assert_eq!(str_or_empty(None), "");
}

// ── format_template ──

#[test]
fn format_template_test() {
    let name = "World";
    let result = format_template("Hello, {}!", &[&name]);
    assert_eq!(result, "Hello, World!");
}
