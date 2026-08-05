//! 对齐: `cn.hutool.core.net.Ipv4Util`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/Ipv4Util.java
//!
//! 基于 `std::net::Ipv4Addr` 的 IPv4 段/掩码工具。

use std::net::Ipv4Addr;

/// 对齐 Java 类: `cn.hutool.core.net.Ipv4Util`
#[derive(Debug, Clone, Copy, Default)]
pub struct Ipv4Util;

impl Ipv4Util {
    /// 对齐 Java: `Ipv4Util.longToIpv4(long)`
    pub fn long_to_ipv4(long_ip: u32) -> String {
        Ipv4Addr::from(long_ip).to_string()
    }

    /// 对齐 Java: `Ipv4Util.ipv4ToLong(String)`
    pub fn ipv4_to_long(str_ip: &str) -> Option<u32> {
        str_ip.parse::<Ipv4Addr>().ok().map(u32::from)
    }

    /// 对齐 Java: `Ipv4Util.ipv4ToLong(String, long defaultValue)`
    pub fn ipv4_to_long_or(str_ip: &str, default: u32) -> u32 {
        Self::ipv4_to_long(str_ip).unwrap_or(default)
    }

    /// 对齐 Java: `Ipv4Util.isMaskBitValid(int)`
    pub fn is_mask_bit_valid(mask_bit: i32) -> bool {
        (0..=32).contains(&mask_bit)
    }

    /// 对齐 Java: `Ipv4Util.getMaskByMaskBit(int)`
    pub fn get_mask_by_mask_bit(mask_bit: i32) -> Option<String> {
        if !Self::is_mask_bit_valid(mask_bit) {
            return None;
        }
        let mask = if mask_bit == 0 {
            0u32
        } else {
            u32::MAX << (32 - mask_bit as u32)
        };
        Some(Self::long_to_ipv4(mask))
    }

    /// 对齐 Java: `Ipv4Util.getMaskBitByMask(String)`
    pub fn get_mask_bit_by_mask(mask: &str) -> Option<i32> {
        let bits = Self::ipv4_to_long(mask)?;
        if bits == 0 {
            return Some(0);
        }
        // contiguous 1-bits from MSB
        let leading = bits.leading_ones();
        if leading + bits.trailing_zeros() == 32 {
            Some(leading as i32)
        } else {
            None
        }
    }

    /// 对齐 Java: `Ipv4Util.isMaskValid(String)`
    pub fn is_mask_valid(mask: &str) -> bool {
        Self::get_mask_bit_by_mask(mask).is_some()
    }

    /// 对齐 Java: `Ipv4Util.getBeginIpLong(String, int)`
    pub fn get_begin_ip_long(ip: &str, mask_bit: i32) -> Option<u32> {
        let ip_long = Self::ipv4_to_long(ip)?;
        let mask = Self::mask_long(mask_bit)?;
        Some(ip_long & mask)
    }

    /// 对齐 Java: `Ipv4Util.getBeginIpStr(String, int)`
    pub fn get_begin_ip_str(ip: &str, mask_bit: i32) -> Option<String> {
        Self::get_begin_ip_long(ip, mask_bit).map(Self::long_to_ipv4)
    }

    /// 对齐 Java: `Ipv4Util.getEndIpLong(String, int)`
    pub fn get_end_ip_long(ip: &str, mask_bit: i32) -> Option<u32> {
        let begin = Self::get_begin_ip_long(ip, mask_bit)?;
        let host_bits = 32 - mask_bit as u32;
        let host_mask = if host_bits == 32 {
            u32::MAX
        } else {
            (1u32 << host_bits) - 1
        };
        Some(begin | host_mask)
    }

    /// 对齐 Java: `Ipv4Util.getEndIpStr(String, int)`
    pub fn get_end_ip_str(ip: &str, mask_bit: i32) -> Option<String> {
        Self::get_end_ip_long(ip, mask_bit).map(Self::long_to_ipv4)
    }

    /// 对齐 Java: `Ipv4Util.countByMaskBit(int, boolean)`
    pub fn count_by_mask_bit(mask_bit: i32, is_all: bool) -> Option<u64> {
        if !Self::is_mask_bit_valid(mask_bit) {
            return None;
        }
        let host = 32 - mask_bit as u32;
        let total = 1u64 << host;
        Some(if is_all || mask_bit >= 31 {
            total
        } else {
            total.saturating_sub(2)
        })
    }

