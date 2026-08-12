//! 对齐: `cn.hutool.core.annotation.GenericSynthesizedAnnotation`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::annotation_attribute::AnnotationAttribute;
use super::cacheable_annotation_attribute::CacheableAnnotationAttribute;
use super::element::global_registry;
use super::hierarchical::Hierarchical;
use super::mirror::{
    AnnotationMirror, AnnotationTypeName, AnnotationValue, AttributeRef, ValueKind,
};
use super::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java 类: `cn.hutool.core.annotation.GenericSynthesizedAnnotation`
pub struct GenericSynthesizedAnnotation {
    root: Arc<AnnotationMirror>,
    annotation: Arc<AnnotationMirror>,
    vertical_distance: i32,
    horizontal_distance: i32,
    attributes: RwLock<HashMap<String, Arc<dyn AnnotationAttribute>>>,
}

impl GenericSynthesizedAnnotation {
    /// 创建合成注解。
    pub fn new(
        root: Arc<AnnotationMirror>,
        annotation: Arc<AnnotationMirror>,
        vertical_distance: i32,
        horizontal_distance: i32,
    ) -> Arc<Self> {
        let attrs = Self::load_attributes(&annotation);
        Arc::new(Self {
            root,
            annotation,
            vertical_distance,
            horizontal_distance,
            attributes: RwLock::new(attrs),
        })
    }

    fn load_attributes(
        annotation: &Arc<AnnotationMirror>,
    ) -> HashMap<String, Arc<dyn AnnotationAttribute>> {
        let registry = global_registry().read();
        let Some(schema) = registry.schema(annotation.annotation_type()) else {
            return HashMap::new();
        };
        schema
            .attributes
            .iter()
            .map(|def| {
                let attr = Arc::new(CacheableAnnotationAttribute::new(
                    Arc::clone(annotation),
                    AttributeRef::new(annotation.annotation_type(), def.name),
                )) as Arc<dyn AnnotationAttribute>;
                (def.name.to_string(), attr)
            })
            .collect()
    }
}

impl Hierarchical for GenericSynthesizedAnnotation {
    fn get_root(&self) -> Option<&dyn std::any::Any> {
        Some(self.root.as_ref())
    }
    fn get_vertical_distance(&self) -> i32 {
        self.vertical_distance
    }
    fn get_horizontal_distance(&self) -> i32 {
        self.horizontal_distance
    }
}

impl SynthesizedAnnotation for GenericSynthesizedAnnotation {
    fn get_annotation(&self) -> Arc<AnnotationMirror> {
        Arc::clone(&self.annotation)
    }

    fn has_attribute(&self, attribute_name: &str, return_type: ValueKind) -> bool {
        self.attributes
            .read()
            .unwrap()
            .get(attribute_name)
            .map(|a| super::mirror::is_assignable(return_type, &a.get_value()))
            .unwrap_or(false)
    }

    fn get_attributes(&self) -> HashMap<String, Arc<dyn AnnotationAttribute>> {
        self.attributes.read().unwrap().clone()
    }

    fn set_attribute(&self, attribute_name: &str, attribute: Arc<dyn AnnotationAttribute>) {
        self.attributes
            .write()
            .unwrap()
            .insert(attribute_name.to_string(), attribute);
    }

    fn replace_attribute(
        &self,
        attribute_name: &str,
        operator: Box<
            dyn Fn(Arc<dyn AnnotationAttribute>) -> Arc<dyn AnnotationAttribute> + Send + Sync,
        >,
    ) {
        let mut map = self.attributes.write().unwrap();
        if let Some(old) = map.get(attribute_name).cloned() {
            map.insert(attribute_name.to_string(), operator(old));
        }
    }

    fn get_attribute_value(&self, attribute_name: &str) -> Option<AnnotationValue> {
        self.attributes
            .read()
            .unwrap()
            .get(attribute_name)
            .map(|a| a.get_value())
    }

    fn annotation_type(&self) -> AnnotationTypeName {
        self.annotation.annotation_type()
    }
}
