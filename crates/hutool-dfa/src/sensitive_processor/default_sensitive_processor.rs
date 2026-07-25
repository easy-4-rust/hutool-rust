//! 对齐: `cn.hutool.dfa.DefaultSensitiveProcessor` (Rust 独有)
//! 中文说明: 默认敏感词处理器，将匹配到的敏感词替换为等长的 * 号

use crate::{DfaError, FoundWord, MatchOptions, WordTree};
use parking_lot::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle};

use super::sensitive_processor::SensitiveProcessor;

/// Default asterisk-sensitive-word processor.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultSensitiveProcessor;

impl SensitiveProcessor for DefaultSensitiveProcessor {}
