//! 对齐: `cn.hutool.core.lang.Segment`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Segment.java

#![allow(dead_code)] // 对齐 Java Segment，暂未接线，预留

/// 对齐 Java: `cn.hutool.core.lang.Segment`
pub trait Segment {
    /// 对齐 Java: `Segment.getStartIndex()`
    fn get_start_index(&self) -> i64;
    /// 对齐 Java: `Segment.getEndIndex()`
    fn get_end_index(&self) -> i64;
    /// 对齐 Java: `Segment.length()` — `abs(end - start)`
    fn length(&self) -> i64 {
        (self.get_end_index() - self.get_start_index()).abs()
    }
}
