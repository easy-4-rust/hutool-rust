use thiserror::Error;

/// Errors returned by custom-radix conversion.

/// 对齐: `cn.hutool.core.util.RadixUtil`
/// 进制错误
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RadixError {
    /// At least two alphabet characters are required.
    #[error("a radix alphabet must contain at least two characters")]
    AlphabetTooShort,
    /// Repeated characters make decoding ambiguous.
    #[error("duplicate character `{0}` in radix alphabet")]
    DuplicateCharacter(char),
    /// Bounded alphabets prevent pathological allocation and arithmetic.
    #[error("radix alphabet exceeds the maximum of 256 characters")]
    AlphabetTooLarge,
    /// Hutool's long overload does not accept negative numbers.
    #[error("negative i64 values are not supported")]
    NegativeI64,
    /// An encoded value must contain at least one character.
    #[error("encoded value must not be empty")]
    EmptyInput,
    /// The value contains a character outside the selected alphabet.
    #[error("character `{0}` does not belong to the radix alphabet")]
    InvalidCharacter(char),
    /// The decoded value cannot be represented as an `i64`.
    #[error("decoded value exceeds i64 range")]
    Overflow,
}
