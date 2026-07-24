//! 对齐: `cn.hutool.core.text.split.SplitIter`
//! 来源: hutool-core SplitIter.java

use crate::text::finder::char_finder::CharFinder;
use crate::text::finder::length_finder::LengthFinder;
use crate::text::finder::pattern_finder::PatternFinder;
use crate::text::finder::str_finder::StrFinder;
use crate::{CoreError, Result};

mod text_finder_kind;
mod split_iter;

pub use text_finder_kind::TextFinderKind;
pub use split_iter::SplitIter;
