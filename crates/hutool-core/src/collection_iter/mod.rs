//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

mod resettable_iter;
mod iterable_iter;
mod array_iter;
mod copied_iter;
mod filter_iter;
mod trans_iter;
mod iter_chain;
mod enumeration_iter;
mod iterator_enumeration;

pub use resettable_iter::ResettableIter;
pub use iterable_iter::IterableIter;
pub use array_iter::ArrayIter;
pub use copied_iter::CopiedIter;
pub use filter_iter::FilterIter;
pub use trans_iter::TransIter;
pub use iter_chain::IterChain;
pub use enumeration_iter::EnumerationIter;
pub use iterator_enumeration::IteratorEnumeration;
