//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

mod char_sequence;
mod object_length;
mod object_contains;
mod char_sequence_element;
mod object_util;

pub use char_sequence::CharSequence;
pub use object_length::ObjectLength;
pub use object_contains::ObjectContains;
pub use char_sequence_element::CharSequenceElement;
pub use object_util::ObjectUtil;
