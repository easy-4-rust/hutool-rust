//! Stateful and view-based collection adapters aligned with Hutool.

use std::{io, io::BufRead, marker::PhantomData};

use crate::{ArrayIter, CollUtil};

/// A buffered reader exposed as a fallible line iterator.
pub struct LineIter<'a> {
    lines: io::Lines<Box<dyn BufRead + 'a>>,
    is_valid: Box<dyn FnMut(&str) -> bool + 'a>,
    finished: bool,
}

impl<'a> LineIter<'a> {
    /// Creates an iterator that returns every line.
    #[must_use]
    pub fn new(reader: impl BufRead + 'a) -> Self {
        Self::with_filter(reader, |_| true)
    }

    /// Creates an iterator that skips lines rejected by `is_valid`.
    #[must_use]
    pub fn with_filter(reader: impl BufRead + 'a, is_valid: impl FnMut(&str) -> bool + 'a) -> Self {
        let reader: Box<dyn BufRead + 'a> = Box::new(reader);
        Self {
            lines: reader.lines(),
            is_valid: Box::new(is_valid),
            finished: false,
        }
    }

    /// Stops reading. Dropping the iterator closes owned reader resources.
    pub fn close(&mut self) {
        self.finished = true;
    }
}

impl Iterator for LineIter<'_> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        loop {
            match self.lines.next() {
                Some(Ok(line)) if (self.is_valid)(&line) => return Some(Ok(line)),
                Some(Ok(_)) => {}
                Some(Err(error)) => {
                    self.finished = true;
                    return Some(Err(error));
                }
                None => {
                    self.finished = true;
                    return None;
                }
            }
        }
    }
}
