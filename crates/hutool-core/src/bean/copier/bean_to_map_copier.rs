//! 对齐: `cn.hutool.core.bean.copier.BeanToMapCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/BeanToMapCopier.java
//!
//! 中文说明: Bean属性拷贝到Map中的拷贝器。
//! 将源 Bean 的属性通过 serde_json 序列化机制拷贝到目标 Map，
//! 支持字段名编辑、大小写忽略、null值忽略等 CopyOptions 配置。

use std::collections::HashMap;

use serde::Serialize;
use serde_json::{Map, Value};

use super::abs_copier::AbsCopier;
use super::copy_options::CopyOptions;

/// 对齐 Java 类: `cn.hutool.core.bean.copier.BeanToMapCopier`
///
/// 中文说明: Bean属性拷贝到Map中的拷贝器。
///
/// 泛型参数:
/// - `S`: 源Bean类型，需实现 `Serialize`
#[derive(Debug)]
pub struct BeanToMapCopier<S> {
    /// 来源Bean对象
    source: S,
    /// 目标Map对象
    target: HashMap<String, Value>,
    /// 拷贝选项
    copy_options: CopyOptions,
}

impl<S> BeanToMapCopier<S>
where
    S: Serialize,
{
    /// 对齐 Java 构造: `BeanToMapCopier(Object source, Map target, Type targetType, CopyOptions copyOptions)`
    ///
    /// 中文说明: 构造Bean到Map的拷贝器。
    ///
    /// - `source`: 来源Bean对象
    /// - `target`: 目标Map对象
    /// - `copy_options`: 拷贝选项
    pub fn new(source: S, target: HashMap<String, Value>, copy_options: CopyOptions) -> Self {
        Self {
            source,
            target,
            copy_options,
        }
    }
}

impl<S> AbsCopier<HashMap<String, Value>> for BeanToMapCopier<S>
where
    S: Serialize,
{
    /// 对齐 Java: `Map copy()`
    ///
    /// 中文说明: 执行Bean到Map的属性拷贝。
    /// 将源 Bean 序列化为 JSON Map，逐字段编辑后写入目标 Map。
    fn copy(&self) -> HashMap<String, Value> {
        let source_value = serde_json::to_value(&self.source).unwrap_or(Value::Null);
        let mut result = self.target.clone();

        let source_map = match source_value {
            Value::Object(map) => map,
            _ => return result,
        };

        for (s_field_name, s_value) in &source_map {
            // 1. 编辑字段名
            let edited_name = match self.copy_options.edit_field_name(s_field_name) {
                Some(name) => name,
                None => continue,
            };

            // 2. 忽略不需要拷贝的 key
            if !self.copy_options.test_key_filter(&edited_name) {
                continue;
            }

            // 3. 处理 null 值
            if s_value.is_null() && self.copy_options.is_ignore_null_value() {
                continue;
            }

            // 4. 非覆盖模式下，如果目标值存在且非null，则跳过
            if !self.copy_options.is_override() {
                if let Some(target_value) = result.get(&edited_name) {
                    if !target_value.is_null() {
                        continue;
                    }
                }
            }

            // 5. 转换并编辑值
            let converted_value = self.copy_options.convert_field("", s_value);
            let final_value = self
                .copy_options
                .edit_field_value(&edited_name, &converted_value);

            // 6. 赋值到目标
            result.insert(edited_name, final_value);
        }

        result
    }

    fn copy_options(&self) -> &CopyOptions {
        &self.copy_options
    }
}

/// 便捷函数：对齐 Java `BeanUtil.beanToMap(Object, CopyOptions)`
///
/// 中文说明: 将源对象的属性拷贝到一个新的 Map 中。
pub fn copy_bean_to_map<S>(source: &S, copy_options: CopyOptions) -> HashMap<String, Value>
where
    S: Serialize,
{
    let target = HashMap::new();
    let copier = BeanToMapCopier::new(source, target, copy_options);
    copier.copy()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct User {
        name: String,
        age: u32,
        email: Option<String>,
    }

    #[test]
    fn bean_to_map_basic_copy() {
        let source = User {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
        };

        let result = copy_bean_to_map(&source, CopyOptions::create());
        assert_eq!(result.get("name").unwrap(), &Value::String("Alice".into()));
        assert_eq!(result.get("age").unwrap(), &Value::Number(30.into()));
        assert_eq!(
            result.get("email").unwrap(),
            &Value::String("alice@example.com".into())
        );
    }

    #[test]
    fn bean_to_map_ignore_null() {
        let source = User {
            name: "Bob".to_string(),
            age: 25,
            email: None,
        };

        let opts = CopyOptions::create().ignore_null_value();
        let result = copy_bean_to_map(&source, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Bob".into()));
        assert_eq!(result.get("age").unwrap(), &Value::Number(25.into()));
        // email 为 null 时被忽略
        assert!(!result.contains_key("email"));
    }

    #[test]
    fn bean_to_map_ignore_properties() {
        let source = User {
            name: "Charlie".to_string(),
            age: 35,
            email: Some("charlie@example.com".to_string()),
        };

        let opts = CopyOptions::create().set_ignore_properties(&["age"]);
        let result = copy_bean_to_map(&source, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Charlie".into()));
        assert!(!result.contains_key("age"));
        assert_eq!(
            result.get("email").unwrap(),
            &Value::String("charlie@example.com".into())
        );
    }

    #[test]
    fn bean_to_map_with_field_editor() {
        let source = User {
            name: "Dave".to_string(),
            age: 40,
            email: Some("dave@example.com".to_string()),
        };

        let opts = CopyOptions::create().set_field_name_editor(|name| {
            Some(name.to_uppercase())
        });
        let result = copy_bean_to_map(&source, opts);
        assert_eq!(
            result.get("NAME").unwrap(),
            &Value::String("Dave".into())
        );
        assert_eq!(result.get("AGE").unwrap(), &Value::Number(40.into()));
    }

    #[test]
    fn bean_to_map_merge_into_existing() {
        let source = User {
            name: "Eve".to_string(),
            age: 28,
            email: None,
        };

        let mut target = HashMap::new();
        target.insert("existing_key".to_string(), Value::Bool(true));
        target.insert("name".to_string(), Value::String("OldName".into()));

        let copier = BeanToMapCopier::new(source, target, CopyOptions::create());
        let result = copier.copy();

        assert_eq!(result.get("name").unwrap(), &Value::String("Eve".into()));
        assert_eq!(result.get("existing_key").unwrap(), &Value::Bool(true));
    }

    #[test]
    fn bean_to_map_no_override() {
        let source = User {
            name: "New".to_string(),
            age: 99,
            email: Some("new@example.com".to_string()),
        };

        let mut target = HashMap::new();
        target.insert("name".to_string(), Value::String("Old".into()));

        let opts = CopyOptions::create().set_override(false);
        let copier = BeanToMapCopier::new(source, target, opts);
        let result = copier.copy();

        // 非覆盖模式，目标已有值则保留
        assert_eq!(result.get("name").unwrap(), &Value::String("Old".into()));
    }

    #[test]
    fn bean_to_map_skip_null_field_name() {
        let source = User {
            name: "Frank".to_string(),
            age: 50,
            email: Some("frank@example.com".to_string()),
        };

        let opts =
            CopyOptions::create().set_field_name_editor(|name| {
                if name == "age" {
                    None
                } else {
                    Some(name.to_string())
                }
            });
        let result = copy_bean_to_map(&source, opts);
        assert_eq!(
            result.get("name").unwrap(),
            &Value::String("Frank".into())
        );
        assert!(!result.contains_key("age"));
    }
}
