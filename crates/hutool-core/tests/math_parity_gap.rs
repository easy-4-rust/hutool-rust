//! `cn.hutool.core.math` 缺口 parity
//!
//! 对齐: `cn.hutool.core.math.*` 未覆盖 @Test
//! 来源: hutool-core/src/test/java/cn/hutool/core/math/**

use hutool_core::math::{
    ArithmeticOverflow, Arrangement, Calculator, Combination, MathUtil, Money,
};
use num_bigint::BigInt;

/// 浮点近似相等断言。
fn assert_f64_eq(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected}, got {actual}"
    );
}

// ── ArrangementTest ──

/// 对齐 Java: `ArrangementTest.selectTest()`
#[test]
fn arrangement_select_test() {
    let arrangement = Arrangement::new(["1", "2", "3", "4"]);
    let list = arrangement.select(2);
    assert_eq!(list.len() as i64, Arrangement::count(4, 2));
    assert_eq!(list[0], vec!["1", "2"]);
    assert_eq!(list[1], vec!["1", "3"]);
    assert_eq!(list[11], vec!["4", "3"]);

    let select_all = arrangement.select_all();
    assert_eq!(select_all.len() as i64, Arrangement::count_all(4));

    let list2 = arrangement.select(0);
    assert_eq!(list2.len(), 1);
    assert!(list2[0].is_empty());
}

/// 对齐 Java: `ArrangementTest.boundaryTest()`
#[test]
fn arrangement_boundary_test() {
    let arr = Arrangement::new(["A", "B", "C"]);
    assert_eq!(arr.select(3).len(), 6);
    assert_eq!(arr.select(1).len(), 3);
    assert_eq!(arr.select(1)[0], vec!["A"]);
    assert!(arr.select(10).is_empty());
    assert!(arr.select(-1).is_empty());
}

/// 对齐 Java: `ArrangementTest.emptyTest()`
#[test]
fn arrangement_empty_test() {
    let arrangement = Arrangement::new([] as [&str; 0]);
    assert_eq!(arrangement.select(0).len(), 1);
    assert!(arrangement.select(1).is_empty());
    assert!(arrangement.select_all().is_empty());
}

/// 对齐 Java: `ArrangementTest.duplicateElementTest()`
#[test]
fn arrangement_duplicate_element_test() {
    let arrangement = Arrangement::new(["1", "1", "3"]);
    let list = arrangement.select(2);
    assert_eq!(list.len(), 6);
}

/// 对齐 Java: `ArrangementTest.selectAllTest()`
#[test]
fn arrangement_select_all_test() {
    let arrangement = Arrangement::new(["1", "2", "3"]);
    let all = arrangement.select_all();
    assert_eq!(all.len() as i64, Arrangement::count_all(3));
    assert_eq!(all.len(), 15);
    assert_eq!(all[0], vec!["1"]);
    assert_eq!(all[3], vec!["1", "2"]);
    assert_eq!(all[9], vec!["1", "2", "3"]);
}

/// 对齐 Java: `ArrangementTest.iteratorTest()`
#[test]
fn arrangement_iterator_test() {
    let arrangement = Arrangement::new(["1", "2", "3"]);
    let iter_result = arrangement.iterate(2);
    assert_eq!(iter_result.len(), 6);
    assert_eq!(iter_result[0], vec!["1", "2"]);
    assert_eq!(iter_result[5], vec!["3", "2"]);
}

/// 对齐 Java: `ArrangementTest.iteratorFullTest()`
#[test]
fn arrangement_iterator_full_test() {
    let arrangement = Arrangement::new(["1", "2", "3"]);
    assert_eq!(arrangement.iterate(3).len(), 6);
}

/// 对齐 Java: `ArrangementTest.iteratorBoundaryTest()`
#[test]
fn arrangement_iterator_boundary_test() {
    let arrangement = Arrangement::new(["1", "2", "3"]);
    assert!(arrangement.iterate(5).is_empty());
}

// ── CalculatorTest ──

