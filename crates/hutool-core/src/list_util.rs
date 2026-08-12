//! List operations aligned with Hutool's `ListUtil` behavior families.

/// 对齐: `cn.hutool.core.collection.ListUtil`
/// 列表工具类
use std::{cmp::Ordering, collections::VecDeque};

use crate::{AvgPartition, CoreError, Partition, Result};

/// Hutool-aligned list operations implemented on Rust slices and vectors.
#[derive(Debug, Clone, Copy, Default)]
pub struct ListUtil;

impl ListUtil {
    /// Collects any iterator into a contiguous list.
    #[must_use]
    pub fn to_list<T>(values: impl IntoIterator<Item = T>) -> Vec<T> {
        values.into_iter().collect()
    }

    /// Collects any iterator into a linked list.
    #[must_use]
    pub fn to_linked_list<T>(values: impl IntoIterator<Item = T>) -> VecDeque<T> {
        values.into_iter().collect()
    }

    /// Returns one zero-based page as a borrowed slice.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `page_size` is zero.
    pub fn page<T>(values: &[T], page_no: usize, page_size: usize) -> Result<&[T]> {
        validate_page_size(page_size)?;
        let Some(start) = page_no.checked_mul(page_size) else {
            return Ok(&values[values.len()..]);
        };
        if start >= values.len() {
            return Ok(&values[values.len()..]);
        }
        let end = start.saturating_add(page_size).min(values.len());
        Ok(&values[start..end])
    }

    /// Calls `consumer` once for every non-empty page.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `page_size` is zero.
    pub fn for_each_page<T>(
        values: &[T],
        page_size: usize,
        mut consumer: impl FnMut(&[T]),
    ) -> Result<()> {
        validate_page_size(page_size)?;
        values.chunks(page_size).for_each(&mut consumer);
        Ok(())
    }

    /// Sorts a list in place with a caller-provided comparator.
    pub fn sort_by<T>(values: &mut [T], compare: impl FnMut(&T, &T) -> Ordering) {
        values.sort_by(compare);
    }

    /// Reverses a list in place.
    pub fn reverse<T>(values: &mut [T]) {
        values.reverse();
    }

    /// Returns a reversed clone without changing the source.
    #[must_use]
    pub fn reverse_new<T: Clone>(values: &[T]) -> Vec<T> {
        values.iter().rev().cloned().collect()
    }

    /// Replaces `index` when present, otherwise appends the value.
    pub fn set_or_append<T>(values: &mut Vec<T>, index: usize, value: T) {
        if let Some(slot) = values.get_mut(index) {
            *slot = value;
        } else {
            values.push(value);
        }
    }

    /// Replaces `index`, or pads to it using `padding` and then appends `value`.
    ///
    /// The default maximum index matches Hutool's OOM guard of
    /// `(current_length + 1) * 10`.
    pub fn set_or_padding<T: Clone>(
        values: &mut Vec<T>,
        index: usize,
        value: T,
        padding: T,
    ) -> Result<()> {
        let limit = values.len().saturating_add(1).saturating_mul(10);
        Self::set_or_padding_with_limit(values, index, value, padding, Some(limit))
    }

    /// Configurable variant of [`ListUtil::set_or_padding`].
    ///
    /// `None` disables the limit. A limit of zero is treated as disabled to
    /// match Hutool's `indexLimit <= 0` behavior.
    pub fn set_or_padding_with_limit<T: Clone>(
        values: &mut Vec<T>,
        index: usize,
        value: T,
        padding: T,
        index_limit: Option<usize>,
    ) -> Result<()> {
        if let Some(slot) = values.get_mut(index) {
            *slot = value;
            return Ok(());
        }
        if index_limit.is_some_and(|limit| limit > 0 && index > limit) {
            return Err(CoreError::InvalidArgument {
                name: "index",
                reason: "exceeds the configured padding limit",
            });
        }
        values.resize(index, padding);
        values.push(value);
        Ok(())
    }

