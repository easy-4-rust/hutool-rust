//! Hutool `hutool-setting` TEST parity —— 对齐 Java `cn.hutool.setting.*` / `yaml.YamlUtilTest`。
//!
//! 对齐: `cn.hutool.setting.SettingTest`
//! 对齐: `cn.hutool.setting.SettingUtilTest`
//! 对齐: `cn.hutool.setting.PropsTest`
//! 对齐: `cn.hutool.setting.PropsUtilTest`
//! 对齐: `cn.hutool.setting.Issue3008Test`
//! 对齐: `cn.hutool.setting.IssueI7G34ETest`
//! 对齐: `cn.hutool.setting.yaml.YamlUtilTest`
//!
//! 来源: hutool-setting/src/test/java/cn/hutool/setting/**

use hutool_setting::{
    DEFAULT_ENCODING, GroupedMap, Props, PropsUtil, Setting, SettingUtil, YamlUtil,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Resolves a file under `tests/resources/` (Hutool classpath fixtures).
fn resource(name: impl AsRef<Path>) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources")
        .join(name)
}

// ---------------------------------------------------------------------------
// Smoke helpers (pre-existing) —— keep; do not delete.
// ---------------------------------------------------------------------------

#[test]
fn grouped_map_put_get_test() {
    let mut map = GroupedMap::new();
    map.put("group", "key1", "value1");
    assert_eq!(map.get("group", "key1"), Some("value1"));
    assert!(map.get("group", "nonexistent").is_none());
}

#[test]
fn grouped_map_groups_test() {
    let mut map = GroupedMap::new();
    map.put("db", "host", "localhost");
    map.put("app", "name", "test");
    let groups: Vec<&str> = map.groups().collect();
    assert!(groups.contains(&"db"));
    assert!(groups.contains(&"app"));
}

#[test]
fn grouped_map_remove_test() {
    let mut map = GroupedMap::new();
    map.put("g", "k", "v");
    assert_eq!(map.remove("g", "k"), Some("v".to_string()));
    assert!(map.get("g", "k").is_none());
}

#[test]
fn grouped_map_contains_test() {
    let mut map = GroupedMap::new();
    map.put("db", "host", "localhost");
    assert!(map.contains_key("db", "host"));
    assert!(map.contains_value("db", "localhost"));
    assert!(!map.contains_key("db", "missing"));
}

#[test]
fn grouped_map_size_test() {
    let mut map = GroupedMap::new();
    map.put("g", "k1", "v1");
    map.put("g", "k2", "v2");
    assert_eq!(map.size(), 2);
}

#[test]
fn setting_loader_test() {
    let loader = hutool_setting::SettingsLoader::new();
    // 验证 builder 模式
    let _ = loader.required_file("/tmp/nonexistent.toml");
}

// ---------------------------------------------------------------------------
// SettingTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `SettingTest.settingTest()`
#[test]
fn setting_test() {
    let setting = Setting::from_path_with_options(resource("test.setting"), DEFAULT_ENCODING, true)
        .expect("load test.setting");

    let driver = setting.get_by_group("driver", "demo");
    assert_eq!(driver.as_deref(), Some("com.mysql.jdbc.Driver"));

    // 本分组变量替换
    let user = setting.get_by_group("user", "demo");
    assert_eq!(user.as_deref(), Some("rootcom.mysql.jdbc.Driver"));

    // 跨分组变量替换
    let user2 = setting.get_by_group("user2", "demo");
    assert_eq!(user2.as_deref(), Some("rootcom.mysql.jdbc.Driver"));

    // 默认值测试（对齐 AbsSetting.getStr(key, defaultValue)）
    let value = setting.get_or("keyNotExist", "", "defaultTest");
    assert_eq!(value, "defaultTest");
}

/// 对齐 Java: `SettingTest.settingTestForAbsPath()`（Java 侧 `@Disabled`，改为可断言的绝对路径加载）
#[test]
fn setting_test_for_abs_path() {
    let abs = resource("test.setting");
    assert!(abs.is_absolute());
    let setting =
        Setting::from_path_with_options(&abs, DEFAULT_ENCODING, true).expect("absolute path load");
    assert_eq!(
        setting.get_by_group("driver", "demo").as_deref(),
        Some("com.mysql.jdbc.Driver")
    );
}

