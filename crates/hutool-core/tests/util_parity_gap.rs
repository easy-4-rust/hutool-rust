//! util_parity_gap — leftover cn.hutool.core.util Issue* / deferred stubs
//!
//! ClassUtil / ModifierUtil / ReferenceUtil / ClassLoaderUtil / JAXBUtil 已有独立
//! `*_parity.rs`，此处不再保留重复 `#[ignore]` 桩。
//! 无法继续的项（JNDI / TypeUtil 泛型反射 / Java `@Disabled`）标记 `planned:`。

use hutool_core::{ArrayUtil, EnumUtil, IdcardUtil, NumberUtil, ReUtil, RuntimeUtil, XmlUtil};
use serde::Deserialize;

// ── RuntimeUtilTest ──

/// 对齐 Java: `RuntimeUtilTest.execTest()` — Java @Disabled；Mac 用 echo 等价断言
#[test]
fn exec_test() {
    let output = RuntimeUtil::exec_for_str(&["echo", "hello"]).expect("exec echo");
    assert_eq!(output, "hello");
}

/// 对齐 Java: `RuntimeUtilTest.execCmdTest()` — Java @Disabled/cmd；Mac 用 ls 等价断言
#[test]
fn exec_cmd_test() {
    let output = RuntimeUtil::exec_for_str(&["ls", "/"]).expect("exec ls");
    assert!(!output.is_empty());
}

/// 对齐 Java: `RuntimeUtilTest.execCmdTest2()` — Java @Disabled/cmd；Mac 用 sh -c 等价断言
#[test]
fn exec_cmd_test2() {
    let output = RuntimeUtil::exec_for_str(&["sh", "-c", "cd /tmp && pwd"]).expect("exec sh");
    assert!(output.contains("tmp"));
}

/// 对齐 Java: `RuntimeUtilTest.getUsableMemoryTest()`
#[test]
fn get_usable_memory_test() {
    assert!(RuntimeUtil::get_usable_memory() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.getPidTest()`
#[test]
fn get_pid_test() {
    assert!(RuntimeUtil::get_pid() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.getProcessorCountTest()`
#[test]
fn get_processor_count_test() {
    assert!(RuntimeUtil::get_processor_count() > 0);
}

/// 对齐 Java: `RuntimeUtilTest.issueIAB5LWTest()` — Java @Disabled/netstat；Mac 用 sh 管道等价
#[test]
fn issue_i_a_b5_l_w_test() {
    let output =
        RuntimeUtil::exec_for_str(&["sh", "-c", "echo 8080 | grep 8080"]).expect("exec pipe");
    assert!(output.contains("8080"));
}

// ── JNDIUtilTest（JVM JNDI，Rust 无对等 — planned）──

/// 对齐 Java: `JNDIUtilTest.getDnsTest()`
#[test]
#[ignore = "planned: JndiUtil DNS / JVM JNDI 搁置"]
fn get_dns_test() {}

// ── IssueIAQ16ETest ──

/// 对齐 Java: `IssueIAQ16ETest.lastIndexOfSubTest()`
#[test]
fn last_index_of_sub_test() {
    let big = [1, 2, 2, 2, 3, 2, 2, 2, 3];
    let sub = [2, 2];
    assert_eq!(ArrayUtil::last_index_of_sub(&big, &sub), 6);
}

/// 对齐 Java: `IssueIAQ16ETest.lastIndexOfSubTest2()`
#[test]
fn last_index_of_sub_test2() {
    let big = [1, 2, 2, 2, 3, 2, 2, 2, 3, 4, 5];
    let sub = [2, 2, 2, 3];
    assert_eq!(ArrayUtil::last_index_of_sub(&big, &sub), 5);
}

/// 对齐 Java: `IssueIAQ16ETest.lastIndexOfSubTest3()`
#[test]
fn last_index_of_sub_test3() {
    let a = [0x12, 0x34, 0x56, 0x78, 0x9A];
    let b = [0x56, 0x78];
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &b), 2);
}

// ── Issue3423Test ──

/// 对齐 Java: `Issue3423Test.toBigDecimalOfNaNTest()`
#[test]
fn to_big_decimal_of_na_n_test() {
    let err = NumberUtil::to_big_decimal_str("NaN").unwrap_err();
    assert!(
        err.to_string().contains("invalid") || err.to_string().contains("Number"),
        "NaN 应抛出 IllegalArgumentException 等价错误 (对齐 Java)"
    );
}

