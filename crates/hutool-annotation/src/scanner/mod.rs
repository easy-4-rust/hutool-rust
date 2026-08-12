//! Workspace-owned scanner entrypoints for `cn.hutool.core.annotation`.
//!
//! Concrete element, field, method, type, and generic scanners now live in
//! this crate. Only the meta-annotation scanner still bridges to
//! `hutool-macro`.

pub mod abstract_type_annotation_scanner;
pub mod annotation_scanner;
pub mod element_annotation_scanner;
pub mod empty_annotation_scanner;
pub mod field_annotation_scanner;
pub mod generic_annotation_scanner;
pub mod method_annotation_scanner;
pub mod type_annotation_scanner;

pub use annotation_scanner::{AnnotationScanner, ScanConsumer, Scanners};
pub use element_annotation_scanner::ElementAnnotationScanner;
pub use empty_annotation_scanner::EmptyAnnotationScanner;
pub use field_annotation_scanner::FieldAnnotationScanner;
pub use generic_annotation_scanner::GenericAnnotationScanner;
pub use hutool_macro::scanner::MetaAnnotationScanner;
pub use hutool_macro::scanner::meta_annotation_scanner;
pub use method_annotation_scanner::MethodAnnotationScanner;
pub use type_annotation_scanner::TypeAnnotationScanner;
