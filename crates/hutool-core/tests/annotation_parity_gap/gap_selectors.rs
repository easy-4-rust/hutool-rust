//! SynthesizedAnnotationSelector parity 测试。

use std::sync::Arc;

use hutool_annotation::{Selectors, SynthesizedAnnotation, SynthesizedAnnotationSelector, TestSynthesizedAnnotation};

/// 对齐 Java: `SynthesizedAnnotationSelectorTest.chooseTest()`
#[test]
fn synthesized_annotation_selector_choose_test() {
    let selector = Selectors::nearest_and_oldest_priority();
    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 1);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 2);
    let chosen = selector.choose(Arc::clone(&a1), Arc::clone(&a2));
    assert!(Arc::ptr_eq(&a1, &chosen));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 1, 3);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 4);
    let chosen = selector.choose(Arc::clone(&a1), Arc::clone(&a2));
    assert!(Arc::ptr_eq(&a1, &chosen));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(1, 0, 5);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 6);
    let chosen = selector.choose(a1, Arc::clone(&a2));
    assert!(Arc::ptr_eq(&a2, &chosen));
}

/// 对齐 Java: `SynthesizedAnnotationSelectorTest.nearestAndNewestPriorityTest()`
#[test]
fn synthesized_annotation_selector_nearest_and_newest_priority_test() {
    let selector = Selectors::nearest_and_newest_priority();
    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 10);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 11);
    assert!(Arc::ptr_eq(
        &a2,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 1, 12);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 13);
    assert!(Arc::ptr_eq(
        &a2,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 14);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(1, 0, 15);
    let chosen = selector.choose(Arc::clone(&a1), a2);
    assert!(Arc::ptr_eq(&a1, &chosen));
}

/// 对齐 Java: `SynthesizedAnnotationSelectorTest.farthestAndOldestPriorityTest()`
#[test]
fn synthesized_annotation_selector_farthest_and_oldest_priority_test() {
    let selector = Selectors::farthest_and_oldest_priority();
    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 20);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 21);
    assert!(Arc::ptr_eq(
        &a1,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 1, 22);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 23);
    assert!(Arc::ptr_eq(
        &a1,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 24);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(1, 0, 25);
    let chosen = selector.choose(a1, Arc::clone(&a2));
    assert!(Arc::ptr_eq(&a2, &chosen));
}

/// 对齐 Java: `SynthesizedAnnotationSelectorTest.farthestAndNewestPriorityTest()`
#[test]
fn synthesized_annotation_selector_farthest_and_newest_priority_test() {
    let selector = Selectors::farthest_and_newest_priority();
    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 30);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 31);
    assert!(Arc::ptr_eq(
        &a2,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 1, 32);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 33);
    assert!(Arc::ptr_eq(
        &a2,
        &selector.choose(Arc::clone(&a1), Arc::clone(&a2))
    ));

    let a1: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(1, 0, 34);
    let a2: Arc<dyn SynthesizedAnnotation> = TestSynthesizedAnnotation::new(0, 0, 35);
    let chosen = selector.choose(a1, Arc::clone(&a2));
    assert!(Arc::ptr_eq(&chosen, &a2));
}
