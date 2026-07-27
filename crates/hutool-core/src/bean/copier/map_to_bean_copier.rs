//! 对齐: `cn.hutool.core.bean.copier.MapToBeanCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/MapToBeanCopier.java
//!
//! 中文说明: Map属性拷贝到Bean中的拷贝器。
//! 将源 Map 的键值对通过 serde_json 机制拷贝到目标 Bean，
//! 支持字段名编辑、大小写忽略、驼峰转换、null值忽略等 CopyOptions 配置。

use std::collections::{HashMap, HashSet};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use super::abs_copier::AbsCopier;
use super::copy_options::CopyOptions;

/// 对齐 Java 类: `cn.hutool.core.bean.copier.MapToBeanCopier<T>`
///
/// 中文说明: Map属性拷贝到Bean中的拷贝器。
///
/// 泛型参数:
/// - `T`: 目标Bean类型，需实现 `Serialize + DeserializeOwned`
#[derive(Debug)]
pub struct MapToBeanCopier<T> {
    /// 来源Map对象
    source: HashMap<String, Value>,
    /// 目标Bean对象
    target: T,
    /// 拷贝选项
    copy_options: CopyOptions,
}

impl<T> MapToBeanCopier<T>
where
    T: Serialize + DeserializeOwned,
{
    /// 对齐 Java 构造: `MapToBeanCopier(Map<?, ?> source, T target, Type targetType, CopyOptions copyOptions)`
    ///
    /// 中文说明: 构造Map到Bean的拷贝器。
    ///
    /// - `source`: 来源Map对象
    /// - `target`: 目标Bean对象
    /// - `copy_options`: 拷贝选项
    pub fn new(source: HashMap<String, Value>, target: T, copy_options: CopyOptions) -> Self {
        Self {
            source,
            target,
            copy_options,
        }
    }
}

impl<T> AbsCopier<T> for MapToBeanCopier<T>
where
    T: Serialize + DeserializeOwned,
{
    /// 对齐 Java: `T copy()`
    ///
    /// 中文说明: 执行Map到Bean的属性拷贝。
    /// 遍历源 Map 的每个键值对，按 CopyOptions 规则编辑字段名和值，
    /// 写入目标 Bean 对应的字段。
    fn copy(&self) -> T {
        // 将目标序列化为 Map 以便逐字段操作
        let target_value = serde_json::to_value(&self.target).unwrap_or(Value::Null);
        let mut target_map = match target_value {
            Value::Object(map) => map,
            _ => Map::new(),
        };

        // 收集目标 Map 的所有 key（用于 camelCase 匹配）
        let target_keys: HashSet<String> = target_map.keys().cloned().collect();

        for (s_key, s_value) in &self.source {
            // 1. 编辑字段名
            let edited_key = match self.copy_options.edit_field_name(s_key) {
                Some(key) => key,
                None => continue,
            };

            // 2. 忽略不需要拷贝的 key
            if !self.copy_options.test_key_filter(&edited_key) {
                continue;
            }

            // 3. 查找目标字段（精确匹配 + camelCase 回退）
            let target_key = match self.copy_options.find_prop_key(&target_keys, &edited_key) {
                Some(key) => key,
                None => edited_key.clone(),
            };

            // 4. 非覆盖模式下，如果目标值存在且非null，则跳过
            if !self.copy_options.is_override() {
                if let Some(target_val) = target_map.get(&target_key) {
                    if !target_val.is_null() {
                        continue;
                    }
                }
            }

            // 5. 转换并编辑值
            let converted_value = self.copy_options.convert_field("", s_value);
            let final_value = self
                .copy_options
                .edit_field_value(&target_key, &converted_value);

            // 6. 忽略空值
            if final_value.is_null() && self.copy_options.is_ignore_null_value() {
                continue;
            }

            // 7. 赋值到目标
            target_map.insert(target_key, final_value);
        }

        // 反序列化为目标类型
        serde_json::from_value(Value::Object(target_map)).unwrap_or_else(|e| {
            if self.copy_options.is_ignore_error() {
                // 忽略错误时返回原始目标
                serde_json::from_value(serde_json::to_value(&self.target).unwrap_or(Value::Null))
                    .expect("Failed to deserialize original target")
            } else {
                panic!("MapToBeanCopier: failed to deserialize target: {}", e)
            }
        })
    }

    fn copy_options(&self) -> &CopyOptions {
        &self.copy_options
    }
}

