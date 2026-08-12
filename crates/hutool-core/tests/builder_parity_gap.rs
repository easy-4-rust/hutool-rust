//! `cn.hutool.core.builder` 缺口 parity
//!
//! 对齐: `cn.hutool.core.builder.GenericBuilderTest`
//! 基于 hutool-core `GenericBuilder` 真实实现。

use hutool_core::GenericBuilder;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct BoxObj {
    id: i64,
    title: String,
    length: i32,
    width: i32,
    height: i32,
    title_alias: String,
}

impl BoxObj {
    /// 多参数构造（对齐 Java Box(Long, String, Integer, Integer, Integer)）。
    fn new5(id: i64, title: String, length: i32, width: i32, height: i32) -> Self {
        Self {
            id,
            title,
            length,
            width,
            height,
            title_alias: String::new(),
        }
    }

    /// 设置别名（对齐 Java Box.alis()）。
    fn alis(&mut self) {
        self.title_alias = format!("TomXin:\"{}\"", self.title);
    }
}

/// 对齐 Java: `GenericBuilderTest.test()`
#[test]
fn generic_builder_test() {
    let box_obj = GenericBuilder::of(BoxObj::default)
        .with(|b| b.id = 1024)
        .with(|b| b.title = "Hello World!".into())
        .with(|b| b.length = 9)
        .with(|b| b.width = 8)
        .with(|b| b.height = 7)
        .build();

    assert_eq!(box_obj.id, 1024);
    assert_eq!(box_obj.title, "Hello World!");
    assert_eq!(box_obj.length, 9);
    assert_eq!(box_obj.width, 8);
    assert_eq!(box_obj.height, 7);

    // 对象修改：复用已有实例再 with
    let box_modified = GenericBuilder::of(|| box_obj.clone())
        .with(|b| b.title = "Hello Friend!".into())
        .with(|b| b.length = 3)
        .with(|b| b.width = 4)
        .with(|b| b.height = 5)
        .build();

    assert_eq!(box_modified.id, 1024);
    assert_eq!(box_modified.title, "Hello Friend!");
    assert_eq!(box_modified.length, 3);
    assert_eq!(box_modified.width, 4);
    assert_eq!(box_modified.height, 5);

    // 多参数构造 + alis
    let box1 = GenericBuilder::of5(BoxObj::new5, 2048, "Hello Partner!".into(), 222, 333, 444)
        .with(BoxObj::alis)
        .build();

    assert_eq!(box1.id, 2048);
    assert_eq!(box1.title, "Hello Partner!");
    assert_eq!(box1.length, 222);
    assert_eq!(box1.width, 333);
    assert_eq!(box1.height, 444);
    assert_eq!(box1.title_alias, "TomXin:\"Hello Partner!\"");
}

/// 对齐 Java: `GenericBuilderTest.buildMapTest()`
#[test]
fn generic_builder_build_map_test() {
    let color_map = GenericBuilder::of(HashMap::<String, String>::new)
        .with(|m| {
            m.insert("red".into(), "#FF0000".into());
        })
        .with(|m| {
            m.insert("yellow".into(), "#FFFF00".into());
        })
        .with(|m| {
            m.insert("blue".into(), "#0000FF".into());
        })
        .build();

    assert_eq!(color_map.get("red").map(String::as_str), Some("#FF0000"));
    assert_eq!(color_map.get("yellow").map(String::as_str), Some("#FFFF00"));
    assert_eq!(color_map.get("blue").map(String::as_str), Some("#0000FF"));
}
