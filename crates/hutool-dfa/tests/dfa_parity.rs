//! DFA parity tests —— 对齐 Hutool `hutool-dfa` 测试。
//!
//! 对齐: `cn.hutool.dfa.DfaTest`
//! 对齐: `cn.hutool.dfa.SensitiveUtilTest`
//! 来源:
//! - hutool-dfa/src/test/java/cn/hutool/dfa/DfaTest.java
//! - hutool-dfa/src/test/java/cn/hutool/dfa/SensitiveUtilTest.java

use hutool_dfa::{FoundWord, MatchOptions, SensitiveUtil, WordTree};
use serde::{Deserialize, Serialize};

/// 构建被查询的文本，包含停顿词（对齐 Java `DfaTest.text`）。
const TEXT: &str = "我有一颗$大土^豆，刚出锅的";

/// 构建与 Java `DfaTest.buildWordTree()` 相同的查询树。
fn build_word_tree() -> WordTree {
    let mut tree = WordTree::new();
    tree.add_word("大");
    tree.add_word("大土豆");
    tree.add_word("土豆");
    tree.add_word("刚出锅");
    tree.add_word("出锅");
    tree
}

/// 将 UTF-8 字节偏移转为 Hutool 字符下标（含）。
fn hutool_char_index(text: &str, byte: usize) -> usize {
    text.get(..byte)
        .map(|prefix| prefix.chars().count())
        .unwrap_or_else(|| text.chars().count())
}

/// 将 Rust 半开字节区间转为 Hutool 闭区间字符下标 `(start, end)`。
fn hutool_span(text: &str, found: &FoundWord) -> (usize, usize) {
    let start = hutool_char_index(text, found.start());
    let end = hutool_char_index(text, found.end()).saturating_sub(1);
    (start, end)
}

/// 对齐 Java: `DfaTest.matchAllTest()`
#[test]
fn match_all_test() {
    let tree = build_word_tree();
    // 标准匹配：最短关键词，跳过已匹配
    let match_all = tree.match_all_with_options(
        TEXT,
        MatchOptions {
            limit: None,
            density: false,
            greedy: false,
        },
    );
    assert_eq!(match_all, vec!["大", "土^豆", "刚出锅"]);
}

/// 对齐 Java: `DfaTest.densityMatchTest()`
#[test]
fn density_match_test() {
    let tree = build_word_tree();
    let match_all = tree.match_all_with_options(
        TEXT,
        MatchOptions {
            limit: None,
            density: true,
            greedy: false,
        },
    );
    assert_eq!(match_all, vec!["大", "土^豆", "刚出锅", "出锅"]);
}

/// 对齐 Java: `DfaTest.greedMatchTest()`
#[test]
fn greed_match_test() {
    let tree = build_word_tree();
    let match_all = tree.match_all_with_options(
        TEXT,
        MatchOptions {
            limit: None,
            density: false,
            greedy: true,
        },
    );
    assert_eq!(match_all, vec!["大", "土^豆", "刚出锅"]);
}

/// 对齐 Java: `DfaTest.densityAndGreedMatchTest()`
#[test]
fn density_and_greed_match_test() {
    let tree = build_word_tree();
    let match_all = tree.match_all_with_options(
        TEXT,
        MatchOptions {
            limit: None,
            density: true,
            greedy: true,
        },
    );
    assert_eq!(match_all, vec!["大", "大土^豆", "土^豆", "刚出锅", "出锅"]);
}

/// 对齐 Java: `DfaTest.densityAndGreedMatchTest2()`
#[test]
fn density_and_greed_match_test2() {
    let mut tree = WordTree::new();
    tree.add_word("赵");
    tree.add_word("赵阿");
    tree.add_word("赵阿三");

    let text = "赵阿三在做什么";
    let result = tree.match_all_words_with_options(
        text,
        MatchOptions {
            limit: None,
            density: true,
            greedy: true,
        },
    );
    assert_eq!(3, result.len());

    assert_eq!("赵", result[0].get_word());
    let (s0, e0) = hutool_span(text, &result[0]);
    assert_eq!(0, s0);
    assert_eq!(0, e0);

    assert_eq!("赵阿", result[1].get_word());
    let (s1, e1) = hutool_span(text, &result[1]);
    assert_eq!(0, s1);
    assert_eq!(1, e1);

    assert_eq!("赵阿三", result[2].get_word());
    let (s2, e2) = hutool_span(text, &result[2]);
    assert_eq!(0, s2);
    assert_eq!(2, e2);
}

/// 对齐 Java: `DfaTest.stopWordTest()`
#[test]
fn stop_word_test() {
    let mut tree = WordTree::new();
    tree.add_word("tio");

    let all = tree.match_all("AAAAAAAt-ioBBBBBBB");
    assert_eq!(all, vec!["t-io"]);
}

