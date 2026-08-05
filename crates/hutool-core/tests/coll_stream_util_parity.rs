//! `CollStreamUtil` 对比验证测试 —— 对齐 Hutool `CollStreamUtilTest`
//!
//! 对齐: `cn.hutool.core.collection.CollStreamUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/CollStreamUtilTest.java

use hutool_core::CollStreamUtil;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Student {
    term_id: i64,
    class_id: i64,
    student_id: i64,
    name: Option<String>,
}

impl Student {
    fn new(term_id: i64, class_id: i64, student_id: i64, name: &str) -> Self {
        Self {
            term_id,
            class_id,
            student_id,
            name: Some(name.to_string()),
        }
    }
}

/// 对齐 Java: `CollStreamUtilTest.testToIdentityMap()`
#[test]
fn test_to_identity_map() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::to_identity_map(empty, |s| s.student_id).is_empty());
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 1, 2, "李四"),
        Student::new(1, 1, 3, "王五"),
    ];
    let map = CollStreamUtil::to_identity_map(list, |s| s.student_id);
    assert_eq!(map.get(&1).and_then(|s| s.name.as_deref()), Some("张三"));
    assert_eq!(map.get(&2).and_then(|s| s.name.as_deref()), Some("李四"));
    assert_eq!(map.get(&3).and_then(|s| s.name.as_deref()), Some("王五"));
}

/// 对齐 Java: `CollStreamUtilTest.testToMap()`
#[test]
fn test_to_map() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::to_map(empty, |s| s.student_id, |s| s.name.clone()).is_empty());
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 1, 2, "李四"),
        Student::new(1, 1, 3, "王五"),
    ];
    let map = CollStreamUtil::to_map(list, |s| s.student_id, |s| s.name.clone());
    assert_eq!(map.get(&1).cloned().flatten().as_deref(), Some("张三"));
    assert_eq!(map.get(&2).cloned().flatten().as_deref(), Some("李四"));
}

/// 对齐 Java: `CollStreamUtilTest.testToMap_KeyCollision_SilentlyOverwrite()`
#[test]
fn test_to_map_key_collision_silently_overwrite() {
    let list = vec![
        Student::new(1, 101, 1, "张三"),
        Student::new(1, 102, 1, "李四"),
    ];
    let map = CollStreamUtil::to_map(list, |s| s.student_id, |s| s.name.clone());
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1).cloned().flatten().as_deref(), Some("李四"));
}

/// 对齐 Java: `CollStreamUtilTest.testToMap_NullKeyOrValue()`
#[test]
fn test_to_map_null_key_or_value() {
    // Rust 无 Option 学生建模 null：跳过 None 元素避免 panic，对齐“空值需处理”
    let list = vec![
        Some(Student::new(1, 1, 1, "张三")),
        None,
        Some(Student {
            term_id: 1,
            class_id: 2,
            student_id: 2,
            name: None,
        }),
    ];
    let map = CollStreamUtil::to_map(
        list.into_iter().flatten(),
        |s| s.student_id,
        |s| s.name.clone(),
    );
    assert_eq!(map.get(&1).cloned().flatten().as_deref(), Some("张三"));
    assert_eq!(map.get(&2).cloned().flatten(), None);
}

/// 对齐 Java: `CollStreamUtilTest.testToMap_LargeInputPerformance()`
#[test]
fn test_to_map_large_input_performance() {
    let list: Vec<_> = (0..10_000)
        .map(|i| Student::new(1, 1, i, &format!("学生{i}")))
        .collect();
    let map = CollStreamUtil::to_map(list, |s| s.student_id, |s| s.name.clone());
    assert_eq!(map.len(), 10_000);
}

/// 对齐 Java: `CollStreamUtilTest.testGroupByKey()`
#[test]
fn test_group_by_key() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::group_by_key(empty, |s| s.class_id).is_empty());
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 2, "李四"),
        Student::new(2, 1, 1, "擎天柱"),
        Student::new(2, 2, 2, "威震天"),
        Student::new(2, 3, 2, "霸天虎"),
    ];
    let map = CollStreamUtil::group_by_key(list, |s| s.class_id);
    assert_eq!(map.get(&1).unwrap().len(), 2);
    assert_eq!(map.get(&2).unwrap().len(), 2);
    assert_eq!(map.get(&3).unwrap().len(), 1);
}

/// 对齐 Java: `CollStreamUtilTest.testGroupBy2Key()`
#[test]
fn test_group_by2_key() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::group_by_two_keys(empty, |s| s.term_id, |s| s.class_id).is_empty());
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 2, "李四"),
        Student::new(1, 2, 3, "王五"),
        Student::new(2, 1, 1, "擎天柱"),
        Student::new(2, 2, 2, "威震天"),
        Student::new(2, 2, 3, "霸天虎"),
    ];
    let map = CollStreamUtil::group_by_two_keys(list, |s| s.term_id, |s| s.class_id);
    assert_eq!(map.get(&1).unwrap().get(&1).unwrap().len(), 1);
    assert_eq!(map.get(&1).unwrap().get(&2).unwrap().len(), 2);
    assert_eq!(map.get(&2).unwrap().get(&2).unwrap().len(), 2);
}

