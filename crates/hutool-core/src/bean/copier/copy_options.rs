//! 对齐: `cn.hutool.core.bean.copier.CopyOptions`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/CopyOptions.java
//!
//! 中文说明: 属性拷贝选项，包括：
//! 1、限制的类或接口，用于限制拷贝的属性
//! 2、是否忽略空值，当源对象的值为null时，true: 忽略而不注入此值，false: 注入null
//! 3、忽略的属性列表，设置一个属性列表，不拷贝这些属性值

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde_json::Value;

/// 对齐 Java 类: `cn.hutool.core.bean.copier.CopyOptions`
///
/// 属性拷贝选项，通过 builder 模式构建。控制 Bean 属性拷贝时的行为，
/// 包括忽略空值、忽略大小写、字段映射、字段过滤等。
pub struct CopyOptions {
    /// 限制的类或接口名称，用于限制拷贝的属性
    editable: Option<String>,
    /// 是否忽略空值，当源对象的值为null时，true: 忽略而不注入此值，false: 注入null
    ignore_null_value: bool,
    /// 是否忽略字段注入错误
    ignore_error: bool,
    /// 是否忽略字段大小写
    ignore_case: bool,
    /// 字段属性编辑器，用于自定义属性转换规则，例如驼峰转下划线等
    field_name_editor: Option<Box<dyn Fn(&str) -> Option<String> + Send + Sync>>,
    /// 字段属性值编辑器，用于自定义属性值转换规则，例如null转""等
    field_value_editor: Option<Box<dyn Fn(&str, &Value) -> Value + Send + Sync>>,
    /// 是否支持transient关键字修饰和@Transient注解
    transient_support: bool,
    /// 是否覆盖目标值，如果不覆盖，会先读取目标对象的值，非null则写，否则忽略
    override_: bool,
    /// 是否自动转换为驼峰方式
    auto_trans_camel_case: bool,
    /// 源对象和目标对象都是Map时, 需要忽略的源对象Map key
    ignore_key_set: HashSet<String>,
    /// 自定义类型转换器：接收 (目标类型名, 原始值) -> 转换后的 Value
    converter: Option<Box<dyn Fn(&str, &Value) -> Value + Send + Sync>>,
    /// 在Bean转换时，如果源是String，目标对象是Date或LocalDateTime，则可自定义转换格式
    format_if_date: Option<String>,
}

impl fmt::Debug for CopyOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CopyOptions")
            .field("editable", &self.editable)
            .field("ignore_null_value", &self.ignore_null_value)
            .field("ignore_error", &self.ignore_error)
            .field("ignore_case", &self.ignore_case)
            .field("field_name_editor", &self.field_name_editor.is_some())
            .field("field_value_editor", &self.field_value_editor.is_some())
            .field("transient_support", &self.transient_support)
            .field("override_", &self.override_)
            .field("auto_trans_camel_case", &self.auto_trans_camel_case)
            .field("ignore_key_set", &self.ignore_key_set)
            .field("converter", &self.converter.is_some())
            .field("format_if_date", &self.format_if_date)
            .finish()
    }
}

impl Default for CopyOptions {
    fn default() -> Self {
        Self {
            editable: None,
            ignore_null_value: false,
            ignore_error: false,
            ignore_case: false,
            field_name_editor: None,
            field_value_editor: None,
            transient_support: true,
            override_: true,
            auto_trans_camel_case: true,
            ignore_key_set: HashSet::new(),
            converter: None,
            format_if_date: None,
        }
    }
}

impl CopyOptions {
    // ── 工厂方法 ──

    /// 对齐 Java: `CopyOptions.create()`
    ///
    /// 中文说明: 创建默认拷贝选项。
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 Java: `CopyOptions.create(Class<?>, boolean, String...)`
    ///
    /// 中文说明: 创建拷贝选项，指定限制类、是否忽略空值、忽略的属性列表。
    ///
    /// - `editable`: 限制的类或接口名称
    /// - `ignore_null_value`: 是否忽略空值
    /// - `ignore_properties`: 忽略的属性列表
    pub fn create_with(
        editable: Option<String>,
        ignore_null_value: bool,
        ignore_properties: &[&str],
    ) -> Self {
        let mut opts = Self::default();
        opts.editable = editable;
        opts.ignore_null_value = ignore_null_value;
        opts.ignore_key_set = ignore_properties.iter().map(|s| s.to_string()).collect();
        opts
    }

