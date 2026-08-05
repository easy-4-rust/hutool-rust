//! re_util parity tests
//! 对齐: `cn.hutool.core.util.ReUtilTest`

use hutool_core::ReUtil;

// ── 匹配判断 ──

#[test]
fn is_match_basic() {
    assert!(ReUtil::is_match(r"^\d+$", "12345"));
    assert!(!ReUtil::is_match(r"^\d+$", "abc"));
}

#[test]
fn is_match_partial() {
    assert!(ReUtil::is_match(r"\d+", "abc123def"));
}

#[test]
fn is_match_invalid_pattern() {
    assert!(!ReUtil::is_match("[invalid", "test"));
}

// ── 提取操作 ──

#[test]
fn find_basic() {
    assert_eq!(ReUtil::find(r"\d+", "abc123def"), Some("123".to_string()));
}

#[test]
fn find_no_match() {
    assert_eq!(ReUtil::find(r"\d+", "abcdef"), None);
}

#[test]
fn group0_basic() {
    assert_eq!(
        ReUtil::group0(r"(\d+)", "abc123def"),
        Some("123".to_string())
    );
}

#[test]
fn group1_basic() {
    assert_eq!(
        ReUtil::group1(r"(\d+)-(\d+)", "123-456"),
        Some("123".to_string())
    );
}

#[test]
fn group_no_match() {
    assert_eq!(ReUtil::group(r"(\d+)", "abcdef", 1), None);
}

// ── 提取所有匹配 ──

#[test]
fn find_all_basic() {
    let result = ReUtil::find_all(r"\d+", "abc123def456");
    assert_eq!(result, vec!["123", "456"]);
}

#[test]
fn find_all_no_match() {
    let result = ReUtil::find_all(r"\d+", "abcdef");
    assert!(result.is_empty());
}

#[test]
fn find_all_groups_basic() {
    let result = ReUtil::find_all_groups(r"(\d+)-(\w+)", "123-abc 456-def", 1);
    assert_eq!(result, vec!["123", "456"]);
}

// ── 替换操作 ──

#[test]
fn replace_all_basic() {
    assert_eq!(ReUtil::replace_all(r"\d+", "abc123def456", "X"), "abcXdefX");
}

#[test]
fn replace_first_basic() {
    assert_eq!(
        ReUtil::replace_first(r"\d+", "abc123def456", "X"),
        "abcXdef456"
    );
}

#[test]
fn replace_no_match() {
    assert_eq!(ReUtil::replace_all(r"\d+", "abcdef", "X"), "abcdef");
}

// ── 分割操作 ──

#[test]
fn split_basic() {
    let result = ReUtil::split(r"\s+", "hello world  test");
    assert_eq!(result, vec!["hello", "world", "test"]);
}

// ── 常用正则 ──

#[test]
fn is_email_valid() {
    assert!(ReUtil::is_email("test@example.com"));
    assert!(ReUtil::is_email("user.name+tag@domain.co"));
}

#[test]
fn is_email_invalid() {
    assert!(!ReUtil::is_email("not-an-email"));
    assert!(!ReUtil::is_email("@no-user.com"));
}

#[test]
fn is_ipv4_valid() {
    assert!(ReUtil::is_ipv4("192.168.1.1"));
    assert!(ReUtil::is_ipv4("0.0.0.0"));
}

#[test]
fn is_ipv4_invalid() {
    assert!(!ReUtil::is_ipv4("not-an-ip"));
}

#[test]
fn is_url_valid() {
    assert!(ReUtil::is_url("https://example.com"));
    assert!(ReUtil::is_url("http://test.org/path"));
}

#[test]
fn is_url_invalid() {
    assert!(!ReUtil::is_url("not-a-url"));
    assert!(!ReUtil::is_url("ftp://example.com"));
}

#[test]
fn is_chinese_valid() {
    assert!(ReUtil::is_chinese("你好世界"));
}

#[test]
fn is_chinese_invalid() {
    assert!(!ReUtil::is_chinese("hello"));
    assert!(!ReUtil::is_chinese("你好world"));
}

