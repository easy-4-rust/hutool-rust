//! 对齐: `cn.hutool.core.util.PrimitiveArrayUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/PrimitiveArrayUtil.java
//!
//! 泛型切片实现覆盖 Java 各原始类型重载（isEmpty/reverse/shuffle/min/max 等）。

use rand::rng;
use rand::seq::SliceRandom;

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.PrimitiveArrayUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct PrimitiveArrayUtil;

impl PrimitiveArrayUtil {
    /// 对齐 Java: `PrimitiveArrayUtil.isEmpty(T[])`
    pub fn is_empty<T>(array: &[T]) -> bool {
        array.is_empty()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.isNotEmpty(T[])`
    pub fn is_not_empty<T>(array: &[T]) -> bool {
        !array.is_empty()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.resize`（截断或零扩展）
    pub fn resize<T: Clone + Default>(array: &[T], new_size: usize) -> Vec<T> {
        let mut out = vec![T::default(); new_size];
        let n = array.len().min(new_size);
        out[..n].clone_from_slice(&array[..n]);
        out
    }

    /// 对齐 Java: `PrimitiveArrayUtil.addAll(T[]...)`
    pub fn add_all<T: Clone>(arrays: &[&[T]]) -> Vec<T> {
        let total: usize = arrays.iter().map(|a| a.len()).sum();
        let mut out = Vec::with_capacity(total);
        for a in arrays {
            out.extend_from_slice(a);
        }
        out
    }

    /// 对齐 Java: `PrimitiveArrayUtil.sub(T[], int, int)`
    pub fn sub<T: Clone>(array: &[T], start: usize, end: usize) -> Vec<T> {
        let start = start.min(array.len());
        let end = end.min(array.len());
        if start >= end {
            return Vec::new();
        }
        array[start..end].to_vec()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.split(T[], int)`
    pub fn split<T: Clone>(array: &[T], size: usize) -> Vec<Vec<T>> {
        if size == 0 || array.is_empty() {
            return vec![array.to_vec()];
        }
        array.chunks(size).map(|c| c.to_vec()).collect()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.indexOf`
    pub fn index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().position(|x| x == value)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.lastIndexOf`
    pub fn last_index_of<T: PartialEq>(array: &[T], value: &T) -> Option<usize> {
        array.iter().rposition(|x| x == value)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.contains`
    pub fn contains<T: PartialEq>(array: &[T], value: &T) -> bool {
        array.contains(value)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.remove(T[], int)`
    pub fn remove<T: Clone>(array: &[T], index: usize) -> Vec<T> {
        let mut out = array.to_vec();
        if index < out.len() {
            out.remove(index);
        }
        out
    }

    /// 对齐 Java: `PrimitiveArrayUtil.removeEle(T[], T)`
    pub fn remove_ele<T: Clone + PartialEq>(array: &[T], element: &T) -> Vec<T> {
        array.iter().filter(|x| *x != element).cloned().collect()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.reverse(T[])`（整段）
    pub fn reverse<T>(array: &mut [T]) {
        array.reverse();
    }

    /// 对齐 Java: `PrimitiveArrayUtil.reverse(T[], int, int)`（半开区间）
    pub fn reverse_range<T>(array: &mut [T], start: usize, end: usize) {
        let end = end.min(array.len());
        let start = start.min(end);
        array[start..end].reverse();
    }

    /// 对齐 Java: `PrimitiveArrayUtil.swap(T[], int, int)`
    pub fn swap<T>(array: &mut [T], i: usize, j: usize) {
        if i < array.len() && j < array.len() {
            array.swap(i, j);
        }
    }

    /// 对齐 Java: `PrimitiveArrayUtil.shuffle(T[])`
    pub fn shuffle<T>(array: &mut [T]) {
        array.shuffle(&mut rng());
    }

    /// 对齐 Java: `PrimitiveArrayUtil.min(T...)`
    pub fn min<T: Ord + Copy>(array: &[T]) -> Result<T> {
        array
            .iter()
            .copied()
            .min()
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.max(T...)`
    pub fn max<T: Ord + Copy>(array: &[T]) -> Result<T> {
        array
            .iter()
            .copied()
            .max()
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.min(double...)` / `float...`
    pub fn min_f64(array: &[f64]) -> Result<f64> {
        array
            .iter()
            .copied()
            .reduce(f64::min)
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.max(double...)`
    pub fn max_f64(array: &[f64]) -> Result<f64> {
        array
            .iter()
            .copied()
            .reduce(f64::max)
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.min(float...)`
    pub fn min_f32(array: &[f32]) -> Result<f32> {
        array
            .iter()
            .copied()
            .reduce(f32::min)
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.max(float...)`
    pub fn max_f32(array: &[f32]) -> Result<f32> {
        array
            .iter()
            .copied()
            .reduce(f32::max)
            .ok_or(CoreError::InvalidArgument {
                name: "array",
                reason: "Number array must not empty !",
            })
    }

    /// 对齐 Java: `PrimitiveArrayUtil.isSortedASC`
    pub fn is_sorted_asc<T: PartialOrd>(array: &[T]) -> bool {
        array.windows(2).all(|w| w[0] <= w[1])
    }

    /// 对齐 Java: `PrimitiveArrayUtil.isSortedDESC`
    pub fn is_sorted_desc<T: PartialOrd>(array: &[T]) -> bool {
        array.windows(2).all(|w| w[0] >= w[1])
    }

    /// 对齐 Java: `PrimitiveArrayUtil.isSorted`（默认升序）
    pub fn is_sorted<T: PartialOrd>(array: &[T]) -> bool {
        Self::is_sorted_asc(array)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.range(int)` → `[0, stop)`
    pub fn range(stop: i32) -> Vec<i32> {
        Self::range_from_to(0, stop)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.range(int, int)`
    pub fn range_from_to(start: i32, stop: i32) -> Vec<i32> {
        Self::range_step(start, stop, 1)
    }

    /// 对齐 Java: `PrimitiveArrayUtil.range(int, int, int)`
    pub fn range_step(start: i32, stop: i32, step: i32) -> Vec<i32> {
        if step == 0 {
            return Vec::new();
        }
        let mut out = Vec::new();
        if step > 0 {
            let mut cur = start;
            while cur < stop {
                out.push(cur);
                cur += step;
            }
        } else {
            let mut cur = start;
            while cur > stop {
                out.push(cur);
                cur += step;
            }
        }
        out
    }

    /// 对齐 Java: `PrimitiveArrayUtil.wrap`（原始切片 → Vec，语义等同拷贝）
    pub fn wrap<T: Clone>(values: &[T]) -> Vec<T> {
        values.to_vec()
    }

    /// 对齐 Java: `PrimitiveArrayUtil.unWrap`（同 wrap，Rust 无装箱区分）
    pub fn un_wrap<T: Clone>(values: &[T]) -> Vec<T> {
        values.to_vec()
    }
}
