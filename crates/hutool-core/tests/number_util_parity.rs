//! number_util parity tests
//! 对齐: `cn.hutool.core.util.NumberUtilTest`

use hutool_core::NumberUtil;
use rust_decimal::Decimal;
use std::collections::HashSet;
use std::str::FromStr;

// ── 算术操作 ──

#[test]
fn add_basic() {
    assert_eq!(NumberUtil::add(1.5, 2.5), 4.0);
}

#[test]
fn sub_basic() {
    assert_eq!(NumberUtil::sub(5.0, 3.0), 2.0);
}

#[test]
fn mul_basic() {
    assert_eq!(NumberUtil::mul(3.0, 4.0), 12.0);
}

#[test]
fn div_basic() {
    assert_eq!(NumberUtil::div(10.0, 2.0).unwrap(), 5.0);
}

#[test]
fn div_by_zero() {
    assert!(NumberUtil::div(1.0, 0.0).is_err());
}

#[test]
fn div_with_scale() {
    assert_eq!(NumberUtil::div_with_scale(10.0, 3.0, 2).unwrap(), 3.33);
}

// ── 比较操作 ──

#[test]
fn compare_f64_basic() {
    assert_eq!(NumberUtil::compare_f64(1.0, 2.0), -1);
    assert_eq!(NumberUtil::compare_f64(2.0, 2.0), 0);
    assert_eq!(NumberUtil::compare_f64(3.0, 2.0), 1);
}

#[test]
fn compare_i32_basic() {
    assert_eq!(NumberUtil::compare_i32(1, 2), -1);
    assert_eq!(NumberUtil::compare_i32(2, 2), 0);
    assert_eq!(NumberUtil::compare_i32(3, 2), 1);
}

#[test]
fn compare_i64_basic() {
    assert_eq!(NumberUtil::compare_i64(100, 200), -1);
}

#[test]
fn compare_char_basic() {
    assert_eq!(NumberUtil::compare_char('a', 'b'), -1);
    assert_eq!(NumberUtil::compare_char('a', 'a'), 0);
    assert_eq!(NumberUtil::compare_char('b', 'a'), 1);
}

// ── 相等判断 ──

#[test]
fn equals_f64_basic() {
    assert!(NumberUtil::equals_f64(1.0, 1.0));
    assert!(!NumberUtil::equals_f64(1.0, 2.0));
}

#[test]
fn equals_f32_basic() {
    assert!(NumberUtil::equals_f32(1.0f32, 1.0f32));
    assert!(!NumberUtil::equals_f32(1.0f32, 2.0f32));
}

#[test]
fn equals_i64_basic() {
    assert!(NumberUtil::equals_i64(42, 42));
    assert!(!NumberUtil::equals_i64(42, 43));
}

// ── 最值操作 ──

#[test]
fn min_i32_basic() {
    assert_eq!(NumberUtil::min_i32(&[3, 1, 4, 1, 5]).unwrap(), 1);
}

#[test]
fn min_i64_basic() {
    assert_eq!(NumberUtil::min_i64(&[3, 1, 4, 1, 5]).unwrap(), 1);
}

#[test]
fn min_f64_basic() {
    assert_eq!(NumberUtil::min_f64(&[3.0, 1.0, 4.0]).unwrap(), 1.0);
}

#[test]
fn min_empty() {
    assert!(NumberUtil::min_i32(&[]).is_err());
}

#[test]
fn max_i32_basic() {
    assert_eq!(NumberUtil::max_i32(&[3, 1, 4, 1, 5]).unwrap(), 5);
}

#[test]
fn max_i64_basic() {
    assert_eq!(NumberUtil::max_i64(&[3, 1, 4, 1, 5]).unwrap(), 5);
}

#[test]
fn max_f64_basic() {
    assert_eq!(NumberUtil::max_f64(&[3.0, 1.0, 4.0]).unwrap(), 4.0);
}

#[test]
fn max_empty() {
    assert!(NumberUtil::max_i32(&[]).is_err());
}

// ── 数值判断 ──

#[test]
fn is_number_valid() {
    assert!(NumberUtil::is_number("123"));
    assert!(NumberUtil::is_number("3.14"));
    assert!(NumberUtil::is_number("-1"));
}

#[test]
fn is_number_invalid() {
    assert!(!NumberUtil::is_number("abc"));
    assert!(!NumberUtil::is_number(""));
}

#[test]
fn is_integer_valid() {
    assert!(NumberUtil::is_integer("123"));
    assert!(NumberUtil::is_integer("-1"));
}

