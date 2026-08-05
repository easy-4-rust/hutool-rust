//! Workspace-owned facade for `cn.hutool.core.annotation`.
//!
//! Foundational metadata annotations and the common annotation scanners now live
//! directly in this crate. Higher-level synthesizer, mirror, and meta-scanner
//! internals still delegate to `hutool-macro` while the migration continues in
//! small, low-risk steps.

#![forbid(unsafe_code)]

pub mod alias;
pub mod alias_for;
pub mod force_alias_for;
pub mod mirror;
pub mod mirror_for;
pub mod prop_ignore;
pub mod relation_type;
pub mod scanner;

pub use hutool_macro::{
    abstract_annotation_synthesizer, abstract_link_annotation_post_processor,
    abstract_wrapped_annotation_attribute, aggregate_annotation, alias_annotation_post_processor,
    alias_link_annotation_post_processor, aliased_annotation_attribute, annotation_attribute,
    annotation_attribute_value_provider, annotation_proxy, annotation_synthesizer, annotation_util,
    cacheable_annotation_attribute, cacheable_synthesized_annotation_attribute_processor,
    combination_annotation_element, element, fixtures, fixtures_aggregate,
    force_aliased_annotation_attribute, generic_synthesized_aggregate_annotation,
    generic_synthesized_annotation, hierarchical, link, mirror_link_annotation_post_processor,
    mirrored_annotation_attribute, synthesized_aggregate_annotation, synthesized_annotation,
    synthesized_annotation_attribute_processor, synthesized_annotation_post_processor,
    synthesized_annotation_proxy, synthesized_annotation_selector, wrapped_annotation_attribute,
};

pub use alias::{Alias, TYPE_NAME as ALIAS_TYPE_NAME};
pub use alias_for::{AliasFor, TYPE_NAME as ALIAS_FOR_TYPE_NAME};
pub use force_alias_for::{ForceAliasFor, TYPE_NAME as FORCE_ALIAS_FOR_TYPE_NAME};
pub use hutool_macro::{
    AbstractWrappedAnnotationAttribute,
    alias_annotation_post_processor::{ALIAS_TYPE, AliasAnnotationPostProcessor},
    aliased_annotation_attribute::AliasedAnnotationAttribute,
    annotation_attribute::AnnotationAttribute,
    annotation_synthesizer::AnnotationSynthesizer,
    annotation_util::{AnnotationUtil, mirror_class_name, mirror_string, value_kind_of_name},
    cacheable_annotation_attribute::CacheableAnnotationAttribute,
    cacheable_synthesized_annotation_attribute_processor::{
        CacheableSynthesizedAnnotationAttributeProcessor, TestValueSynthesizedAnnotation,
    },
    combination_annotation_element::{clear_combination_cache_for_test, to_combination},
    element::{
        AnnotatedElement, AnnotationRegistry, ElementHandle, FieldBuilder, MethodBuilder,
        TypeBuilder, global_registry,
    },
    force_aliased_annotation_attribute::ForceAliasedAnnotationAttribute,
    generic_synthesized_aggregate_annotation::{
        GENERIC_SYNTHESIZED_AGGREGATE_TYPE, GenericSynthesizedAggregateAnnotation,
    },
    generic_synthesized_annotation::GenericSynthesizedAnnotation,
    mirrored_annotation_attribute::MirroredAnnotationAttribute,
    synthesized_aggregate_annotation::SynthesizedAggregateAnnotation,
    synthesized_annotation::SynthesizedAnnotation,
    synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor,
    synthesized_annotation_post_processor::{PostProcessors, SynthesizedAnnotationPostProcessor},
    synthesized_annotation_selector::{
        Selectors, SynthesizedAnnotationSelector, TestSynthesizedAnnotation,
    },
    wrapped_annotation_attribute::WrappedAnnotationAttribute,
};
pub use mirror::{
    AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue, AttributeDef,
    AttributeRef, ValueKind, is_assignable, is_jdk_meta_annotation, is_not_jdk_meta_annotation,
};
pub use mirror_for::{MirrorFor, TYPE_NAME as MIRROR_FOR_TYPE_NAME};
pub use prop_ignore::{PropIgnore, TYPE_NAME as PROP_IGNORE_TYPE};
pub use relation_type::RelationType;
pub use scanner::{
    AnnotationScanner, ElementAnnotationScanner, EmptyAnnotationScanner, FieldAnnotationScanner,
    GenericAnnotationScanner, MetaAnnotationScanner, MethodAnnotationScanner, Scanners,
    TypeAnnotationScanner,
};