#[test]
fn is_mobile_valid() {
    assert!(ReUtil::is_mobile("13800138000"));
    assert!(ReUtil::is_mobile("15912345678"));
}

#[test]
fn is_mobile_invalid() {
    assert!(!ReUtil::is_mobile("12345678901"));
    assert!(!ReUtil::is_mobile("1380013800"));
}

// ── 提取数字 ──

#[test]
fn extract_number_basic() {
    assert_eq!(ReUtil::extract_number("abc123def"), Some(123));
}

#[test]
fn extract_number_no_match() {
    assert_eq!(ReUtil::extract_number("abcdef"), None);
}

#[test]
fn extract_numbers_basic() {
    assert_eq!(ReUtil::extract_numbers("abc123def456"), vec![123, 456]);
}

// ── 转义 ──

#[test]
fn escape_special_basic() {
    assert_eq!(ReUtil::escape_special("a.b*c"), r"a\.b\*c");
}

#[test]
fn escape_special_no_special() {
    assert_eq!(ReUtil::escape_special("abc"), "abc");
}

// ── 对齐 Hutool ReUtilTest ──

const CONTENT: &str = "ZZZaaabbbccc中文1234";

/// 对齐 Java: `ReUtilTest.getTest()`
#[test]
fn get_test() {
    let result_get = ReUtil::group0(r"(?-u)\w{2}", CONTENT).unwrap();
    assert_eq!("ZZ", result_get);
}

/// 对齐 Java: `ReUtilTest.isMatchTest()`
#[test]
fn is_match_test() {
    let is_match = ReUtil::is_match(r"\w+[\u{4E00}-\u{9FFF}]+\d+", CONTENT);
    assert!(is_match);
}

/// 对齐 Java: `ReUtilTest.findAllTest()`
#[test]
fn find_all_test() {
    // Java `\w` 为 ASCII；Rust regex 默认 Unicode，故用 `(?-u:\w{2})` 对齐
    let result_find_all = ReUtil::find_all(r"(?-u)\w{2}", CONTENT);
    let expected = vec!["ZZ", "Za", "aa", "bb", "bc", "cc", "12", "34"];
    assert_eq!(expected, result_find_all);
}

/// 对齐 Java: `ReUtilTest.getFirstNumberTest()`
#[test]
fn get_first_number_test() {
    let result = ReUtil::extract_number(CONTENT).unwrap();
    assert_eq!(1234, result);
}

/// 对齐 Java: `ReUtilTest.delFirstTest()`
#[test]
fn del_first_test() {
    let result_del_first = ReUtil::replace_first(r"(?-u)(\w)aa(\w)", CONTENT, "");
    assert_eq!("ZZbbbccc中文1234", result_del_first);
}

/// 对齐 Java: `ReUtilTest.delAllTest()`
#[test]
fn del_all_test() {
    let content =
        "发东方大厦eee![images]http://abc.com/2.gpg]好机会eee![images]http://abc.com/2.gpg]好机会";
    let result_del_all = ReUtil::replace_all(r"!\[images\][^\u{4e00}-\u{9fa5}\\s]*", content, "");
    assert_eq!("发东方大厦eee好机会eee好机会", result_del_all);
}

/// 对齐 Java: `ReUtilTest.replaceAllTest()`
#[test]
fn replace_all_test() {
    let replace_all = ReUtil::replace_all(r"(\d+)", CONTENT, "->$1<-");
    assert_eq!("ZZZaaabbbccc中文->1234<-", replace_all);
}

/// 对齐 Java: `ReUtilTest.replaceAllTest2()`
#[test]
fn replace_all_test_2() {
    let replace_all = ReUtil::replace_all(r"(\d+)", CONTENT, "->$1<-");
    assert_eq!("ZZZaaabbbccc中文->1234<-", replace_all);
}

/// 对齐 Java: `ReUtilTest.escapeTest()`
#[test]
fn escape_test() {
    let escape = ReUtil::escape_special("我有个$符号{}");
    assert_eq!(r"我有个\$符号\{\}", escape);
}