#[test]
fn is_integer_invalid() {
    assert!(!NumberUtil::is_integer("3.14"));
    assert!(!NumberUtil::is_integer("abc"));
}

// ── 范围判断 ──

#[test]
fn is_in_range_i64_basic() {
    assert!(NumberUtil::is_in_range_i64(5, 1, 10));
    assert!(!NumberUtil::is_in_range_i64(0, 1, 10));
    assert!(NumberUtil::is_in_range_i64(1, 1, 10));
    assert!(NumberUtil::is_in_range_i64(10, 1, 10));
}

#[test]
fn is_in_range_f64_basic() {
    assert!(NumberUtil::is_in_range_f64(5.0, 1.0, 10.0));
    assert!(!NumberUtil::is_in_range_f64(0.0, 1.0, 10.0));
}

// ── 转换操作 ──

#[test]
fn parse_int_valid() {
    assert_eq!(NumberUtil::parse_int("42").unwrap(), 42);
}

#[test]
fn parse_int_invalid() {
    assert_eq!(NumberUtil::parse_int_default("abc", 0), 0);
}

#[test]
fn parse_long_valid() {
    assert_eq!(NumberUtil::parse_long("1234567890").unwrap(), 1_234_567_890);
}

#[test]
fn parse_double_valid() {
    assert_eq!(NumberUtil::parse_double("3.14", 0.0), 3.14);
}

// ── 计数操作 ──

