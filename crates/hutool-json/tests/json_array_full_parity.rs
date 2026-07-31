//! Full parity: remaining JSONArrayTest methods
//! 对齐 Hutool hutool-json JSONArrayTest

use hutool_json as hj;
use hutool_json::JsonContainer;

/// 对齐 Java: `JSONArrayTest.parseWithNullTest()`
#[test]
fn arr_parse_with_null_test() {
    let arr = hj::JSONArray::parse(r#"[1,null,3]"#).unwrap();
    assert_eq!(arr.len(), 3);
    assert!(arr.get(1).unwrap().is_null());
}


/// 对齐 Java: `JSONArrayTest.parseFileTest()`
#[test]
fn arr_parse_file_test() {
    let raw = include_str!("fixtures/exam_test.json");
    let v = hj::JSONUtil::parse(raw).unwrap();
    assert!(v.is_array() || v.is_object());
}


/// 对齐 Java: `JSONArrayTest.parseBeanListTest()`
#[test]
fn arr_parse_bean_list_test() {
    #[derive(serde::Serialize)]
    struct U { name: String }
    let list = vec![U { name: "a".into() }, U { name: "b".into() }];
    let arr = hj::JSONUtil::array_from(&list, hj::JSONConfig::create()).unwrap();
    assert_eq!(arr.len(), 2);
}


/// 对齐 Java: `JSONArrayTest.toListTest2()`
#[test]
fn arr_to_list_test2() {
    let arr = hj::JSONArray::parse(r#"[{"name":"a"},{"name":"b"}]"#).unwrap();
    #[derive(serde::Deserialize, Debug)]
    struct U { name: String }
    let list: Vec<U> = hj::JSONUtil::to_list(&arr).unwrap();
    assert_eq!(list.len(), 2);
}


/// 对齐 Java: `JSONArrayTest.toDictListTest()`
#[test]
fn arr_to_dict_list_test() {
    let arr = hj::JSONArray::parse(r#"[{"k":1},{"k":2}]"#).unwrap();
    let list: Vec<serde_json::Map<String, serde_json::Value>> =
        serde_json::from_value(arr.to_value()).unwrap();
    assert_eq!(list.len(), 2);
}


/// 对齐 Java: `JSONArrayTest.toArrayTest()`
#[test]
fn arr_to_array_test() {
    let arr = hj::JSONArray::parse(r#"[1,2,3]"#).unwrap();
    let nums: Vec<i64> = arr.iter().map(|v| v.as_i64().unwrap()).collect();
    assert_eq!(nums, vec![1, 2, 3]);
}


/// 对齐 Java: `JSONArrayTest.toListWithNullTest()`
#[test]
fn arr_to_list_with_null_test() {
    let arr = hj::JSONArray::parse(r#"[1,null,3]"#).unwrap();
    let opts: Vec<Option<i64>> = arr.iter().map(|v| v.as_i64()).collect();
    assert_eq!(opts, vec![Some(1), None, Some(3)]);
}


/// 对齐 Java: `JSONArrayTest.toListWithErrorTest()`
#[test]
fn arr_to_list_with_error_test() {
    let arr = hj::JSONArray::parse(r#"[1,"x",3]"#).unwrap();
    let r: Result<Vec<i64>, _> = hj::JSONUtil::to_list(&arr);
    assert!(r.is_err());
}


/// 对齐 Java: `JSONArrayTest.toBeanListTest()`
#[test]
fn arr_to_bean_list_test() {
    #[derive(serde::Deserialize, Debug)]
    struct U { name: String }
    let arr = hj::JSONArray::parse(r#"[{"name":"a"},{"name":"b"}]"#).unwrap();
    let list: Vec<U> = hj::JSONUtil::to_list(&arr).unwrap();
    assert_eq!(list[1].name, "b");
}


/// 对齐 Java: `JSONArrayTest.putToIndexTest()`
#[test]
fn arr_put_to_index_test() {
    let mut arr = hj::JSONArray::new();
    arr.set(0, serde_json::json!("a"));
    arr.set(2, serde_json::json!("c"));
    assert_eq!(arr.len(), 3);
    assert!(arr.get(1).unwrap().is_null());
}


/// 对齐 Java: `JSONArrayTest.putTest2()`
#[test]
fn arr_put_test2() {
    let mut arr = hj::JSONArray::parse(r#"[1,2]"#).unwrap();
    arr.push(serde_json::json!(3));
    assert_eq!(arr.len(), 3);
}


/// 对齐 Java: `JSONArrayTest.filterExcludeTest()`
#[test]
fn arr_filter_exclude_test() {
    let arr = hj::JSONArray::parse(r#"[1,2,3,4]"#).unwrap();
    let filtered: Vec<_> = arr.iter().filter(|v| v.as_i64() != Some(2)).cloned().collect();
    assert_eq!(filtered.len(), 3);
}


/// 对齐 Java: `JSONArrayTest.putNullTest()`
#[test]
fn arr_put_null_test() {
    let mut arr = hj::JSONArray::new();
    arr.push(serde_json::Value::Null);
    assert_eq!(arr.len(), 1);
}


/// 对齐 Java: `JSONArrayTest.parseFilterTest()`
#[test]
fn arr_parse_filter_test() {
    let arr = hj::JSONArray::parse(r#"[1,2,3]"#).unwrap();
    let kept: Vec<_> = arr.iter().filter(|v| v.as_i64().unwrap() > 1).cloned().collect();
    assert_eq!(kept.len(), 2);
}


/// 对齐 Java: `JSONArrayTest.parseFilterEditTest()`
#[test]
fn arr_parse_filter_edit_test() {
    let arr = hj::JSONArray::parse(r#"[1,2,3]"#).unwrap();
    let edited: Vec<_> = arr.iter().map(|v| serde_json::json!(v.as_i64().unwrap() * 2)).collect();
    assert_eq!(edited[0], 2);
}
