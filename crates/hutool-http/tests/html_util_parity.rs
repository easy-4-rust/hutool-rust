//! HtmlUtil / HTMLFilter parity
//! 对齐: `cn.hutool.http.HtmlUtilTest`, `HTMLFilterTest`, `IssueI8YV0KTest`

use hutool_http::html::HtmlUtil;

/// 对齐 Java: `HtmlUtilTest.removeHtmlTagTest()`
#[test]
fn remove_html_tag_test() {
    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\">";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["img"]), "pre");

    let str = "pre<img>";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["img"]), "pre");

    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\" />";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["img"]), "pre");

    let str = "pre<img />";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["img"]), "pre");

    let str = "pre<div class=\"test_div\">dfdsfdsfdsf</div>";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["div"]), "pre");

    let str = "pre<div class=\"test_div\">\r\n\t\tdfdsfdsfdsf\r\n</div>";
    assert_eq!(HtmlUtil::remove_html_tag(str, &["div"]), "pre");
}

/// 对齐 Java: `HtmlUtilTest.cleanHtmlTagTest()`
#[test]
fn clean_html_tag_test() {
    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\">";
    assert_eq!(HtmlUtil::clean_html_tag(str), "pre");

    let str = "pre<img>";
    assert_eq!(HtmlUtil::clean_html_tag(str), "pre");

    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\" />";
    assert_eq!(HtmlUtil::clean_html_tag(str), "pre");

    let str = "pre<img />";
    assert_eq!(HtmlUtil::clean_html_tag(str), "pre");

    let str = "pre<div class=\"test_div\">dfdsfdsfdsf</div>";
    assert_eq!(HtmlUtil::clean_html_tag(str), "predfdsfdsfdsf");

    let str = "pre<div class=\"test_div\">\r\n\t\tdfdsfdsfdsf\r\n</div><div class=\"test_div\">BBBB</div>";
    assert_eq!(
        HtmlUtil::clean_html_tag(str),
        "pre\r\n\t\tdfdsfdsfdsf\r\nBBBB"
    );
}

/// 对齐 Java: `HtmlUtilTest.cleanEmptyTagTest()`
#[test]
fn clean_empty_tag_test() {
    assert_eq!(HtmlUtil::clean_empty_tag("<p></p><div></div>"), "");
    assert_eq!(
        HtmlUtil::clean_empty_tag("<p>TEXT</p><div></div>"),
        "<p>TEXT</p>"
    );
    assert_eq!(
        HtmlUtil::clean_empty_tag("<p></p><div>TEXT</div>"),
        "<div>TEXT</div>"
    );
    assert_eq!(
        HtmlUtil::clean_empty_tag("<p>TEXT</p><div>TEXT</div>"),
        "<p>TEXT</p><div>TEXT</div>"
    );
    assert_eq!(
        HtmlUtil::clean_empty_tag("TEXT<p></p><div></div>TEXT"),
        "TEXTTEXT"
    );
}

/// 对齐 Java: `HtmlUtilTest.unwrapHtmlTagTest()`
#[test]
fn unwrap_html_tag_test() {
    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\">";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["img"]), "pre");

    let str = "pre<img>";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["img"]), "pre");

    let str = "pre<img src=\"xxx/dfdsfds/test.jpg\" />";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["img"]), "pre");

    let str = "pre<img />";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["img"]), "pre");

    let str = "pre<img/>";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["img"]), "pre");

    let str = "pre<div class=\"test_div\">abc</div>";
    assert_eq!(HtmlUtil::unwrap_html_tag(str, &["div"]), "preabc");

    let str = "pre<div class=\"test_div\">\r\n\t\tabc\r\n</div>";
    assert_eq!(
        HtmlUtil::unwrap_html_tag(str, &["div"]),
        "pre\r\n\t\tabc\r\n"
    );
}

/// 对齐 Java: `HtmlUtilTest.unwrapTest2()`
#[test]
fn unwrap_test2() {
    let html_string = "<html><img src='aaa'><i>测试文本</i></html>";
    let clean_txt = HtmlUtil::remove_html_tag_with(html_string, false, &["i", "br"]);
    assert_eq!(clean_txt, "<html><img src='aaa'>测试文本</html>");
}

/// 对齐 Java: `HtmlUtilTest.escapeTest()`
#[test]
fn escape_test() {
    let html = "<html><body>123'123'</body></html>";
    let escape = HtmlUtil::escape(html);
    assert_eq!(
        escape,
        "&lt;html&gt;&lt;body&gt;123&#039;123&#039;&lt;/body&gt;&lt;/html&gt;"
    );
    let restore = HtmlUtil::unescape(&escape);
    assert_eq!(restore, html);
    assert_eq!(HtmlUtil::unescape("&apos;"), "'");
}

