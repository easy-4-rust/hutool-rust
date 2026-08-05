//! 对齐: `cn.hutool.core.bean.copier.BeanCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/BeanCopier.java
//!
//! 中文说明: Bean拷贝门面类，提供：
//! 1. Bean 转 Bean
//! 2. Bean 转 Map
//! 3. Map  转 Bean
//! 4. Map  转 Map
//!
//! 根据 source 和 target 的类型自动选择合适的拷贝器实现。

use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::abs_copier::AbsCopier;
use super::bean_to_bean_copier::BeanToBeanCopier;
use super::bean_to_map_copier::BeanToMapCopier;
use super::copy_options::CopyOptions;
use super::map_to_bean_copier::MapToBeanCopier;
use super::map_to_map_copier::MapToMapCopier;
use super::value_provider_to_bean_copier::ValueProviderToBeanCopier;

/// 对齐 Java 类: `cn.hutool.core.bean.copier.BeanCopier<T>`
///
/// 中文说明: Bean拷贝门面类，根据来源对象和目标对象的类型自动选择拷贝策略。
///
/// 支持的拷贝模式：
/// - Bean -> Bean（`BeanToBeanCopier`）
/// - Bean -> Map（`BeanToMapCopier`）
/// - Map  -> Bean（`MapToBeanCopier`）
/// - Map  -> Map（`MapToMapCopier`）
/// - ValueProvider -> Bean（`ValueProviderToBeanCopier`）
pub enum BeanCopier<S, T> {
    /// Bean 到 Bean 拷贝
    BeanToBean(BeanToBeanCopier<S, T>),
    /// Map 到 Bean 拷贝
    MapToBean(MapToBeanCopier<T>),
    /// ValueProvider 到 Bean 拷贝
    ProviderToBean(ValueProviderToBeanCopier<T>),
}

impl<S, T> BeanCopier<S, T>
where
    S: Serialize,
    T: Serialize + DeserializeOwned,
{
    /// 对齐 Java: `BeanCopier.create(Object source, T target, CopyOptions copyOptions)`
    ///
    /// 中文说明: 创建BeanCopier实例。根据 source 类型自动选择拷贝策略。
    ///
    /// - `source`: 来源对象（Bean）
    /// - `target`: 目标Bean对象
    /// - `copy_options`: 拷贝选项
    pub fn create(source: S, target: T, copy_options: CopyOptions) -> Self {
        BeanCopier::BeanToBean(BeanToBeanCopier::new(source, target, copy_options))
    }

    /// 对齐 Java: `T copy()`
    ///
    /// 中文说明: 执行拷贝操作，返回拷贝后的目标对象。
    pub fn copy(&self) -> T {
        match self {
            BeanCopier::BeanToBean(copier) => copier.copy(),
            BeanCopier::MapToBean(copier) => copier.copy(),
            BeanCopier::ProviderToBean(copier) => copier.copy(),
        }
    }
}

/// BeanCopier 的 Map 来源版本
///
/// 中文说明: 当 source 是 Map 时使用的 BeanCopier 变体。
pub enum MapSourceBeanCopier<T> {
    /// Map 到 Bean 拷贝
    MapToBean(MapToBeanCopier<T>),
    /// Map 到 Map 拷贝（返回 HashMap<String, Value>）
    MapToMap(MapToMapCopier),
}

impl<T> MapSourceBeanCopier<T>
where
    T: Serialize + DeserializeOwned,
{
    /// 创建 Map -> Bean 的 BeanCopier
    pub fn create_map_to_bean(
        source: HashMap<String, Value>,
        target: T,
        copy_options: CopyOptions,
    ) -> Self {
        MapSourceBeanCopier::MapToBean(MapToBeanCopier::new(source, target, copy_options))
    }

    /// 创建 Map -> Map 的 BeanCopier
    pub fn create_map_to_map(
        source: HashMap<String, Value>,
        target: HashMap<String, Value>,
        copy_options: CopyOptions,
    ) -> Self {
        MapSourceBeanCopier::MapToMap(MapToMapCopier::new(source, target, copy_options))
    }
}