    // ── Builder / Setter 方法 ──

    /// 对齐 Java: `CopyOptions.setEditable(Class<?>)`
    ///
    /// 中文说明: 设置限制的类或接口，必须为目标对象的实现接口或父类，用于限制拷贝的属性。
    pub fn set_editable(mut self, editable: impl Into<String>) -> Self {
        self.editable = Some(editable.into());
        self
    }

    /// 对齐 Java: `CopyOptions.setIgnoreNullValue(boolean)`
    ///
    /// 中文说明: 设置是否忽略空值，当源对象的值为null时，true: 忽略而不注入此值，false: 注入null。
    pub fn set_ignore_null_value(mut self, ignore_null_value: bool) -> Self {
        self.ignore_null_value = ignore_null_value;
        self
    }

    /// 对齐 Java: `CopyOptions.ignoreNullValue()`
    ///
    /// 中文说明: 设置忽略空值，当源对象的值为null时，忽略而不注入此值。
    pub fn ignore_null_value(mut self) -> Self {
        self.ignore_null_value = true;
        self
    }

    /// 对齐 Java: `CopyOptions.setIgnoreError(boolean)`
    ///
    /// 中文说明: 设置是否忽略字段的注入错误。
    pub fn set_ignore_error(mut self, ignore_error: bool) -> Self {
        self.ignore_error = ignore_error;
        self
    }

    /// 对齐 Java: `CopyOptions.ignoreError()`
    ///
    /// 中文说明: 设置忽略字段的注入错误。
    pub fn ignore_error(mut self) -> Self {
        self.ignore_error = true;
        self
    }