/// 对齐 Java: `SettingTest.settingTestForCustom()`
#[test]
fn setting_test_for_custom() {
    let setting = Setting::new();

    setting.put_by_group("user", "group1", "root");
    setting.put_by_group("user", "group2", "root2");
    setting.put_by_group("user", "group3", "root3");
    setting.set("user", "root4");

    assert_eq!(
        setting.get_by_group("user", "group1").as_deref(),
        Some("root")
    );
    assert_eq!(
        setting.get_by_group("user", "group2").as_deref(),
        Some("root2")
    );
    assert_eq!(
        setting.get_by_group("user", "group3").as_deref(),
        Some("root3")
    );
    assert_eq!(setting.get("user").as_deref(), Some("root4"));
}

/// 对齐 Java: `SettingTest.storeTest()`
///
/// Java 写回 classpath 资源；此处写入临时目录，避免污染 fixtures。
#[test]
fn store_test() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("test.setting");
    let bytes = std::fs::read(resource("test.setting")).expect("read fixture");
    std::fs::write(&path, bytes).expect("write temp fixture");

    let setting = Setting::from_path(&path).expect("load copy");
    setting.set("testKey", "testValue");
    setting.store(&path).expect("store");

    let reloaded = Setting::from_path(&path).expect("reload");
    assert_eq!(reloaded.get("testKey").as_deref(), Some("testValue"));
}

// ---------------------------------------------------------------------------
// SettingUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `SettingUtilTest.getTest()`
#[test]
fn setting_util_get_test() {
    let driver = SettingUtil::get(resource("test"))
        .expect("SettingUtil::get(test)")
        .get_by_group("driver", "demo");
    assert_eq!(driver.as_deref(), Some("com.mysql.jdbc.Driver"));
}

/// 对齐 Java: `SettingUtilTest.getTest2()`
#[test]
fn setting_util_get_test2() {
    let value = SettingUtil::get(resource("example/example"))
        .expect("SettingUtil::get(example/example)")
        .get_by_group("key", "demo");
    assert_eq!(value.as_deref(), Some("value"));
}

/// 对齐 Java: `SettingUtilTest.getFirstFoundTest()`
#[test]
fn setting_util_get_first_found_test() {
    let driver = SettingUtil::get_first_found([resource("test2"), resource("test")])
        .expect("get_first_found")
        .expect("found test.setting")
        .get_by_group("driver", "demo");
    assert_eq!(driver.as_deref(), Some("com.mysql.jdbc.Driver"));
}

// ---------------------------------------------------------------------------
// PropsTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `PropsTest.propTest()`
#[test]
fn prop_test() {
    let props = Props::from_path(resource("test.properties")).expect("load test.properties");
    let user = props.get_str("user");
    assert_eq!(user, Some("root"));

    let driver = props.get_str("driver");
    assert_eq!(driver, Some("com.mysql.jdbc.Driver"));
}

/// 对齐 Java: `PropsTest.propTestForAbsPAth()`（Java 侧 `@Disabled`，改为可断言的绝对路径加载）
#[test]
fn prop_test_for_abs_path() {
    let abs = resource("test.properties");
    assert!(abs.is_absolute());
    let props = Props::from_path(&abs).expect("absolute path load");
    assert_eq!(props.get_str("user"), Some("root"));
    assert_eq!(props.get_str("driver"), Some("com.mysql.jdbc.Driver"));
}

