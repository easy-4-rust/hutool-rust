//! Thread-safe sensitive-word facade.

mod default_sensitive_processor;
mod sensitive_processor;

pub use default_sensitive_processor::DefaultSensitiveProcessor;
pub use sensitive_processor::SensitiveProcessor;
