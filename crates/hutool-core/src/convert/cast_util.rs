//! 对齐: `cn.hutool.core.convert.CastUtil`

#![allow(dead_code)]

/// 对齐 Java 类: `cn.hutool.core.convert.CastUtil`
///
/// Java 的 `castUp`/`castDown` 仅为泛型擦除下的强制转换，运行时对象恒等。
#[derive(Debug, Clone, Default)]
pub struct CastUtil;

impl CastUtil {
    /// 兼容 sentinel。
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    /// 对齐 Java: `CastUtil.castUp` — 返回同一引用语义（Rust 中按值恒等返回）
    pub fn cast_up<T>(value: T) -> T {
        value
    }

    /// 对齐 Java: `CastUtil.castDown`
    pub fn cast_down<T>(value: T) -> T {
        value
    }
}