#[derive(Debug, Deserialize, PartialEq)]
struct ConfigProperties {
    host: String,
    port: i32,
    from: String,
    credentials: Credentials,
    #[serde(rename = "defaultRecipients")]
    default_recipients: Vec<String>,
    #[serde(rename = "additionalHeaders")]
    additional_headers: HashMap<String, String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Credentials {
    #[serde(rename = "authMethod")]
    auth_method: String,
    username: String,
    password: String,
}

/// 对齐 Java: `PropsTest.toBeanTest()`
#[test]
fn props_to_bean_test() {
    let props = PropsUtil::get(resource("to_bean_test")).expect("PropsUtil::get(to_bean_test)");
    let cfg: ConfigProperties = props.to_bean(Some("mail")).expect("to_bean(mail)");

    assert_eq!(cfg.host, "mailer@mail.com");
    assert_eq!(cfg.port, 9000);
    assert_eq!(cfg.from, "mailer@mail.com");

    assert_eq!(cfg.credentials.username, "john");
    assert_eq!(cfg.credentials.password, "password");
    assert_eq!(cfg.credentials.auth_method, "SHA1");

    assert_eq!(
        cfg.additional_headers.get("redelivery").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        cfg.additional_headers.get("secure").map(String::as_str),
        Some("true")
    );

    assert_eq!(cfg.default_recipients[0], "admin@mail.com");
    assert_eq!(cfg.default_recipients[1], "owner@mail.com");
}

#[derive(Debug, Deserialize, PartialEq)]
struct SystemConfig {
    #[serde(rename = "isInit")]
    is_init: bool,
    #[serde(rename = "createTime")]
    create_time: String,
    version: String,
    #[serde(rename = "stairPlan")]
    stair_plan: String,
    #[serde(rename = "stageNum")]
    stage_num: i32,
}

/// 对齐 Java: `PropsTest.toBeanWithNullPrefixTest()`
#[test]
fn to_bean_with_null_prefix_test() {
    let mut config_prop = Props::new();
    // Java: setProperty("createTime", DateUtil.parse("2020-01-01")) → string then toBean
    config_prop.set_property("createTime", "2020-01-01");
    config_prop.set_property("isInit", true);
    config_prop.set_property("stairPlan", 1);
    config_prop.set_property("stageNum", 2);
    config_prop.set_property("version", 3);

    let system_config: SystemConfig = config_prop.to_bean(None).expect("to_bean(null prefix)");

    assert_eq!(system_config.create_time, "2020-01-01");
    assert!(system_config.is_init);
    assert_eq!(system_config.stair_plan, "1");
    assert_eq!(system_config.stage_num, 2);
    assert_eq!(system_config.version, "3");
}

// ---------------------------------------------------------------------------
// PropsUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `PropsUtilTest.getTest()`
#[test]
fn props_util_get_test() {
    let props = PropsUtil::get(resource("test")).expect("PropsUtil::get(test)");
    assert_eq!(props.get_str("driver"), Some("com.mysql.jdbc.Driver"));
}

/// 对齐 Java: `PropsUtilTest.getFirstFoundTest()`
#[test]
fn props_util_get_first_found_test() {
    let props = PropsUtil::get_first_found([resource("test2"), resource("test")])
        .expect("get_first_found")
        .expect("found test.properties");
    assert_eq!(props.get_str("driver"), Some("com.mysql.jdbc.Driver"));
}

// ---------------------------------------------------------------------------
// Issue3008Test
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, PartialEq)]
struct MyUser {
    hobby: Vec<String>,
}

/// 对齐 Java: `Issue3008Test.toBeanTest()`
#[test]
fn issue3008_to_bean_test() {
    let props = PropsUtil::get(resource("issue3008")).expect("PropsUtil::get(issue3008)");
    let user: MyUser = props.to_bean(Some("person")).expect("to_bean(person)");
    // Java: ArrayUtil.toString(user.getHobby()) → "[LOL, KFC, COFFE]"
    assert_eq!(
        format!("{:?}", user.hobby),
        format!("{:?}", ["LOL", "KFC", "COFFE"])
    );
    assert_eq!(user.hobby, vec!["LOL", "KFC", "COFFE"]);
}

// ---------------------------------------------------------------------------
// IssueI7G34ETest
// ---------------------------------------------------------------------------

/// 对齐 Java: `IssueI7G34ETest.readWithBomTest()`
#[test]
fn read_with_bom_test() {
    let setting = Setting::from_path(resource("test_with_bom.setting")).expect("BOM setting");
    // Java: setting.get("line1", "key1") —— group, key
    let s = setting.get_by_group("key1", "line1");
    assert_eq!(s.as_deref(), Some("value1"));
}

// ---------------------------------------------------------------------------
// YamlUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `YamlUtilTest.loadByPathTest()`
#[test]
fn load_by_path_test() {
    let result = YamlUtil::load_by_path(resource("test.yaml")).expect("load test.yaml");

    assert_eq!(result["firstName"].as_str(), Some("John"));

    // Java Dict.getByPath("contactDetails.number") 收集列表项字段
    let numbers = result["contactDetails"]
        .as_sequence()
        .expect("contactDetails sequence");
    assert_eq!(numbers[0]["number"].as_i64(), Some(123_456_789));
    assert_eq!(numbers[1]["number"].as_i64(), Some(456_786_868));
}

/// 对齐 Java: `YamlUtilTest.dumpTest()`（Java 侧 `@Disabled` 写死盘路径；改为临时文件可断言）
#[test]
fn dump_test() {
    let mut dict = HashMap::new();
    dict.insert("name", serde_yaml_ng::Value::String("hutool".into()));
    dict.insert("count", serde_yaml_ng::Value::Number(1000.into()));

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("dump.yaml");
    let file = std::fs::File::create(&path).expect("create dump.yaml");
    YamlUtil::dump(&dict, file).expect("YamlUtil::dump");

    let text = std::fs::read_to_string(&path).expect("read dump");
    assert!(text.contains("hutool"), "dump should contain name=hutool");
    assert!(text.contains("1000"), "dump should contain count=1000");
}
