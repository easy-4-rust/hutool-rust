//! 对齐: `cn.hutool.core.net.MaskBit`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/MaskBit.java

use crate::net::ipv4_util::Ipv4Util;

/// 对齐 Java 类: `cn.hutool.core.net.MaskBit`
#[allow(dead_code)] // 对齐 Java 静态工具类，暂未被库内接线调用，预留
#[derive(Debug, Clone, Copy, Default)]
pub struct MaskBit;

#[allow(dead_code)] // 对齐 Java 静态方法，预留供外部调用
impl MaskBit {
    /// 对齐 Java: `MaskBit.get(int)`
    pub fn get(mask_bit: i32) -> Option<String> {
        Ipv4Util::get_mask_by_mask_bit(mask_bit)
    }

    /// 对齐 Java: `MaskBit.getMaskBit(String)`
    pub fn get_mask_bit(mask: &str) -> Option<i32> {
        Ipv4Util::get_mask_bit_by_mask(mask)
    }
}