/// 便捷函数：对齐 Java `BeanUtil.mapToBean(Map, Class, CopyOptions)`
///
/// 中文说明: 将 Map 的键值对拷贝到一个新的 Bean 中。
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct User {
        name: String,
        age: u32,
        email: Option<String>,
    }

    #[test]
    fn map_to_bean_basic_copy() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("age".to_string(), Value::Number(30.into()));
        source.insert(
            "email".to_string(),
            Value::String("alice@example.com".into()),
        );

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let result = copy_map_to_bean(source, target, CopyOptions::create());
        assert_eq!(result.name, "Alice");
        assert_eq!(result.age, 30);
        assert_eq!(result.email, Some("alice@example.com".to_string()));
    }

    #[test]
    fn map_to_bean_ignore_null() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Bob".into()));
        source.insert("age".to_string(), Value::Number(25.into()));
        source.insert("email".to_string(), Value::Null);

        let target = User {
            name: String::new(),
            age: 0,
            email: Some("old@example.com".to_string()),
        };

        let opts = CopyOptions::create().ignore_null_value();
        let result = copy_map_to_bean(source, target, opts);
        assert_eq!(result.name, "Bob");
        assert_eq!(result.age, 25);
        // email 为 null 时忽略，保留目标原值
        assert_eq!(result.email, Some("old@example.com".to_string()));
    }

    #[test]
    fn map_to_bean_ignore_properties() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Charlie".into()));
        source.insert("age".to_string(), Value::Number(35.into()));
        source.insert(
            "email".to_string(),
            Value::String("charlie@example.com".into()),
        );

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let opts = CopyOptions::create().set_ignore_properties(&["age"]);
        let result = copy_map_to_bean(source, target, opts);
        assert_eq!(result.name, "Charlie");
        assert_eq!(result.age, 0);
        assert_eq!(result.email, Some("charlie@example.com".to_string()));
    }

    #[test]
    fn map_to_bean_no_override() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("New".into()));
        source.insert("age".to_string(), Value::Number(99.into()));

        let target = User {
            name: "Old".to_string(),
            age: 20,
            email: None,
        };

        let opts = CopyOptions::create().set_override(false);
        let result = copy_map_to_bean(source, target, opts);
        // 非覆盖模式，目标已有值则保留
        assert_eq!(result.name, "Old");
        assert_eq!(result.age, 20);
    }

    #[test]
    fn map_to_bean_camel_case_auto_convert() {
        let mut source = HashMap::new();
        source.insert("user_name".to_string(), Value::String("Dave".into()));
        source.insert("user_age".to_string(), Value::Number(40.into()));

        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct UserCamel {
            user_name: String,
            user_age: u32,
        }

        let target = UserCamel {
            user_name: String::new(),
            user_age: 0,
        };

        let result = copy_map_to_bean(source, target, CopyOptions::create());
        assert_eq!(result.user_name, "Dave");
        assert_eq!(result.user_age, 40);
    }

    #[test]
    fn map_to_bean_with_field_name_editor() {
        let mut source = HashMap::new();
        source.insert("NAME".to_string(), Value::String("Eve".into()));
        source.insert("AGE".to_string(), Value::Number(28.into()));

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let opts =
            CopyOptions::create().set_field_name_editor(|name| Some(name.to_lowercase()));
        let result = copy_map_to_bean(source, target, opts);
        assert_eq!(result.name, "Eve");
        assert_eq!(result.age, 28);
    }

    #[test]
    fn map_to_bean_with_value_editor() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Frank".into()));
        source.insert("email".to_string(), Value::Null);

        let target = User {
            name: String::new(),
            age: 0,
            email: None,
        };

        let opts = CopyOptions::create().set_field_value_editor(|_name, value| {
            if value.is_null() {
                Value::String("default@placeholder.com".into())
            } else {
                value.clone()
            }
        });
        let result = copy_map_to_bean(source, target, opts);
        assert_eq!(result.name, "Frank");
        assert_eq!(
            result.email,
            Some("default@placeholder.com".to_string())
        );
    }
}
