//! Thread-safe sensitive-word facade.

mod sensitive_processor;
mod default_sensitive_processor;

pub use sensitive_processor::SensitiveProcessor;
pub use default_sensitive_processor::DefaultSensitiveProcessor;
