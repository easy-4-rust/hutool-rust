//! 对齐: `cn.hutool.core.lang.DefaultSegment`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/DefaultSegment.java

#![allow(dead_code)] // 对齐 Java DefaultSegment，暂未接线，预留

use super::segment::Segment;

/// 对齐 Java: `cn.hutool.core.lang.DefaultSegment`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DefaultSegment {
    start_index: i64,
    end_index: i64,
}

impl DefaultSegment {
    /// 对齐 Java: `DefaultSegment(T, T)`
    #[must_use]
    pub fn new(start_index: i64, end_index: i64) -> Self {
        Self {
            start_index,
            end_index,
        }
    }
}

impl Segment for DefaultSegment {
    fn get_start_index(&self) -> i64 {
        self.start_index
    }

    fn get_end_index(&self) -> i64 {
        self.end_index
    }
}

#[cfg(test)]
mod default_segment_idiomatic_parity {
    use super::*;

    #[test]
    fn default_segment_indexes_and_length() {
        let s = DefaultSegment::new(3, 10);
        assert_eq!(s.get_start_index(), 3);
        assert_eq!(s.get_end_index(), 10);
        assert_eq!(s.length(), 7);
    }
}
