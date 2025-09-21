//! Slug utility functions for generating URL-safe identifiers from names.
//!
//! Provides consistent slug generation logic used across different entities
//! like boards, packages, and shapes.

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    /// Regex pattern for valid slug format: starts with a letter, followed by letters, numbers, or hyphens, doesn't end with hyphen
    pub static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9-]*[a-z0-9]$|^[a-z]$").unwrap();
}

/// Generate a slug from a given name.
/// 
/// Rules:
/// - Convert to lowercase
/// - Replace non-alphanumeric characters with hyphens
/// - Remove leading/trailing hyphens
/// - Ensure it starts with a letter (prepend 'item-' if it starts with a number)
/// - Remove consecutive hyphens
/// 
/// # Examples
/// 
/// ```
/// use crate::common::slug::generate_slug;
/// 
/// assert_eq!(generate_slug("My Awesome Board"), "my-awesome-board");
/// assert_eq!(generate_slug("Board 123!"), "board-123");
/// assert_eq!(generate_slug("123 Numbers"), "item-123-numbers");
/// assert_eq!(generate_slug("  Spaced  Out  "), "spaced-out");
/// ```
pub fn generate_slug(name: &str) -> String {
    let slug = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        // Remove consecutive hyphens
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    
    // Ensure it starts with a letter
    if slug.chars().next().map_or(true, |c| !c.is_alphabetic()) {
        format!("item-{}", slug)
    } else {
        slug
    }
}

/// Validate if a string matches the slug format requirements.
/// 
/// # Examples
/// 
/// ```
/// use crate::common::slug::is_valid_slug;
/// 
/// assert!(is_valid_slug("my-board"));
/// assert!(is_valid_slug("board123"));
/// assert!(!is_valid_slug("123-board")); // starts with number
/// assert!(!is_valid_slug("my_board"));  // underscore not allowed
/// assert!(!is_valid_slug(""));          // empty not allowed
/// ```
pub fn is_valid_slug(slug: &str) -> bool {
    !slug.is_empty() && SLUG_REGEX.is_match(slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_slug_basic() {
        assert_eq!(generate_slug("My Board"), "my-board");
        assert_eq!(generate_slug("Simple"), "simple");
    }

    #[test]
    fn generate_slug_with_numbers() {
        assert_eq!(generate_slug("Board 123"), "board-123");
        assert_eq!(generate_slug("Version 2.0"), "version-2-0");
    }

    #[test]
    fn generate_slug_starts_with_number() {
        assert_eq!(generate_slug("123 Board"), "item-123-board");
        assert_eq!(generate_slug("42 Answer"), "item-42-answer");
    }

    #[test]
    fn generate_slug_special_characters() {
        assert_eq!(generate_slug("My Board!"), "my-board");
        assert_eq!(generate_slug("Test & Demo"), "test-demo");
        assert_eq!(generate_slug("A/B Testing"), "a-b-testing");
    }

    #[test]
    fn generate_slug_whitespace() {
        assert_eq!(generate_slug("  Spaced  Out  "), "spaced-out");
        assert_eq!(generate_slug("\tTab\tSeparated\t"), "tab-separated");
    }

    #[test]
    fn generate_slug_empty_or_invalid() {
        assert_eq!(generate_slug(""), "item-");
        assert_eq!(generate_slug("!!!"), "item-");
        assert_eq!(generate_slug("   "), "item-");
    }

    #[test]
    fn is_valid_slug_positive() {
        assert!(is_valid_slug("my-board"));
        assert!(is_valid_slug("board123"));
        assert!(is_valid_slug("a"));
        assert!(is_valid_slug("simple-name"));
    }

    #[test]
    fn is_valid_slug_negative() {
        assert!(!is_valid_slug("123-board")); // starts with number
        assert!(!is_valid_slug("my_board"));   // underscore not allowed
        assert!(!is_valid_slug(""));           // empty not allowed
        assert!(!is_valid_slug("My-Board"));   // uppercase not allowed
        assert!(!is_valid_slug("-my-board"));  // starts with hyphen
        assert!(!is_valid_slug("my-board-"));  // ends with hyphen
    }
}