//! 对齐: `cn.hutool.core.comparator.ComparatorChain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/comparator/ComparatorChain.java
//!
//! 多比较器链式综合：相等则落到下一个；可对单步反序。

use std::cmp::Ordering;

/// 对齐 Java 类: `cn.hutool.core.comparator.ComparatorChain`
pub struct ComparatorChain<E> {
    chain: Vec<Box<dyn Fn(&E, &E) -> Ordering + Send + Sync>>,
    /// true = reverse（对齐 Java BitSet set）
    reverse_bits: Vec<bool>,
    locked: bool,
}

impl<E> Default for ComparatorChain<E> {
    /// 对齐 Java: `ComparatorChain()`
    fn default() -> Self {
        Self::new()
    }
}

impl<E> ComparatorChain<E> {
    /// 对齐 Java: `ComparatorChain()`
    #[must_use]
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            reverse_bits: Vec::new(),
            locked: false,
        }
    }

    /// 对齐 Java: `of(Comparator)`
    #[must_use]
    pub fn of<F>(comparator: F) -> Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        Self::of_reverse(comparator, false)
    }

    /// 对齐 Java: `of(Comparator, boolean reverse)`
    #[must_use]
    pub fn of_reverse<F>(comparator: F, reverse: bool) -> Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        let mut chain = Self::new();
        chain.add_comparator_reverse(comparator, reverse);
        chain
    }

    /// 对齐 Java: `of(Comparator...)` / `of(List)`
    #[must_use]
    pub fn of_list(comparators: Vec<Box<dyn Fn(&E, &E) -> Ordering + Send + Sync>>) -> Self {
        let n = comparators.len();
        Self {
            chain: comparators,
            reverse_bits: vec![false; n],
            locked: false,
        }
    }

    /// 对齐 Java: `of(List, BitSet)`
    #[must_use]
    pub fn of_list_bits(
        comparators: Vec<Box<dyn Fn(&E, &E) -> Ordering + Send + Sync>>,
        reverse_bits: Vec<bool>,
    ) -> Self {
        assert_eq!(comparators.len(), reverse_bits.len());
        Self {
            chain: comparators,
            reverse_bits,
            locked: false,
        }
    }

    /// 对齐 Java: `ComparatorChain(Comparator)`
    #[must_use]
    pub fn from_comparator<F>(comparator: F) -> Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        Self::of(comparator)
    }

    /// 对齐 Java: `ComparatorChain(Comparator, boolean)`
    #[must_use]
    pub fn from_comparator_reverse<F>(comparator: F, reverse: bool) -> Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        Self::of_reverse(comparator, reverse)
    }

    /// 对齐 Java: `addComparator(Comparator)`
    pub fn add_comparator<F>(&mut self, comparator: F) -> &mut Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        self.add_comparator_reverse(comparator, false)
    }

    /// 对齐 Java: `addComparator(Comparator, boolean)`
    pub fn add_comparator_reverse<F>(&mut self, comparator: F, reverse: bool) -> &mut Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        self.check_locked();
        self.chain.push(Box::new(comparator));
        self.reverse_bits.push(reverse);
        self
    }

    /// 对齐 Java: `addChain` —— Chain 接口别名。
    pub fn add_chain<F>(&mut self, comparator: F) -> &mut Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        self.add_comparator(comparator)
    }

    /// 对齐 Java: `setComparator(int, Comparator)`
    pub fn set_comparator<F>(&mut self, index: usize, comparator: F) -> &mut Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        self.set_comparator_reverse(index, comparator, false)
    }

    /// 对齐 Java: `setComparator(int, Comparator, boolean)`
    pub fn set_comparator_reverse<F>(
        &mut self,
        index: usize,
        comparator: F,
        reverse: bool,
    ) -> &mut Self
    where
        F: Fn(&E, &E) -> Ordering + Send + Sync + 'static,
    {
        self.check_locked();
        self.chain[index] = Box::new(comparator);
        self.reverse_bits[index] = reverse;
        self
    }

    /// 对齐 Java: `setForwardSort(int)`
    pub fn set_forward_sort(&mut self, index: usize) -> &mut Self {
        self.check_locked();
        self.reverse_bits[index] = false;
        self
    }

    /// 对齐 Java: `setReverseSort(int)`
    pub fn set_reverse_sort(&mut self, index: usize) -> &mut Self {
        self.check_locked();
        self.reverse_bits[index] = true;
        self
    }

    /// 对齐 Java: `size()`
    #[must_use]
    pub fn size(&self) -> usize {
        self.chain.len()
    }

    /// 对齐 Java: `isLocked()`
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// 对齐 Java: `compare(E, E)`
    pub fn compare(&mut self, o1: &E, o2: &E) -> i32 {
        if !self.locked {
            assert!(
                !self.chain.is_empty(),
                "ComparatorChains must contain at least one Comparator"
            );
            self.locked = true;
        }
        for (i, cmp) in self.chain.iter().enumerate() {
            let mut retval = match cmp(o1, o2) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            if retval != 0 {
                if self.reverse_bits[i] {
                    retval = if retval > 0 { -1 } else { 1 };
                }
                return retval;
            }
        }
        0
    }

    fn check_locked(&self) {
        assert!(
            !self.locked,
            "Comparator ordering cannot be changed after the first comparison is performed"
        );
    }
}