    /// 对齐 Java: `CopyOptions.setIgnoreCase(boolean)`
    ///
    /// 中文说明: 设置是否忽略字段的大小写。
    pub fn set_ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }

    /// 对齐 Java: `CopyOptions.ignoreCase()`
    ///
    /// 中文说明: 设置忽略字段的大小写。
    pub fn ignore_case(mut self) -> Self {
        self.ignore_case = true;
        self
    }

    /// 对齐 Java: `CopyOptions.setIgnoreProperties(String...)`
    ///
    /// 中文说明: 设置忽略的目标对象中属性列表，设置一个属性列表，不拷贝这些属性值。
    pub fn set_ignore_properties(mut self, ignore_properties: &[&str]) -> Self {
        self.ignore_key_set = ignore_properties.iter().map(|s| s.to_string()).collect();
        self
    }

    /// 对齐 Java: `CopyOptions.setFieldMapping(Map<String, String>)`
    ///
    /// 中文说明: 设置拷贝属性的字段映射，用于不同的属性之间拷贝做对应表用。
    /// 需要注意的是，当使用ValueProvider作为数据提供者时，这个映射是相反的。
    pub fn set_field_mapping(mut self, field_mapping: HashMap<String, String>) -> Self {
        self.field_name_editor = Some(Box::new(move |key: &str| -> Option<String> {
            field_mapping
                .get(key)
                .cloned()
                .or_else(|| Some(key.to_string()))
        }));
        self
    }

    /// 对齐 Java: `CopyOptions.setFieldNameEditor(Editor<String>)`
    ///
    /// 中文说明: 设置字段属性编辑器，用于自定义属性转换规则，例如驼峰转下划线等。
    /// 此转换器只针对源端的字段做转换。当转换后的字段名为null时忽略这个字段。
    pub fn set_field_name_editor<F>(mut self, editor: F) -> Self
    where
        F: Fn(&str) -> Option<String> + Send + Sync + 'static,
    {
        self.field_name_editor = Some(Box::new(editor));
        self
    }

    /// 对齐 Java: `CopyOptions.setFieldValueEditor(BiFunction<String, Object, Object>)`
    ///
    /// 中文说明: 设置字段属性值编辑器，用于自定义属性值转换规则，例如null转""等。
    pub fn set_field_value_editor<F>(mut self, editor: F) -> Self
    where
        F: Fn(&str, &Value) -> Value + Send + Sync + 'static,
    {
        self.field_value_editor = Some(Box::new(editor));
        self
    }

    /// 对齐 Java: `CopyOptions.setTransientSupport(boolean)`
    ///
    /// 中文说明: 设置是否支持transient关键字修饰和@Transient注解，
    /// 如果支持，被修饰的字段或方法对应的字段将被忽略。
    pub fn set_transient_support(mut self, transient_support: bool) -> Self {
        self.transient_support = transient_support;
        self
    }

    /// 对齐 Java: `CopyOptions.setOverride(boolean)`
    ///
    /// 中文说明: 设置是否覆盖目标值，如果不覆盖，会先读取目标对象的值，
    /// 非null则写，否则忽略。如果覆盖，则不判断直接写。
    pub fn set_override(mut self, override_: bool) -> Self {
        self.override_ = override_;
        self
    }

    /// 对齐 Java: `CopyOptions.setAutoTransCamelCase(boolean)`
    ///
    /// 中文说明: 设置是否自动转换为驼峰方式。
    /// 一般用于map转bean和bean转bean出现非驼峰格式时，在尝试转换失败的情况下，
    /// 是否二次检查转为驼峰匹配。
    pub fn set_auto_trans_camel_case(mut self, auto_trans_camel_case: bool) -> Self {
        self.auto_trans_camel_case = auto_trans_camel_case;
        self
    }

    /// 对齐 Java: `CopyOptions.setConverter(TypeConverter)`
    ///
    /// 中文说明: 设置自定义类型转换器，默认使用全局万能转换器转换。
    pub fn set_converter<F>(mut self, converter: F) -> Self
    where
        F: Fn(&str, &Value) -> Value + Send + Sync + 'static,
    {
        self.converter = Some(Box::new(converter));
        self
    }

    /// 对齐 Java: `CopyOptions.setFormatIfDate(String)`
    ///
    /// 中文说明: 设置日期格式，用于日期转字符串，默认为null。
    pub fn set_format_if_date(mut self, format: impl Into<String>) -> Self {
        self.format_if_date = Some(format.into());
        self
    }

    // ── Getter / 查询方法 ──

    /// 对齐 Java: 字段 `editable`
    ///
    /// 中文说明: 获取限制的类或接口名称。
    pub fn editable(&self) -> Option<&str> {
        self.editable.as_deref()
    }

    /// 对齐 Java: `isIgnoreNullValue` 风格
    ///
    /// 中文说明: 是否忽略空值。
    pub fn is_ignore_null_value(&self) -> bool {
        self.ignore_null_value
    }

    /// 对齐 Java: `isIgnoreCase` 风格
    ///
    /// 中文说明: 是否忽略字段大小写。
    pub fn is_ignore_case(&self) -> bool {
        self.ignore_case
    }

    /// 对齐 Java: `isIgnoreError` 风格
    ///
    /// 中文说明: 是否忽略字段注入错误。
    pub fn is_ignore_error(&self) -> bool {
        self.ignore_error
    }

    /// 对齐 Java: `isTransientSupport` 风格
    ///
    /// 中文说明: 是否支持transient关键字修饰和@Transient注解。
    pub fn is_transient_support(&self) -> bool {
        self.transient_support
    }

    /// 对齐 Java: `isOverride` 风格
    ///
    /// 中文说明: 是否覆盖目标值。
    pub fn is_override(&self) -> bool {
        self.override_
    }

    /// 对齐 Java: `isAutoTransCamelCase` 风格
    ///
    /// 中文说明: 是否自动转换为驼峰方式。
    pub fn is_auto_trans_camel_case(&self) -> bool {
        self.auto_trans_camel_case
    }

    /// 对齐 Java: `getFormatIfDate()`
    ///
    /// 中文说明: 获取日期格式。
    pub fn format_if_date(&self) -> Option<&str> {
        self.format_if_date.as_deref()
    }

    /// 对齐 Java: `getIgnoreKeySet()` — 不可变引用
    pub fn ignore_key_set(&self) -> &HashSet<String> {
        &self.ignore_key_set
    }

    // ── 内部辅助方法 ──

    /// 对齐 Java: `editFieldValue(String, Object)`
    ///
    /// 中文说明: 编辑字段值。如果有自定义值编辑器则调用，否则返回原值。
    pub fn edit_field_value(&self, field_name: &str, field_value: &Value) -> Value {
        match &self.field_value_editor {
            Some(editor) => editor(field_name, field_value),
            None => field_value.clone(),
        }
    }

    /// 对齐 Java: `editFieldName(String)`
    ///
    /// 中文说明: 转换字段名为编辑后的字段名。如果有自定义名称编辑器则调用，否则返回原名。
    /// 返回 `None` 表示跳过该字段。
    pub fn edit_field_name(&self, field_name: &str) -> Option<String> {
        match &self.field_name_editor {
            Some(editor) => editor(field_name),
            None => Some(field_name.to_string()),
        }
    }

    /// 对齐 Java: `testKeyFilter(Object)`
    ///
    /// 中文说明: 测试是否保留key。`true` 保留，`false` 不保留（即应跳过）。
    pub fn test_key_filter(&self, key: &str) -> bool {
        if self.ignore_key_set.is_empty() {
            return true;
        }

        if self.ignore_case {
            // 忽略大小写时要遍历检查
            for ignore_key in &self.ignore_key_set {
                if key.eq_ignore_ascii_case(ignore_key) {
                    return false;
                }
            }
            true
        } else {
            !self.ignore_key_set.contains(key)
        }
    }

    /// 对齐 Java: `convertField(Type, Object)`
    ///
    /// 中文说明: 使用自定义转换器转换字段值。如果自定义转换器为null，则返回原值。
    pub fn convert_field(&self, target_type: &str, field_value: &Value) -> Value {
        match &self.converter {
            Some(converter) => converter(target_type, field_value),
            None => {
                // 默认转换：尝试基本类型匹配
                if field_value.is_null() {
                    Value::Null
                } else {
                    field_value.clone()
                }
            }
        }
    }

    /// 对齐 Java: `findPropDesc(Map<String, PropDesc>, String)`
    ///
    /// 中文说明: 查找Map对应Bean的名称。尝试原名称、转驼峰名称。
    /// 返回匹配的key（可能已转换为驼峰），None表示未找到。
    pub fn find_prop_key(&self, available_keys: &HashSet<String>, key: &str) -> Option<String> {
        // 先精确匹配
        if available_keys.contains(key) {
            return Some(key.to_string());
        }

        // 转驼峰尝试查找
        if self.auto_trans_camel_case {
            let camel_case_key = snake_to_camel(key);
            if camel_case_key != key && available_keys.contains(&camel_case_key) {
                return Some(camel_case_key);
            }
        }

        None
    }
}

