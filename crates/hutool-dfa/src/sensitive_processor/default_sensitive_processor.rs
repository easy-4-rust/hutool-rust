//! Thread-safe sensitive-word facade.

use crate::{DfaError, FoundWord, MatchOptions, WordTree};
use parking_lot::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle};

use super::sensitive_processor::SensitiveProcessor;

/// Default asterisk-sensitive-word processor.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultSensitiveProcessor;

impl SensitiveProcessor for DefaultSensitiveProcessor {}