/// 对齐 Java: `DfaTest.addWordWithTrailingFilteredCharTest()`
#[test]
fn add_word_with_trailing_filtered_char_test() {
    let mut tree = WordTree::new();
    tree.add_word("hello("); // 以停顿词 '(' 结尾

    let matches = tree.match_all_with_options(
        "hello",
        MatchOptions {
            limit: None,
            density: false,
            greedy: false,
        },
    );
    assert_eq!(1, matches.len());
    assert_eq!("hello", matches[0]);
}

/// 对齐 Java: `DfaTest.addWordWithMiddleFilteredCharTest()`
#[test]
fn add_word_with_middle_filtered_char_test() {
    let mut tree = WordTree::new();
    tree.add_word("he(llo"); // 中间 '(' 被过滤

    let matches = tree.match_all_with_options(
        "hello",
        MatchOptions {
            limit: None,
            density: false,
            greedy: false,
        },
    );
    assert_eq!(1, matches.len());
    assert_eq!("hello", matches[0]);
}

/// 对齐 Java: `DfaTest.aTest()`
#[test]
fn a_test() {
    let mut tree = WordTree::new();
    tree.add_word("women");
    let text = "a WOMEN todo.".to_lowercase();
    let match_all = tree.match_all_with_options(
        &text,
        MatchOptions {
            limit: None,
            density: false,
            greedy: false,
        },
    );
    // Java `List.toString()` → `[women]`（无引号）
    assert_eq!(format!("[{}]", match_all.join(", ")), "[women]");
}

/// 对齐 Java: `DfaTest.clearTest()`
#[test]
fn clear_test() {
    let mut tree = WordTree::new();
    tree.add_word("黑");
    assert!(tree.match_all("黑大衣").contains(&"黑".to_string()));
    tree.clear();
    tree.add_words(["黑大衣", "红色大衣"]);

    assert!(tree.match_all("黑大衣").contains(&"黑大衣".to_string()));
    assert!(!tree.match_all("黑大衣").contains(&"黑".to_string()));
    assert!(tree.match_all("红色大衣").contains(&"红色大衣".to_string()));

    let mut tree = WordTree::new();
    tree.add_words(["黑大衣", "红色大衣"]);
    assert!(tree.match_all("黑大衣").contains(&"黑大衣".to_string()));
    assert!(tree.match_all("红色大衣").contains(&"红色大衣".to_string()));
}

/// 对齐 Java `SensitiveUtilTest.TestBean`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TestBean {
    str: String,
    num: i32,
}

/// 对齐 Java: `SensitiveUtilTest.testSensitiveFilter()`
#[test]
fn test_sensitive_filter() {
    let word_list = ["大", "大土豆", "土豆", "刚出锅", "出锅"];
    let bean = TestBean {
        str: "我有一颗$大土^豆，刚出锅的".to_string(),
        num: 100,
    };
    let util = SensitiveUtil::from_words(word_list);
    let bean = util
        .filter_serialized(&bean, true, &hutool_dfa::DefaultSensitiveProcessor)
        .expect("filter_serialized");
    assert_eq!(bean.str, "我有一颗$****，***的");
}

/// 对齐 Java: `SensitiveUtilTest.issue2126()`
#[test]
fn issue2126() {
    let util = SensitiveUtil::from_words(["赵", "赵阿", "赵阿三"]);
    let result = util.filter_sensitive_with(
        "赵阿三在做什么。",
        true,
        &hutool_dfa::DefaultSensitiveProcessor,
    );
    assert_eq!("***在做什么。", result);
}

/// 对齐 Java: `SensitiveUtilTest.issue4182Test()`
#[test]
fn issue4182_test() {
    let util = SensitiveUtil::from_words(["12宝宝龙", "34皮卡丘"]);
    let s = util.filter_sensitive("creator_user_id=2000907612345839744");
    assert_eq!("creator_user_id=2000907612345839744", s);
}

// ---------------------------------------------------------------------------
// 既有 smoke 用例（非 Hutool @Test 清单项，保留不删）
// ---------------------------------------------------------------------------

#[test]
fn dfa_word_tree_test() {
    let mut tree = WordTree::new();
    tree.add_word("ab");
    tree.add_word("abc");
    tree.add_word("abcd");
    let result = tree.match_first("hello abc world");
    assert!(result.is_some(), "应找到匹配");
    let found = result.unwrap();
    assert!(
        found == "ab" || found == "abc" || found == "abcd",
        "匹配结果应为 ab/abc/abcd 中的一个, 实际: {}",
        found
    );
}

#[test]
fn dfa_word_tree_no_match_test() {
    let mut tree = WordTree::new();
    tree.add_word("xyz");
    let result = tree.match_first("hello abc");
    assert!(result.is_none(), "xyz 不应匹配");
}

#[test]
fn dfa_matcher_test() {
    let matcher = hutool_dfa::DfaMatcher::new(["ab", "abc", "abcd"]).unwrap();
    let results = matcher.find_all("hello abc world");
    assert!(!results.is_empty(), "应找到匹配");
}
