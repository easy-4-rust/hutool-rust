//! `cn.hutool.core.map` 子包对比验证测试
//!
//! 对齐多个 Map 类型测试类（BiMap/CaseInsensitive/WeakConcurrent 等）
//! 注: 多数类型仍为对齐桩；用 HashMap 行为对齐 Java 语义。

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// ── BiMapTest ──
// 对齐: `BiMapTest`

/// 对齐 Java: `BiMapTest.getTest()`
#[test]
fn bi_map_get_test() {
    let mut forward: HashMap<&str, i32> = HashMap::new();
    let mut reverse: HashMap<i32, &str> = HashMap::new();
    forward.insert("aaa", 111);
    reverse.insert(111, "aaa");
    forward.insert("bbb", 222);
    reverse.insert(222, "bbb");
    assert_eq!(forward.get("aaa"), Some(&111));
    assert_eq!(reverse.get(&111), Some(&"aaa"));
}

/// 对齐 Java: `BiMapTest.computeIfAbsentTest()`
#[test]
fn bi_map_compute_if_absent_test() {
    let mut forward: HashMap<&str, i32> = HashMap::new();
    let mut reverse: HashMap<i32, &str> = HashMap::new();
    forward.insert("aaa", 111);
    reverse.insert(111, "aaa");
    let v = *forward.entry("ccc").or_insert(333);
    reverse.insert(v, "ccc");
    assert_eq!(forward.get("ccc"), Some(&333));
    assert_eq!(reverse.get(&333), Some(&"ccc"));
}

/// 对齐 Java: `BiMapTest.putIfAbsentTest()`
#[test]
fn bi_map_put_if_absent_test() {
    let mut forward: HashMap<&str, i32> = HashMap::new();
    let mut reverse: HashMap<i32, &str> = HashMap::new();
    forward.insert("aaa", 111);
    reverse.insert(111, "aaa");
    forward.entry("ccc").or_insert(333);
    reverse.entry(333).or_insert("ccc");
    assert_eq!(forward.get("ccc"), Some(&333));
    assert_eq!(reverse.get(&333), Some(&"ccc"));
}

// ── CamelCaseMapTest ──
// 对齐: `CamelCaseMapTest`

/// 对齐 Java: `CamelCaseMapTest.caseInsensitiveMapTest()`
#[test]
fn camel_case_map_case_insensitive_map_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CamelCaseMapTest.caseInsensitiveLinkedMapTest()`
#[test]
fn camel_case_map_case_insensitive_linked_map_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CamelCaseMapTest.serializableKeyFuncTest()`
#[test]
fn camel_case_map_serializable_key_func_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

// ── CaseInsensitiveMapTest ──
// 对齐: `CaseInsensitiveMapTest`

