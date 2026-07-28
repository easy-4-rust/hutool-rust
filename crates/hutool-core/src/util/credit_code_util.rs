//! 对齐: `cn.hutool.core.util.CreditCodeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/CreditCodeUtil.java
//!
//! GB 32100-2015 unified social credit-code helpers.

use rand::{Rng, RngExt};

/// 对齐 Java: `cn.hutool.core.util.CreditCodeUtil`
/// 信用代码工具类
#[derive(Debug, Clone, Copy, Default)]
pub struct CreditCodeUtil;

const WEIGHTS: [usize; 17] = [
    1, 3, 9, 27, 19, 26, 16, 17, 20, 29, 25, 13, 8, 24, 10, 30, 28,
];
const BASE_CODE: &[u8; 31] = b"0123456789ABCDEFGHJKLMNPQRTUWXY";

impl CreditCodeUtil {
    /// Checks the structural GB 32100 shape without validating the check digit.
    #[must_use]
    pub fn is_credit_code_simple(credit_code: &str) -> bool {
        let bytes = credit_code.as_bytes();
        if credit_code.trim().is_empty() || bytes.len() != 18 {
            return false;
        }
        bytes[..2].iter().all(|byte| base_index(*byte).is_some())
            && bytes[2..8].iter().all(u8::is_ascii_digit)
            && bytes[8..].iter().all(|byte| base_index(*byte).is_some())
    }

    /// Validates both the structural shape and the GB 32100 check digit.
    #[must_use]
    pub fn is_credit_code(credit_code: &str) -> bool {
        if !Self::is_credit_code_simple(credit_code) {
            return false;
        }
        parity_index(&credit_code.as_bytes()[..17])
            .is_some_and(|index| credit_code.as_bytes()[17] == BASE_CODE[index])
    }

    /// Generates a structurally valid code with a correct GB 32100 check digit.
    #[must_use]
    pub fn random_credit_code() -> String {
        random_credit_code_with(&mut rand::rng())
    }
}

fn random_credit_code_with(rng: &mut impl Rng) -> String {
    let mut bytes = Vec::with_capacity(18);
    for _ in 0..2 {
        bytes.push(BASE_CODE[rng.random_range(0..BASE_CODE.len())]);
    }
    for _ in 2..8 {
        bytes.push(BASE_CODE[rng.random_range(0..10)]);
    }
    for _ in 8..17 {
        bytes.push(BASE_CODE[rng.random_range(0..BASE_CODE.len())]);
    }
    let parity = parity_index(&bytes).expect("generated characters belong to the base alphabet");
    bytes.push(BASE_CODE[parity]);
    String::from_utf8(bytes).expect("the credit-code alphabet is ASCII")
}

fn parity_index(bytes: &[u8]) -> Option<usize> {
    if bytes.len() != WEIGHTS.len() {
        return None;
    }
    let sum = bytes
        .iter()
        .zip(WEIGHTS)
        .try_fold(0_usize, |sum, (byte, weight)| {
            base_index(*byte).map(|index| sum + index * weight)
        })?;
    Some((31 - sum % 31) % 31)
}

fn base_index(byte: u8) -> Option<usize> {
    BASE_CODE.iter().position(|candidate| *candidate == byte)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn simple_validation_enforces_every_structural_section() {
        assert!(CreditCodeUtil::is_credit_code_simple("91310115591693856A"));
        assert!(!CreditCodeUtil::is_credit_code_simple(""));
        assert!(!CreditCodeUtil::is_credit_code_simple("                  "));
        assert!(!CreditCodeUtil::is_credit_code_simple("91310115591693856"));
        assert!(!CreditCodeUtil::is_credit_code_simple("i1310115591693856A"));
        assert!(!CreditCodeUtil::is_credit_code_simple("91A10115591693856A"));
        assert!(!CreditCodeUtil::is_credit_code_simple("9131011559169385ZA"));
    }

    #[test]
    fn weighted_validation_matches_hutool_vectors_and_rejects_bad_check_digits() {
        assert!(CreditCodeUtil::is_credit_code("91310110666007217T"));
        assert!(!CreditCodeUtil::is_credit_code("91350211M00013FA1N"));
        assert!(!CreditCodeUtil::is_credit_code("91310110666007217U"));
        assert!(!CreditCodeUtil::is_credit_code("not-a-credit-code"));
        assert_eq!(parity_index(b"short"), None);
        assert_eq!(parity_index(b"9131011066600721Z"), None);
    }

    #[test]
    fn generated_codes_are_reproducible_with_an_injected_rng_and_always_valid() {
        let mut first = StdRng::seed_from_u64(7);
        let mut second = StdRng::seed_from_u64(7);
        let code = random_credit_code_with(&mut first);
        assert_eq!(code, random_credit_code_with(&mut second));
        assert!(CreditCodeUtil::is_credit_code(&code));
        for _ in 0..32 {
            assert!(CreditCodeUtil::is_credit_code(
                &CreditCodeUtil::random_credit_code()
            ));
        }
    }
}
