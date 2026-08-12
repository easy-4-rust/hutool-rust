//! 对齐: `cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotation`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::annotation_util::AnnotationUtil;
use super::cacheable_synthesized_annotation_attribute_processor::CacheableSynthesizedAnnotationAttributeProcessor;
use super::generic_synthesized_annotation::GenericSynthesizedAnnotation;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};
use super::scanner::annotation_scanner::AnnotationScanner;
use super::scanner::meta_annotation_scanner::MetaAnnotationScanner;
use super::synthesized_aggregate_annotation::SynthesizedAggregateAnnotation;
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;
use super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;
use super::synthesized_annotation_selector::SynthesizedAnnotationSelector;

/// 对齐 Java 类: `cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotation`
///
/// 部分字段仅用于镜像 Java 类结构（parity 对照），当前实现未读取。
#[allow(dead_code)]
pub struct GenericSynthesizedAggregateAnnotation {
    root: Arc<AnnotationMirror>,
    source: Vec<Arc<AnnotationMirror>>,
    vertical_distance: i32,
    horizontal_distance: i32,
    selector: Arc<dyn SynthesizedAnnotationSelector>,
    attribute_processor: Arc<dyn SynthesizedAnnotationAttributeProcessor>,
    post_processors: Vec<Arc<dyn SynthesizedAnnotationPostProcessor>>,
    scanner: Arc<dyn AnnotationScanner>,
    synthesized_map: HashMap<AnnotationTypeName, Arc<dyn SynthesizedAnnotation>>,
    proxy_cache: RwLock<HashMap<AnnotationTypeName, Arc<AnnotationMirror>>>,
}

impl GenericSynthesizedAggregateAnnotation {
    /// 基于根注解构造聚合合成注解。
    pub fn new(source: Arc<AnnotationMirror>) -> Self {
        Self::with_scanner(vec![source], Arc::new(MetaAnnotationScanner::new(true)))
    }

    /// 指定扫描器构造。
    pub fn with_scanner(
        source: Vec<Arc<AnnotationMirror>>,
        scanner: Arc<dyn AnnotationScanner>,
    ) -> Self {
        let selector =
            super::synthesized_annotation_selector::Selectors::nearest_and_oldest_priority();
        let attribute_processor = Arc::new(CacheableSynthesizedAnnotationAttributeProcessor::new());
        let post_processors = vec![
            super::synthesized_annotation_post_processor::PostProcessors::alias_annotation_post_processor(),
            super::synthesized_annotation_post_processor::PostProcessors::mirror_link_annotation_post_processor(),
            super::synthesized_annotation_post_processor::PostProcessors::alias_link_annotation_post_processor(),
        ];
        Self::full(
            None,
            0,
            0,
            source,
            selector,
            attribute_processor,
            post_processors,
            scanner,
        )
    }

    /// 完整构造。
    pub fn full(
        root: Option<Arc<AnnotationMirror>>,
        vertical_distance: i32,
        horizontal_distance: i32,
        source: Vec<Arc<AnnotationMirror>>,
        selector: Arc<dyn SynthesizedAnnotationSelector>,
        attribute_processor: Arc<dyn SynthesizedAnnotationAttributeProcessor>,
        post_processors: Vec<Arc<dyn SynthesizedAnnotationPostProcessor>>,
        scanner: Arc<dyn AnnotationScanner>,
    ) -> Self {
        assert!(!source.is_empty(), "source must not empty");
        for s in &source {
            assert!(
                !AnnotationUtil::is_synthesized_annotation(s),
                "source has been synthesized"
            );
        }
        let root = root.unwrap_or_else(|| Arc::clone(&source[0]));
        let mut synthesized_map: HashMap<AnnotationTypeName, Arc<dyn SynthesizedAnnotation>> =
            HashMap::new();
        for (i, source_annotation) in source.iter().enumerate() {
            let root_syn: Arc<dyn SynthesizedAnnotation> = GenericSynthesizedAnnotation::new(
                Arc::clone(&root),
                Arc::clone(source_annotation),
                0,
                i as i32,
            );
            synthesized_map.insert(source_annotation.annotation_type(), Arc::clone(&root_syn));
            assert!(
                scanner.support_type(source_annotation.annotation_type()),
                "scanner cannot support type"
            );
            let selector_ref = Arc::clone(&selector);
            let root_ref = Arc::clone(&root);
            let mut scanned: Vec<(i32, Arc<AnnotationMirror>)> = Vec::new();
            {
                let scanned_ref = &mut scanned;
                let mut consumer: super::scanner::annotation_scanner::ScanConsumer<'_> =
                    Box::new(move |index, annotation| scanned_ref.push((index, annotation)));
                scanner.scan_meta(source_annotation.annotation_type(), &mut consumer);
            }
            for (index, annotation) in scanned {
                let new_syn: Arc<dyn SynthesizedAnnotation> = GenericSynthesizedAnnotation::new(
                    Arc::clone(&root_ref),
                    annotation,
                    index + 1,
                    synthesized_map.len() as i32,
                );
                let type_name = new_syn.annotation_type();
                if let Some(old) = synthesized_map.get(&type_name).cloned() {
                    synthesized_map.insert(type_name, selector_ref.choose(old, new_syn));
                } else {
                    synthesized_map.insert(type_name, new_syn);
                }
            }
        }

        let aggregate = Self {
            root: Arc::clone(&root),
            source,
            vertical_distance,
            horizontal_distance,
            selector,
            attribute_processor,
            post_processors,
            scanner,
            synthesized_map,
            proxy_cache: RwLock::new(HashMap::new()),
        };
        aggregate.run_post_processors();
        aggregate
    }