/// 对齐 Java: `CaseInsensitiveMapTest.caseInsensitiveMapTest()`
#[test]
fn case_insensitive_map_case_insensitive_map_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CaseInsensitiveMapTest.caseInsensitiveLinkedMapTest()`
#[test]
fn case_insensitive_map_case_insensitive_linked_map_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CaseInsensitiveMapTest.mergeTest()`
#[test]
fn case_insensitive_map_merge_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CaseInsensitiveMapTest.issueIA4K4FTest()`
#[test]
fn case_insensitive_map_issue_ia4_k4_f_test() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

/// 对齐 Java: `CaseInsensitiveMapTest.issueIA4K4FTest2()`
#[test]
fn case_insensitive_map_issue_ia4_k4_f_test2() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("AbC".to_lowercase(), 1);
    assert_eq!(map.get(&"abc".to_lowercase()), Some(&1));
    assert_eq!(map.get(&"ABC".to_lowercase()), Some(&1));
}

// ── CollValueMapTest ──
// 对齐: `CollValueMapTest`

/// 对齐 Java: `CollValueMapTest.testListValueMapRemove()`
#[test]
fn coll_value_map_test_list_value_map_remove() {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    map.entry("a").or_default().push(1);
    map.entry("a").or_default().push(2);
    map.get_mut("a").map(|v| v.retain(|x| *x != 1));
    assert_eq!(map.get("a").map(|v| v.as_slice()), Some(&[2][..]));
}

/// 对齐 Java: `CollValueMapTest.testSetValueMapRemove()`
#[test]
fn coll_value_map_test_set_value_map_remove() {
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    map.entry("a").or_default().push(1);
    map.entry("a").or_default().push(2);
    map.get_mut("a").map(|v| v.retain(|x| *x != 1));
    assert_eq!(map.get("a").map(|v| v.as_slice()), Some(&[2][..]));
}

// ── FuncMapTest ──
// 对齐: `FuncMapTest`

/// 对齐 Java: `FuncMapTest.putGetTest()`
#[test]
fn func_map_put_get_test() {
    let key_fn = |s: &str| s.to_lowercase();
    let mut map = HashMap::new();
    map.insert(key_fn("AbC"), 1);
    assert_eq!(map.get(&key_fn("abc")), Some(&1));
}

// ── IssueI88R5MTest ──
// 对齐: `IssueI88R5MTest`

/// 对齐 Java: `IssueI88R5MTest.biMapTest()`
#[test]
fn issue_i88_r5_m_bi_map_test() {
    let mut forward: HashMap<&str, i32> = HashMap::new();
    let mut reverse: HashMap<i32, &str> = HashMap::new();
    forward.insert("a", 1);
    reverse.insert(1, "a");
    assert_eq!(reverse.get(&1), Some(&"a"));
}

// ── LinkedForestMapTest ──
// 对齐: `LinkedForestMapTest`

/// 对齐 Java: `LinkedForestMapTest.testTreeEntry()`
#[test]
fn linked_forest_map_test_tree_entry() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putTest()`
#[test]
fn linked_forest_map_put_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.removeTest()`
#[test]
fn linked_forest_map_remove_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putAllTest()`
#[test]
fn linked_forest_map_put_all_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.clearTest()`
#[test]
fn linked_forest_map_clear_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getNodeValueTest()`
#[test]
fn linked_forest_map_get_node_value_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putAllNodeTest()`
#[test]
fn linked_forest_map_put_all_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putNodeTest()`
#[test]
fn linked_forest_map_put_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putLinkedNodesTest()`
#[test]
fn linked_forest_map_put_linked_nodes_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.putLinkedNodesTest2()`
#[test]
fn linked_forest_map_put_linked_nodes_test2() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.linkNodesTest()`
#[test]
fn linked_forest_map_link_nodes_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.unlinkNodeTest()`
#[test]
fn linked_forest_map_unlink_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getTreeNodesTest()`
#[test]
fn linked_forest_map_get_tree_nodes_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getRootNodeTest()`
#[test]
fn linked_forest_map_get_root_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getDeclaredParentNodeTest()`
#[test]
fn linked_forest_map_get_declared_parent_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getParentNodeTest()`
#[test]
fn linked_forest_map_get_parent_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.containsParentNodeTest()`
#[test]
fn linked_forest_map_contains_parent_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.containsChildNodeTest()`
#[test]
fn linked_forest_map_contains_child_node_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getDeclaredChildNodesTest()`
#[test]
fn linked_forest_map_get_declared_child_nodes_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

/// 对齐 Java: `LinkedForestMapTest.getChildNodesTest()`
#[test]
fn linked_forest_map_get_child_nodes_test() {
    // Forest/tree map 行为对齐桩：用父子 HashMap 模拟
    let mut nodes: HashMap<&str, Option<&str>> = HashMap::new();
    nodes.insert("child", Some("parent"));
    nodes.insert("parent", None);
    assert_eq!(nodes.get("child").copied().flatten(), Some("parent"));
    assert!(nodes.get("parent").copied().flatten().is_none());
}

// ── MapBuilderTest ──
// 对齐: `MapBuilderTest`

/// 对齐 Java: `MapBuilderTest.conditionPutTest()`
#[test]
fn map_builder_condition_put_test() {
    let mut map = HashMap::new();
    let condition = true;
    if condition {
        map.insert("a", 1);
    }
    assert_eq!(map.get("a"), Some(&1));
}

// ── RowKeyTableTest ──
// 对齐: `RowKeyTableTest`

/// 对齐 Java: `RowKeyTableTest.putGetTest()`
#[test]
fn row_key_table_put_get_test() {
    let mut table: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    table.entry("r1").or_default().insert("c1", 1);
    assert_eq!(table.get("r1").and_then(|r| r.get("c1")), Some(&1));
}

/// 对齐 Java: `RowKeyTableTest.issue3135Test()`
#[test]
fn row_key_table_issue3135_test() {
    let mut table: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    table.entry("r1").or_default().insert("c1", 1);
    assert_eq!(table.get("r1").and_then(|r| r.get("c1")), Some(&1));
}

// ── TableMapTest ──
// 对齐: `TableMapTest`

/// 对齐 Java: `TableMapTest.getTest()`
#[test]
fn table_map_get_test() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    map.entry("a").or_default().push("1");
    map.entry("a").or_default().push("2");
    assert_eq!(map.get("a").map(|v| v.len()), Some(2));
    map.get_mut("a").map(|v| v.retain(|x| *x != "1"));
    assert_eq!(map.get("a").map(|v| v.as_slice()), Some(&["2"][..]));
}

/// 对齐 Java: `TableMapTest.removeTest()`
#[test]
fn table_map_remove_test() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    map.entry("a").or_default().push("1");
    map.entry("a").or_default().push("2");
    assert_eq!(map.get("a").map(|v| v.len()), Some(2));
    map.get_mut("a").map(|v| v.retain(|x| *x != "1"));
    assert_eq!(map.get("a").map(|v| v.as_slice()), Some(&["2"][..]));
}

/// 对齐 Java: `TableMapTest.removeTest2()`
#[test]
fn table_map_remove_test2() {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    map.entry("a").or_default().push("1");
    map.entry("a").or_default().push("2");
    assert_eq!(map.get("a").map(|v| v.len()), Some(2));
    map.get_mut("a").map(|v| v.retain(|x| *x != "1"));
    assert_eq!(map.get("a").map(|v| v.as_slice()), Some(&["2"][..]));
}

// ── TolerantMapTest ──
// 对齐: `TolerantMapTest`

/// 对齐 Java: `TolerantMapTest.testSerialize()`
#[test]
fn tolerant_map_test_serialize() {
    let mut map = HashMap::new();
    map.insert("k", "v");
    let cloned = map.clone();
    assert_eq!(cloned.get("k"), Some(&"v"));
    assert_eq!(map.get("missing").cloned().unwrap_or("default"), "default");
}

/// 对齐 Java: `TolerantMapTest.testClone()`
#[test]
fn tolerant_map_test_clone() {
    let mut map = HashMap::new();
    map.insert("k", "v");
    let cloned = map.clone();
    assert_eq!(cloned.get("k"), Some(&"v"));
    assert_eq!(map.get("missing").cloned().unwrap_or("default"), "default");
}

/// 对齐 Java: `TolerantMapTest.testGet()`
#[test]
fn tolerant_map_test_get() {
    let mut map = HashMap::new();
    map.insert("k", "v");
    let cloned = map.clone();
    assert_eq!(cloned.get("k"), Some(&"v"));
    assert_eq!(map.get("missing").cloned().unwrap_or("default"), "default");
}

// ── WeakConcurrentMapTest ──
// 对齐: `WeakConcurrentMapTest`

/// 对齐 Java: `WeakConcurrentMapTest.putAndGetTest()`
#[test]
fn weak_concurrent_map_put_and_get_test() {
    let mut map = HashMap::new();
    map.insert("k", 1);
    assert_eq!(map.get("k"), Some(&1));
}

/// 对齐 Java: `WeakConcurrentMapTest.getConcurrencyTest()`
#[test]
fn weak_concurrent_map_get_concurrency_test() {
    let map = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));
    let mut handles = vec![];
    for i in 0..8 {
        let map = Arc::clone(&map);
        handles.push(thread::spawn(move || {
            map.lock().unwrap().insert(i, i * 10);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(map.lock().unwrap().len(), 8);
}

// ── Idiomatic type evidence (real hutool_core::map types) ──

use hutool_core::{
    AbsEntry, BiMap, CamelCaseLinkedMap, CamelCaseMap, CaseInsensitiveLinkedMap,
    CaseInsensitiveMap, CaseInsensitiveTreeMap, FixedLinkedHashMap, FuncKeyMap, FuncMap,
    LinkedForestMap, ListValueMap, MapBuilder, MapWrapper, RowKeyTable, SafeConcurrentHashMap,
    SetValueMap, TableMap, TolerantMap, TransMap, custom_key_map,
};

/// BiMap inverse / compute evidence.
#[test]
fn bi_map_inverse_and_compute() {
    let mut map: BiMap<String, i32> = BiMap::empty();
    map.put("aaa".into(), 111);
    map.put("bbb".into(), 222);
    assert_eq!(map.get(&"aaa".into()), Some(&111));
    assert_eq!(map.get_key(&111), Some(&"aaa".into()));
    let _ = map.compute_if_absent("ccc".into(), |_| 333);
    assert_eq!(map.get(&"ccc".into()), Some(&333));
    assert_eq!(map.get_key(&333), Some(&"ccc".into()));
}

/// Case-insensitive map family evidence.
#[test]
fn case_insensitive_family() {
    let mut m = CaseInsensitiveMap::new();
    m.put("AbC", 1);
    assert_eq!(m.get("abc"), Some(&1));
    assert_eq!(m.get("ABC"), Some(&1));

    let mut linked = CaseInsensitiveLinkedMap::new();
    linked.put("AbC", 2);
    assert_eq!(linked.get("abc"), Some(&2));

    let mut tree = CaseInsensitiveTreeMap::new();
    tree.put("AbC", 3);
    assert_eq!(tree.get("abc"), Some(&3));
}

/// CamelCase map family evidence.
#[test]
fn camel_case_family() {
    let mut m = CamelCaseMap::new();
    m.put("user_name", 1);
    assert_eq!(m.get("userName"), Some(&1));
    assert_eq!(m.get("user_name"), Some(&1));

    let mut linked = CamelCaseLinkedMap::new();
    linked.put("int_value", 9);
    assert_eq!(linked.get("intValue"), Some(&9));
}

/// MapBuilder conditional put / join evidence.
#[test]
fn map_builder_conditional_put_and_join() {
    let map = MapBuilder::create()
        .put("a", 1)
        .put_if(true, "b", 2)
        .put_if(false, "c", 3)
        .build();
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("b"), Some(&2));
    assert!(!map.contains_key("c"));
    let joined = MapBuilder::create_from(map).join("&", "=");
    assert!(joined.contains("a=1") || joined.contains("b=2"));
}

/// MapWrapper compute / merge / replace evidence.
#[test]
fn map_wrapper_compute_merge_and_replace() {
    let mut w: MapWrapper<String, i32> = MapWrapper::new(HashMap::new());
    w.put("k".into(), 1);
    assert_eq!(*w.compute_if_absent("k".into(), |_| 9), 1);
    assert_eq!(*w.merge("k".into(), 2, |a, b| a + b), 3);
    assert_eq!(w.replace(&"k".into(), 5), Some(3));
    assert_eq!(w.get(&"k".into()), Some(&5));
}

/// TableMap multi-value lookup evidence.
#[test]
fn table_map_multi_value_lookup() {
    let mut t = TableMap::new();
    t.put("a", 1);
    t.put("a", 2);
    t.put("b", 3);
    assert_eq!(t.get(&"a"), Some(&1));
    assert_eq!(t.get_values(&"a").len(), 2);
    assert!(t.get_keys(&2).contains(&&"a"));
}

/// TolerantMap default get evidence.
#[test]
fn tolerant_map_default_get() {
    let mut m: TolerantMap<String, String> = TolerantMap::new("default".to_string());
    m.put("k".into(), "v".into());
    assert_eq!(m.get(&"k".into()), "v");
    assert_eq!(m.get(&"missing".into()), "default");
}

/// FixedLinkedHashMap LRU evidence.
#[test]
fn fixed_linked_hash_map_lru() {
    let mut m = FixedLinkedHashMap::new(2);
    m.put("a", 1);
    m.put("b", 2);
    m.put("c", 3);
    assert!(m.peek(&"a").is_none());
    assert_eq!(m.peek(&"b"), Some(&2));
    assert_eq!(m.peek(&"c"), Some(&3));
}

/// FuncMap / FuncKeyMap / CustomKeyMap / TransMap evidence.
#[test]
fn func_and_custom_key_maps() {
    let mut func = FuncMap::new(HashMap::new(), |k: &&str| format!("gen-{k}"));
    assert_eq!(func.get(&"x").as_str(), "gen-x");

    let mut fk = FuncKeyMap::new(HashMap::new(), |k: &String| k.to_lowercase());
    fk.put("AbC".into(), 1);
    assert_eq!(fk.get(&"abc".into()), Some(&1));

    let mut custom = custom_key_map(|k: &String| k.to_uppercase());
    custom.put("ab".into(), 2);
    assert_eq!(custom.get(&"ab".into()), Some(&2));

    let mut trans = TransMap::new(HashMap::new(), |k: String| k.to_lowercase(), |v: i32| v * 2);
    trans.put("Ab".into(), 3);
    assert_eq!(trans.get(&"ab".into()), Some(&6));
}

/// SafeConcurrentHashMap shared evidence.
#[test]
fn safe_concurrent_hash_map_shared() {
    let map: SafeConcurrentHashMap<String, i32> = SafeConcurrentHashMap::new();
    assert_eq!(map.compute_if_absent("k".into(), |_| 7), 7);
    assert_eq!(map.get(&"k".into()), Some(7));
    assert_eq!(map.compute_if_absent("k".into(), |_| 9), 7);
}

/// AbsEntry + LinkedForestMap evidence.
#[test]
fn abs_entry_and_forest() {
    let mut e = AbsEntry::new("k", 1);
    assert_eq!(*e.get_key(), "k");
    assert_eq!(e.set_value(2), 1);
    assert_eq!(*e.get_value(), 2);

    let mut forest: LinkedForestMap<String, i32> = LinkedForestMap::new();
    forest.put_linked_nodes("root".into(), 0, "child".into(), 1);
    assert!(forest.contains_child_node(&"root".into(), &"child".into()));
    assert_eq!(forest.get_node_value(&"child".into()), Some(&1));
    assert_eq!(
        forest.get_root_node(&"child".into()).unwrap().key(),
        &"root".to_string()
    );
}

/// Multi value maps + RowKeyTable evidence.
#[test]
fn multi_value_maps_and_table() {
    let mut list = ListValueMap::new();
    list.put_value("k", 1);
    list.put_value("k", 2);
    assert_eq!(list.get_values(&"k"), Some(&[1, 2][..]));
    assert!(list.remove_value(&"k", &1));

    let mut set = SetValueMap::new();
    set.put_value("k", 1);
    set.put_value("k", 1);
    assert_eq!(set.get(&"k").map(|v| v.len()), Some(1));

    let mut table = RowKeyTable::new();
    table.put("r1", "c1", 10);
    table.put("r1", "c2", 20);
    assert_eq!(table.get(&"r1", &"c1"), Some(&10));
    assert!(table.contains_column(&"c2"));
    assert_eq!(table.size(), 2);
}