/// 对齐 Java: `CalculatorTest.conversationTest()`
#[test]
fn calculator_conversation_test() {
    let v = Calculator::conversion("(0*1--3)-5/-4-(3*(-2.13))");
    assert_f64_eq(v, 10.64, 0.01);
}

/// 对齐 Java: `CalculatorTest.conversationTest2()`
#[test]
fn calculator_conversation_test2() {
    assert_f64_eq(Calculator::conversion("77 * 12"), 924.0, 0.001);
}

/// 对齐 Java: `CalculatorTest.conversationTest3()`
#[test]
fn calculator_conversation_test3() {
    assert_f64_eq(Calculator::conversion("1"), 1.0, 0.001);
}

/// 对齐 Java: `CalculatorTest.conversationTest4()`
#[test]
fn calculator_conversation_test4() {
    let expected = (88.0 * 66.0 / 23.0) % 26.0;
    assert_f64_eq(Calculator::conversion("(88*66/23)%26+45%9"), expected, 1e-7);
}

/// 对齐 Java: `CalculatorTest.conversationTest5()`
#[test]
fn calculator_conversation_test5() {
    assert_f64_eq(
        Calculator::conversion("((1/1) / (1/1) -1) * 100"),
        0.0,
        0.001,
    );
}

/// 对齐 Java: `CalculatorTest.conversationTest6()`
#[test]
fn calculator_conversation_test6() {
    let expected = -1.0 * (2.12 - 2.0) * 100.0;
    assert_f64_eq(Calculator::conversion("-((2.12-2) * 100)"), expected, 0.01);
}

/// 对齐 Java: `CalculatorTest.conversationTest7()`
#[test]
fn calculator_conversation_test7() {
    assert_f64_eq(
        Calculator::conversion("((-2395+0) * 0.3+140.24+35+90)/30"),
        -15.11,
        0.01,
    );
}

/// 对齐 Java: `CalculatorTest.issue2964Test()`
#[test]
fn calculator_issue2964_test() {
    assert_f64_eq(Calculator::conversion("(11+2)12"), 156.0, 0.001);
}

/// 对齐 Java: `CalculatorTest.issue3787Test()`
#[test]
fn calculator_issue3787_test() {
    let mut calc = Calculator::new();
    assert_f64_eq(calc.calculate("0+50/100x(1/0.5)"), 1.0, 0.001);
    assert_f64_eq(calc.calculate("0+50/100X(1/0.5)"), 1.0, 0.001);
}

/// 对齐 Java: `CalculatorTest.scientificNotationPlusTest()`
#[test]
fn calculator_scientific_notation_plus_test() {
    assert_f64_eq(Calculator::conversion("1e+3"), 1000.0, 0.001);
    assert_f64_eq(Calculator::conversion("2.5e+2 + 1.0e-1"), 250.1, 0.001);
}

/// 对齐 Java: `CalculatorTest.unaryOperatorConsistencyTest()`
#[test]
fn calculator_unary_operator_consistency_test() {
    assert_f64_eq(Calculator::conversion("--3"), 3.0, 0.001);
    assert_f64_eq(Calculator::conversion("+-3"), -3.0, 0.001);
    assert_f64_eq(Calculator::conversion("+3"), 3.0, 0.001);
    assert_f64_eq(Calculator::conversion("-3"), -3.0, 0.001);
}

/// 对齐 Java: `CalculatorTest.percentOperatorTest()`
#[test]
fn calculator_percent_operator_test() {
    assert_f64_eq(Calculator::conversion("10 % 3"), 1.0, 0.001);
    assert_f64_eq(Calculator::conversion("10 % +-3"), 1.0, 0.001);
    assert_f64_eq(Calculator::conversion("10 % -3"), 1.0, 0.001);
    assert_f64_eq(Calculator::conversion("10 % (-3)"), 1.0, 0.001);
    assert_f64_eq(Calculator::conversion("10 * 5 % 3"), 2.0, 0.001);
    assert_f64_eq(Calculator::conversion("20 / 5 % 3"), 1.0, 0.001);
    assert_f64_eq(Calculator::conversion("100 % 7 % 3"), 2.0, 0.001);
    assert_f64_eq(Calculator::conversion("10 + 15 % 4"), 13.0, 0.001);
    assert_f64_eq(Calculator::conversion("-10 % 3"), -1.0, 0.001);
    assert_f64_eq(Calculator::conversion("-10 % -3"), -1.0, 0.001);
    assert_f64_eq(Calculator::conversion("10.5 % 3.2"), 0.9, 0.001);
}

