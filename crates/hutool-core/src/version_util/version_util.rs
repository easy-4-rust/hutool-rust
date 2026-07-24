use std::cmp::Ordering;

use thiserror::Error;

use super::version_error::VersionError;

/// Hutool-compatible matching for loose Java module-style version strings.
pub struct VersionUtil;

impl VersionUtil {
    /// Returns whether the current version equals any supplied version expression.
    pub fn any_match<I, S>(current_version: &str, compare_versions: I) -> bool
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let expression = compare_versions
            .into_iter()
            .map(|version| version.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(DEFAULT_DELIMITER);
        Self::match_el(current_version, &expression).unwrap_or(false)
    }

    /// Returns whether the current version is greater than the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_greater_than(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) == Ordering::Greater
    }

    /// Returns whether the current version is greater than or equal to the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_greater_than_or_equal(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) != Ordering::Less
    }

    /// Returns whether the current version is less than the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_less_than(current_version: Option<&str>, compare_version: Option<&str>) -> bool {
        compare_nullable(current_version, compare_version) == Ordering::Less
    }

    /// Returns whether the current version is less than or equal to the comparison version.
    ///
    /// `None` aligns with Java `null` and sorts as the smallest version.
    #[must_use]
    pub fn is_less_than_or_equal(
        current_version: Option<&str>,
        compare_version: Option<&str>,
    ) -> bool {
        compare_nullable(current_version, compare_version) != Ordering::Greater
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_greater_than_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_greater_than(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_greater_than_or_equal_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_greater_than_or_equal(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_less_than_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_less_than(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Convenience wrapper for non-null `&str` arguments.
    ///
    /// Treats the literal strings `"null"` / `"NULL"` like Java `null` on the compare side.
    #[must_use]
    #[inline]
    pub fn is_less_than_or_equal_str(current_version: &str, compare_version: &str) -> bool {
        Self::is_less_than_or_equal(
            Some(current_version),
            comparison_target(compare_version),
        )
    }

    /// Matches one or more `;`-separated exact, comparison, or inclusive range expressions.
    pub fn match_el(current_version: &str, version_el: &str) -> Result<bool, VersionError> {
        Self::match_el_with_delimiter(current_version, version_el, DEFAULT_DELIMITER)
    }

    /// Matches expressions using an explicit non-operator delimiter.
    pub fn match_el_with_delimiter(
        current_version: &str,
        version_el: &str,
        versions_delimiter: &str,
    ) -> Result<bool, VersionError> {
        validate_delimiter(versions_delimiter)?;
        if version_el.trim().is_empty() {
            return Ok(false);
        }
        let current = current_version.trim();
        for expression in version_el
            .split(versions_delimiter)
            .map(str::trim)
            .filter(|expression| !expression.is_empty())
        {
            if let Some((operator, version)) = split_operator(expression) {
                let version = if version.eq_ignore_ascii_case("null") {
                    None
                } else {
                    Some(version)
                };
                let ordering = compare_nullable(Some(current), version);
                let matches = match operator {
                    ">=" | "≥" => ordering != Ordering::Less,
                    "<=" | "≤" => ordering != Ordering::Greater,
                    ">" => ordering == Ordering::Greater,
                    "<" => ordering == Ordering::Less,
                    _ => false,
                };
                if matches {
                    return Ok(true);
                }
            } else if let Some(index) = expression.find('-') {
                let left = expression[..index].trim();
                let right = expression[index + 1..].trim();
                let left_matches =
                    left.is_empty() || compare_versions(left, current) != Ordering::Greater;
                let right_matches =
                    right.is_empty() || compare_versions(right, current) != Ordering::Less;
                if left_matches && right_matches {
                    return Ok(true);
                }
            } else if current == expression {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

use super::{DEFAULT_DELIMITER, LooseVersion, Token, compare_nullable, compare_tokens, compare_versions, comparison_target, java_string_cmp};
use super::{parse_version, split_operator, take_number, take_text, validate_delimiter};
