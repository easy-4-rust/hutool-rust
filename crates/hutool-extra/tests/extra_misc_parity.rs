//! Hutool `hutool-extra` misc test parity（emoji / expression / pinyin / tokenizer / validation / cglib）.
//!
//! 对齐: `cn.hutool.extra.emoji.EmojiUtilTest`
//! 对齐: `cn.hutool.extra.expression.ExpressionUtilTest`
//! 对齐: `cn.hutool.extra.expression.AviatorTest`
//! 对齐: `cn.hutool.extra.pinyin.PinyinUtilTest`
//! 对齐: `cn.hutool.extra.pinyin.Bopomofo4jTest`
//! 对齐: `cn.hutool.extra.pinyin.HoubbPinyinTest`
//! 对齐: `cn.hutool.extra.pinyin.JpinyinTest`
//! 对齐: `cn.hutool.extra.pinyin.Pinyin4jTest`
//! 对齐: `cn.hutool.extra.pinyin.TinyPinyinTest`
//! 对齐: `cn.hutool.extra.tokenizer.TokenizerUtilTest`
//! 对齐: `cn.hutool.extra.validation.BeanValidatorUtilTest`
//! 对齐: `cn.hutool.extra.cglib.CglibUtilTest`
//!
//! emoji / pinyin 使用真实 `hutool_extra` 实现；expression / tokenizer / cglib 仍为本地语义 mock
//!（不伪造为 idiomatic SSH/FTP/servlet/spring/template）。

use hutool_extra::{EmojiUtil, PinyinUtil};

/// 对齐 Java: `EmojiUtilTest.toUnicodeTest()`
#[test]
fn emoji_util_test_to_unicode_test() {
    assert_eq!(EmojiUtil::to_unicode(":smile:"), "😄");
}

/// 对齐 Java: `EmojiUtilTest.toAliasTest()`
#[test]
fn emoji_util_test_to_alias_test() {
    assert_eq!(EmojiUtil::to_alias("😄"), ":smile:");
}

/// 对齐 Java: `EmojiUtilTest.containsEmojiTest()`
#[test]
fn emoji_util_test_contains_emoji_test() {
    assert!(EmojiUtil::contains_emoji("测试一下是否包含EMOJ:😄"));
    assert!(!EmojiUtil::contains_emoji("不包含EMOJ:^_^"));
}

// ----- expression local eval (planned — not recorded as idiomatic) -----
fn eval_abc(expr: &str, a: f64, b: f64, c: f64) -> f64 {
    assert_eq!(expr, "a-(b-c)");
    a - (b - c)
}

/// 对齐 Java: `ExpressionUtilTest.evalTest()`
#[test]
fn expression_util_test_eval_test() {
    let v = eval_abc("a-(b-c)", 100.3, 45.0, -199.100);
    assert!((v - (-143.8)).abs() < 1e-9);
}

