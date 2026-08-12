//! 对齐: `cn.hutool.core.math.Money`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Money.java
//!
//! 单币种货币：内部以「分」(`cent`) 存储；默认币种 CNY（2 位小数）。
//! `Currency` 在 Rust 侧用币种代码 + 小数位数表达。

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::prelude::ToPrimitive;
use std::cmp::Ordering;
use std::fmt;

/// 缺省币种代码 —— 对齐 Java: `Money.DEFAULT_CURRENCY_CODE`
pub const DEFAULT_CURRENCY_CODE: &str = "CNY";

/// 一组可能的元/分换算比例 —— 对齐 Java: `CENT_FACTORS`
const CENT_FACTORS: [i64; 4] = [1, 10, 100, 1000];

/// 货币金额 —— 对齐 Java `Money`。
#[derive(Debug, Clone)]
pub struct Money {
    cent: i64,
    /// 币种代码（如 CNY / JPY）。
    currency_code: String,
    /// 币种默认小数位数。
    fraction_digits: u32,
}

impl Default for Money {
    /// 对齐 Java: `Money()` —— 0 元，默认 CNY。
    fn default() -> Self {
        Self::new()
    }
}

impl Money {
    /// 对齐 Java: `Money()`
    #[must_use]
    pub fn new() -> Self {
        Self::with_currency_code(0, DEFAULT_CURRENCY_CODE, 2)
    }

    /// 对齐 Java: `Money(long yuan, int cent)`
    #[must_use]
    pub fn from_yuan_cent(yuan: i64, cent: i32) -> Self {
        Self::from_yuan_cent_currency(yuan, cent, DEFAULT_CURRENCY_CODE, 2)
    }

    /// 对齐 Java: `Money(long yuan, int cent, Currency currency)`
    #[must_use]
    pub fn from_yuan_cent_currency(
        yuan: i64,
        cent: i32,
        currency_code: impl Into<String>,
        fraction_digits: u32,
    ) -> Self {
        let currency_code = currency_code.into();
        let factor = cent_factor(fraction_digits);
        let cent_value = if yuan == 0 {
            cent as i64
        } else {
            yuan * factor + (cent as i64 % factor)
        };
        Self {
            cent: cent_value,
            currency_code,
            fraction_digits,
        }
    }

    /// 对齐 Java: `Money(String amount)`（默认 CNY，HALF_EVEN）
    #[must_use]
    pub fn from_yuan_str(amount: &str) -> Self {
        Self::from_decimal_str(
            amount,
            DEFAULT_CURRENCY_CODE,
            2,
            RoundingStrategy::MidpointNearestEven,
        )
    }

    /// 对齐 Java: `Money(String amount, Currency currency)`
    #[must_use]
    pub fn from_yuan_str_currency(
        amount: &str,
        currency_code: impl Into<String>,
        fraction_digits: u32,
    ) -> Self {
        Self::from_decimal_str(
            amount,
            currency_code,
            fraction_digits,
            RoundingStrategy::MidpointNearestEven,
        )
    }

    /// 对齐 Java: `Money(String amount, Currency currency, RoundingMode)`
    #[must_use]
    pub fn from_yuan_str_currency_rounding(
        amount: &str,
        currency_code: impl Into<String>,
        fraction_digits: u32,
        rounding: RoundingStrategy,
    ) -> Self {
        Self::from_decimal_str(amount, currency_code, fraction_digits, rounding)
    }

    /// 对齐 Java: `Money(double amount)`
    #[must_use]
    pub fn from_yuan_f64(yuan: f64) -> Self {
        Self::from_yuan_f64_currency(yuan, DEFAULT_CURRENCY_CODE, 2)
    }

    /// 对齐 Java: `Money(double amount, Currency currency)`
    #[must_use]
    pub fn from_yuan_f64_currency(
        yuan: f64,
        currency_code: impl Into<String>,
        fraction_digits: u32,
    ) -> Self {
        let factor = cent_factor(fraction_digits) as f64;
        Self {
            cent: (yuan * factor).round() as i64,
            currency_code: currency_code.into(),
            fraction_digits,
        }
    }

