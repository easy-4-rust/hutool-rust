//! 对齐: Rust 扩展（Java 无此类）
//!
//! 中文说明: BeanCopier 工厂类，提供便捷的拷贝器创建方法。
//! 集中管理 BeanCopier 的创建逻辑，简化调用方代码。

use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use super::abs_copier::AbsCopier;
use super::bean_to_bean_copier::BeanToBeanCopier;
use super::bean_to_map_copier::BeanToMapCopier;
use super::copy_options::CopyOptions;
use super::map_to_bean_copier::MapToBeanCopier;
use super::map_to_map_copier::MapToMapCopier;
use super::value_provider::ValueProvider;
use super::value_provider_to_bean_copier::ValueProviderToBeanCopier;

/// 对齐: Rust 扩展（Java 无此类）
///
/// 中文说明: BeanCopier 工厂，提供静态方法创建各种类型的拷贝器。
/// 集中管理创建逻辑，避免调用方直接依赖具体拷贝器类型。
pub struct BeanCopierFactory;

impl BeanCopierFactory {
    /// 对齐: 工厂方法
    ///
    /// 中文说明: 创建 Bean -> Bean 拷贝器。
    ///
    /// - `source`: 来源Bean对象
    /// - `target`: 目标Bean对象
    /// - `copy_options`: 拷贝选项
    /// - 返回: 拷贝后的目标对象
    pub fn copy_bean_to_bean<S, T>(source: S, target: T, copy_options: CopyOptions) -> T
    where
        S: Serialize,
        T: Serialize + DeserializeOwned,
    {
        let copier = BeanToBeanCopier::new(source, target, copy_options);
        copier.copy()
    }

    /// 对齐: 工厂方法
    ///
    /// 中文说明: 创建 Bean -> Map 拷贝器。
    ///
    /// - `source`: 来源Bean对象
    /// - `copy_options`: 拷贝选项
    /// - 返回: 拷贝后的 Map
    pub fn copy_bean_to_map<S>(source: S, copy_options: CopyOptions) -> HashMap<String, Value>
    where
        S: Serialize,
    {
        let target = HashMap::new();
        let copier = BeanToMapCopier::new(source, target, copy_options);
        copier.copy()
    }

    /// 对齐: 工厂方法
    ///
    /// 中文说明: 创建 Map -> Bean 拷贝器。
    ///
    /// - `source`: 来源Map对象
    /// - `target`: 目标Bean对象（提供类型信息）
    /// - `copy_options`: 拷贝选项
    /// - 返回: 拷贝后的目标对象
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

    /// 对齐: 工厂方法
    ///
    /// 中文说明: 创建 Map -> Map 拷贝器。
    ///
    /// - `source`: 来源Map对象
    /// - `target`: 目标Map对象
    /// - `copy_options`: 拷贝选项
    /// - 返回: 拷贝后的 Map
    pub fn copy_map_to_map(
        source: HashMap<String, Value>,
        target: HashMap<String, Value>,
        copy_options: CopyOptions,
    ) -> HashMap<String, Value> {
        let copier = MapToMapCopier::new(source, target, copy_options);
        copier.copy()
    }

    /// 对齐: 工厂方法
    ///
    /// 中文说明: 创建 ValueProvider -> Bean 拷贝器。
    ///
    /// - `source`: 来源 ValueProvider
    /// - `target`: 目标Bean对象（提供类型信息）
    /// - `copy_options`: 拷贝选项
    /// - 返回: 拷贝后的目标对象
    pub fn copy_provider_to_bean<T>(
        source: Box<dyn ValueProvider>,
        target: T,
        copy_options: CopyOptions,
    ) -> T
    where
        T: Serialize + DeserializeOwned,
    {
        let copier = ValueProviderToBeanCopier::new(source, target, copy_options);
        copier.copy()
    }

    /// 对齐: 通用拷贝入口
    ///
    /// 中文说明: 根据参数类型自动选择拷贝策略。
    /// 这是最通用的拷贝方法，对齐 Java `BeanUtil.copyProperties`。
    pub fn copy<S, T>(source: S, target: T, copy_options: CopyOptions) -> T
    where
        S: Serialize,
        T: Serialize + DeserializeOwned,
    {
        Self::copy_bean_to_bean(source, target, copy_options)
    }
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
    fn factory_bean_to_bean() {
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

        let result = BeanCopierFactory::copy_bean_to_bean(source, target, CopyOptions::create());
        assert_eq!(result.name, "Alice");
        assert_eq!(result.age, 30);
    }

    #[test]
    fn factory_bean_to_map() {
        let source = User {
            name: "Bob".to_string(),
            age: 25,
            email: None,
        };

        let result =
            BeanCopierFactory::copy_bean_to_map(source, CopyOptions::create().ignore_null_value());
        assert_eq!(result.get("name").unwrap(), &Value::String("Bob".into()));
        assert!(!result.contains_key("email"));
    }

    #[test]
    fn factory_map_to_bean() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Charlie".into()));
        source.insert("age".to_string(), Value::Number(35.into()));

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let result = BeanCopierFactory::copy_map_to_bean(source, target, CopyOptions::create());
        assert_eq!(result.name, "Charlie");
        assert_eq!(result.age, 35);
    }

    #[test]
    fn factory_map_to_map() {
        let mut source = HashMap::new();
        source.insert("x".to_string(), Value::Number(1.into()));

        let target = HashMap::new();

        let result = BeanCopierFactory::copy_map_to_map(source, target, CopyOptions::create());
        assert_eq!(result.get("x").unwrap(), &Value::Number(1.into()));
    }

    #[test]
    fn factory_generic_copy() {
        let source = User {
            name: "Dave".to_string(),
            age: 40,
            email: Some("dave@example.com".to_string()),
        };
        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let result = BeanCopierFactory::copy(source, target, CopyOptions::create());
        assert_eq!(result.name, "Dave");
        assert_eq!(result.age, 40);
    }
}
