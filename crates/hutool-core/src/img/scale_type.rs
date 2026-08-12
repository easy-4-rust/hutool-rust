//! 对齐: `cn.hutool.core.img.ScaleType`
//! 来源: hutool-core/src/main/java/cn/hutool/core/img/ScaleType.java
//!
//! 状态: 对齐桩,等待完整实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java enum: `cn.hutool.core.img.ScaleType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] // 对齐 Java 枚举常量名 AREA_AVERAGING
pub enum ScaleType {
    /// 对齐 Java 枚举常量: `DEFAULT`
    DEFAULT,
    /// 对齐 Java 枚举常量: `FAST`
    FAST,
    /// 对齐 Java 枚举常量: `SMOOTH`
    SMOOTH,
    /// 对齐 Java 枚举常量: `REPLICATE`
    REPLICATE,
    /// 对齐 Java 枚举常量: `AREA_AVERAGING`
    AREA_AVERAGING,
}
