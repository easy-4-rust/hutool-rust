/// Supported default masking strategies.

/// Supported default masking strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesensitizedType {
    /// Replace a user identifier with zero.
    UserId,
    /// Preserve only the first character of a name.
    ChineseName,
    /// Preserve one leading and two trailing identity-card characters.
    IdCard,
    /// Preserve four leading and two trailing landline characters.
    FixedPhone,
    /// Preserve three leading and four trailing mobile characters.
    MobilePhone,
    /// Preserve the address prefix and mask eight trailing characters.
    Address,
    /// Preserve the first mailbox character and domain.
    Email,
    /// Mask every password character.
    Password,
    /// Mask the middle of a Chinese vehicle plate.
    CarLicense,
    /// Preserve the first four and final group of a bank card.
    BankCard,
    /// Preserve the first IPv4 component.
    Ipv4,
    /// Preserve the first IPv6 component.
    Ipv6,
    /// Preserve two leading and two trailing passport characters.
    Passport,
    /// Preserve four leading and four trailing credit-code characters.
    CreditCode,
    /// Preserve only the first character.
    FirstMask,
    /// Return no value.
    ClearToNull,
    /// Return an empty value.
    ClearToEmpty,
}
