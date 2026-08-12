//! 对齐: `cn.hutool.core.img.ColorUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/img/ColorUtil.java

/// 对齐 Java 类: `cn.hutool.core.img.ColorUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ColorUtil;

impl ColorUtil {
    /// 对齐 Java: `ColorUtil.toHex(Color)` / RGB
    pub fn to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    /// 对齐 Java: `ColorUtil.getColor(String)`
    pub fn get_color(color: &str) -> Option<(u8, u8, u8)> {
        let s = color.trim().trim_start_matches('#');
        if s.len() == 3 {
            let r = u8::from_str_radix(&s[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&s[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&s[2..3].repeat(2), 16).ok()?;
            return Some((r, g, b));
        }
        if s.len() != 6 {
            return None;
        }
        Some((
            u8::from_str_radix(&s[0..2], 16).ok()?,
            u8::from_str_radix(&s[2..4], 16).ok()?,
            u8::from_str_radix(&s[4..6], 16).ok()?,
        ))
    }

    /// 对齐 Java: `ColorUtil.toRgb(String)`
    pub fn to_rgb(color: &str) -> Option<[u8; 3]> {
        Self::get_color(color).map(|(r, g, b)| [r, g, b])
    }

    /// 对齐 Java: `ColorUtil.getColor(int)` — packed RGB int。
    pub fn from_rgb_int(rgb: i32) -> (u8, u8, u8) {
        let v = rgb as u32;
        (
            ((v >> 16) & 0xff) as u8,
            ((v >> 8) & 0xff) as u8,
            (v & 0xff) as u8,
        )
    }

    /// 对齐 Java: `ColorUtil.toInt(Color)`
    pub fn to_rgb_int(r: u8, g: u8, b: u8) -> i32 {
        ((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)) as i32
    }

    /// 对齐 Java: `ColorUtil.randomColor()`
    pub fn random_color() -> (u8, u8, u8) {
        use rand::RngExt;
        let mut rng = rand::rng();
        (rng.random(), rng.random(), rng.random())
    }

    /// 对齐 Java: `ColorUtil.hexToColor(String)`
    pub fn hex_to_color(hex: &str) -> Option<(u8, u8, u8)> {
        Self::get_color(hex)
    }

    /// 对齐 Java: `ColorUtil.add(Color, Color)` — 通道饱和相加。
    pub fn add(a: (u8, u8, u8), b: (u8, u8, u8)) -> (u8, u8, u8) {
        (
            a.0.saturating_add(b.0),
            a.1.saturating_add(b.1),
            a.2.saturating_add(b.2),
        )
    }

    /// 对齐 Java: `ColorUtil.computeColorDistance` — 欧氏距离平方根。
    pub fn compute_color_distance(a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
        let dr = f64::from(a.0 as i16 - b.0 as i16);
        let dg = f64::from(a.1 as i16 - b.1 as i16);
        let db = f64::from(a.2 as i16 - b.2 as i16);
        (dr * dr + dg * dg + db * db).sqrt()
    }

    /// 对齐 Java: `ColorUtil.maxDistance` — RGB 立方体最大距离。
    pub fn max_distance() -> f64 {
        Self::compute_color_distance((0, 0, 0), (255, 255, 255))
    }

    /// 对齐 Java: `ColorUtil.getMainColor` — 像素均值近似主色。
    pub fn get_main_color(pixels: &[(u8, u8, u8)]) -> Option<(u8, u8, u8)> {
        if pixels.is_empty() {
            return None;
        }
        let n = pixels.len() as u64;
        let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);
        for &(pr, pg, pb) in pixels {
            r += u64::from(pr);
            g += u64::from(pg);
            b += u64::from(pb);
        }
        Some(((r / n) as u8, (g / n) as u8, (b / n) as u8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        assert_eq!(ColorUtil::to_hex(10, 20, 30), "#0A141E");
        assert_eq!(ColorUtil::get_color("#0A141E"), Some((10, 20, 30)));
        assert_eq!(ColorUtil::from_rgb_int(0x0A141E), (10, 20, 30));
    }
}
