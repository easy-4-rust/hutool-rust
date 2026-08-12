//! 对齐: `cn.hutool.core.bean.copier.MapToMapCopier`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/MapToMapCopier.java
//!
//! 中文说明: Map属性拷贝到Map中的拷贝器。
//! 将源 Map 的键值对拷贝到目标 Map，
//! 支持字段名编辑、大小写忽略、null值忽略等 CopyOptions 配置。

use std::collections::HashMap;

use serde_json::Value;

use super::abs_copier::AbsCopier;
use super::copy_options::CopyOptions;

/// 对齐 Java 类: `cn.hutool.core.bean.copier.MapToMapCopier`
///
/// 中文说明: Map属性拷贝到Map中的拷贝器。
#[derive(Debug)]
pub struct MapToMapCopier {
    /// 来源Map对象
    source: HashMap<String, Value>,
    /// 目标Map对象
    target: HashMap<String, Value>,
    /// 拷贝选项
    copy_options: CopyOptions,
}

impl MapToMapCopier {
    /// 对齐 Java 构造: `MapToMapCopier(Map source, Map target, Type targetType, CopyOptions copyOptions)`
    ///
    /// 中文说明: 构造Map到Map的拷贝器。
    ///
    /// - `source`: 来源Map对象
    /// - `target`: 目标Map对象
    /// - `copy_options`: 拷贝选项
    pub fn new(
        source: HashMap<String, Value>,
        target: HashMap<String, Value>,
        copy_options: CopyOptions,
    ) -> Self {
        Self {
            source,
            target,
            copy_options,
        }
    }
}

impl AbsCopier<HashMap<String, Value>> for MapToMapCopier {
    /// 对齐 Java: `Map copy()`
    ///
    /// 中文说明: 执行Map到Map的属性拷贝。
    /// 遍历源 Map 的每个键值对，按 CopyOptions 规则编辑字段名和值，
    /// 写入目标 Map。
    fn copy(&self) -> HashMap<String, Value> {
        let mut result = self.target.clone();

        for (s_key, s_value) in &self.source {
            // 1. 编辑字段名（仅对 String 类型的 key 做编辑）
            let edited_key = match self.copy_options.edit_field_name(s_key) {
                Some(key) => key,
                None => continue,
            };

            // 2. 忽略不需要拷贝的 key
            if !self.copy_options.test_key_filter(&edited_key) {
                continue;
            }

            // 3. 获取目标值
            let target_value = result.get(&edited_key);

            // 4. 非覆盖模式下，如果目标值存在且非null，则跳过
            if !self.copy_options.is_override() {
                if let Some(tv) = target_value {
                    if !tv.is_null() {
                        continue;
                    }
                }
            }

            // 5. 转换并编辑值
            let converted_value = self.copy_options.convert_field("", s_value);
            let final_value = self
                .copy_options
                .edit_field_value(&edited_key, &converted_value);

            // 6. 忽略空值
            if final_value.is_null() && self.copy_options.is_ignore_null_value() {
                continue;
            }

            // 7. 赋值到目标
            result.insert(edited_key, final_value);
        }

        result
    }

    fn copy_options(&self) -> &CopyOptions {
        &self.copy_options
    }
}

