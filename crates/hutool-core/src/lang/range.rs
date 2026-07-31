//! 对齐: `cn.hutool.core.lang.Range`

#![allow(dead_code)] // 对齐 Java Range，暂未接线，预留

/// 对齐 Java: `Range<T>` 步进迭代器
pub struct Range<T> {
    current: Option<T>,
    end: T,
    steppers: Box<dyn Fn(&T, &T, usize) -> Option<T>>,
    index: usize,
    include_end_consumed: bool,
}

impl<T: Clone> Range<T> {
    /// 创建 Range：`stepper(current, end, index) -> next`，返回 `None` 结束
    pub fn new<F>(start: T, end: T, stepper: F) -> Self
    where
        F: Fn(&T, &T, usize) -> Option<T> + 'static,
    {
        Self {
            current: Some(start),
            end,
            steppers: Box::new(stepper),
            index: 0,
            include_end_consumed: false,
        }
    }

    /// 对齐 `hasNext`
    pub fn has_next(&self) -> bool {
        self.current.is_some()
    }

    /// 对齐 `next`
    pub fn next(&mut self) -> Option<T> {
        let cur = self.current.take()?;
        let next = (self.steppers)(&cur, &self.end, self.index);
        self.index += 1;
        self.current = next;
        Some(cur)
    }

    /// 收集为列表
    pub fn to_list(mut self) -> Vec<T> {
        let mut out = Vec::new();
        while let Some(v) = self.next() {
            out.push(v);
        }
        out
    }
}

/// 整数步进 Range 辅助
pub fn int_range(start: i32, end: i32, step: i32) -> Range<i32> {
    Range::new(start, end, move |cur, end, _| {
        let n = cur + step;
        if step > 0 {
            if n > *end {
                None
            } else {
                Some(n)
            }
        } else if n < *end {
            None
        } else {
            Some(n)
        }
    })
}

#[cfg(test)]
mod range_idiomatic_parity {
    use super::*;

    /// 对齐 Java Range.hasNext/next 可执行证据。
    #[test]
    fn int_range_to_list() {
        let list = int_range(1, 3, 1).to_list();
        assert_eq!(list, vec![1, 2, 3]);
        assert!(!int_range(5, 5, 1).to_list().is_empty());
    }
}