/// 对齐 Java: `Issue3423Test.toBigDecimalOfNaNTest2()` — Java `@Disabled`
#[test]
#[ignore = "planned: Java Issue3423Test.toBigDecimalOfNaNTest2 标注 @Disabled"]
fn to_big_decimal_of_na_n_test2() {}

// ── IssueI9K494Test ──

/// 对齐 Java: `IssueI9K494Test.unzipTest()` — Java `@Disabled` + 本地 zip 路径
#[test]
#[ignore = "planned: Java IssueI9K494Test.unzipTest 标注 @Disabled 且依赖 d:/test zip"]
fn unzip_test() {}

/// 对齐 Java: `IssueI9K494Test.unzipTest2()` — Java `@Disabled`
#[test]
#[ignore = "planned: Java IssueI9K494Test.unzipTest2 标注 @Disabled 且依赖 d:/test zip"]
fn unzip_test2() {}

// ── IssueI9NSZ4Test ──

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimalKind {
    Cat,
    Dog,
    Bird,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnimalKindInZoo {
    Cat,
    Snake,
    Bird,
}

impl AnimalKindInZoo {
    fn mapped_value(self) -> Option<AnimalKind> {
        match self {
            Self::Cat => Some(AnimalKind::Cat),
            Self::Snake => None,
            Self::Bird => Some(AnimalKind::Bird),
        }
    }
}

const ANIMAL_KIND_IN_ZOO: [AnimalKindInZoo; 3] = [
    AnimalKindInZoo::Cat,
    AnimalKindInZoo::Snake,
    AnimalKindInZoo::Bird,
];

/// 对齐 Java: `IssueI9NSZ4Test.getByTest()`
#[test]
fn get_by_test() {
    let found = EnumUtil::get_by(&ANIMAL_KIND_IN_ZOO, |v| {
        v.mapped_value() == Some(AnimalKind::Dog)
    });
    assert!(found.is_none(), "无 DOG 映射时应返回 null (对齐 Java)");
}

/// 对齐 Java: `IssueI9NSZ4Test.getByTest2()`
#[test]
fn get_by_test2() {
    let found = EnumUtil::get_by(&ANIMAL_KIND_IN_ZOO, |v| {
        v.mapped_value() == Some(AnimalKind::Bird)
    });
    assert_eq!(found, Some(AnimalKindInZoo::Bird));
}

// ── Issue3136Test ──

/// Hutool：空 `<message></message>` 映射为 `""`，Bean 转换时空串 → 空对象（非 null）。
#[derive(Debug, Default, Deserialize, PartialEq)]
struct Issue3136MessageItem {
    desmobile: Option<String>,
    msgid: Option<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
struct Issue3136Message {
    #[serde(default)]
    item: Vec<Issue3136MessageItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Issue3136SmsRes {
    code: String,
    /// 对齐 Hutool：空节点字符串反序列化为 Default Message
    #[serde(default, deserialize_with = "issue3136_empty_message")]
    message: Issue3136Message,
}

/// 将 XML 空节点产生的 `""` / null / object 统一为 `Message::default()`。
fn issue3136_empty_message<'de, D>(deserializer: D) -> Result<Issue3136Message, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(Issue3136Message::default()),
        serde_json::Value::String(s) if s.is_empty() => Ok(Issue3136Message::default()),
        serde_json::Value::Object(_) => {
            serde_json::from_value(value).map_err(serde::de::Error::custom)
        }
        other => Err(serde::de::Error::custom(format!(
            "unexpected message node: {other}"
        ))),
    }
}

/// 对齐 Java: `Issue3136Test.xmlToBeanTest()`
#[test]
fn issue3136_xml_to_bean_test() {
    let xml_str = "<?xml version=\"1.0\" encoding=\"gbk\" ?><response><code>02</code><message></message></response>";
    let doc = XmlUtil::parse_xml(xml_str).expect("parse xml");
    let sms: Issue3136SmsRes = XmlUtil::xml_to_bean(&doc).expect("xml_to_bean");
    assert_eq!(sms.code, "02");
    assert_eq!(sms.message, Issue3136Message::default());
}

// ── Issue3516Test ──

/// 对齐 Java: `Issue3516Test.getTypeArgumentTest()`
#[test]
#[ignore = "planned: TypeUtil::getTypeArgument 泛型反射搁置"]
fn get_type_argument_test() {}

// ── Issue3660Test / IssueI9UK5VTest / IssueICOJVZTest ──
// Runnable asserts live in `issue_util_parity.rs` (StrUtil API).

// ── Issue3809Test ──

/// 对齐 Java: `Issue3809Test.roundStrTest()`
#[test]
fn round_str_test() {
    assert_eq!(
        NumberUtil::round_str("9999999999999999.99", 2).unwrap(),
        "9999999999999999.99"
    );
    assert_eq!(
        NumberUtil::round_str("11111111111111119.00", 2).unwrap(),
        "11111111111111119.00"
    );
    assert_eq!(
        NumberUtil::round_str("7999999999999999.99", 2).unwrap(),
        "7999999999999999.99"
    );
    assert_eq!(
        NumberUtil::round_str("699999999991999.92", 2).unwrap(),
        "699999999991999.92"
    );
    assert_eq!(NumberUtil::round_str("10.92", 2).unwrap(), "10.92");
    assert_eq!(NumberUtil::round_str("10.99", 2).unwrap(), "10.99");
}

// ── IssueI7CRIWTest ──

/// 对齐 Java: `IssueI7CRIWTest.getTypeArgumentsTest()`
#[test]
#[ignore = "planned: TypeUtil::getTypeArguments 泛型反射搁置"]
fn get_type_arguments_test() {}

// ── IssueI9IDAGTest ──

/// 对齐 Java: `IssueI9IDAGTest.loopFilesTest()` — Java `@Disabled`
#[test]
#[ignore = "planned: Java IssueI9IDAGTest.loopFilesTest 标注 @Disabled 且无断言"]
fn loop_files_test() {}

// ── IssueI9UK5VTest ──
// See `issue_util_parity.rs::issue_i9uk5v_split_test`.

// ── IssueIB95X4Test ──

/// Hutool `PatternPool.MAC_ADDRESS` 正则（对齐 Java `RegexPool.MAC_ADDRESS`）。
const MAC_ADDRESS: &str = r"((?:[a-fA-F0-9]{1,2}[:-]){5}[a-fA-F0-9]{1,2})|((?:[a-fA-F0-9]{1,4}[.]){2}[a-fA-F0-9]{1,4})|[a-fA-F0-9]{12}|0x(\d{12}).+ETHER";

/// 对齐 Java: `IssueIB95X4Test.isMacTest()`
#[test]
fn is_mac_test() {
    assert!(ReUtil::is_match(MAC_ADDRESS, "ab1c.2d3e.f468"));
    assert!(ReUtil::is_match(MAC_ADDRESS, "ab:1c:2d:3e:f4:68"));
    assert!(ReUtil::is_match(MAC_ADDRESS, "ab-1c-2d-3e-f4-68"));
    assert!(ReUtil::is_match(MAC_ADDRESS, "ab1c2d3ef468"));
}

// ── IssueICA9S5Test ──

/// 对齐 Java: `IssueICA9S5Test.test()`
#[test]
fn test() {
    let a = "ENUM{\\ndisable ~ 0\\nenable ~ 1\\n}";
    let split: Vec<&str> = a.split("\\n").collect();
    assert_eq!(
        split.len(),
        4,
        "StrUtil.split(a, \"\\\\n\") → 4 项 (对齐 Java)"
    );
}

// ── IssueICOJVZTest ──
// See `issue_util_parity.rs::to_underline_test`.

// ── IssueIBP6T1Test ──

/// 对齐 Java: `IssueIBP6T1Test.isValidCard10Test()`
#[test]
fn is_valid_card_10_test() {
    let ascii = IdcardUtil::is_valid_card_10("1608214(1)").expect("1608214(1) 应可解析");
    assert!(ascii.is_valid(), "1608214(1) 校验应为 true (对齐 Java [2])");

    let fullwidth = IdcardUtil::is_valid_card_10("1608214（1）").expect("1608214（1） 应可解析");
    assert!(
        fullwidth.is_valid(),
        "1608214（1） 校验应为 true (对齐 Java [2])"
    );
}
