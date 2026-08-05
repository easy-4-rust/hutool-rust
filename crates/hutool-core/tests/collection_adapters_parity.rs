//! collection_adapters module parity tests
//! 对齐: hutool-core CollectionUtil/TransCollection tests

use hutool_core::{CollectionUtil, SpliteratorUtil, TransCollection};

// ── CollectionUtil (alias for CollUtil) ──

#[test]
fn collection_util_is_empty() {
    assert!(CollectionUtil::is_empty(Some(&[] as &[i32])));
    assert!(!CollectionUtil::is_empty(Some(&[1, 2] as &[i32])));
}

#[test]
fn collection_util_is_not_empty() {
    assert!(!CollectionUtil::is_not_empty(Some(&[] as &[i32])));
    assert!(CollectionUtil::is_not_empty(Some(&[1, 2] as &[i32])));
}

// ── TransCollection ──

#[test]
fn trans_collection_basic() {
    let mut source = vec![1, 2, 3];
    let tc = TransCollection::new(&mut source, |x: &i32| x * 10);
    let collected: Vec<i32> = tc.iter().collect();
    assert_eq!(collected, vec![10, 20, 30]);
}

#[test]
fn trans_collection_len() {
    let mut source = vec![1, 2, 3];
    let tc = TransCollection::new(&mut source, |x: &i32| x * 10);
    assert_eq!(tc.len(), 3);
    assert!(!tc.is_empty());
}

// ── SpliteratorUtil ──

#[test]
fn spliterator_util_trans() {
    let source = vec![1, 2, 3];
    let trans = SpliteratorUtil::trans(source.into_iter(), |x: i32| x * 2);
    let collected: Vec<i32> = trans.collect();
    assert_eq!(collected, vec![2, 4, 6]);
}
