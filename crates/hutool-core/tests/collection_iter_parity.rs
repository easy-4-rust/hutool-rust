//! collection_iter module parity tests
//! 对齐: hutool-core FilterIter/IterChain/ArrayIter etc.

use hutool_core::{ArrayIter, CopiedIter, FilterIter, IterChain, TransIter};

// ── ArrayIter ──

#[test]
fn array_iter_basic() {
    let arr = [1, 2, 3];
    let iter = ArrayIter::new(&arr);
    let collected: Vec<&i32> = iter.collect();
    assert_eq!(collected, vec![&1, &2, &3]);
}

// ── CopiedIter ──

#[test]
fn copied_iter_basic() {
    let items = vec![1, 2, 3];
    let iter = CopiedIter::new(items.iter().copied());
    let collected: Vec<i32> = iter.collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

// ── FilterIter ──

#[test]
fn filter_iter_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let iter = FilterIter::new(items.into_iter(), Some(|x: &i32| x % 2 == 0));
    let collected: Vec<i32> = iter.collect();
    assert_eq!(collected, vec![2, 4]);
}

// ── TransIter ──

#[test]
fn trans_iter_basic() {
    let items = vec![1, 2, 3];
    let iter = TransIter::new(items.into_iter(), |x: i32| x * 2);
    let collected: Vec<i32> = iter.collect();
    assert_eq!(collected, vec![2, 4, 6]);
}

// ── IterChain ──

#[test]
fn iter_chain_basic() {
    let a = vec![1, 2];
    let b = vec![3, 4];
    let chain = IterChain::with_iterators(vec![
        Box::new(a.into_iter()) as Box<dyn Iterator<Item = i32>>,
        Box::new(b.into_iter()) as Box<dyn Iterator<Item = i32>>,
    ]);
    let collected: Vec<i32> = chain.collect();
    assert_eq!(collected, vec![1, 2, 3, 4]);
}

#[test]
fn iter_chain_empty() {
    let chain: IterChain<i32> = IterChain::new();
    let collected: Vec<i32> = chain.collect();
    assert!(collected.is_empty());
}