/// 将 snake_case / underline_case 转换为 camelCase
fn snake_to_camel(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = false;
    for (i, ch) in s.chars().enumerate() {
        if ch == '_' || ch == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_options_default_values() {
        let opts = CopyOptions::create();
        assert!(!opts.is_ignore_null_value());
        assert!(!opts.is_ignore_error());
        assert!(!opts.is_ignore_case());
        assert!(opts.is_transient_support());
        assert!(opts.is_override());
        assert!(opts.is_auto_trans_camel_case());
        assert!(opts.editable().is_none());
        assert!(opts.format_if_date().is_none());
        assert!(opts.ignore_key_set().is_empty());
    }

    #[test]
    fn copy_options_builder_chain() {
        let opts = CopyOptions::create()
            .ignore_null_value()
            .ignore_error()
            .ignore_case()
            .set_override(false)
            .set_transient_support(false)
            .set_auto_trans_camel_case(false)
            .set_editable("com.example.Parent")
            .set_format_if_date("yyyy-MM-dd");

        assert!(opts.is_ignore_null_value());
        assert!(opts.is_ignore_error());
        assert!(opts.is_ignore_case());
        assert!(!opts.is_override());
        assert!(!opts.is_transient_support());
        assert!(!opts.is_auto_trans_camel_case());
        assert_eq!(opts.editable(), Some("com.example.Parent"));
        assert_eq!(opts.format_if_date(), Some("yyyy-MM-dd"));
    }

    #[test]
    fn copy_options_create_with() {
        let opts = CopyOptions::create_with(
            Some("com.example.Base".into()),
            true,
            &["password", "secret"],
        );
        assert!(opts.is_ignore_null_value());
        assert_eq!(opts.editable(), Some("com.example.Base"));
        assert!(!opts.test_key_filter("password"));
        assert!(!opts.test_key_filter("secret"));
        assert!(opts.test_key_filter("username"));
    }

    #[test]
    fn copy_options_ignore_properties() {
        let opts = CopyOptions::create()
            .set_ignore_properties(&["id", "createTime"]);
        assert!(!opts.test_key_filter("id"));
        assert!(!opts.test_key_filter("createTime"));
        assert!(opts.test_key_filter("name"));
    }

    #[test]
    fn copy_options_ignore_case_key_filter() {
        let opts = CopyOptions::create()
            .ignore_case()
            .set_ignore_properties(&["UserName"]);
        assert!(!opts.test_key_filter("username"));
        assert!(!opts.test_key_filter("USERNAME"));
        assert!(opts.test_key_filter("email"));
    }

    #[test]
    fn copy_options_field_name_editor() {
        let opts = CopyOptions::create()
            .set_field_name_editor(|name| {
                if name == "skip_me" {
                    None
                } else {
                    Some(name.to_uppercase())
                }
            });
        assert_eq!(opts.edit_field_name("hello"), Some("HELLO".to_string()));
        assert_eq!(opts.edit_field_name("skip_me"), None);
    }

    #[test]
    fn copy_options_field_mapping() {
        let mut mapping = HashMap::new();
        mapping.insert("src_name".to_string(), "dest_name".to_string());
        mapping.insert("src_age".to_string(), "dest_age".to_string());

        let opts = CopyOptions::create().set_field_mapping(mapping);
        assert_eq!(
            opts.edit_field_name("src_name"),
            Some("dest_name".to_string())
        );
        assert_eq!(
            opts.edit_field_name("src_age"),
            Some("dest_age".to_string())
        );
        // unmapped keys pass through
        assert_eq!(
            opts.edit_field_name("other"),
            Some("other".to_string())
        );
    }

    #[test]
    fn copy_options_field_value_editor() {
        let opts = CopyOptions::create()
            .set_field_value_editor(|_name, value| {
                if value.is_null() {
                    Value::String("default".to_string())
                } else {
                    value.clone()
                }
            });
        assert_eq!(
            opts.edit_field_value("field", &Value::Null),
            Value::String("default".to_string())
        );
        assert_eq!(
            opts.edit_field_value("field", &Value::Number(42.into())),
            Value::Number(42.into())
        );
    }

    #[test]
    fn copy_options_convert_field_default() {
        let opts = CopyOptions::create();
        assert_eq!(
            opts.convert_field("String", &Value::String("hi".into())),
            Value::String("hi".into())
        );
        assert_eq!(opts.convert_field("String", &Value::Null), Value::Null);
    }

    #[test]
    fn copy_options_convert_field_custom() {
        let opts = CopyOptions::create().set_converter(|_type, value| {
            if value.is_string() {
                Value::Bool(true)
            } else {
                value.clone()
            }
        });
        assert_eq!(
            opts.convert_field("bool", &Value::String("x".into())),
            Value::Bool(true)
        );
    }

    #[test]
    fn copy_options_find_prop_key_exact() {
        let mut keys = HashSet::new();
        keys.insert("userName".to_string());
        keys.insert("age".to_string());

        let opts = CopyOptions::create();
        assert_eq!(
            opts.find_prop_key(&keys, "userName"),
            Some("userName".to_string())
        );
    }

    #[test]
    fn copy_options_find_prop_key_camel_case() {
        let mut keys = HashSet::new();
        keys.insert("userName".to_string());

        let opts = CopyOptions::create();
        // "user_name" should be found via camelCase conversion
        assert_eq!(
            opts.find_prop_key(&keys, "user_name"),
            Some("userName".to_string())
        );
    }

    #[test]
    fn copy_options_find_prop_key_camel_case_disabled() {
        let mut keys = HashSet::new();
        keys.insert("userName".to_string());

        let opts = CopyOptions::create().set_auto_trans_camel_case(false);
        assert_eq!(opts.find_prop_key(&keys, "user_name"), None);
    }

    #[test]
    fn snake_to_camel_basic() {
        assert_eq!(snake_to_camel("user_name"), "userName");
        assert_eq!(snake_to_camel("create_time"), "createTime");
        assert_eq!(snake_to_camel("id"), "id");
        assert_eq!(snake_to_camel("already"), "already");
        assert_eq!(snake_to_camel("a-b-c"), "aBC");
    }
}