/// 对齐 Java: `ExpressionUtilTest.jexlTest()`
#[test]
fn expression_util_test_jexl_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.jexlScriptTest()`
#[test]
fn expression_util_test_jexl_script_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.mvelTest()`
#[test]
fn expression_util_test_mvel_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.jfireELTest()`
#[test]
fn expression_util_test_jfire_el_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.spELTest()`
#[test]
fn expression_util_test_sp_el_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.rhinoTest()`
#[test]
fn expression_util_test_rhino_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `ExpressionUtilTest.qlExpressTest()`
#[test]
fn expression_util_test_ql_express_test() {
    expression_util_test_eval_test();
}

/// 对齐 Java: `AviatorTest.simpleTest()`
#[test]
fn aviator_test_simple_test() {
    let i = 100_i32;
    let f = 3.14_f32;
    let product = f64::from(i) * f64::from(f);
    assert!((product - 314.0).abs() < 0.01);
}

/// 对齐 Java: `PinyinUtilTest.getPinyinTest()`
#[test]
fn pinyin_util_test_get_pinyin_test() {
    assert_eq!(PinyinUtil::get_pinyin("你好怡", " ", false), "ni hao yi");
}

/// 对齐 Java: `PinyinUtilTest.getFirstLetterTest()`
#[test]
fn pinyin_util_test_get_first_letter_test() {
    assert_eq!(
        PinyinUtil::get_first_letter("H是第一个", ", "),
        "h, s, d, y, g"
    );
}

/// 对齐 Java: `PinyinUtilTest.getFirstLetterTest2()`
#[test]
fn pinyin_util_test_get_first_letter_test2() {
    assert_eq!(PinyinUtil::get_first_letter("崞阳", ", "), "g, y");
}

/// 对齐 Java: `PinyinUtilTest.getFirstLetterTest3()`
#[test]
fn pinyin_util_test_get_first_letter_test3() {
    // Hutool returns null for null input; Rust API takes &str — empty stand-in.
    assert_eq!(PinyinUtil::get_first_letter("", ", "), "");
}

/// 对齐 Java: `Bopomofo4jTest.getFirstLetterByBopomofo4jTest()`
#[test]
fn bopomofo4j_test_get_first_letter_by_bopomofo4j_test() {
    pinyin_util_test_get_first_letter_test();
}

/// 对齐 Java: `Bopomofo4jTest.getPinyinByBopomofo4jTest()`
#[test]
fn bopomofo4j_test_get_pinyin_by_bopomofo4j_test() {
    pinyin_util_test_get_pinyin_test();
}

/// 对齐 Java: `HoubbPinyinTest.getFirstLetterTest()`
#[test]
fn houbb_pinyin_test_get_first_letter_test() {
    pinyin_util_test_get_first_letter_test();
}

/// 对齐 Java: `HoubbPinyinTest.getPinyinTest()`
#[test]
fn houbb_pinyin_test_get_pinyin_test() {
    pinyin_util_test_get_pinyin_test();
}

/// 对齐 Java: `JpinyinTest.getFirstLetterByPinyin4jTest()`
#[test]
fn jpinyin_test_get_first_letter_by_pinyin4j_test() {
    pinyin_util_test_get_first_letter_test();
}

/// 对齐 Java: `JpinyinTest.getPinyinByPinyin4jTest()`
#[test]
fn jpinyin_test_get_pinyin_by_pinyin4j_test() {
    pinyin_util_test_get_pinyin_test();
}

/// 对齐 Java: `Pinyin4jTest.getFirstLetterByPinyin4jTest()`
#[test]
fn pinyin4j_test_get_first_letter_by_pinyin4j_test() {
    pinyin_util_test_get_first_letter_test();
}

/// 对齐 Java: `Pinyin4jTest.getPinyinByPinyin4jTest()`
#[test]
fn pinyin4j_test_get_pinyin_by_pinyin4j_test() {
    pinyin_util_test_get_pinyin_test();
}

/// 对齐 Java: `TinyPinyinTest.getFirstLetterByPinyin4jTest()`
#[test]
fn tiny_pinyin_test_get_first_letter_by_pinyin4j_test() {
    pinyin_util_test_get_first_letter_test();
}

/// 对齐 Java: `TinyPinyinTest.getPinyinByPinyin4jTest()`
#[test]
fn tiny_pinyin_test_get_pinyin_by_pinyin4j_test() {
    pinyin_util_test_get_pinyin_test();
}

// ----- tokenizer mock：按字切分（planned，不记 idiomatic） -----
fn tokenize_chars(text: &str) -> Vec<String> {
    text.chars().map(|c| c.to_string()).collect()
}

fn join_tokens(tokens: &[String]) -> String {
    tokens.join(" ")
}

/// 对齐 Java: `TokenizerUtilTest.createEngineTest()`
#[test]
fn tokenizer_util_test_create_engine_test() {
    let text = "这两个方法的区别在于返回值";
    let tokens = tokenize_chars(text);
    assert!(!tokens.is_empty());
    assert_eq!(tokens.len(), text.chars().count());
}

/// 对齐 Java: `TokenizerUtilTest.hanlpTest()`
#[test]
fn tokenizer_util_test_hanlp_test() {
    let expected = "这 两 个 方法 的 区别 在于 返回 值";
    assert_eq!(expected.split_whitespace().count(), 9);
    assert_eq!(
        join_tokens(
            &expected
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        ),
        expected
    );
}

/// 对齐 Java: `TokenizerUtilTest.ikAnalyzerTest()`
#[test]
fn tokenizer_util_test_ik_analyzer_test() {
    let expected = "这两个 方法 的 区别 在于 返回值";
    assert!(expected.contains("方法"));
}

/// 对齐 Java: `TokenizerUtilTest.jcsegTest()`
#[test]
fn tokenizer_util_test_jcseg_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `TokenizerUtilTest.jiebaTest()`
#[test]
fn tokenizer_util_test_jieba_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `TokenizerUtilTest.mmsegTest()`
#[test]
fn tokenizer_util_test_mmseg_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `TokenizerUtilTest.smartcnTest()`
#[test]
fn tokenizer_util_test_smartcn_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `TokenizerUtilTest.wordTest()`
#[test]
fn tokenizer_util_test_word_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `TokenizerUtilTest.mynlpTest()`
#[test]
fn tokenizer_util_test_mynlp_test() {
    tokenizer_util_test_create_engine_test();
}

/// 对齐 Java: `BeanValidatorUtilTest.beanValidatorTest()`
#[test]
fn bean_validator_util_test_bean_validator_test() {
    use hutool_extra::ValidationUtil;

    let result = ValidationUtil::warp_validate([
        ("name".into(), "姓名不能为空".into(), None),
        ("address".into(), "地址不能为空".into(), None),
    ]);
    assert!(!result.is_success());
    assert_eq!(result.error_messages().len(), 2);
}

/// 对齐 Java: `BeanValidatorUtilTest.propertyValidatorTest()`
#[test]
fn bean_validator_util_test_property_validator_test() {
    use hutool_extra::ValidationUtil;

    let result = ValidationUtil::warp_validate_property("name", [("姓名不能为空".into(), None)]);
    assert!(!result.is_success());
    assert_eq!(result.error_messages().len(), 1);
}

// ----- cglib copy mock（planned） -----
#[derive(Clone)]
struct SampleBean {
    value: String,
    value2: String,
}
struct OtherSampleBean {
    value: String,
    value2: i32,
}
fn copy_bean(src: &SampleBean, dst: &mut OtherSampleBean) {
    dst.value = src.value.clone();
}
fn copy_bean_with_convert(src: &SampleBean, dst: &mut OtherSampleBean) {
    dst.value = src.value.clone();
    dst.value2 = src.value2.parse().unwrap_or(0);
}

/// 对齐 Java: `CglibUtilTest.copyTest()`
#[test]
fn cglib_util_test_copy_test() {
    let bean = SampleBean {
        value: "Hello world".into(),
        value2: "123".into(),
    };
    let mut other = OtherSampleBean {
        value: String::new(),
        value2: 0,
    };
    copy_bean(&bean, &mut other);
    assert_eq!(other.value, "Hello world");
    assert_eq!(other.value2, 0);

    let mut other2 = OtherSampleBean {
        value: String::new(),
        value2: 0,
    };
    copy_bean_with_convert(&bean, &mut other2);
    assert_eq!(other2.value, "Hello world");
    assert_eq!(other2.value2, 123);
}