// ── CombinationTest ──

/// 对齐 Java: `CombinationTest.countTest()`
#[test]
fn combination_count_test() {
    assert_eq!(Combination::count(5, 2), 10);
    assert_eq!(Combination::count(5, 5), 1);
    assert_eq!(Combination::count(5, 0), 1);
    assert_eq!(Combination::count_all(5), 31);

    let combination = Combination::new(["1", "2", "3", "4", "5"]);
    let list = combination.select(2);
    assert_eq!(list.len() as i64, Combination::count(5, 2));
    assert_eq!(list[0], vec!["1", "2"]);
    assert_eq!(list[9], vec!["4", "5"]);
    assert_eq!(
        combination.select_all().len() as i64,
        Combination::count_all(5)
    );
    assert_eq!(combination.select(0).len(), 1);
}

/// 对齐 Java: `CombinationTest.selectTest()`
#[test]
fn combination_select_test() {
    let combination = Combination::new(["1", "2", "3", "4", "5"]);
    let list = combination.select(2);
    assert_eq!(list.len(), 10);
    assert_eq!(list[4], vec!["2", "3"]);
}

/// 对齐 Java: `CombinationTest.testCountBig_basicCases()`
#[test]
fn combination_test_count_big_basic_cases() {
    assert_eq!(Combination::count_big(5, 0), BigInt::from(1));
    assert_eq!(Combination::count_big(5, 5), BigInt::from(1));
    assert_eq!(Combination::count_big(5, 3), BigInt::from(10));
    assert_eq!(Combination::count_big(5, 2), BigInt::from(10));
}

/// 对齐 Java: `CombinationTest.testCountBig_mGreaterThanN()`
#[test]
fn combination_test_count_big_m_greater_than_n() {
    assert_eq!(Combination::count_big(5, 6), BigInt::from(0));
}

/// 对齐 Java: `CombinationTest.testCountBig_negativeInput()`
#[test]
#[should_panic(expected = "non-negative")]
fn combination_test_count_big_negative_input() {
    let _ = Combination::count_big(-1, 3);
}

/// 对齐 Java: `CombinationTest.testCountBig_symmetry()`
#[test]
fn combination_test_count_big_symmetry() {
    assert_eq!(
        Combination::count_big(20, 3),
        Combination::count_big(20, 17)
    );
}

/// 对齐 Java: `CombinationTest.testCountBig_largeNumbers()`
#[test]
fn combination_test_count_big_large_numbers() {
    assert_eq!(Combination::count_big(50, 3), BigInt::from(19_600));
    let expected = BigInt::parse_bytes(b"100891344545564193334812497256", 10).unwrap();
    assert_eq!(Combination::count_big(100, 50), expected);
}

/// 对齐 Java: `CombinationTest.testCountBig_veryLargeCombination()`
#[test]
fn combination_test_count_big_very_large_combination() {
    let result = Combination::count_big(2000, 1000);
    assert!(result.sign() == num_bigint::Sign::Plus);
}

/// 对齐 Java: `CombinationTest.testCount_basic()`
#[test]
fn combination_test_count_basic() {
    assert_eq!(Combination::count(5, 3), 10);
    assert_eq!(Combination::count(5, 0), 1);
    assert_eq!(Combination::count(5, 6), 0);
}

/// 对齐 Java: `CombinationTest.testCount_overflowBehavior()`
#[test]
fn combination_test_count_overflow_behavior() {
    let _ = Combination::count(100, 50);
}

/// 对齐 Java: `CombinationTest.testCount_noException()`
#[test]
fn combination_test_count_no_exception() {
    let _ = Combination::count(5000, 2500);
}