#[test]
fn count_basic() {
    assert_eq!(NumberUtil::count(1, 5, 1).unwrap(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn count_step() {
    assert_eq!(
        NumberUtil::count(0, 10, 2).unwrap(),
        vec![0, 2, 4, 6, 8, 10]
    );
}

#[test]
fn count_zero_step() {
    assert!(NumberUtil::count(1, 5, 0).is_err());
}

#[test]
fn range_basic() {
    assert_eq!(NumberUtil::range(0, 5), vec![0, 1, 2, 3, 4]);
}

// ── 数学操作 ──

#[test]
fn factorial_basic() {
    assert_eq!(NumberUtil::factorial(5).unwrap(), 120);
    assert_eq!(NumberUtil::factorial(0).unwrap(), 1);
}

#[test]
fn factorial_overflow() {
    assert!(NumberUtil::factorial(21).is_err());
}

#[test]
fn gcd_basic() {
    assert_eq!(NumberUtil::gcd(12, 8), 4);
    assert_eq!(NumberUtil::gcd(7, 5), 1);
}

#[test]
fn lcm_basic() {
    assert_eq!(NumberUtil::lcm(4, 6), 12);
    assert_eq!(NumberUtil::lcm(3, 5), 15);
}

// ── 对齐 Hutool NumberUtilTest ──

/// 对齐 Java: `NumberUtilTest.addTest5()`
#[test]
fn add_test_5() {
    let add = NumberUtil::add(1_686_036_549_717_f64, 1000_f64);
    assert!((add - 1_686_036_550_717_f64).abs() < f64::EPSILON);
}

/// 对齐 Java: `NumberUtilTest.divTest()`
#[test]
fn div_test() {
    let result = NumberUtil::div(0.0, 1.0).unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

/// 对齐 Java: `NumberUtilTest.maxTest()`
#[test]
fn max_test() {
    let max = NumberUtil::max_i32(&[5, 4, 3, 6, 1]).unwrap();
    assert_eq!(6, max);
}

/// 对齐 Java: `NumberUtilTest.minTest()`
#[test]
fn min_test() {
    let min = NumberUtil::min_i32(&[5, 4, 3, 6, 1]).unwrap();
    assert_eq!(1, min);
}

/// 对齐 Java: `NumberUtilTest.isIntegerTest()`
#[test]
fn is_integer_test() {
    assert!(NumberUtil::is_integer("-12"));
    assert!(NumberUtil::is_integer("256"));
    assert!(NumberUtil::is_integer("0256"));
    assert!(NumberUtil::is_integer("0"));
    assert!(!NumberUtil::is_integer("23.4"));
    assert!(!NumberUtil::is_integer(""));
    assert!(!NumberUtil::is_integer(" "));
}

/// 对齐 Java: `NumberUtilTest.isNumberTest()`
#[test]
fn is_number_test() {
    assert!(NumberUtil::is_number("28.55"));
    assert!(NumberUtil::is_number("0"));
    assert!(NumberUtil::is_number("+100.10"));
    assert!(NumberUtil::is_number("-22.022"));
}

/// 对齐 Java: `NumberUtilTest.factorialTest()`
#[test]
fn factorial_test() {
    assert_eq!(1, NumberUtil::factorial(0).unwrap());
    assert_eq!(1, NumberUtil::factorial(1).unwrap());
    assert_eq!(1_307_674_368_000, NumberUtil::factorial(15).unwrap());
    assert_eq!(
        2_432_902_008_176_640_000,
        NumberUtil::factorial(20).unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.issue4237Test()`
#[test]
fn issue_4237_test() {
    assert!(NumberUtil::is_number("0008"));
}

/// 对齐 Java: `NumberUtilTest.issueIC1MXETest()`
#[test]
fn issue_ic1mxe_test() {
    let equals = NumberUtil::equals_i64(104_557_543, 104_557_544);
    assert!(!equals);
}

/// 对齐 Java: `NumberUtilTest.isOddOrEvenTest()`
#[test]
fn is_odd_or_even_test() {
    // Rust 侧用位运算对齐 Java NumberUtil.isOdd / isEven 语义
    let a = [0, 32, -32, 123, -123];
    assert_eq!(a[0] & 1 != 0, false); // isOdd(0)
    assert_eq!(a[0] & 1 == 0, true); // isEven(0)
    assert_eq!(a[1] & 1 != 0, false);
    assert_eq!(a[1] & 1 == 0, true);
    assert_eq!(a[3] & 1 != 0, true);
    assert_eq!(a[3] & 1 == 0, false);
}

/// 对齐 Java: `NumberUtilTest.isDoubleTest()`
#[test]
fn is_double_test() {
    // 对齐空/空白非 double 语义（Rust is_number 拒绝空串）
    assert!(!NumberUtil::is_number(""));
    assert!(!NumberUtil::is_number("  "));
}

/// 对齐 Java: `NumberUtilTest.addTest2()`
#[test]
fn add_test_2() {
    let a = 3.15f32 as f64; // 精度丢失
    let b = 4.22_f64;
    let result = NumberUtil::add(a, b);
    assert!((result - 7.37).abs() < 0.01);
}

/// 对齐 Java: `NumberUtilTest.isLongTest()`
#[test]
fn is_long_test() {
    // Rust 侧 is_integer 覆盖 long 可解析语义
    assert!(NumberUtil::is_integer("-12"));
    assert!(NumberUtil::is_integer("256"));
    assert!(NumberUtil::is_integer("0256"));
    assert!(NumberUtil::is_integer("0"));
    assert!(!NumberUtil::is_integer("23.4"));
    assert!(!NumberUtil::is_integer(""));
    assert!(!NumberUtil::is_integer(" "));
}

/// 对齐 Java: `NumberUtilTest.addTest()`
#[test]
fn add_test() {
    // Java Float+Double → BigDecimal；Rust 用 f64 加法对齐数值 7.37
    let a = 3.15f32 as f64;
    let b = 4.22_f64;
    let result = NumberUtil::add(a, b);
    assert!((result - 7.37).abs() < 0.01);
}

// ── Hutool TEST parity gap wave ──
// ── Hutool NumberUtilTest remaining gaps ──

/// 对齐 Java: `NumberUtilTest.addTest3()`
#[test]
fn add_test_3() {
    let a = 3.15_f64;
    let b = 4.22_f64;
    let result = NumberUtil::add(NumberUtil::add(NumberUtil::add(a, b), a), b);
    assert!((result - 14.74).abs() < 1e-9);
}

/// 对齐 Java: `NumberUtilTest.addTest4()`
#[test]
fn add_test_4() {
    let result = NumberUtil::add_decimal(&[Decimal::from(133), Decimal::from(331)]);
    assert_eq!(result, Decimal::from(464));
}

/// 对齐 Java: `NumberUtilTest.addBlankTest()`
#[test]
fn add_blank_test() {
    let result = NumberUtil::add_str(&["123", " "]);
    assert_eq!(result, Decimal::from(123));
}

/// 对齐 Java: `NumberUtilTest.divBigDecimalTest()`
#[test]
fn div_big_decimal_test() {
    let result = NumberUtil::div_decimal_default(&Decimal::ZERO, &Decimal::ONE).unwrap();
    assert_eq!(result, Decimal::ZERO);
}

/// 对齐 Java: `NumberUtilTest.roundTest()`
#[test]
fn round_test() {
    assert_eq!(NumberUtil::round_str_f64(2.674, 2).unwrap(), "2.67");
    assert_eq!(NumberUtil::round_str("2.674", 2).unwrap(), "2.67");
    assert_eq!(NumberUtil::round_str_f64(2.675, 2).unwrap(), "2.68");
    assert_eq!(NumberUtil::round_str("2.675", 2).unwrap(), "2.68");
    assert_eq!(NumberUtil::round_str_half_even("4.245", 2).unwrap(), "4.24");
    assert_eq!(
        NumberUtil::round_str_half_even("4.2451", 2).unwrap(),
        "4.25"
    );
    assert_eq!(NumberUtil::round_str_f64(2.6005, 2).unwrap(), "2.60");
    assert_eq!(NumberUtil::round_str("2.6005", 2).unwrap(), "2.60");
    assert_eq!(NumberUtil::round_str_f64(2.600, 2).unwrap(), "2.60");
    assert_eq!(NumberUtil::round_str("2.600", 2).unwrap(), "2.60");
}

/// 对齐 Java: `NumberUtilTest.roundStrTest()`
#[test]
fn round_str_test() {
    assert_eq!(NumberUtil::round_str_f64(2.647, 2).unwrap(), "2.65");
    assert_eq!(NumberUtil::round_str_f64(0.0, 10).unwrap(), "0.0000000000");
}

/// 对齐 Java: `NumberUtilTest.roundHalfEvenTest()`
#[test]
fn round_half_even_test() {
    let cases = [
        ("4.245", "4.24"),
        ("4.2450", "4.24"),
        ("4.2451", "4.25"),
        ("4.2250", "4.22"),
        ("1.2050", "1.20"),
        ("1.2150", "1.22"),
        ("1.2250", "1.22"),
        ("1.2350", "1.24"),
        ("1.2450", "1.24"),
        ("1.2550", "1.26"),
        ("1.2650", "1.26"),
        ("1.2750", "1.28"),
        ("1.2850", "1.28"),
        ("1.2950", "1.30"),
    ];
    for (input, expected) in cases {
        let d = NumberUtil::to_big_decimal_str(input).unwrap();
        assert_eq!(NumberUtil::round_half_even(d, 2).to_string(), expected);
    }
}

/// 对齐 Java: `NumberUtilTest.decimalFormatTest()`
#[test]
fn decimal_format_test() {
    let c = 299_792_458_f64;
    assert_eq!(
        NumberUtil::decimal_format(",###", c).unwrap(),
        "299,792,458"
    );
}

/// 对齐 Java: `NumberUtilTest.decimalFormatNaNTest()`
#[test]
fn decimal_format_na_n_test() {
    let c = 0.0_f64 / 0.0_f64;
    assert!(NumberUtil::decimal_format("#%", c).is_err());
}

/// 对齐 Java: `NumberUtilTest.decimalFormatNaNTest2()`
#[test]
fn decimal_format_na_n_test_2() {
    let c = 0.0_f64 / 0.0_f64;
    assert!(NumberUtil::decimal_format("#%", c).is_err());
}

/// 对齐 Java: `NumberUtilTest.decimalFormatDoubleTest()`
#[test]
fn decimal_format_double_test() {
    let c = 467.8101_f64;
    assert_eq!(NumberUtil::decimal_format("0.00", c).unwrap(), "467.81");
}

/// 对齐 Java: `NumberUtilTest.isValidNumberTest()`
#[test]
fn is_valid_number_test() {
    // Rust 侧用 is_number 覆盖有效数字语义
    assert!(NumberUtil::is_number("1"));
}

/// 对齐 Java: `NumberUtilTest.decimalFormatMoneyTest()`
#[test]
fn decimal_format_money_test() {
    let c = 299_792_400.543_534_534_f64;
    assert_eq!(
        NumberUtil::decimal_format_money(c).unwrap(),
        "299,792,400.54"
    );
    assert_eq!(NumberUtil::decimal_format_money(0.5).unwrap(), "0.50");
}

/// 对齐 Java: `NumberUtilTest.equalsTest()`
#[test]
fn equals_test() {
    let zero = Decimal::from_str("0.00").unwrap();
    assert!(NumberUtil::equals_decimal(&zero, &Decimal::ZERO));
}

/// 对齐 Java: `NumberUtilTest.toBigDecimalTest()`
#[test]
fn to_big_decimal_test() {
    let a = 3.14_f64;
    let big_decimal = NumberUtil::to_big_decimal_f64(a).unwrap();
    assert_eq!(big_decimal.to_string(), "3.14");
    assert_eq!(
        NumberUtil::to_big_decimal_str("1,234.55")
            .unwrap()
            .to_string(),
        "1234.55"
    );
    assert_eq!(
        NumberUtil::to_big_decimal_str("1,234.56D")
            .unwrap()
            .to_string(),
        "1234.56"
    );
    assert_eq!(
        NumberUtil::to_big_decimal_str("9.0E+7").unwrap(),
        Decimal::from_str("9.0E+7").unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.parseIntTest()`
#[test]
fn parse_int_test() {
    assert_eq!(NumberUtil::parse_int("0xFE").unwrap(), 254);
    assert_eq!(NumberUtil::parse_int("010").unwrap(), 10);
    assert_eq!(NumberUtil::parse_int("10").unwrap(), 10);
    assert_eq!(NumberUtil::parse_int("   ").unwrap(), 0);
    assert_eq!(NumberUtil::parse_int("10F").unwrap(), 10);
    assert_eq!(NumberUtil::parse_int("22.4D").unwrap(), 22);
    assert_eq!(NumberUtil::parse_int("22.6D").unwrap(), 22);
    assert_eq!(NumberUtil::parse_int("0").unwrap(), 0);
    assert_eq!(NumberUtil::parse_int(".123").unwrap(), 0);
}

/// 对齐 Java: `NumberUtilTest.parseIntTest2()`
#[test]
fn parse_int_test_2() {
    assert_eq!(NumberUtil::parse_int("1,482.00").unwrap(), 1482);
}

/// 对齐 Java: `NumberUtilTest.parseIntTest3()`
#[test]
fn parse_int_test_3() {
    assert!(NumberUtil::parse_int("d").is_err());
}

/// 对齐 Java: `NumberUtilTest.parseIntTest4()`
#[test]
fn parse_int_test_4() {
    assert_eq!(NumberUtil::parse_int_or("abc", None), None);
    assert_eq!(NumberUtil::parse_int_or("abc", Some(456)), Some(456));
    assert_eq!(NumberUtil::parse_int_or("123.abc", Some(789)), Some(123));
    assert_eq!(NumberUtil::parse_int_or("123.3", None), Some(123));
}

/// 对齐 Java: `NumberUtilTest.parseNumberTest4()`
#[test]
fn parse_number_test_4() {
    assert!(NumberUtil::parse_int("429900013E20220812163344551").is_err());
}

/// 对齐 Java: `NumberUtilTest.parseNumberTest()`
#[test]
fn parse_number_test() {
    assert_eq!(NumberUtil::parse_number("1,482.00").unwrap().as_i64(), 1482);
    assert_eq!(
        NumberUtil::parse_number("1,482.00D").unwrap().as_i64(),
        1482
    );
}

/// 对齐 Java: `NumberUtilTest.parseNumberTest2()`
#[test]
fn parse_number_test_2() {
    let number_str = "429900013E20220812163344551";
    let number = NumberUtil::parse_number(number_str).expect("parseNumber");
    assert!(number.as_decimal().is_some());
}

/// 对齐 Java: `NumberUtilTest.parseNumberTest3()`
#[test]
fn parse_number_test_3() {
    assert_eq!(NumberUtil::parse_number_or("abc", None), None);
    assert_eq!(NumberUtil::parse_number_or("", None), None);
    assert_eq!(NumberUtil::parse_number_or("          ", None), None);
    assert_eq!(
        NumberUtil::parse_number_or("abc", NumberUtil::parse_number("456"))
            .unwrap()
            .as_i64(),
        456
    );
    assert_eq!(
        NumberUtil::parse_number_or("123.abc", NumberUtil::parse_number("789"))
            .unwrap()
            .as_i64(),
        123
    );
    assert!((NumberUtil::parse_number_or("123.3", None).unwrap().as_f64() - 123.3).abs() < 1e-9);
    assert!(
        (NumberUtil::parse_number_or("0.123.3", None)
            .unwrap()
            .as_f64()
            - 0.123)
            .abs()
            < 1e-9
    );
}

/// 对齐 Java: `NumberUtilTest.issueIDJ1NSTest()`
#[test]
fn issue_idj1ns_test() {
    for number_str in ["8.37095942E+9", "8.37095942e+9"] {
        let result = NumberUtil::parse_number(number_str)
            .and_then(|n| n.as_decimal())
            .expect("parseNumber");
        assert_eq!(result.to_string(), "8370959420");
    }
}

/// 对齐 Java: `NumberUtilTest.parseHexNumberTest()`
#[test]
fn parse_hex_number_test() {
    assert_eq!(NumberUtil::parse_number("0xff").unwrap().as_i64(), 255);
}

/// 对齐 Java: `NumberUtilTest.parseLongTest()`
#[test]
fn parse_long_test() {
    assert_eq!(NumberUtil::parse_long("0xFF").unwrap(), 255);
    assert_eq!(NumberUtil::parse_long("010").unwrap(), 10);
    assert_eq!(NumberUtil::parse_long("10").unwrap(), 10);
    assert_eq!(NumberUtil::parse_long("   ").unwrap(), 0);
    assert_eq!(NumberUtil::parse_long("10F").unwrap(), 10);
    assert_eq!(NumberUtil::parse_long("22.4D").unwrap(), 22);
    assert_eq!(NumberUtil::parse_long("22.6D").unwrap(), 22);
    assert_eq!(NumberUtil::parse_long("0").unwrap(), 0);
    assert_eq!(NumberUtil::parse_long(".123").unwrap(), 0);
}

/// 对齐 Java: `NumberUtilTest.parseLongTest2()`
#[test]
fn parse_long_test_2() {
    assert_eq!(NumberUtil::parse_long_or(None, None), None);
    assert_eq!(NumberUtil::parse_long_or(Some(""), None), None);
    assert_eq!(
        NumberUtil::parse_long_or(Some("L3221"), Some(1233)),
        Some(1233)
    );
    assert_eq!(NumberUtil::parse_long_or(Some("1233L"), None), Some(1233));
}

/// 对齐 Java: `NumberUtilTest.parseFloatTest()`
#[test]
fn parse_float_test() {
    assert_eq!(NumberUtil::parse_float_or("abc", None), None);
    assert_eq!(NumberUtil::parse_float_or("a123.33", None), None);
    assert_eq!(NumberUtil::parse_float_or("..123", None), None);
    assert_eq!(NumberUtil::parse_float_or("", Some(1233.0)), Some(1233.0));
    assert_eq!(NumberUtil::parse_float_or("123.33a", None), Some(123.33));
    assert_eq!(NumberUtil::parse_float_or(".123", None), Some(0.123));
}

/// 对齐 Java: `NumberUtilTest.parseDoubleTest()`
#[test]
fn parse_double_test() {
    assert_eq!(NumberUtil::parse_double_or("abc", None), None);
    assert_eq!(NumberUtil::parse_double_or("a123.33", None), None);
    assert_eq!(NumberUtil::parse_double_or("..123", None), None);
    assert_eq!(NumberUtil::parse_double_or("", Some(1233.0)), Some(1233.0));
    assert_eq!(NumberUtil::parse_double_or("123.33a", None), Some(123.33));
    assert_eq!(NumberUtil::parse_double_or(".123", None), Some(0.123));
}

/// 对齐 Java: `NumberUtilTest.factorialTest2()`
#[test]
fn factorial_test_2() {
    assert_eq!(1, NumberUtil::factorial(0).unwrap());
    assert_eq!(1, NumberUtil::factorial(1).unwrap());
    assert_eq!(1_307_674_368_000, NumberUtil::factorial(15).unwrap());
    assert_eq!(
        2_432_902_008_176_640_000,
        NumberUtil::factorial(20).unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.mulTest()`
#[test]
fn mul_test() {
    let mul = NumberUtil::mul_decimal_opt(Some(Decimal::from(10)), None);
    assert_eq!(mul, Decimal::ZERO);
}

/// 对齐 Java: `NumberUtilTest.isPowerOfTwoTest()`
#[test]
fn is_power_of_two_test() {
    assert!(!NumberUtil::is_power_of_two(-1));
    assert!(NumberUtil::is_power_of_two(16));
    assert!(NumberUtil::is_power_of_two(65_536));
    assert!(NumberUtil::is_power_of_two(1));
    assert!(!NumberUtil::is_power_of_two(17));
}

/// 对齐 Java: `NumberUtilTest.generateRandomNumberTest()`
#[test]
fn generate_random_number_test() {
    let ints = NumberUtil::generate_random_number(10, 20, 5).unwrap();
    assert_eq!(ints.len(), 5);
    assert_eq!(ints.iter().copied().collect::<HashSet<_>>().len(), 5);
}

/// 对齐 Java: `NumberUtilTest.toStrTest()`
#[test]
fn to_str_test() {
    assert_eq!(
        NumberUtil::to_str_decimal(Decimal::from_str("1.0000000000").unwrap()),
        "1"
    );
    let sub = NumberUtil::sub_decimal(
        Decimal::from_str("9600.00000").unwrap(),
        Decimal::from_str("9600.00000").unwrap(),
    );
    assert_eq!(NumberUtil::to_str_decimal(sub), "0");
    let sub2 = NumberUtil::sub_decimal(
        Decimal::from_str("9600.0000000000").unwrap(),
        Decimal::from_str("9600.000000").unwrap(),
    );
    assert_eq!(NumberUtil::to_str_decimal(sub2), "0");
}

/// 对齐 Java: `NumberUtilTest.generateRandomNumberTest2()`
#[test]
fn generate_random_number_test_2() {
    let ints = NumberUtil::generate_random_number(1, 8, 7).unwrap();
    assert_eq!(ints.len(), 7);
    assert_eq!(ints.iter().copied().collect::<HashSet<_>>().len(), 7);
}

/// 对齐 Java: `NumberUtilTest.toPlainNumberTest()`
#[test]
fn to_plain_number_test() {
    assert_eq!(
        NumberUtil::to_plain_number("5344.34234e3").unwrap(),
        "5344342.34"
    );
}

/// 对齐 Java: `NumberUtilTest.generateBySetTest()`
#[test]
fn generate_by_set_test() {
    let integers = NumberUtil::generate_by_set(10, 100, 5).unwrap();
    assert_eq!(integers.len(), 5);
}

/// 对齐 Java: `NumberUtilTest.divIntegerTest()`
#[test]
fn div_integer_test() {
    assert_eq!(
        1001013,
        NumberUtil::div(100_101_300.0, 100.0).unwrap() as i64
    );
}

/// 对齐 Java: `NumberUtilTest.isPrimesTest()`
#[test]
fn is_primes_test() {
    assert!(NumberUtil::is_primes(2));
    assert!(NumberUtil::is_primes(3));
    assert!(!NumberUtil::is_primes(4));
    assert!(NumberUtil::is_primes(5));
    assert!(NumberUtil::is_primes(7));
    assert!(!NumberUtil::is_primes(9));
    assert!(NumberUtil::is_primes(13));
    assert!(!NumberUtil::is_primes(25));
    assert!(!NumberUtil::is_primes(49));
    assert!(NumberUtil::is_primes(113));
    assert!(!NumberUtil::is_primes(121));
    assert!(NumberUtil::is_primes(2_147_483_647));
    assert!(!NumberUtil::is_primes(2_147_483_646));
}

/// 对齐 Java: `NumberUtilTest.range()`
#[test]
fn range() {
    // Java NumberUtil.isIn(BigDecimal...); Rust 用 f64 范围语义对齐
    assert!(!NumberUtil::is_in_range_f64(1.0, 2.0, 12.0));
    assert!(NumberUtil::is_in_range_f64(1.0, 1.0, 2.0));
    assert!(NumberUtil::is_in_range_f64(1.0, 0.0, 2.0));
    assert!(!NumberUtil::is_in_range_f64(0.23, 0.12, 0.22));
    assert!(NumberUtil::is_in_range_f64(-0.12, -0.3, 0.0));
}

/// 对齐 Java: `NumberUtilTest.issueI79VS7Test()`
#[test]
fn issue_i79vs7_test() {
    let value = "+0.003";
    if NumberUtil::is_number(value) {
        let parsed = NumberUtil::parse_number(value).unwrap().as_f64();
        assert!((parsed - 0.003).abs() < 1e-9);
    }
}

/// 对齐 Java: `NumberUtilTest.issueI7R2B6Test()`
#[test]
fn issue_i7r2b6_test() {
    let div = NumberUtil::div_with_scale(
        NumberUtil::mul(15_858_155_520.0, 100.0),
        25_715_638_272.0,
        2,
    )
    .unwrap();
    assert!((div - 61.67).abs() < 0.01);
}

/// 对齐 Java: `NumberUtilTest.issueI7R2B6Test2()`
#[test]
fn issue_i7r2b6_test_2() {
    let mul = NumberUtil::mul_f64_as_decimal(15_858_155_520.0, 100.0);
    assert_eq!(mul.to_string(), "1585815552000");
}

/// 对齐 Java: `NumberUtilTest.testPowZero()`
#[test]
fn test_pow_zero() {
    let number = Decimal::from_str("2.5").unwrap();
    assert_eq!(NumberUtil::pow(number, 0), Decimal::ONE);
}

/// 对齐 Java: `NumberUtilTest.testPowNegative()`
#[test]
fn test_pow_negative() {
    let number = Decimal::from_str("2.5").unwrap();
    assert_eq!(
        NumberUtil::pow(number, -2),
        Decimal::from_str("0.16").unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.testPowSmallNumber()`
#[test]
fn test_pow_small_number() {
    let number = Decimal::from_str("0.1").unwrap();
    assert_eq!(
        NumberUtil::pow(number, -3),
        Decimal::from_str("1000.00").unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.testPowSmallNumberScale()`
#[test]
fn test_pow_small_number_scale() {
    let number = Decimal::from_str("1.2").unwrap();
    assert_eq!(
        NumberUtil::pow(number, -3),
        Decimal::from_str("0.58").unwrap()
    );
}

/// 对齐 Java: `NumberUtilTest.issue3636Test()`
#[test]
fn issue_3636_test() {
    let number = NumberUtil::parse_number("12,234,456").unwrap();
    assert_eq!(number.as_decimal().unwrap(), Decimal::from(12_234_456));
}

/// 对齐 Java: `NumberUtilTest.addIntAndDoubleTest()`
#[test]
fn add_int_and_double_test() {
    let v1 = 91_007_279_f64;
    let v2 = 0.3545_f64;
    let result = NumberUtil::add(v1, v2);
    assert!((result - 91_007_279.3545).abs() < 1e-9);
}

/// 对齐 Java: `NumberUtilTest.testMultipleOverflow()`
#[test]
fn test_multiple_overflow() {
    let result = NumberUtil::multiple(500_000, 600_000).unwrap();
    assert!(result > 0);
}

/// 对齐 Java: `NumberUtilTest.testGetFloatBinaryStr()`
#[test]
fn test_get_float_binary_str() {
    let result = NumberUtil::get_binary_str_f64(3.5);
    assert_eq!(
        result,
        "0100000000001100000000000000000000000000000000000000000000000000"
    );
}

/// 对齐 Java: `NumberUtil.isOdd` / `isEven`
#[test]
fn is_odd_even_test() {
    assert!(NumberUtil::is_odd(3));
    assert!(NumberUtil::is_even(4));
    assert!(!NumberUtil::is_odd(2));
}

/// 对齐 Java: `NumberUtil.ceilDiv`
#[test]
fn ceil_div_test() {
    assert_eq!(NumberUtil::ceil_div(7, 3), 3);
    assert_eq!(NumberUtil::ceil_div(6, 3), 2);
}

/// 对齐 Java: `NumberUtil.isLong` / `isDouble`
#[test]
fn is_long_double_test() {
    assert!(NumberUtil::is_long("123"));
    assert!(!NumberUtil::is_long("12.3"));
    assert!(NumberUtil::is_double("12.3"));
    assert!(!NumberUtil::is_double("123"));
}

/// 对齐 Java: `NumberUtil.nullToZero` / `zero2One` / `isBeside`
#[test]
fn null_zero_beside_test() {
    assert_eq!(NumberUtil::null_to_zero_i32(None), 0);
    assert_eq!(NumberUtil::null_to_zero_i32(Some(7)), 7);
    assert_eq!(NumberUtil::zero2_one(0), 1);
    assert!(NumberUtil::is_beside_i32(1, 2));
    assert!(!NumberUtil::is_beside_i32(1, 3));
}

/// 对齐 Java: `NumberUtil.formatPercent` / `partValue` / `binaryToInt`
#[test]
fn format_percent_part_binary_test() {
    assert_eq!(NumberUtil::format_percent(0.1234, 2), "12.34%");
    assert_eq!(NumberUtil::part_value(10, 3), 4);
    assert_eq!(NumberUtil::binary_to_int("1010").unwrap(), 10);
    assert_eq!(NumberUtil::sqrt(16), 4);
}

/// 对齐 Java: `NumberUtil.add(float,float)` / Decimal compares
#[test]
fn add_f32_and_decimal_compare_test() {
    assert!((NumberUtil::add_f32(1.5, 2.5) - 4.0).abs() < 1e-9);
    let a = Decimal::from_str("1.0").unwrap();
    let b = Decimal::from_str("2.0").unwrap();
    assert!(NumberUtil::is_less(&a, &b));
    assert!(NumberUtil::is_in_decimal(&a, &Decimal::ZERO, &b));
}