    /// 对齐 Java: `Money(BigDecimal amount)`
    #[must_use]
    pub fn from_decimal(amount: Decimal) -> Self {
        Self::from_decimal_currency(
            amount,
            DEFAULT_CURRENCY_CODE,
            2,
            RoundingStrategy::MidpointNearestEven,
        )
    }

    /// 对齐 Java: `Money(BigDecimal amount, RoundingMode)`
    #[must_use]
    pub fn from_decimal_rounding(amount: Decimal, rounding: RoundingStrategy) -> Self {
        Self::from_decimal_currency(amount, DEFAULT_CURRENCY_CODE, 2, rounding)
    }

    /// 对齐 Java: `Money(BigDecimal amount, Currency currency)`
    #[must_use]
    pub fn from_decimal_currency_default(
        amount: Decimal,
        currency_code: impl Into<String>,
        fraction_digits: u32,
    ) -> Self {
        Self::from_decimal_currency(
            amount,
            currency_code,
            fraction_digits,
            RoundingStrategy::MidpointNearestEven,
        )
    }

    /// 对齐 Java: `Money(BigDecimal amount, Currency currency, RoundingMode)`
    #[must_use]
    pub fn from_decimal_currency(
        amount: Decimal,
        currency_code: impl Into<String>,
        fraction_digits: u32,
        rounding: RoundingStrategy,
    ) -> Self {
        let factor = Decimal::from(cent_factor(fraction_digits));
        let scaled = (amount * factor).round_dp_with_strategy(0, rounding);
        Self {
            cent: scaled.to_i64().unwrap_or(0),
            currency_code: currency_code.into(),
            fraction_digits,
        }
    }

    /// 对齐 Java: `Money(long yuan, Currency)` —— 按币种小数位构造（测试辅助）。
    #[must_use]
    pub fn with_currency_fraction(yuan: i64, fraction_digits: u32) -> Self {
        Self::with_currency_code(
            yuan * cent_factor(fraction_digits),
            "CUSTOM",
            fraction_digits,
        )
    }

    /// 以「分」直接构造。
    #[must_use]
    pub fn with_currency_code(
        cent: i64,
        currency_code: impl Into<String>,
        fraction_digits: u32,
    ) -> Self {
        Self {
            cent,
            currency_code: currency_code.into(),
            fraction_digits,
        }
    }

    /// 对齐 Java: `getAmount()`
    #[must_use]
    pub fn get_amount(&self) -> Decimal {
        let factor = Decimal::from(self.get_cent_factor());
        Decimal::from(self.cent) / factor
    }

    /// 对齐 Java: `setAmount(BigDecimal)`
    pub fn set_amount(&mut self, amount: Decimal) {
        let factor = Decimal::from(self.get_cent_factor());
        let scaled =
            (amount * factor).round_dp_with_strategy(0, RoundingStrategy::MidpointNearestEven);
        self.cent = scaled.to_i64().unwrap_or(0);
    }

    /// 对齐 Java: `getCent()`
    #[must_use]
    pub fn get_cent(&self) -> i64 {
        self.cent
    }

    /// 对齐 Java: `setCent(long)`
    pub fn set_cent(&mut self, cent: i64) {
        self.cent = cent;
    }

    /// 对齐 Java: `getCurrency()` —— 返回币种代码。
    #[must_use]
    pub fn get_currency(&self) -> &str {
        &self.currency_code
    }

    /// 对齐 Java: `getCentFactor()`
    #[must_use]
    pub fn get_cent_factor(&self) -> i64 {
        cent_factor(self.fraction_digits)
    }

    /// 对齐 Java: `equals(Money other)`
    #[must_use]
    pub fn equals_money(&self, other: &Money) -> bool {
        self.currency_code == other.currency_code && self.cent == other.cent
    }