    fn run_post_processors(&self) {
        let mut processors = self.post_processors.clone();
        processors.sort_by_key(|p| p.order());
        for processor in processors {
            for syn in self.synthesized_map.values() {
                processor.process(Arc::clone(syn), self);
            }
        }
    }

    fn build_proxy(&self, annotation_type: AnnotationTypeName) -> Option<Arc<AnnotationMirror>> {
        let _syn = self.synthesized_map.get(&annotation_type)?;
        let schema_attrs = super::element::global_registry()
            .read()
            .schema(annotation_type)?
            .attributes
            .iter()
            .map(|def| def.name)
            .collect::<Vec<_>>();
        let mut values = HashMap::new();
        for name in schema_attrs {
            if let Some(v) = self.get_attribute_value(
                name,
                super::element::global_registry()
                    .read()
                    .schema(annotation_type)
                    .and_then(|s| s.attribute(name))
                    .map(|a| a.value_kind)
                    .unwrap_or(ValueKind::Void),
            ) {
                values.insert(name.to_string(), v);
            }
        }
        let mut mirror = AnnotationMirror::new(annotation_type, values);
        mirror = mirror.mark_synthesized();
        Some(Arc::new(mirror))
    }

    /// 获取 selector（测试用）。
    pub fn get_annotation_selector(&self) -> Arc<dyn SynthesizedAnnotationSelector> {
        Arc::clone(&self.selector)
    }

    /// 获取 attribute processor（测试用）。
    pub fn get_annotation_attribute_processor(
        &self,
    ) -> Arc<dyn SynthesizedAnnotationAttributeProcessor> {
        Arc::clone(&self.attribute_processor)
    }

    /// 获取 post processors（测试用）。
    pub fn get_annotation_post_processors(
        &self,
    ) -> Vec<Arc<dyn SynthesizedAnnotationPostProcessor>> {
        self.post_processors.clone()
    }
}

impl AnnotationSynthesizer for GenericSynthesizedAggregateAnnotation {
    fn get_source(&self) -> Vec<Arc<AnnotationMirror>> {
        self.source.clone()
    }

    fn get_annotation_selector(&self) -> Arc<dyn SynthesizedAnnotationSelector> {
        Arc::clone(&self.selector)
    }

    fn get_annotation_attribute_processor(
        &self,
    ) -> Arc<dyn SynthesizedAnnotationAttributeProcessor> {
        Arc::clone(&self.attribute_processor)
    }

    fn get_annotation_post_processors(&self) -> Vec<Arc<dyn SynthesizedAnnotationPostProcessor>> {
        self.post_processors.clone()
    }

    fn get_synthesized_annotation(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<dyn SynthesizedAnnotation>> {
        self.synthesized_map.get(&annotation_type).cloned()
    }

    fn get_all_synthesized_annotation(
        &self,
    ) -> HashMap<AnnotationTypeName, Arc<dyn SynthesizedAnnotation>> {
        self.synthesized_map.clone()
    }

    fn synthesize(&self, annotation_type: AnnotationTypeName) -> Option<Arc<AnnotationMirror>> {
        if let Some(cached) = self
            .proxy_cache
            .read()
            .unwrap()
            .get(&annotation_type)
            .cloned()
        {
            return Some(cached);
        }
        let built = self.build_proxy(annotation_type)?;
        self.proxy_cache
            .write()
            .unwrap()
            .insert(annotation_type, Arc::clone(&built));
        Some(built)
    }

    fn get_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue> {
        let list: Vec<Arc<dyn SynthesizedAnnotation>> =
            self.synthesized_map.values().cloned().collect();
        self.attribute_processor
            .get_attribute_value(attribute_name, attribute_type, &list)
    }
}

impl SynthesizedAggregateAnnotation for GenericSynthesizedAggregateAnnotation {
    fn get_root(&self) -> Arc<AnnotationMirror> {
        Arc::clone(&self.root)
    }

    fn get_annotation(&self, annotation_type: AnnotationTypeName) -> Option<Arc<AnnotationMirror>> {
        self.synthesized_map
            .get(&annotation_type)
            .map(|s| s.get_annotation())
    }

    fn is_annotation_present(&self, annotation_type: AnnotationTypeName) -> bool {
        self.synthesized_map.contains_key(&annotation_type)
    }

    fn get_annotations(&self) -> Vec<Arc<AnnotationMirror>> {
        self.synthesized_map
            .values()
            .map(|s| s.get_annotation())
            .collect()
    }

    fn synthesize_view(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<AnnotationMirror>> {
        self.synthesize(annotation_type)
    }
}

/// 聚合注解类型常量（parity 测试）。
pub const GENERIC_SYNTHESIZED_AGGREGATE_TYPE: AnnotationTypeName =
    "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotation";