    /// 对齐 Java: `Ipv4Util.countByIpRange(String, String)`
    pub fn count_by_ip_range(from_ip: &str, to_ip: &str) -> Option<u64> {
        let from = Self::ipv4_to_long(from_ip)? as u64;
        let to = Self::ipv4_to_long(to_ip)? as u64;
        if to < from {
            return None;
        }
        Some(to - from + 1)
    }

    /// 对齐 Java: `Ipv4Util.list(String, String)` — 闭区间 IP 列表（上限 65536）。
    pub fn list_range(from_ip: &str, to_ip: &str) -> Option<Vec<String>> {
        let from = Self::ipv4_to_long(from_ip)? as u64;
        let to = Self::ipv4_to_long(to_ip)? as u64;
        if to < from || to - from > 65_536 {
            return None;
        }
        Some((from..=to).map(|v| Self::long_to_ipv4(v as u32)).collect())
    }

    /// 对齐 Java: `Ipv4Util.list(String, int, boolean)`
    pub fn list_by_mask(ip: &str, mask_bit: i32, is_all: bool) -> Option<Vec<String>> {
        let begin = Self::get_begin_ip_long(ip, mask_bit)? as u64;
        let end = Self::get_end_ip_long(ip, mask_bit)? as u64;
        let (start, stop) = if is_all || mask_bit >= 31 {
            (begin, end)
        } else {
            (begin + 1, end.saturating_sub(1))
        };
        if stop < start || stop - start > 65_536 {
            return None;
        }
        Some(
            (start..=stop)
                .map(|v| Self::long_to_ipv4(v as u32))
                .collect(),
        )
    }

    /// 对齐 Java: `Ipv4Util.formatIpBlock(String, String)`
    pub fn format_ip_block(ip: &str, mask: &str) -> Option<String> {
        let bit = Self::get_mask_bit_by_mask(mask)?;
        Some(format!("{ip}/{bit}"))
    }

    /// 对齐 Java: `Ipv4Util.isInnerIP(String)`
    pub fn is_inner_ip(ip_address: &str) -> bool {
        let Some(ip) = Self::ipv4_to_long(ip_address) else {
            return false;
        };
        // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16, 127.0.0.0/8
        matches!(ip >> 24, 10 | 127) || (ip >> 20) == 0xAC1 || (ip >> 16) == 0xC0A8
    }

    /// 对齐 Java: `Ipv4Util.matches(String wildcard, String ipAddress)`
    ///
    /// 通配符使用 `*`，例如 `192.168.*.*`。
    pub fn matches(wildcard: &str, ip_address: &str) -> bool {
        let w: Vec<&str> = wildcard.split('.').collect();
        let i: Vec<&str> = ip_address.split('.').collect();
        if w.len() != 4 || i.len() != 4 {
            return false;
        }
        w.iter().zip(i.iter()).all(|(a, b)| *a == "*" || *a == *b)
    }

    /// 对齐 Java: `Ipv4Util.getMaskByIpRange(String, String)`
    pub fn get_mask_by_ip_range(from_ip: &str, to_ip: &str) -> Option<String> {
        let from = Self::ipv4_to_long(from_ip)?;
        let to = Self::ipv4_to_long(to_ip)?;
        if to < from {
            return None;
        }
        let xor = from ^ to;
        let host_bits = 32 - xor.leading_zeros();
        let mask_bit = 32 - host_bits as i32;
        Self::get_mask_by_mask_bit(mask_bit)
    }

    fn mask_long(mask_bit: i32) -> Option<u32> {
        if !Self::is_mask_bit_valid(mask_bit) {
            return None;
        }
        Some(if mask_bit == 0 {
            0
        } else {
            u32::MAX << (32 - mask_bit as u32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_mask() {
        assert_eq!(Ipv4Util::long_to_ipv4(0xC0A80101), "192.168.1.1");
        assert_eq!(Ipv4Util::ipv4_to_long("192.168.1.1"), Some(0xC0A80101));
        assert_eq!(
            Ipv4Util::get_mask_by_mask_bit(24).as_deref(),
            Some("255.255.255.0")
        );
        assert_eq!(Ipv4Util::get_mask_bit_by_mask("255.255.255.0"), Some(24));
        assert!(Ipv4Util::is_inner_ip("192.168.0.1"));
        assert!(!Ipv4Util::is_inner_ip("8.8.8.8"));
        assert!(Ipv4Util::matches("192.168.*.*", "192.168.1.2"));
    }
}