    /// 对齐 Java: `compareTo(Money)`
    #[must_use]
    pub fn compare_to(&self, other: &Money) -> i32 {
        self.assert_same_currency(other);
        match self.cent.cmp(&other.cent) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    /// 对齐 Java: `greaterThan(Money)`
    #[must_use]
    pub fn greater_than(&self, other: &Money) -> bool {
        self.compare_to(other) > 0
    }

    /// 对齐 Java: `add(Money)`
    #[must_use]
    pub fn add(&self, other: &Money) -> Money {
        self.assert_same_currency(other);
        self.with_same_currency(self.cent + other.cent)
    }

    /// 对齐 Java: `addTo(Money)`
    pub fn add_to(&mut self, other: &Money) -> &mut Self {
        self.assert_same_currency(other);
        self.cent += other.cent;
        self
    }

    /// 对齐 Java: `subtract(Money)`
    #[must_use]
    pub fn subtract(&self, other: &Money) -> Money {
        self.assert_same_currency(other);
        self.with_same_currency(self.cent - other.cent)
    }

    /// 对齐 Java: `subtractFrom(Money)`
    pub fn subtract_from(&mut self, other: &Money) -> &mut Self {
        self.assert_same_currency(other);
        self.cent -= other.cent;
        self
    }

    /// 对齐 Java: `multiply(long)`
    #[must_use]
    pub fn multiply_long(&self, val: i64) -> Money {
        self.with_same_currency(self.cent * val)
    }

    /// 对齐 Java: `multiplyBy(long)`
    pub fn multiply_by_long(&mut self, val: i64) -> &mut Self {
        self.cent *= val;
        self
    }

    /// 对齐 Java: `multiply(double)`
    #[must_use]
    pub fn multiply_f64(&self, val: f64) -> Money {
        self.with_same_currency((self.cent as f64 * val).round() as i64)
    }

    /// 对齐 Java: `multiplyBy(double)`
    pub fn multiply_by_f64(&mut self, val: f64) -> &mut Self {
        self.cent = (self.cent as f64 * val).round() as i64;
        self
    }

    /// 对齐 Java: `multiply(BigDecimal)`
    #[must_use]
    pub fn multiply_decimal(&self, val: Decimal) -> Money {
        self.multiply_decimal_rounding(val, RoundingStrategy::MidpointNearestEven)
    }

    /// 对齐 Java: `multiplyBy(BigDecimal)`
    pub fn multiply_by_decimal(&mut self, val: Decimal) -> &mut Self {
        self.multiply_by_decimal_rounding(val, RoundingStrategy::MidpointNearestEven)
    }

    /// 对齐 Java: `multiply(BigDecimal, RoundingMode)`
    #[must_use]
    pub fn multiply_decimal_rounding(&self, val: Decimal, rounding: RoundingStrategy) -> Money {
        let new_cent = Decimal::from(self.cent) * val;
        let rounded = new_cent.round_dp_with_strategy(0, rounding);
        self.with_same_currency(rounded.to_i64().unwrap_or(0))
    }

    /// 对齐 Java: `multiplyBy(BigDecimal, RoundingMode)`
    pub fn multiply_by_decimal_rounding(
        &mut self,
        val: Decimal,
        rounding: RoundingStrategy,
    ) -> &mut Self {
        let new_cent = Decimal::from(self.cent) * val;
        self.cent = new_cent
            .round_dp_with_strategy(0, rounding)
            .to_i64()
            .unwrap_or(0);
        self
    }

    /// 对齐 Java: `divide(double)`
    #[must_use]
    pub fn divide_f64(&self, val: f64) -> Money {
        self.with_same_currency((self.cent as f64 / val).round() as i64)
    }

    /// 对齐 Java: `divideBy(double)`
    pub fn divide_by_f64(&mut self, val: f64) -> &mut Self {
        self.cent = (self.cent as f64 / val).round() as i64;
        self
    }

    /// 对齐 Java: `divide(BigDecimal)`
    #[must_use]
    pub fn divide_decimal(&self, val: Decimal) -> Money {
        self.divide_decimal_rounding(val, RoundingStrategy::MidpointNearestEven)
    }

    /// 对齐 Java: `divide(BigDecimal, RoundingMode)`
    #[must_use]
    pub fn divide_decimal_rounding(&self, val: Decimal, rounding: RoundingStrategy) -> Money {
        let new_cent = Decimal::from(self.cent) / val;
        let rounded = new_cent.round_dp_with_strategy(0, rounding);
        self.with_same_currency(rounded.to_i64().unwrap_or(0))
    }

    /// 对齐 Java: `divideBy(BigDecimal)`
    pub fn divide_by_decimal(&mut self, val: Decimal) -> &mut Self {
        self.divide_by_decimal_rounding(val, RoundingStrategy::MidpointNearestEven)
    }

    /// 对齐 Java: `divideBy(BigDecimal, RoundingMode)`
    pub fn divide_by_decimal_rounding(
        &mut self,
        val: Decimal,
        rounding: RoundingStrategy,
    ) -> &mut Self {
        let new_cent = Decimal::from(self.cent) / val;
        self.cent = new_cent
            .round_dp_with_strategy(0, rounding)
            .to_i64()
            .unwrap_or(0);
        self
    }

    /// 对齐 Java: `allocate(int targets)` —— 均分，余数从前往后 +1。
    #[must_use]
    pub fn allocate(&self, targets: i32) -> Vec<Money> {
        assert!(targets > 0, "targets must be > 0");
        let n = targets as i64;
        let low = self.cent / n;
        let remainder = self.cent % n;
        let mut results = Vec::with_capacity(targets as usize);
        for i in 0..targets {
            let extra = if (i as i64) < remainder { 1 } else { 0 };
            results.push(self.with_same_currency(low + extra));
        }
        results
    }

    /// 对齐 Java: `allocate(long[] ratios)` —— 按比例分配。
    #[must_use]
    pub fn allocate_by_ratios(&self, ratios: &[i64]) -> Vec<Money> {
        assert!(!ratios.is_empty(), "ratios must not be empty");
        let total: i64 = ratios.iter().sum();
        assert!(total > 0, "ratio sum must be > 0");
        let mut results = Vec::with_capacity(ratios.len());
        let mut allocated = 0i64;
        for (i, &ratio) in ratios.iter().enumerate() {
            if i + 1 == ratios.len() {
                results.push(self.with_same_currency(self.cent - allocated));
            } else {
                let part = self.cent * ratio / total;
                allocated += part;
                results.push(self.with_same_currency(part));
            }
        }
        results
    }

    /// 对齐 Java: `dump()`
    #[must_use]
    pub fn dump(&self) -> String {
        format!(
            "cent={}\ncurrency={}\nfractionDigits={}",
            self.cent, self.currency_code, self.fraction_digits
        )
    }

    fn from_decimal_str(
        amount: &str,
        currency_code: impl Into<String>,
        fraction_digits: u32,
        rounding: RoundingStrategy,
    ) -> Self {
        let d: Decimal = amount.parse().expect("invalid money amount");
        Self::from_decimal_currency(d, currency_code, fraction_digits, rounding)
    }

    fn with_same_currency(&self, cent: i64) -> Money {
        Money {
            cent,
            currency_code: self.currency_code.clone(),
            fraction_digits: self.fraction_digits,
        }
    }

    fn assert_same_currency(&self, other: &Money) {
        assert_eq!(
            self.currency_code, other.currency_code,
            "Money math requires same currency"
        );
    }
}

impl PartialEq for Money {
    /// 对齐 Java: `equals(Object)`
    fn eq(&self, other: &Self) -> bool {
        self.equals_money(other)
    }
}

impl Eq for Money {}

impl std::hash::Hash for Money {
    /// 对齐 Java: `hashCode()` —— 仅基于 cent。
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.cent.hash(state);
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.assert_same_currency(other);
        self.cent.cmp(&other.cent)
    }
}

impl fmt::Display for Money {
    /// 对齐 Java: `toString()` —— 固定小数位展示。
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let amount = self.get_amount();
        let s = format!("{:.prec$}", amount, prec = self.fraction_digits as usize);
        f.write_str(&s)
    }
}

fn cent_factor(fraction_digits: u32) -> i64 {
    let idx = fraction_digits as usize;
    if idx < CENT_FACTORS.len() {
        CENT_FACTORS[idx]
    } else {
        10i64.pow(fraction_digits)
    }
}