/// 便捷函数：对齐 Java Map 到 Map 的拷贝
///
/// 中文说明: 将源 Map 的键值对拷贝到目标 Map 中。
pub fn copy_map_to_map(
    source: &HashMap<String, Value>,
    target: &HashMap<String, Value>,
    copy_options: CopyOptions,
) -> HashMap<String, Value> {
    let copier = MapToMapCopier::new(source.clone(), target.clone(), copy_options);
    copier.copy()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_to_map_basic_copy() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("age".to_string(), Value::Number(30.into()));

        let target = HashMap::new();
        let result = copy_map_to_map(&source, &target, CopyOptions::create());
        assert_eq!(result.get("name").unwrap(), &Value::String("Alice".into()));
        assert_eq!(result.get("age").unwrap(), &Value::Number(30.into()));
    }

    #[test]
    fn map_to_map_ignore_null() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Bob".into()));
        source.insert("email".to_string(), Value::Null);

        let target = HashMap::new();
        let opts = CopyOptions::create().ignore_null_value();
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Bob".into()));
        assert!(!result.contains_key("email"));
    }

    #[test]
    fn map_to_map_no_override() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("New".into()));

        let mut target = HashMap::new();
        target.insert("name".to_string(), Value::String("Old".into()));

        let opts = CopyOptions::create().set_override(false);
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Old".into()));
    }

    #[test]
    fn map_to_map_override_mode() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("New".into()));

        let mut target = HashMap::new();
        target.insert("name".to_string(), Value::String("Old".into()));

        let opts = CopyOptions::create().set_override(true);
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("New".into()));
    }

    #[test]
    fn map_to_map_ignore_properties() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("age".to_string(), Value::Number(30.into()));
        source.insert(
            "email".to_string(),
            Value::String("alice@example.com".into()),
        );

        let target = HashMap::new();
        let opts = CopyOptions::create().set_ignore_properties(&["age"]);
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Alice".into()));
        assert!(!result.contains_key("age"));
        assert_eq!(
            result.get("email").unwrap(),
            &Value::String("alice@example.com".into())
        );
    }

    #[test]
    fn map_to_map_with_field_name_editor() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("age".to_string(), Value::Number(30.into()));

        let target = HashMap::new();
        let opts = CopyOptions::create().set_field_name_editor(|name| Some(name.to_uppercase()));
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("NAME").unwrap(), &Value::String("Alice".into()));
        assert_eq!(result.get("AGE").unwrap(), &Value::Number(30.into()));
    }

    #[test]
    fn map_to_map_skip_null_field_name() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("age".to_string(), Value::Number(30.into()));

        let target = HashMap::new();
        let opts = CopyOptions::create().set_field_name_editor(|name| {
            if name == "age" {
                None
            } else {
                Some(name.to_string())
            }
        });
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("name").unwrap(), &Value::String("Alice".into()));
        assert!(!result.contains_key("age"));
    }

    #[test]
    fn map_to_map_with_value_editor() {
        let mut source = HashMap::new();
        source.insert("name".to_string(), Value::String("Alice".into()));
        source.insert("email".to_string(), Value::Null);

        let target = HashMap::new();
        let opts = CopyOptions::create().set_field_value_editor(|_name, value| {
            if value.is_null() {
                Value::String("N/A".into())
            } else {
                value.clone()
            }
        });
        let result = copy_map_to_map(&source, &target, opts);
        assert_eq!(result.get("email").unwrap(), &Value::String("N/A".into()));
    }

    #[test]
    fn map_to_map_ignore_case_key_filter() {
        let mut source = HashMap::new();
        source.insert("Name".to_string(), Value::String("Alice".into()));
        source.insert("Age".to_string(), Value::Number(30.into()));

        let target = HashMap::new();
        let opts = CopyOptions::create()
            .ignore_case()
            .set_ignore_properties(&["NAME"]);
        let result = copy_map_to_map(&source, &target, opts);
        // "Name" 被忽略（大小写不敏感匹配 "NAME"）
        assert!(!result.contains_key("Name"));
        assert_eq!(result.get("Age").unwrap(), &Value::Number(30.into()));
    }

    #[test]
    fn map_to_map_preserves_existing_target_keys() {
        let mut source = HashMap::new();
        source.insert("a".to_string(), Value::Number(1.into()));

        let mut target = HashMap::new();
        target.insert("b".to_string(), Value::Number(2.into()));

        let result = copy_map_to_map(&source, &target, CopyOptions::create());
        assert_eq!(result.get("a").unwrap(), &Value::Number(1.into()));
        assert_eq!(result.get("b").unwrap(), &Value::Number(2.into()));
    }
}
