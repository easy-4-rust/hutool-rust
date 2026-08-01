use super::coordinate::Coordinate;

/// Hutool-compatible WGS84, GCJ-02, BD-09 and Web Mercator conversions.

/// 对齐: `cn.hutool.core.util.CoordinateUtil`
/// 坐标工具类
pub struct CoordinateUtil;

impl CoordinateUtil {
    /// GCJ-02/BD-09 intermediate conversion parameter.
    pub const X_PI: f64 = std::f64::consts::PI * 3000.0 / 180.0;
    /// Pi used by Hutool's formulas.
    pub const PI: f64 = std::f64::consts::PI;
    /// Krasovsky 1940 earth radius.
    pub const RADIUS: f64 = 6_378_245.0;
    /// Krasovsky eccentricity correction parameter.
    pub const CORRECTION_PARAM: f64 = 0.006_693_421_622_965_943;

    /// Returns true when GCJ-02 is not defined for the coordinate.
    #[must_use]
    pub fn out_of_china(lng: f64, lat: f64) -> bool {
        !(72.004..=137.8347).contains(&lng) || !(0.8293..=55.8271).contains(&lat)
    }

    /// Converts WGS84 to GCJ-02 using Hutool's approximate offset algorithm.
    #[must_use]
    pub fn wgs84_to_gcj02(lng: f64, lat: f64) -> Coordinate {
        let mut coordinate = Coordinate::new(lng, lat);
        coordinate.offset(offset(lng, lat, true));
        coordinate
    }

    /// Converts WGS84 to BD-09 through GCJ-02.
    #[must_use]
    pub fn wgs84_to_bd09(lng: f64, lat: f64) -> Coordinate {
        let gcj02 = Self::wgs84_to_gcj02(lng, lat);
        Self::gcj02_to_bd09(gcj02.lng, gcj02.lat)
    }

    /// Converts GCJ-02 to WGS84 using Hutool's approximate inverse offset.
    #[must_use]
    pub fn gcj02_to_wgs84(lng: f64, lat: f64) -> Coordinate {
        let mut coordinate = Coordinate::new(lng, lat);
        coordinate.offset(offset(lng, lat, false));
        coordinate
    }

    /// Converts GCJ-02 to BD-09.
    #[must_use]
    pub fn gcj02_to_bd09(lng: f64, lat: f64) -> Coordinate {
        let z = (lng * lng + lat * lat).sqrt() + 0.00002 * (lat * Self::X_PI).sin();
        let theta = lat.atan2(lng) + 0.000_003 * (lng * Self::X_PI).cos();
        Coordinate::new(z * theta.cos() + 0.0065, z * theta.sin() + 0.006)
    }

    /// Converts BD-09 to GCJ-02.
    #[must_use]
    pub fn bd09_to_gcj02(lng: f64, lat: f64) -> Coordinate {
        let x = lng - 0.0065;
        let y = lat - 0.006;
        let z = (x * x + y * y).sqrt() - 0.00002 * (y * Self::X_PI).sin();
        let theta = y.atan2(x) - 0.000_003 * (x * Self::X_PI).cos();
        Coordinate::new(z * theta.cos(), z * theta.sin())
    }

    /// Converts BD-09 to WGS84 through GCJ-02.
    #[must_use]
    pub fn bd09_to_wgs84(lng: f64, lat: f64) -> Coordinate {
        let gcj02 = Self::bd09_to_gcj02(lng, lat);
        Self::gcj02_to_wgs84(gcj02.lng, gcj02.lat)
    }

    /// Projects WGS84 longitude/latitude to Web Mercator.
    #[must_use]
    pub fn wgs84_to_mercator(lng: f64, lat: f64) -> Coordinate {
        let x = lng * 20_037_508.342_789_244 / 180.0;
        let y = ((90.0 + lat) * Self::PI / 360.0).tan().ln() / (Self::PI / 180.0);
        Coordinate::new(x, y * 20_037_508.342_789_244 / 180.0)
    }

    /// Converts Web Mercator coordinates back to WGS84.
    #[must_use]
    pub fn mercator_to_wgs84(mercator_x: f64, mercator_y: f64) -> Coordinate {
        let x = mercator_x / 20_037_508.342_789_244 * 180.0;
        let y = mercator_y / 20_037_508.342_789_244 * 180.0;
        let y = 180.0 / Self::PI * (2.0 * (y * Self::PI / 180.0).exp().atan() - Self::PI / 2.0);
        Coordinate::new(x, y)
    }
}

use super::offset;