/// 对齐 Java: `HtmlUtilTest.escapeTest2()`
#[test]
fn escape_test2() {
    let c = '\u{00A0}';
    assert_eq!(c as u32, 160);
    let html = "<html><body> </body></html>";
    let escape = HtmlUtil::escape(html);
    assert_eq!(
        escape,
        "&lt;html&gt;&lt;body&gt;&nbsp;&lt;/body&gt;&lt;/html&gt;"
    );
    assert_eq!(HtmlUtil::unescape("&nbsp;"), " ");
}

/// 对齐 Java: `HtmlUtilTest.filterTest()`
#[test]
fn filter_test() {
    let html = "<alert></alert>";
    assert_eq!(HtmlUtil::filter(html), "");
}

/// 对齐 Java: `HtmlUtilTest.removeHtmlAttrTest()`
#[test]
fn remove_html_attr_test() {
    let html = "<div class=\"test_div\"></div><span class=\"test_div\"></span>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<div></div><span></span>"
    );

    let html = "<div class=test_div></div><span Class='test_div' ></span>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<div></div><span></span>"
    );

    let html =
        "<div style=\"margin:100%\" class=test_div></div><span Class='test_div' width=100></span>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<div style=\"margin:100%\"></div><span width=100></span>"
    );

    let html = "<div style = \"margin:100%\" class = test_div></div><span Class = 'test_div' width=100></span>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<div style = \"margin:100%\"></div><span width=100></span>"
    );
}

/// 对齐 Java: `HtmlUtilTest.removeAllHtmlAttrTest()`
#[test]
fn remove_all_html_attr_test() {
    let html = "<div class=\"test_div\" width=\"120\"></div>";
    assert_eq!(
        HtmlUtil::remove_all_html_attr(html, &["div"]),
        "<div></div>"
    );
}

/// 对齐 Java: `HtmlUtilTest.issueI6YNTFTest()`
#[test]
fn issue_i6_yntf_test() {
    let html = "<html><body><div class=\"a1 a2\">hello world</div></body></html>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<html><body><div>hello world</div></body></html>"
    );

    let html = "<html><body><div class=a1>hello world</div></body></html>";
    assert_eq!(
        HtmlUtil::remove_html_attr(html, &["class"]),
        "<html><body><div>hello world</div></body></html>"
    );
}

/// 对齐 Java: `HTMLFilterTest.issue3433Test()`
#[test]
fn html_filter_issue3433_test() {
    use hutool_http::html::HtmlFilter;
    let filter = HtmlFilter::new();
    assert_eq!(filter.filter("<p>a</p>"), "<p>a</p>");
    assert_eq!(filter.filter("<p onclick=\"bbbb\">a</p>"), "<p>a</p>");
    assert!(filter.is_always_make_tags());
    assert!(filter.is_strip_comments());
    assert_eq!(HtmlFilter::chr(65), "A");
    assert_eq!(
        HtmlFilter::html_special_chars("<&>\""),
        "&lt;&amp;&gt;&quot;"
    );
    let debug = HtmlFilter::with_debug(true);
    assert!(debug.is_always_make_tags());
    let mut conf = std::collections::HashMap::new();
    conf.insert("stripComment".into(), "false".into());
    conf.insert("alwaysMakeTags".into(), "false".into());
    let configured = HtmlFilter::with_conf(&conf);
    assert!(!configured.is_strip_comments());
    assert!(!configured.is_always_make_tags());
}

/// 对齐 Java: `IssueI8YV0KTest.removeHtmlAttrTest()`
#[test]
fn issue_i8_yv0_k_remove_html_attr_test() {
    let str = "<content styleCode=\"xmChange yes\">";
    assert_eq!(HtmlUtil::remove_html_attr(str, &["styleCode"]), "<content>");
}

/// 对齐 Java: `IssueI8YV0KTest.removeHtmlAttrTest2()`
#[test]
fn issue_i8_yv0_k_remove_html_attr_test2() {
    let str = "<content styleCode=\"xmChange\"/>";
    assert_eq!(
        HtmlUtil::remove_html_attr(str, &["styleCode"]),
        "<content/>"
    );
}

/// 对齐 Java: `IssueI8YV0KTest.removeHtmlAttrTest3()`
#[test]
fn issue_i8_yv0_k_remove_html_attr_test3() {
    let str = "<content styleCode=\"dada ada\" data=\"dsad\" >";
    assert_eq!(
        HtmlUtil::remove_html_attr(str, &["styleCode"]),
        "<content data=\"dsad\">"
    );
}
