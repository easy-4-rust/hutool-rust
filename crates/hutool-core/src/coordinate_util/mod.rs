use std::fmt;
use std::hash::{Hash, Hasher};

mod coordinate;
mod coordinate_util;

pub use coordinate::Coordinate;
pub use coordinate_util::CoordinateUtil;

struct JavaDouble(f64);

fn java_double_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}

fn double_hash_code(value: f64) -> i32 {
    let bits = java_double_bits(value);
    let folded = (bits ^ (bits >> 32)).to_le_bytes();
    i32::from_le_bytes([folded[0], folded[1], folded[2], folded[3]])
}

fn offset(lng: f64, lat: f64, is_plus: bool) -> Coordinate {
    let mut dlng = trans_lng(lng - 105.0, lat - 35.0);
    let mut dlat = trans_lat(lng - 105.0, lat - 35.0);
    let mut magic = (lat / 180.0 * CoordinateUtil::PI).sin();
    magic = 1.0 - CoordinateUtil::CORRECTION_PARAM * magic * magic;
    let sqrt_magic = magic.sqrt();
    dlng = dlng * 180.0
        / (CoordinateUtil::RADIUS / sqrt_magic
            * (lat / 180.0 * CoordinateUtil::PI).cos()
            * CoordinateUtil::PI);
    dlat = dlat * 180.0
        / (CoordinateUtil::RADIUS * (1.0 - CoordinateUtil::CORRECTION_PARAM)
            / (magic * sqrt_magic)
            * CoordinateUtil::PI);
    if !is_plus {
        dlng = -dlng;
        dlat = -dlat;
    }
    Coordinate::new(dlng, dlat)
}

fn trans_lng(lng: f64, lat: f64) -> f64 {
    let mut result =
        300.0 + lng + 2.0 * lat + 0.1 * lng * lng + 0.1 * lng * lat + 0.1 * lng.abs().sqrt();
    result += (20.0 * (6.0 * lng * CoordinateUtil::PI).sin()
        + 20.0 * (2.0 * lng * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (20.0 * (lng * CoordinateUtil::PI).sin()
        + 40.0 * (lng / 3.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (150.0 * (lng / 12.0 * CoordinateUtil::PI).sin()
        + 300.0 * (lng / 30.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result
}

fn trans_lat(lng: f64, lat: f64) -> f64 {
    let mut result =
        -100.0 + 2.0 * lng + 3.0 * lat + 0.2 * lat * lat + 0.1 * lng * lat + 0.2 * lng.abs().sqrt();
    result += (20.0 * (6.0 * lng * CoordinateUtil::PI).sin()
        + 20.0 * (2.0 * lng * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (20.0 * (lat * CoordinateUtil::PI).sin()
        + 40.0 * (lat / 3.0 * CoordinateUtil::PI).sin())
        * 2.0
        / 3.0;
    result += (160.0 * (lat / 12.0 * CoordinateUtil::PI).sin()
        + 320.0 * (lat * CoordinateUtil::PI / 30.0).sin())
        * 2.0
        / 3.0;
    result
}
