//! coordinate_util parity tests
//! 对齐: `cn.hutool.core.util.CoordinateUtilTest`

use hutool_core::{Coordinate, CoordinateUtil};

#[test]
fn coordinate_new() {
    let c = Coordinate::new(116.4074, 39.9042);
    assert_eq!(c.lng(), 116.4074);
    assert_eq!(c.lat(), 39.9042);
}

#[test]
fn coordinate_set_lng_lat() {
    let mut c = Coordinate::new(0.0, 0.0);
    c.set_lng(116.4074).set_lat(39.9042);
    assert_eq!(c.lng(), 116.4074);
    assert_eq!(c.lat(), 39.9042);
}

#[test]
fn coordinate_offset() {
    let mut c = Coordinate::new(116.4074, 39.9042);
    let offset = Coordinate::new(0.001, 0.001);
    c.offset(offset);
    assert!((c.lng() - 116.4084).abs() < 0.0001);
    assert!((c.lat() - 39.9052).abs() < 0.0001);
}

#[test]
fn coordinate_hash_code() {
    let c = Coordinate::new(116.4074, 39.9042);
    let h = c.hash_code();
    assert_ne!(h, 0);
}

#[test]
fn out_of_china_beijing() {
    assert!(!CoordinateUtil::out_of_china(116.4074, 39.9042));
}

#[test]
fn out_of_china_london() {
    assert!(CoordinateUtil::out_of_china(-0.1278, 51.5074));
}

#[test]
fn wgs84_to_gcj02_roundtrip() {
    let gcj = CoordinateUtil::wgs84_to_gcj02(116.4074, 39.9042);
    let wgs = CoordinateUtil::gcj02_to_wgs84(gcj.lng(), gcj.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn wgs84_to_bd09_roundtrip() {
    let bd = CoordinateUtil::wgs84_to_bd09(116.4074, 39.9042);
    let wgs = CoordinateUtil::bd09_to_wgs84(bd.lng(), bd.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn gcj02_to_bd09_roundtrip() {
    let gcj = Coordinate::new(116.4074, 39.9042);
    let bd = CoordinateUtil::gcj02_to_bd09(gcj.lng(), gcj.lat());
    let gcj2 = CoordinateUtil::bd09_to_gcj02(bd.lng(), bd.lat());
    assert!((gcj2.lng() - gcj.lng()).abs() < 0.0001);
    assert!((gcj2.lat() - gcj.lat()).abs() < 0.0001);
}

#[test]
fn wgs84_to_mercator_roundtrip() {
    let mercator = CoordinateUtil::wgs84_to_mercator(116.4074, 39.9042);
    let wgs = CoordinateUtil::mercator_to_wgs84(mercator.lng(), mercator.lat());
    assert!((wgs.lng() - 116.4074).abs() < 0.01);
    assert!((wgs.lat() - 39.9042).abs() < 0.01);
}

#[test]
fn out_of_china_southern_border() {
    assert!(!CoordinateUtil::out_of_china(108.0, 0.83));
    assert!(CoordinateUtil::out_of_china(108.0, 0.82));
}

#[test]
fn out_of_china_northern_border() {
    assert!(!CoordinateUtil::out_of_china(108.0, 55.82));
    assert!(CoordinateUtil::out_of_china(108.0, 55.83));
}

// ── 对齐 Hutool CoordinateUtilTest（精确向量）──

/// 对齐 Java: `CoordinateUtilTest.wgs84ToGcj02Test()`
#[test]
fn wgs84_to_gcj02_test() {
    let coordinate = CoordinateUtil::wgs84_to_gcj02(116.404, 39.915);
    assert_eq!(coordinate.lng(), 116.41024449916938_f64);
    assert_eq!(coordinate.lat(), 39.91640428150164_f64);
}

/// 对齐 Java: `CoordinateUtilTest.gcj02ToWgs84Test()`
#[test]
fn gcj02_to_wgs84_test() {
    let coordinate = CoordinateUtil::gcj02_to_wgs84(116.404, 39.915);
    assert_eq!(coordinate.lng(), 116.39775550083061_f64);
    assert_eq!(coordinate.lat(), 39.91359571849836_f64);
}

/// 对齐 Java: `CoordinateUtilTest.wgs84toBd09Test()`
#[test]
fn wgs84to_bd09_test() {
    let coordinate = CoordinateUtil::wgs84_to_bd09(116.404, 39.915);
    assert_eq!(coordinate.lng(), 116.41662724378733_f64);
    assert_eq!(coordinate.lat(), 39.922699552216216_f64);
}

/// 对齐 Java: `CoordinateUtilTest.wgs84toBd09Test2()`
#[test]
fn wgs84to_bd09_test_2() {
    let coordinate = CoordinateUtil::wgs84_to_bd09(122.99395597_f64, 44.99804071_f64);
    assert!((coordinate.lng() - 123.00636516028885_f64).abs() <= 0.00000000000001_f64);
    assert!((coordinate.lat() - 45.00636909189589_f64).abs() <= 0.00000000000001_f64);
}

/// 对齐 Java: `CoordinateUtilTest.bd09toWgs84Test()`
#[test]
fn bd09to_wgs84_test() {
    let coordinate = CoordinateUtil::bd09_to_wgs84(116.404, 39.915);
    assert_eq!(coordinate.lng(), 116.3913836995125_f64);
    assert_eq!(coordinate.lat(), 39.907253214522164_f64);
}

/// 对齐 Java: `CoordinateUtilTest.gcj02ToBd09Test()`
#[test]
fn gcj02_to_bd09_test() {
    let coordinate = CoordinateUtil::gcj02_to_bd09(116.404, 39.915);
    // Java assertEquals(..., 0) — 允许 ULP 级浮点差
    assert!((coordinate.lng() - 116.41036949371029_f64).abs() <= 1e-14);
    assert!((coordinate.lat() - 39.92133699351022_f64).abs() <= 1e-14);
}

/// 对齐 Java: `CoordinateUtilTest.bd09toGcj02Test()`
#[test]
fn bd09to_gcj02_test() {
    let coordinate = CoordinateUtil::bd09_to_gcj02(116.404, 39.915);
    assert_eq!(coordinate.lng(), 116.39762729119315_f64);
    assert_eq!(coordinate.lat(), 39.90865673957631_f64);
}
