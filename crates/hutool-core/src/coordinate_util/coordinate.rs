use std::fmt;
use std::hash::{Hash, Hasher};

use super::coordinate_util::CoordinateUtil;

/// A longitude/latitude pair used by Hutool-compatible coordinate conversions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    pub(crate) lng: f64,
    pub(crate) lat: f64,
}

impl Coordinate {
    /// Creates a coordinate from longitude and latitude.
    #[must_use]
    pub const fn new(lng: f64, lat: f64) -> Self {
        Self { lng, lat }
    }

    /// Returns the longitude.
    #[must_use]
    pub const fn lng(self) -> f64 {
        self.lng
    }

    /// Changes the longitude and returns this coordinate for chaining.
    pub fn set_lng(&mut self, lng: f64) -> &mut Self {
        self.lng = lng;
        self
    }

    /// Returns the latitude.
    #[must_use]
    pub const fn lat(self) -> f64 {
        self.lat
    }

    /// Changes the latitude and returns this coordinate for chaining.
    pub fn set_lat(&mut self, lat: f64) -> &mut Self {
        self.lat = lat;
        self
    }

    /// Applies a longitude/latitude offset and returns this coordinate for chaining.
    pub fn offset(&mut self, offset: Self) -> &mut Self {
        self.lng += offset.lng;
        self.lat += offset.lat;
        self
    }

    /// Returns the value produced by Java's `Objects.hash(lng, lat)`.
    #[must_use]
    pub fn hash_code(self) -> i32 {
        let lng = double_hash_code(self.lng);
        let lat = double_hash_code(self.lat);
        31_i32
            .wrapping_mul(31_i32.wrapping_add(lng))
            .wrapping_add(lat)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        java_double_bits(self.lng) == java_double_bits(other.lng)
            && java_double_bits(self.lat) == java_double_bits(other.lat)
    }
}

impl Eq for Coordinate {}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        java_double_bits(self.lng).hash(state);
        java_double_bits(self.lat).hash(state);
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Coordinate{{lng={}, lat={}}}",
            JavaDouble(self.lng),
            JavaDouble(self.lat)
        )
    }
}

impl fmt::Display for JavaDouble {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.0;
        if value.is_nan() {
            return formatter.write_str("NaN");
        }
        if value == f64::INFINITY {
            return formatter.write_str("Infinity");
        }
        if value == f64::NEG_INFINITY {
            return formatter.write_str("-Infinity");
        }

        let absolute = value.abs();
        if value != 0.0 && !(0.001..10_000_000.0).contains(&absolute) {
            let scientific = format!("{value:e}");
            let (mantissa, exponent) = scientific
                .split_once('e')
                .expect("Rust scientific float formatting always contains an exponent");
            if mantissa.contains('.') {
                write!(formatter, "{mantissa}E{exponent}")
            } else {
                write!(formatter, "{mantissa}.0E{exponent}")
            }
        } else if value.fract() == 0.0 {
            write!(formatter, "{value:.1}")
        } else {
            write!(formatter, "{value}")
        }
    }
}

use super::{JavaDouble, double_hash_code, java_double_bits, offset, trans_lat, trans_lng};
