//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::annotation_schema::AnnotationSchema;
use super::annotation_type_name::AnnotationTypeName;
use super::annotation_value::AnnotationValue;
use super::attribute_def::AttributeDef;

/// 注解实例镜像。
#[derive(Clone)]
pub struct AnnotationMirror {
    pub type_name: AnnotationTypeName,
    values: HashMap<String, AnnotationValue>,
    synthesized: bool,
}

impl fmt::Debug for AnnotationMirror {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnnotationMirror")
            .field("type_name", &self.type_name)
            .field("values", &self.values)
            .field("synthesized", &self.synthesized)
            .finish()
    }
}

impl PartialEq for AnnotationMirror {
    fn eq(&self, other: &Self) -> bool {
        self.type_name == other.type_name && self.values == other.values
    }
}

impl AnnotationMirror {
    /// 构造注解镜像。
    pub fn new(type_name: AnnotationTypeName, values: HashMap<String, AnnotationValue>) -> Self {
        Self {
            type_name,
            values,
            synthesized: false,
        }
    }

    /// 标记为合成注解。
    pub fn mark_synthesized(mut self) -> Self {
        self.synthesized = true;
        self
    }

    /// 是否为合成注解。
    pub fn is_synthesized(&self) -> bool {
        self.synthesized
    }

    /// 注解类型名。
    pub fn annotation_type(&self) -> AnnotationTypeName {
        self.type_name
    }

    /// 读取属性值；缺失时按 schema 默认值填充。
    pub fn get_raw(&self, name: &str) -> Option<&AnnotationValue> {
        self.values.get(name)
    }

    /// 按 schema 解析属性值；缺失时跟随 `@Alias` 指向的属性。
    pub fn resolve_value(&self, schema: &AnnotationSchema, name: &str) -> AnnotationValue {
        if let Some(v) = self.values.get(name) {
            return v.clone();
        }
        // 跟随 AttributeDef 上的 @Alias 元注解（非 JVM 反射）
        if let Some(attr) = schema.attribute(name) {
            for meta in &attr.meta {
                if meta.annotation_type() == "cn.hutool.core.annotation.Alias" {
                    if let Some(AnnotationValue::String(target)) = meta.get_raw("value") {
                        if target != name {
                            return self.resolve_value(schema, target);
                        }
                    }
                }
            }
            return attr.default_value.clone();
        }
        AnnotationValue::Unit
    }

    /// 设置属性值。
    pub fn set_value(&mut self, name: impl Into<String>, value: AnnotationValue) {
        self.values.insert(name.into(), value);
    }

    /// 全部显式设置的属性。
    pub fn explicit_values(&self) -> &HashMap<String, AnnotationValue> {
        &self.values
    }
}