    /// Returns an owned stepped sub-list with Hutool-style negative indices.
    ///
    /// Start and end are normalized relative to the end, reversed bounds are
    /// swapped, and non-positive steps are normalized to one.
    #[allow(clippy::cast_sign_loss)] // Bounds are rejected above before converting to slice indices.
    pub fn sub<T: Clone>(values: &[T], start: isize, end: isize, step: isize) -> Result<Vec<T>> {
        if values.is_empty() {
            return Ok(Vec::new());
        }
        let size = values.len() as i128;
        let mut start = normalize_index(start as i128, size);
        let mut end = normalize_index(end as i128, size);
        if start == size {
            return Ok(Vec::new());
        }
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        if start < 0 || end < 0 {
            return Err(CoreError::InvalidArgument {
                name: "range",
                reason: "negative index exceeds the list length",
            });
        }
        if end > size {
            if start >= size {
                return Ok(Vec::new());
            }
            end = size;
        }
        let step = usize::try_from(step)
            .ok()
            .filter(|step| *step > 0)
            .unwrap_or(1);
        let start = start as usize;
        let end = end as usize;
        Ok((start..end)
            .step_by(step)
            .map(|index| values[index].clone())
            .collect())
    }

    /// Returns the last matching index.
    #[must_use]
    pub fn last_index_of<T>(values: &[T], mut matcher: impl FnMut(&T) -> bool) -> Option<usize> {
        values.iter().rposition(&mut matcher)
    }

    /// Returns every matching index in ascending order.
    #[must_use]
    pub fn index_of_all<T>(values: &[T], mut matcher: impl FnMut(&T) -> bool) -> Vec<usize> {
        values
            .iter()
            .enumerate()
            .filter_map(|(index, value)| matcher(value).then_some(index))
            .collect()
    }

    /// Creates a borrowed fixed-size partition view.
    pub fn partition<T>(values: &[T], size: usize) -> Result<Partition<'_, T>> {
        Partition::new(values, size)
    }

    /// Creates a borrowed average partition view.
    pub fn split_avg<T>(values: &[T], limit: usize) -> Result<AvgPartition<'_, T>> {
        AvgPartition::new(values, limit)
    }

    /// Swaps the first matching element with `target_index`.
    ///
    /// Returns `false` when the element is absent.
    pub fn swap_to<T: PartialEq>(
        values: &mut [T],
        element: &T,
        target_index: usize,
    ) -> Result<bool> {
        if target_index >= values.len() {
            return Err(CoreError::InvalidArgument {
                name: "target_index",
                reason: "must be within the list",
            });
        }
        let Some(index) = values.iter().position(|value| value == element) else {
            return Ok(false);
        };
        values.swap(index, target_index);
        Ok(true)
    }

    /// Swaps the first matching element with the first target element.
    pub fn swap_element<T: PartialEq>(values: &mut [T], element: &T, target: &T) -> bool {
        let Some(target_index) = values.iter().position(|value| value == target) else {
            return false;
        };
        let Some(index) = values.iter().position(|value| value == element) else {
            return false;
        };
        values.swap(index, target_index);
        true
    }

    /// Moves an existing equal element, or inserts a new element, at a position.
    pub fn move_element<T: PartialEq>(
        values: &mut Vec<T>,
        element: T,
        new_position: usize,
    ) -> Result<()> {
        if let Some(index) = values.iter().position(|value| value == &element) {
            values.remove(index);
        }
        if new_position > values.len() {
            return Err(CoreError::InvalidArgument {
                name: "new_position",
                reason: "must be within or immediately after the list",
            });
        }
        values.insert(new_position, element);
        Ok(())
    }

    /// Zips two lists to the shorter length using a caller-provided function.
    #[must_use]
    pub fn zip<A, B, R>(
        left: impl IntoIterator<Item = A>,
        right: impl IntoIterator<Item = B>,
        mut zipper: impl FnMut(A, B) -> R,
    ) -> Vec<R> {
        left.into_iter()
            .zip(right)
            .map(|(left, right)| zipper(left, right))
            .collect()
    }
}

fn validate_page_size(page_size: usize) -> Result<()> {
    if page_size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "page_size",
            reason: "must be greater than zero",
        });
    }
    Ok(())
}

