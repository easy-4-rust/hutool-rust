use std::ops::Range;

use thiserror::Error;

use super::page_error::PageError;

/// Hutool-compatible pagination arithmetic with explicit, owned numbering state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PageUtil {
    first_page_no: i32,
}

impl PageUtil {
    /// Creates a paginator with an explicit zero- or one-based first page number.
    #[must_use]
    pub const fn new(first_page_no: i32) -> Self {
        Self { first_page_no }
    }

    /// Returns the configured first page number, initially zero for `Default`.
    #[must_use]
    pub const fn first_page_no(self) -> i32 {
        self.first_page_no
    }

    /// Changes this paginator's first page number.
    pub fn set_first_page_no(&mut self, first_page_no: i32) {
        self.first_page_no = first_page_no;
    }

    /// Selects one-based page numbering for this paginator.
    pub fn set_one_as_first_page_no(&mut self) {
        self.set_first_page_no(1);
    }

    /// Calculates the inclusive start offset for a page.
    #[must_use]
    pub fn start(self, page_no: i32, page_size: i32) -> i32 {
        let first = self.first_page_no();
        let page_no = page_no.max(first);
        let page_size = page_size.max(0);
        page_no.wrapping_sub(first).wrapping_mul(page_size)
    }

    /// Calculates the exclusive end offset for a page.
    #[must_use]
    pub fn end(self, page_no: i32, page_size: i32) -> i32 {
        self.start(page_no, page_size)
            .wrapping_add(page_size.max(0))
    }

    /// Returns Hutool's `[start, end]` pair with an exclusive end.
    #[must_use]
    pub fn start_end(self, page_no: i32, page_size: i32) -> [i32; 2] {
        let start = self.start(page_no, page_size);
        [start, start.wrapping_add(page_size.max(0))]
    }

    /// Uses Rust's built-in half-open range as the segment representation.
    #[must_use]
    pub fn segment(self, page_no: i32, page_size: i32) -> Range<i32> {
        let [start, end] = self.start_end(page_no, page_size);
        start..end
    }

    /// Calculates total pages for an `i32` item count.
    pub fn total_page_i32(total_count: i32, page_size: i32) -> Result<i32, PageError> {
        Self::total_page_i64(i64::from(total_count), page_size)
    }

    /// Calculates total pages using Java-compatible signed division and checked narrowing.
    pub fn total_page_i64(total_count: i64, page_size: i32) -> Result<i32, PageError> {
        if page_size == 0 {
            return Ok(0);
        }
        let page_size = i64::from(page_size);
        let quotient = total_count.wrapping_div(page_size);
        let pages = if total_count.wrapping_rem(page_size) == 0 {
            quotient
        } else {
            quotient.wrapping_add(1)
        };
        i32::try_from(pages).map_err(|_| PageError::PageCountOverflow)
    }

    /// Produces a centered rainbow page window.
    pub fn rainbow(
        page_no: i32,
        total_page: i32,
        display_count: i32,
    ) -> Result<Vec<i32>, PageError> {
        if total_page < 0 || display_count < 0 {
            return Err(PageError::NegativeRainbowSize);
        }
        let is_even = display_count & 1 == 0;
        let left = display_count >> 1;
        let mut right = display_count >> 1;
        if is_even {
            right += 1;
        }
        let length = total_page.min(display_count);
        let start = if total_page < display_count || page_no <= left {
            1
        } else if page_no > total_page - right {
            total_page - display_count + 1
        } else {
            page_no - left + i32::from(is_even)
        };
        Ok((0..length).map(|index| start.wrapping_add(index)).collect())
    }

    /// Produces a rainbow page window with Hutool's default display count of ten.
    pub fn rainbow_default(page_no: i32, total_page: i32) -> Result<Vec<i32>, PageError> {
        Self::rainbow(page_no, total_page, 10)
    }
}
