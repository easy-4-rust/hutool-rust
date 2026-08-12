//! `cn.hutool.core.annotation` 子包对齐
//!
//! 结构化注解模型：以 [`mirror::AnnotationMirror`] + [`element::ElementHandle`] 对齐 Java 注解语义。

pub mod abstract_annotation_synthesizer;
pub mod abstract_link_annotation_post_processor;
pub mod abstract_wrapped_annotation_attribute;
pub mod aggregate_annotation;
pub mod alias;
pub mod alias_annotation_post_processor;
pub mod alias_for;
pub mod alias_link_annotation_post_processor;
pub mod aliased_annotation_attribute;
pub mod annotation_attribute;
pub mod annotation_attribute_value_provider;
pub mod annotation_proxy;
pub mod annotation_synthesizer;
pub mod annotation_util;
pub mod cacheable_annotation_attribute;
pub mod cacheable_synthesized_annotation_attribute_processor;
pub mod combination_annotation_element;
pub mod element;
pub mod fixtures;
pub mod fixtures_aggregate;
pub mod force_alias_for;
pub mod force_aliased_annotation_attribute;
pub mod generic_synthesized_aggregate_annotation;
pub mod generic_synthesized_annotation;
pub mod hierarchical;
pub mod link;
pub mod mirror;
pub mod mirror_for;
pub mod mirror_link_annotation_post_processor;
pub mod mirrored_annotation_attribute;
pub mod prop_ignore;
pub mod relation_type;
pub mod scanner;
pub mod synthesized_aggregate_annotation;
pub mod synthesized_annotation;
pub mod synthesized_annotation_attribute_processor;
pub mod synthesized_annotation_post_processor;
pub mod synthesized_annotation_proxy;
pub mod synthesized_annotation_selector;
pub mod wrapped_annotation_attribute;

pub use abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
pub use alias::{Alias, TYPE_NAME as ALIAS_TYPE_NAME};
pub use alias_annotation_post_processor::{ALIAS_TYPE, AliasAnnotationPostProcessor};
pub use alias_for::{AliasFor, TYPE_NAME as ALIAS_FOR_TYPE_NAME};
pub use aliased_annotation_attribute::AliasedAnnotationAttribute;
pub use annotation_attribute::AnnotationAttribute;
pub use annotation_synthesizer::AnnotationSynthesizer;
pub use annotation_util::{AnnotationUtil, mirror_class_name, mirror_string, value_kind_of_name};
pub use cacheable_annotation_attribute::CacheableAnnotationAttribute;
pub use cacheable_synthesized_annotation_attribute_processor::{
    CacheableSynthesizedAnnotationAttributeProcessor, TestValueSynthesizedAnnotation,
};
pub use combination_annotation_element::{clear_combination_cache_for_test, to_combination};
pub use element::{
    AnnotatedElement, AnnotationRegistry, ElementHandle, FieldBuilder, MethodBuilder, TypeBuilder,
    global_registry,
};
pub use force_alias_for::{ForceAliasFor, TYPE_NAME as FORCE_ALIAS_FOR_TYPE_NAME};
pub use force_aliased_annotation_attribute::ForceAliasedAnnotationAttribute;
pub use generic_synthesized_aggregate_annotation::{
    GENERIC_SYNTHESIZED_AGGREGATE_TYPE, GenericSynthesizedAggregateAnnotation,
};
pub use generic_synthesized_annotation::GenericSynthesizedAnnotation;
pub use mirror::{
    AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue, AttributeDef,
    AttributeRef, ValueKind,
};
pub use mirror_for::{MirrorFor, TYPE_NAME as MIRROR_FOR_TYPE_NAME};
pub use mirrored_annotation_attribute::MirroredAnnotationAttribute;
pub use prop_ignore::{PropIgnore, TYPE_NAME as PROP_IGNORE_TYPE};
pub use relation_type::RelationType;
pub use scanner::{
    AnnotationScanner, ElementAnnotationScanner, FieldAnnotationScanner, GenericAnnotationScanner,
    MetaAnnotationScanner, MethodAnnotationScanner, Scanners, TypeAnnotationScanner,
};
pub use synthesized_aggregate_annotation::SynthesizedAggregateAnnotation;
pub use synthesized_annotation::SynthesizedAnnotation;
pub use synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;
pub use synthesized_annotation_post_processor::{
    PostProcessors, SynthesizedAnnotationPostProcessor,
};
pub use synthesized_annotation_selector::{
    Selectors, SynthesizedAnnotationSelector, TestSynthesizedAnnotation,
};
pub use wrapped_annotation_attribute::WrappedAnnotationAttribute;