fn normalize_index(index: i128, size: i128) -> i128 {
    if index < 0 { index + size } else { index }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_util_matches_hutool_creation_paging_mutation_and_search() {
        assert_eq!(ListUtil::to_list(1..=3), [1, 2, 3]);
        assert_eq!(ListUtil::to_linked_list(1..=2), VecDeque::from([1, 2]));

        let values = [0, 1, 2, 3, 4];
        assert_eq!(ListUtil::page(&values, 0, 9).unwrap(), values);
        assert_eq!(ListUtil::page(&values, 1, 2).unwrap(), [2, 3]);
        assert!(ListUtil::page(&values, 3, 2).unwrap().is_empty());
        assert!(ListUtil::page(&values, usize::MAX, 2).unwrap().is_empty());
        assert!(ListUtil::page(&values, 0, 0).is_err());
        let mut pages = Vec::new();
        {
            let mut collect_page = |page: &[i32]| pages.push(page.to_vec());
            ListUtil::for_each_page(&values, 2, &mut collect_page).unwrap();
            assert!(ListUtil::for_each_page(&values, 0, &mut collect_page).is_err());
        }
        assert_eq!(pages, [vec![0, 1], vec![2, 3], vec![4]]);

        let mut sortable = vec![3, 1, 2];
        ListUtil::sort_by(&mut sortable, Ord::cmp);
        assert_eq!(sortable, [1, 2, 3]);
        assert_eq!(ListUtil::reverse_new(&sortable), [3, 2, 1]);
        ListUtil::reverse(&mut sortable);
        assert_eq!(sortable, [3, 2, 1]);

        ListUtil::set_or_append(&mut sortable, 1, 9);
        ListUtil::set_or_append(&mut sortable, 99, 4);
        assert_eq!(sortable, [3, 9, 1, 4]);
        ListUtil::set_or_padding(&mut sortable, 6, 7, 0).unwrap();
        assert_eq!(sortable, [3, 9, 1, 4, 0, 0, 7]);
        ListUtil::set_or_padding(&mut sortable, 0, 8, 0).unwrap();
        assert_eq!(sortable[0], 8);
        assert!(ListUtil::set_or_padding_with_limit(&mut sortable, 20, 1, 0, Some(10)).is_err());
        ListUtil::set_or_padding_with_limit(&mut sortable, 8, 6, 0, Some(0)).unwrap();
        assert_eq!(sortable[8], 6);

        assert_eq!(ListUtil::sub(&values, 1, 5, 2).unwrap(), [1, 3]);
        assert_eq!(ListUtil::sub(&values, 1, 9, 2).unwrap(), [1, 3]);
        assert_eq!(ListUtil::sub(&values, -4, -1, 0).unwrap(), [1, 2, 3]);
        assert_eq!(ListUtil::sub(&values, 4, 1, 1).unwrap(), [1, 2, 3]);
        assert!(ListUtil::sub(&values, 5, 9, 1).unwrap().is_empty());
        assert!(ListUtil::sub(&values, 9, 10, 1).unwrap().is_empty());
        assert!(ListUtil::sub(&values, -9, 2, 1).is_err());
        assert!(ListUtil::sub::<i32>(&[], 0, 1, 1).unwrap().is_empty());
        assert_eq!(
            ListUtil::last_index_of(&values, |value| value % 2 == 0),
            Some(4)
        );
        assert_eq!(ListUtil::last_index_of(&values, |value| *value > 9), None);
        assert_eq!(
            ListUtil::index_of_all(&values, |value| value % 2 == 0),
            [0, 2, 4]
        );

        assert_eq!(ListUtil::partition(&values, 2).unwrap().len(), 3);
        assert_eq!(
            ListUtil::split_avg(&values, 2).unwrap().get(0),
            Some(&[0, 1, 2][..])
        );

        let mut swaps = vec![1, 2, 3];
        assert!(ListUtil::swap_to(&mut swaps, &1, 2).unwrap());
        assert!(!ListUtil::swap_to(&mut swaps, &9, 2).unwrap());
        assert!(ListUtil::swap_to(&mut swaps, &1, 4).is_err());
        assert!(ListUtil::swap_element(&mut swaps, &2, &3));
        assert!(!ListUtil::swap_element(&mut swaps, &2, &9));
        assert!(!ListUtil::swap_element(&mut swaps, &9, &2));

        ListUtil::move_element(&mut swaps, 3, 0).unwrap();
        ListUtil::move_element(&mut swaps, 8, 3).unwrap();
        assert_eq!(swaps, [3, 2, 1, 8]);
        assert!(ListUtil::move_element(&mut swaps, 9, 9).is_err());

        assert_eq!(
            ListUtil::zip([1, 2, 3], ["a", "b"], |number, text| format!(
                "{number}{text}"
            )),
            ["1a", "2b"]
        );
        assert!(format!("{ListUtil:?}").contains("ListUtil"));
    }
}
