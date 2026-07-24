//! 对齐: `cn.hutool.core.math.Combination`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Combination.java

use num_bigint::{BigInt, Sign};

use super::arithmetic_overflow::ArithmeticOverflow;

/// 组合 C(n, m) —— 对齐 Java `Combination`。
#[derive(Debug, Clone)]
pub struct Combination {
    datas: Vec<String>,
}

impl Combination {
    /// 对齐 Java: `Combination(String[] datas)`
    pub fn new(datas: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            datas: datas.into_iter().map(Into::into).collect(),
        }
    }

    /// 对齐 Java: `Combination.count(int n, int m)`（可能溢出 long，按 Java longValue 截断）
    pub fn count(n: i32, m: i32) -> i64 {
        long_value(&Self::count_big(n, m))
    }

    /// 对齐 Java: `Combination.countBig(int n, int m)`
    pub fn count_big(n: i32, m: i32) -> BigInt {
        if n < 0 || m < 0 {
            panic!("n and m must be non-negative. got n={n}, m={m}");
        }
        if m > n {
            return BigInt::from(0);
        }
        if m == 0 || n == m {
            return BigInt::from(1);
        }
        let mm = m.min(n - m);
        let mut result = BigInt::from(1);
        for i in 1..=mm {
            let numerator = n - mm + i;
            result = result * BigInt::from(numerator) / BigInt::from(i);
        }
        result
    }

    /// 对齐 Java: `Combination.countSafe(int n, int m)`
    pub fn count_safe(n: i32, m: i32) -> Result<i64, ArithmeticOverflow> {
        if n < 0 || m < 0 {
            return Err(ArithmeticOverflow {
                message: format!("n and m must be non-negative. got n={n}, m={m}"),
            });
        }
        let big = Self::count_big(n, m);
        match i64::try_from(&big) {
            Ok(v) => Ok(v),
            Err(_) => Err(ArithmeticOverflow {
                message: format!("C({n},{m}) overflows i64"),
            }),
        }
    }

    /// 对齐 Java: `Combination.countAll(int n)` → 2^n - 1
    pub fn count_all(n: i32) -> i64 {
        if !(0..=63).contains(&n) {
            panic!("countAll must have n >= 0 and n <= 63, but got n={n}");
        }
        if n == 63 {
            i64::MAX
        } else {
            (1i64 << n) - 1
        }
    }

    /// 对齐 Java: `select(int m)`
    pub fn select(&self, m: i32) -> Vec<Vec<String>> {
        if m < 0 {
            return Vec::new();
        }
        let mut result = Vec::new();
        let mut buffer = vec![String::new(); m as usize];
        self.select_rec(0, &mut buffer, 0, &mut result);
        result
    }

    /// 对齐 Java: `selectAll()`
    pub fn select_all(&self) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        for i in 1..=self.datas.len() as i32 {
            result.extend(self.select(i));
        }
        result
    }

    fn select_rec(
        &self,
        data_index: usize,
        result_list: &mut [String],
        result_index: usize,
        result: &mut Vec<Vec<String>>,
    ) {
        let result_len = result_list.len();
        let result_count = result_index + 1;
        if result_count > result_len {
            result.push(result_list.to_vec());
            return;
        }
        let upper = self.datas.len() + result_count - result_len;
        for i in data_index..upper {
            result_list[result_index] = self.datas[i].clone();
            self.select_rec(i + 1, result_list, result_index + 1, result);
        }
    }
}

fn long_value(v: &BigInt) -> i64 {
    let mut bytes = v.to_signed_bytes_le();
    let fill = if v.sign() == Sign::Minus { 0xff } else { 0 };
    bytes.resize(8, fill);
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[..8]);
    i64::from_le_bytes(arr)
}
