//! Thread-safe sensitive-word facade.

use crate::{DfaError, FoundWord, MatchOptions, WordTree};
use parking_lot::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle};

mod sensitive_processor;
mod default_sensitive_processor;

pub use sensitive_processor::SensitiveProcessor;
pub use default_sensitive_processor::DefaultSensitiveProcessor;