/// 对齐 Java: `CombinationTest.testCountSafe_exactFitsLong()`
#[test]
fn combination_test_count_safe_exact_fits_long() {
    assert_eq!(Combination::count_safe(50, 3).unwrap(), 19_600);
}

/// 对齐 Java: `CombinationTest.testCountSafe_overflowThrows()`
#[test]
fn combination_test_count_safe_overflow_throws() {
    assert!(Combination::count_safe(100, 50).is_err());
}

/// 对齐 Java: `CombinationTest.testCountSafe_invalidInput()`
#[test]
fn combination_test_count_safe_invalid_input() {
    assert!(Combination::count_safe(-1, 3).is_err());
    assert!(Combination::count_safe(3, -1).is_err());
}

// ── MoneyTest ──

/// 对齐 Java: `MoneyTest.yuanToCentTest()`
#[test]
fn money_yuan_to_cent_test() {
    let money = Money::from_yuan_str("1234.56");
    assert_eq!(money.get_cent(), 123_456);
    assert_eq!(MathUtil::yuan_to_cent(1234.56), 123_456);
}

/// 对齐 Java: `MoneyTest.centToYuanTest()`
#[test]
fn money_cent_to_yuan_test() {
    let money = Money::from_yuan_cent(1234, 56);
    assert_f64_eq(
        money.get_amount().to_string().parse::<f64>().unwrap(),
        1234.56,
        0.001,
    );
    assert_f64_eq(MathUtil::cent_to_yuan(123_456), 1234.56, 0.001);
}

/// 对齐 Java: `MoneyTest.currencyScalingTest()`
#[test]
fn money_currency_scaling_test() {
    let mut jpy = Money::with_currency_fraction(0, 0);
    jpy.set_amount(rust_decimal::Decimal::ONE);
    assert_eq!(jpy.get_cent(), 1);
}

// 额外：覆盖 parity.rs 中 arrangementTest 映射
/// 对齐 Java: `ArrangementTest.arrangementTest()`
#[test]
fn arrangement_test() {
    assert_eq!(Arrangement::count(4, 2), 12);
    assert_eq!(Arrangement::count(4, 1), 4);
    assert_eq!(Arrangement::count(4, 0), 1);
    assert_eq!(Arrangement::count_all(4), 64);
}

// 抑制 unused import 警告（文档对齐 Java ArithmeticException）
#[allow(dead_code)]
fn _arithmetic_overflow_marker(_: ArithmeticOverflow) {}

// ── BitStatusUtil / Money arithmetic (Wave2) ──

use hutool_core::math::BitStatusUtil;

/// 对齐 Java: `BitStatusUtil` add/has/remove/clear
#[test]
fn bit_status_util_wave2_test() {
    let s = BitStatusUtil::add(0, 2);
    assert!(BitStatusUtil::has(s, 2));
    let s2 = BitStatusUtil::add(s, 4);
    assert!(BitStatusUtil::has(s2, 2) && BitStatusUtil::has(s2, 4));
    let s3 = BitStatusUtil::remove(s2, 2);
    assert!(!BitStatusUtil::has(s3, 2));
    assert_eq!(BitStatusUtil::clear(), 0);
}

/// 对齐 Java: Money add/subtract/multiply/allocate
#[test]
fn money_arithmetic_wave2_test() {
    let a = Money::from_yuan_str("10.00");
    let b = Money::from_yuan_str("2.50");
    assert_eq!(a.add(&b).get_cent(), 1250);
    assert_eq!(a.subtract(&b).get_cent(), 750);
    assert_eq!(a.multiply_long(3).get_cent(), 3000);
    assert_eq!(a.divide_f64(2.0).get_cent(), 500);
    let parts = a.allocate(3);
    assert_eq!(parts.len(), 3);
    assert_eq!(parts.iter().map(|m| m.get_cent()).sum::<i64>(), 1000);
    assert!(a.greater_than(&b));
    assert_eq!(format!("{a}"), "10.00");
}