/// 对齐 Java: `ReUtilTest.escapeTest2()`
#[test]
fn escape_test_2() {
    let str_val = "a[bbbc";
    let re = "[";
    let s = ReUtil::group0(&ReUtil::escape_special(re), str_val).unwrap();
    assert_eq!("[", s);
}

/// 对齐 Java: `ReUtilTest.escapeTest3()`
#[test]
fn escape_test_3() {
    let context = "{prefix}_";
    let regex = "{prefix}_";
    let b = ReUtil::is_match(&ReUtil::escape_special(regex), context);
    assert!(b);
}

/// 对齐 Java: `ReUtilTest.matchTest()`
#[test]
fn match_test() {
    let match_ = ReUtil::is_match(r"(.+?)省(.+?)市(.+?)区", "广东省深圳市南山区");
    assert!(match_);
}

/// 对齐 Java: `ReUtilTest.getAllGroupsTest()`
#[test]
fn get_all_groups_test() {
    // 对齐 getAllGroups(pattern, content) 含 group0
    let all_groups = ReUtil::find_all_groups(r"(\d+)-(\d+)-(\d+)", "192-168-1-1", 0);
    // find_all_groups(group=0) 返回完整匹配序列；首个完整匹配为 192-168-1
    assert!(!all_groups.is_empty());
    assert_eq!("192-168-1", all_groups[0]);
    assert_eq!(
        "192",
        ReUtil::group(r"(\d+)-(\d+)-(\d+)", "192-168-1-1", 1).unwrap()
    );
    assert_eq!(
        "168",
        ReUtil::group(r"(\d+)-(\d+)-(\d+)", "192-168-1-1", 2).unwrap()
    );
    assert_eq!(
        "1",
        ReUtil::group(r"(\d+)-(\d+)-(\d+)", "192-168-1-1", 3).unwrap()
    );
}

// ── Hutool TEST parity gap wave ──
// ── Hutool ReUtilTest remaining gaps ──

/// 对齐 Java: `ReUtilTest.extractMultiTest()`
#[test]
fn extract_multi_test() {
    // Java `\w` 为 ASCII；用 `(?-u)` 关闭 Unicode 类
    let result = ReUtil::extract_multi(r"(?-u)(\w)aa(\w)", CONTENT, "$1-$2").unwrap();
    assert_eq!("Z-a", result);
}

/// 对齐 Java: `ReUtilTest.extractMultiTest2()`
#[test]
fn extract_multi_test_2() {
    let pattern = r"(?-u)(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)(\w)";
    let template = "$1-$2-$3-$4-$5-$6-$7-$8-$9-$10";
    let result = ReUtil::extract_multi(pattern, CONTENT, template).unwrap();
    assert_eq!("Z-Z-Z-a-a-a-b-b-b-c", result);
}

/// 对齐 Java: `ReUtilTest.delLastTest()`
#[test]
fn del_last_test() {
    let blank = "";
    let word = "180公斤";
    let sentence = "10.商品KLS100021型号xxl适合身高180体重130斤的用户";

    assert_eq!(blank, ReUtil::del_last(r"\d+", blank));
    assert_eq!(blank, ReUtil::del_last(r"\d+", blank));

    assert_eq!("公斤", ReUtil::del_last(r"\d+", word));
    assert_eq!("公斤", ReUtil::del_last(r"\d+", word));

    assert_eq!("180", ReUtil::del_last(r"[\u{4E00}-\u{9FFF}]+", word));
    assert_eq!("180", ReUtil::del_last(r"[\u{4E00}-\u{9FFF}]+", word));

    let s = ReUtil::del_last(r"\d+", sentence);
    assert_eq!("10.商品KLS100021型号xxl适合身高180体重斤的用户", s);
    let s = ReUtil::del_last(r"\d+", sentence);
    assert_eq!("10.商品KLS100021型号xxl适合身高180体重斤的用户", s);

    assert!(!ReUtil::del_last(r"[\u{4E00}-\u{9FFF}]+", sentence).contains("斤的用户"));
    assert!(!ReUtil::del_last(r"[\u{4E00}-\u{9FFF}]+", sentence).contains("斤的用户"));
}

