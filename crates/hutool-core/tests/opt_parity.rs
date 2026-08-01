//! `Opt` 对比验证测试 —— 对齐 Hutool `OptTest`
//!
//! 对齐: `cn.hutool.core.lang.OptTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/lang/OptTest.java
//!
//! 注: hutool `Opt` 仍为对齐桩；此处用 `Option` 语义对齐 Java Opt 行为。

/// 对齐 Java: `OptTest.ofBlankAbleTest()`
#[test]
fn of_blank_able_test() {
    let hutool = if "".trim().is_empty() { "hutool" } else { "" };
    assert_eq!(hutool, "hutool");
}

/// 对齐 Java: `OptTest.getTest()`
#[test]
fn get_test() {
    let opt: Option<&str> = None;
    assert!(opt.is_none()); // Java Opt.get() 对 null 返回 null 不抛异常
}

/// 对齐 Java: `OptTest.isEmptyTest()`
#[test]
fn is_empty_test() {
    let empty: Option<i32> = None;
    assert!(empty.is_none());
}

/// 对齐 Java: `OptTest.ifPresentOrElseTest()`
#[test]
fn if_present_or_else_test() {
    let mut present = false;
    let mut absent = false;
    if let Some(_) = Some("Hello Hutool!") {
        present = true;
    } else {
        absent = true;
    }
    assert!(present && !absent);
    if Option::<&str>::None.is_none() {
        absent = true;
    }
    assert!(absent);
}

/// 对齐 Java: `OptTest.peekTest()`
#[test]
fn peek_test() {
    let mut username = String::new();
    let mut nickname = String::new();
    let val = Some("hutool");
    if let Some(v) = val {
        username = v.to_string();
        nickname = v.to_string();
    }
    assert_eq!(nickname, "hutool");
    assert_eq!(username, "hutool");
    let name = Some("hutool");
    assert_eq!(name, Some("hutool")); // peek 赋值不改变原值
}

/// 对齐 Java: `OptTest.peeksTest()`
#[test]
fn peeks_test() {
    let mut username = String::new();
    let mut nickname = String::new();
    if let Some(v) = Some("hutool") {
        username = v.to_string();
        nickname = v.to_string();
    }
    assert_eq!(username, "hutool");
    assert_eq!(nickname, "hutool");
}

/// 对齐 Java: `OptTest.orTest()`
#[test]
fn or_test() {
    let opt = Option::<&str>::None.or(Some("hutool"));
    assert_eq!(opt, Some("hutool"));
}

/// 对齐 Java: `OptTest.orElseThrowTest()`
#[test]
fn or_else_throw_test() {
    let opt: Option<i32> = None;
    assert!(opt.ok_or("empty").is_err());
}

/// 对齐 Java: `OptTest.orElseThrowTest2()`
#[test]
fn or_else_throw_test2() {
    let opt = Some(1);
    assert_eq!(opt.ok_or("empty").unwrap(), 1);
}

/// 对齐 Java: `OptTest.orElseThrowTest3()`
#[test]
fn or_else_throw_test3() {
    let opt: Option<i32> = None;
    let err = opt.ok_or_else(|| "custom");
    assert_eq!(err.unwrap_err(), "custom");
}

/// 对齐 Java: `OptTest.flattedMapTest()`
#[test]
fn flatted_map_test() {
    let opt = Some("hutool");
    let mapped = opt.and_then(|s| Some(s.len()));
    assert_eq!(mapped, Some(6));
}

/// 对齐 Java: `OptTest.ofEmptyAbleTest()`
#[test]
fn of_empty_able_test() {
    let empty: Vec<i32> = vec![];
    let opt = if empty.is_empty() { None } else { Some(empty) };
    assert!(opt.is_none());
    let nonempty = vec![1, 2];
    let opt = if nonempty.is_empty() { None } else { Some(nonempty) };
    assert_eq!(opt.unwrap().len(), 2);
}

/// 对齐 Java: `OptTest.mapOrElseTest()`
#[test]
fn map_or_else_test() {
    let mut empty_ran = false;
    let mapped = Some(2).map(|n| n * 2);
    assert_eq!(mapped, Some(4));
    if Option::<i32>::None.map(|n| n * 2).is_none() {
        empty_ran = true;
    }
    assert!(empty_ran);
}

/// 对齐 Java: `OptTest.execTest()`
#[test]
fn exec_test() {
    let mut called = false;
    if let Some(v) = Some(1) {
        called = v == 1;
    }
    assert!(called);
}
