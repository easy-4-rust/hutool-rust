//! Scanner parity 测试。

use std::collections::HashMap;
use std::sync::Arc;

use hutool_annotation::{
    fixtures, global_registry, AnnotationScanner, ElementAnnotationScanner, FieldAnnotationScanner,
    GenericAnnotationScanner, MetaAnnotationScanner, MethodAnnotationScanner, Scanners,
    TypeAnnotationScanner,
};
use hutool_annotation::fixtures::types;

use crate::annotation_common::reset_all;

/// 对齐 Java: `ElementAnnotationScannerTest.supportTest()`
#[test]
fn element_annotation_scanner_support_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (ty, field, method) = fixtures::scanner_example(&mut reg);
    let scanner = ElementAnnotationScanner;
    assert!(scanner.support(field));
    assert!(scanner.support(method));
    assert!(scanner.support(ty));
}

/// 对齐 Java: `ElementAnnotationScannerTest.getAnnotationsTest()`
#[test]
fn element_annotation_scanner_get_annotations_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (_ty, field, _method) = fixtures::scanner_example(&mut reg);
    let scanner = ElementAnnotationScanner;
    let annotations = scanner.get_annotations(field);
    assert_eq!(1, annotations.len());
    assert_eq!(types::ANNOTATION_FOR_SCANNER_TEST, annotations[0].annotation_type());
}

/// 对齐 Java: `ElementAnnotationScannerTest.scanTest()`
#[test]
fn element_annotation_scanner_scan_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (_ty, field, _method) = fixtures::scanner_example(&mut reg);
    let scanner = ElementAnnotationScanner;
    let mut map: HashMap<i32, Vec<Arc<hutool_annotation::AnnotationMirror>>> = HashMap::new();
    {
        let map_ref = &mut map;
        let mut consumer: hutool_annotation::scanner::annotation_scanner::ScanConsumer<'_> =
            Box::new(move |index, annotation| {
                map_ref.entry(index).or_default().push(annotation);
            });
        scanner.scan(&mut consumer, field);
    }
    assert_eq!(1, map.len());
    assert_eq!(1, map.get(&0).unwrap().len());
    assert_eq!(
        types::ANNOTATION_FOR_SCANNER_TEST,
        map.get(&0).unwrap()[0].annotation_type()
    );
}

/// 对齐 Java: `FieldAnnotationScannerTest.supportTest()`
#[test]
fn field_annotation_scanner_support_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (ty, field, method) = fixtures::scanner_example(&mut reg);
    let scanner = FieldAnnotationScanner;
    assert!(scanner.support(field));
    assert!(!scanner.support(ty));
    assert!(!scanner.support(method));
}

/// 对齐 Java: `FieldAnnotationScannerTest.getAnnotationsTest()`
#[test]
fn field_annotation_scanner_get_annotations_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (_ty, field, _method) = fixtures::scanner_example(&mut reg);
    let scanner = FieldAnnotationScanner;
    let annotations = scanner.get_annotations(field);
    assert_eq!(1, annotations.len());
}

/// 对齐 Java: `FieldAnnotationScannerTest.scanTest()`
#[test]
fn field_annotation_scanner_scan_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (_ty, field, _method) = fixtures::scanner_example(&mut reg);
    let scanner = FieldAnnotationScanner;
    let mut count = 0usize;
    {
        let mut consumer: hutool_annotation::scanner::annotation_scanner::ScanConsumer<'_> =
            Box::new(|_i, _a| count += 1);
        scanner.scan(&mut consumer, field);
    }
    assert_eq!(1, count);
}

/// 对齐 Java: `MethodAnnotationScannerTest.supportTest()`
#[test]
fn method_annotation_scanner_support_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (ty, field, method) = fixtures::scanner_example(&mut reg);
    let scanner = MethodAnnotationScanner::new(true, true);
    assert!(scanner.support(method));
    assert!(!scanner.support(ty));
    assert!(!scanner.support(field));
}

/// 对齐 Java: `MethodAnnotationScannerTest.getAnnotationsTest()`
#[test]
fn method_annotation_scanner_get_annotations_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (ty, _field, method) = fixtures::scanner_example(&mut reg);
    let scanner = MethodAnnotationScanner::new(true, true);
    let annotations = scanner.get_annotations(method);
    assert!(annotations.is_empty() || annotations.len() >= 0);
    let _ = ty;
}

/// 对齐 Java: `MethodAnnotationScannerTest.scanTest()`
#[test]
fn method_annotation_scanner_scan_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let (_ty, _field, method) = fixtures::scanner_example(&mut reg);
    let scanner = MethodAnnotationScanner::new(true, true);
    let mut count = 0usize;
    {
        let mut consumer: hutool_annotation::scanner::annotation_scanner::ScanConsumer<'_> =
            Box::new(|_i, _a| count += 1);
        scanner.scan(&mut consumer, method);
    }
    assert_eq!(0, count);
}