/// 通用 Bean 拷贝入口
///
/// 中文说明: 将 source 的属性拷贝到 target，返回新的 target。
/// 这是最常用的拷贝方法，对齐 Java 的 `BeanUtil.copyProperties`。
pub fn copy_properties<S, T>(source: S, target: T, copy_options: CopyOptions) -> T
where
    S: Serialize,
    T: Serialize + DeserializeOwned,
{
    let copier = BeanToBeanCopier::new(source, target, copy_options);
    copier.copy()
}

/// Map -> Bean 拷贝入口
///
/// 中文说明: 将 Map 的键值对拷贝到 target Bean，返回新的 target。
pub fn copy_map_to_bean<T>(
    source: HashMap<String, Value>,
    target: T,
    copy_options: CopyOptions,
) -> T
where
    T: Serialize + DeserializeOwned,
{
    let copier = MapToBeanCopier::new(source, target, copy_options);
    copier.copy()
}

/// Bean -> Map 拷贝入口
///
/// 中文说明: 将 source Bean 的属性拷贝到一个新 Map 中。
pub fn copy_bean_to_map<S>(source: S, copy_options: CopyOptions) -> HashMap<String, Value>
where
    S: Serialize,
{
    let target = HashMap::new();
    let copier = BeanToMapCopier::new(source, target, copy_options);
    copier.copy()
}

/// Map -> Map 拷贝入口
///
/// 中文说明: 将 source Map 的键值对拷贝到 target Map 中。
pub fn copy_map_to_map(
    source: HashMap<String, Value>,
    target: HashMap<String, Value>,
    copy_options: CopyOptions,
) -> HashMap<String, Value> {
    let copier = MapToMapCopier::new(source, target, copy_options);
    copier.copy()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct User {
        name: String,
        age: u32,
        email: Option<String>,
    }

    #[test]
    fn bean_copier_bean_to_bean() {
        let source = User {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
        };
        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let copier = BeanCopier::create(source, target, CopyOptions::create());
        let result = copier.copy();
        assert_eq!(result.name, "Alice");
        assert_eq!(result.age, 30);
    }

    #[test]
    fn copy_properties_basic() {
        let source = User {
            name: "Bob".to_string(),
            age: 25,
            email: Some("bob@example.com".to_string()),
        };
        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let result = copy_properties(source, target, CopyOptions::create());
        assert_eq!(result.name, "Bob");
        assert_eq!(result.age, 25);
        assert_eq!(result.email, Some("bob@example.com".to_string()));
    }

    #[test]
    fn copy_bean_to_map_entry() {
        let source = User {
            name: "Charlie".to_string(),
            age: 35,
            email: None,
        };

        let result = copy_bean_to_map(source, CopyOptions::create().ignore_null_value());
        assert_eq!(
            result.get("name").unwrap(),
            &Value::String("Charlie".into())
        );
        assert_eq!(result.get("age").unwrap(), &Value::Number(35.into()));
        assert!(!result.contains_key("email"));
    }

    #[test]
    fn copy_map_to_bean_entry() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Dave".into()));
        source.insert("age".to_string(), Value::Number(40.into()));

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let result = copy_map_to_bean(source, target, CopyOptions::create());
        assert_eq!(result.name, "Dave");
        assert_eq!(result.age, 40);
    }

    #[test]
    fn copy_map_to_map_entry() {
        let mut source = HashMap::new();
        source.insert("a".to_string(), Value::Number(1.into()));

        let mut target = HashMap::new();
        target.insert("b".to_string(), Value::Number(2.into()));

        let result = copy_map_to_map(source, target, CopyOptions::create());
        assert_eq!(result.get("a").unwrap(), &Value::Number(1.into()));
        assert_eq!(result.get("b").unwrap(), &Value::Number(2.into()));
    }
}
