use thiserror::Error;

/// Errors returned by identity-card accessors that Java Hutool exposes through exceptions.

/// 对齐: `cn.hutool.core.util.IdcardUtil`
/// 身份证错误
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdcardError {
    /// A required identity-card value was blank.
    #[error("identity card must not be blank")]
    Blank,
    /// A value cannot be interpreted as the requested identity-card representation.
    #[error("invalid identity card")]
    InvalidCard,
    /// The embedded birthday is not a real Gregorian date.
    #[error("invalid identity-card birthday: {0}")]
    InvalidBirthDate(String),
    /// Age cannot be calculated before the embedded birthday.
    #[error("comparison date precedes identity-card birthday")]
    BirthAfterComparison,
}