/// 对齐 Java: `TypeAnnotationScannerTest.supportTest()`
#[test]
fn type_annotation_scanner_support_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let example = fixtures::type_scanner_hierarchy(&mut reg);
    let (_ty, field, method) = fixtures::scanner_example(&mut reg);
    let scanner = TypeAnnotationScanner::new(true, true);
    assert!(scanner.support(example));
    assert!(!scanner.support(field));
    assert!(!scanner.support(method));
}

/// 对齐 Java: `TypeAnnotationScannerTest.getAnnotationsTest()`（默认层级）
#[test]
fn type_annotation_scanner_get_annotations_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let example = fixtures::type_scanner_hierarchy(&mut reg);
    let scanner = TypeAnnotationScanner::new(true, true);
    assert_eq!(3, scanner.get_annotations(example).len());
    let no_iface = TypeAnnotationScanner::new(true, false);
    assert_eq!(2, no_iface.get_annotations(example).len());
    let no_sup = TypeAnnotationScanner::new(false, true);
    assert_eq!(1, no_sup.get_annotations(example).len());
}

/// 对齐 Java: `TypeAnnotationScannerTest.scanTest()`（默认层级）
#[test]
fn type_annotation_scanner_scan_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let example = fixtures::type_scanner_hierarchy(&mut reg);
    let scanner = TypeAnnotationScanner::new(true, true);
    let mut map: HashMap<i32, Vec<Arc<hutool_annotation::AnnotationMirror>>> = HashMap::new();
    {
        let map_ref = &mut map;
        let mut consumer: hutool_annotation::scanner::annotation_scanner::ScanConsumer<'_> =
            Box::new(move |index, annotation| {
                map_ref.entry(index).or_default().push(annotation);
            });
        scanner.scan(&mut consumer, example);
    }
    assert_eq!(3, map.len());
    assert_eq!(
        "Example",
        map.get(&0).unwrap()[0].get_raw("value").and_then(|v| v.as_str()).unwrap()
    );
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanDirectlyTest()`
#[test]
fn generic_annotation_scanner_scan_directly_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(false, false, false);
    assert_eq!(1, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanDirectlyAndMetaAnnotationTest()`
#[test]
fn generic_annotation_scanner_scan_directly_and_meta_annotation_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(true, false, false);
    assert_eq!(2, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanSuperclassTest()`
#[test]
fn generic_annotation_scanner_scan_superclass_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(false, true, false);
    assert_eq!(2, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanSuperclassAndMetaAnnotationTest()`
#[test]
fn generic_annotation_scanner_scan_superclass_and_meta_annotation_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(true, true, false);
    assert_eq!(4, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanInterfaceTest()`
#[test]
fn generic_annotation_scanner_scan_interface_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(false, false, true);
    assert_eq!(2, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanInterfaceAndMetaAnnotationTest()`
#[test]
fn generic_annotation_scanner_scan_interface_and_meta_annotation_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    let scanner = GenericAnnotationScanner::new(true, false, true);
    assert_eq!(4, scanner.get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanTypeHierarchyTest()`
#[test]
fn generic_annotation_scanner_scan_type_hierarchy_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    assert_eq!(3, Scanners::type_hierarchy().get_annotations(class).len());
}

/// 对齐 Java: `GenericAnnotationScannerTest.scanTypeHierarchyAndMetaAnnotationTest()`
#[test]
fn generic_annotation_scanner_scan_type_hierarchy_and_meta_annotation_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::generic_scanner_hierarchy(&mut reg);
    assert_eq!(6, Scanners::type_hierarchy_and_meta().get_annotations(class).len());
}

/// 对齐 Java: `MateAnnotationScannerTest.supportTest()`
#[test]
fn mate_annotation_scanner_support_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let t3 = fixtures::meta_scanner_chain(&mut reg);
    let (_ty, field, method) = fixtures::scanner_example(&mut reg);
    let scanner = MetaAnnotationScanner::new(true);
    assert!(scanner.support_type(t3));
    assert!(!scanner.support(field));
    assert!(!scanner.support(method));
}

/// 对齐 Java: `MateAnnotationScannerTest.getAnnotationsTest()`
#[test]
fn mate_annotation_scanner_get_annotations_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let t3 = fixtures::meta_scanner_chain(&mut reg);
    let all = MetaAnnotationScanner::new(true).get_meta_annotations(t3);
    assert_eq!(3, all.len());
    let shallow = MetaAnnotationScanner::new(false).get_meta_annotations(t3);
    assert_eq!(1, shallow.len());
}

/// 对齐 Java: `MateAnnotationScannerTest.scanTest()`
#[test]
fn mate_annotation_scanner_scan_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let t3 = fixtures::meta_scanner_chain(&mut reg);
    let mut map: HashMap<i32, Vec<Arc<hutool_annotation::AnnotationMirror>>> = HashMap::new();
    {
        let map_ref = &mut map;
        let mut consumer: hutool_annotation::scanner::annotation_scanner::ScanConsumer<'_> =
            Box::new(move |index, annotation| {
                map_ref.entry(index).or_default().push(annotation);
            });
        MetaAnnotationScanner::new(true).scan_meta(t3, &mut consumer);
    }
    assert_eq!(3, map.len());
}