/// 对齐 Java: `CollStreamUtilTest.testGroup2Map()`
#[test]
fn test_group2_map() {
    let empty: Vec<Student> = vec![];
    assert!(
        CollStreamUtil::group_to_two_level_map(empty, |s| s.term_id, |s| s.class_id).is_empty()
    );
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 1, "李四"),
        Student::new(2, 2, 1, "王五"),
    ];
    let map = CollStreamUtil::group_to_two_level_map(list, |s| s.term_id, |s| s.class_id);
    assert_eq!(
        map.get(&1).unwrap().get(&1).unwrap().name.as_deref(),
        Some("张三")
    );
    assert_eq!(
        map.get(&1).unwrap().get(&2).unwrap().name.as_deref(),
        Some("李四")
    );
    assert_eq!(
        map.get(&2).unwrap().get(&2).unwrap().name.as_deref(),
        Some("王五")
    );
}

/// 对齐 Java: `CollStreamUtilTest.testGroupKeyValue()`
#[test]
fn test_group_key_value() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::group_key_value(empty, |s| s.term_id, |s| s.class_id).is_empty());
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 1, "李四"),
        Student::new(2, 2, 1, "王五"),
    ];
    let map = CollStreamUtil::group_key_value(list, |s| s.term_id, |s| s.class_id);
    assert_eq!(map.get(&1), Some(&vec![1, 2]));
    assert_eq!(map.get(&2), Some(&vec![2]));
}

/// 对齐 Java: `CollStreamUtilTest.testGroupBy()`
#[test]
fn test_group_by() {
    let list = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 1, "李四"),
        Student::new(2, 2, 1, "王五"),
    ];
    // group_fold 对齐 groupBy + collector
    let max_by_class = CollStreamUtil::group_fold(
        list.clone(),
        |s| s.term_id,
        || None::<Student>,
        |acc, s| match acc {
            None => *acc = Some(s),
            Some(cur) if s.class_id > cur.class_id => *acc = Some(s),
            _ => {}
        },
    );
    assert_eq!(
        max_by_class
            .get(&1)
            .unwrap()
            .as_ref()
            .unwrap()
            .name
            .as_deref(),
        Some("李四")
    );
    let counts = CollStreamUtil::group_fold(list, |s| s.term_id, || 0usize, |c, _| *c += 1);
    assert_eq!(counts.get(&1), Some(&2));
    assert_eq!(counts.get(&2), Some(&1));
}

/// 对齐 Java: `CollStreamUtilTest.testTranslate2List()`
#[test]
fn test_translate2_list() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::filter_map_to_list(empty, |s| s.name).is_empty());
    let students = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 2, "李四"),
        Student::new(2, 1, 1, "李四"),
        Student::new(2, 2, 2, "李四"),
        Student::new(2, 3, 2, "霸天虎"),
    ];
    let list = CollStreamUtil::filter_map_to_list(students, |s| s.name);
    assert_eq!(list, vec!["张三", "李四", "李四", "李四", "霸天虎"]);
}

/// 对齐 Java: `CollStreamUtilTest.testTranslate2Set()`
#[test]
fn test_translate2_set() {
    let empty: Vec<Student> = vec![];
    assert!(CollStreamUtil::filter_map_to_set(empty, |s| s.name).is_empty());
    let students = vec![
        Student::new(1, 1, 1, "张三"),
        Student::new(1, 2, 2, "李四"),
        Student::new(2, 1, 1, "李四"),
        Student::new(2, 2, 2, "李四"),
        Student::new(2, 3, 2, "霸天虎"),
    ];
    let set = CollStreamUtil::filter_map_to_set(students, |s| s.name);
    assert_eq!(set.len(), 3);
    assert!(set.contains("张三"));
    assert!(set.contains("李四"));
    assert!(set.contains("霸天虎"));
}

/// 对齐 Java: `CollStreamUtilTest.testMerge()`
#[test]
fn test_merge() {
    let map1: HashMap<i64, Student> = HashMap::new();
    let map2: HashMap<i64, Student> = HashMap::new();
    let merged = CollStreamUtil::merge_maps(&map1, &map2, |_k, a, b| match (a, b) {
        (None, None) => None,
        (Some(s), None) | (None, Some(s)) => s.name.clone(),
        (Some(s1), Some(s2)) => Some(format!(
            "{}{}",
            s1.name.as_deref().unwrap_or(""),
            s2.name.as_deref().unwrap_or("")
        )),
    });
    assert!(merged.is_empty());

    let mut map1 = HashMap::new();
    map1.insert(1, Student::new(1, 1, 1, "张三"));
    let map2: HashMap<i64, Student> = HashMap::new();
    let merged = CollStreamUtil::merge_maps(&map1, &map2, |_k, a, b| match (a, b) {
        (None, None) => None,
        (Some(s), None) | (None, Some(s)) => s.name.clone(),
        (Some(s1), Some(s2)) => Some(format!(
            "{}{}",
            s1.name.as_deref().unwrap_or(""),
            s2.name.as_deref().unwrap_or("")
        )),
    });
    assert_eq!(merged.get(&1).map(|s| s.as_str()), Some("张三"));

    let mut map2 = HashMap::new();
    map2.insert(1, Student::new(2, 1, 1, "李四"));
    let merged = CollStreamUtil::merge_maps(&map1, &map2, |_k, a, b| match (a, b) {
        (None, None) => None,
        (Some(s), None) | (None, Some(s)) => s.name.clone(),
        (Some(s1), Some(s2)) => Some(format!(
            "{}{}",
            s1.name.as_deref().unwrap_or(""),
            s2.name.as_deref().unwrap_or("")
        )),
    });
    assert_eq!(merged.get(&1).map(|s| s.as_str()), Some("张三李四"));
}
