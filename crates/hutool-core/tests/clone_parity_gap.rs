//! `cn.hutool.core.clone` 缺口 parity
//!
//! 对齐: `cn.hutool.core.clone.CloneTest` / `DefaultCloneTest`
//! 基于 hutool-core `CloneSupport` / `DefaultCloneable` / `Cloneable` 真实实现。

use hutool_core::{CloneSupport, DefaultCloneable};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cat {
    name: String,
    age: i32,
}

impl Cat {
    /// 对齐 Java Cat 默认字段。
    fn new() -> Self {
        Self {
            name: "miaomiao".into(),
            age: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dog {
    name: String,
    age: i32,
}

impl Dog {
    /// 对齐 Java Dog 默认字段。
    fn new() -> Self {
        Self {
            name: "wangwang".into(),
            age: 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wheel {
    direction: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Car {
    id: i32,
    wheel_list: Vec<Wheel>,
}

/// 对齐 Java: `CloneTest.cloneTest()`
#[test]
fn clone_test_clone_test() {
    // 实现 Cloneable（Rust Clone）方式
    let cat = Cat::new();
    let cat2: Cat = cat.clone();
    assert_eq!(cat, cat2);
}

/// 对齐 Java: `CloneTest.cloneTest2()`
#[test]
fn clone_test_clone_test2() {
    // 继承 CloneSupport 语义：包装后 clone
    let dog = CloneSupport::new(Dog::new());
    let dog2 = dog.clone();
    assert_eq!(*dog, *dog2);
    assert_eq!(dog.into_inner(), dog2.into_inner());
}

/// 对齐 Java: `DefaultCloneTest.clone0()`
#[test]
fn default_clone_test_clone0() {
    let old_car = Car {
        id: 1,
        wheel_list: vec![Wheel {
            direction: "h".into(),
        }],
    };
    let mut new_car = old_car.clone0();
    assert_eq!(old_car.id, new_car.id);
    assert_eq!(old_car.wheel_list, new_car.wheel_list);

    new_car.id = 2;
    assert_ne!(old_car.id, new_car.id);
    new_car.wheel_list.push(Wheel {
        direction: "s".into(),
    });
    assert_ne!(old_car.wheel_list.len(), new_car.wheel_list.len());
}