/// 对齐 Java: `ReUtilTest.replaceTest()`
#[test]
fn replace_test() {
    let r = ReUtil::replace_all(r"\d+", "a1b2c", "X");
    assert_eq!(r, "aXbXc");
}

/// 对齐 Java: `ReUtilTest.getByGroupNameTest()`
#[test]
fn get_by_group_name_test() {
    let content = "2021-10-11";
    let regex = r"(?<year>\d+)-(?<month>\d+)-(?<day>\d+)";
    assert_eq!(
        "2021",
        ReUtil::group_by_name(regex, content, "year").unwrap()
    );
    assert_eq!(
        "10",
        ReUtil::group_by_name(regex, content, "month").unwrap()
    );
    assert_eq!("11", ReUtil::group_by_name(regex, content, "day").unwrap());
}

/// 对齐 Java: `ReUtilTest.getAllGroupNamesTest()`
#[test]
fn get_all_group_names_test() {
    let content = "2021-10-11";
    let regex = r"(?<year>\d+)-(?<month>\d+)-(?<day>\d+)";
    let map = ReUtil::get_all_group_names(regex, content);
    assert_eq!("2021", map.get("year").unwrap());
    assert_eq!("10", map.get("month").unwrap());
    assert_eq!("11", map.get("day").unwrap());
}

/// 对齐 Java: `ReUtilTest.issuesI5TQDRTest()`
#[test]
fn issues_i5tqdr_test() {
    let pattern_ip = r"((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})\.((2(5[0-5]|[0-4]\d))|[0-1]?\d{1,2})";
    let s = ReUtil::replace_all(pattern_ip, "1.2.3.4", "$1.**.**.$10");
    assert_eq!("1.**.**.4", s);
}

/// 对齐 Java: `ReUtilTest.issueI6GIMTTest()`
#[test]
fn issue_i6gimt_test() {
    assert_eq!("", ReUtil::del_all(r"[\s]*", " "));
}

/// 对齐 Java: `ReUtilTest.issueI9T1TGTest()`
#[test]
fn issue_i9t1tg_test() {
    let regex = "^model";
    let content = "model-v";
    assert_eq!("model", ReUtil::group(regex, content, 0).unwrap());

    let regex = "^model.*?";
    let content = "model-v";
    assert!(ReUtil::is_match(regex, content));
}

/// 对齐 Java: `ReUtilTest.getEmailAddressTest()`
#[test]
fn get_email_address_test() {
    assert!(ReUtil::is_email("a@b.com"));
}

/// 对齐 Java: `ReUtilTest.issueIDPHVWTest()`
#[test]
fn issue_idphvw_test() {
    let s = ReUtil::replace_all(
        r"(^\d+(\.\d+)*)(\s)(((.*?)(DEM|DOM)?)([（|\(](.*?)[）|\)])?$)",
        "2 倾斜摄影成果",
        "$1$3$5($9)",
    );
    assert_eq!("2 倾斜摄影成果()", s);
}

/// Wave2 portable ReUtil coverage for parity ledger evidence.
#[test]
fn wave2_re_util_portable_parity() {
    assert_eq!(ReUtil::get_group0(r"(\d+)", "a12b").as_deref(), Some("12"));
    assert_eq!(ReUtil::get_group1(r"(\d+)", "a12b").as_deref(), Some("12"));
    assert_eq!(
        ReUtil::get_all_groups(r"(\w+)-(\d+)", "ab-12", true).len(),
        3
    );
    assert!(ReUtil::contains(r"\d+", "x9y"));
    assert_eq!(ReUtil::count(r"\d", "a1b2c3"), 3);
    assert_eq!(ReUtil::index_of(r"\d+", "ab12cd"), Some(2));
    assert_eq!(ReUtil::last_index_of(r"\d+", "a1b22c"), Some(3));
    assert_eq!(ReUtil::get_first_number("x42y"), Some(42));
    assert_eq!(
        ReUtil::find_all_group1(r"(\d+)", "1-2-3"),
        vec!["1", "2", "3"]
    );
    assert!(ReUtil::escape("a.b").contains(r"\."));
}
