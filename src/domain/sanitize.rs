use unicode_segmentation::UnicodeSegmentation;

use validator::ValidateEmail;

static FORBIDDEN_CHARACTERS: &[char] = &['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

/// Implement this trait for any data which is send to the API and ensure it is safe to use.
pub trait Sanitize {
    fn sanitize(&mut self);
}

/// Returns `true` if the input contains any sql escape characters.
pub fn contains_sql_escape_chars(input: &str) -> bool {
    input.chars().any(|c| FORBIDDEN_CHARACTERS.contains(&c))
}

/// Returns `true` if the input is longer than max_length.
pub fn is_too_long(input: &str, max_length: usize) -> bool {
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    input.graphemes(true).count() > max_length
}

/// Returns `true` if the input is empty or contains only whitespace characters.
pub fn is_empty_or_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}

/// Checks whether the input is empty, has a max length of 256
/// and does not contain sql escape chars
pub fn is_valid_text_input(input: &str) -> bool {
    !is_empty_or_whitespace(input) && !is_too_long(input, 256) && !contains_sql_escape_chars(input)
}

/// Returns an instance of `SubscriberName` if the input satisfies all
/// our validation constraints on subscriber names.
pub fn sanitize_username(username: String) -> String {
    if !is_valid_text_input(&username) {
        panic!("{} is not a valid subscriber name.", username)
    }

    username.trim().into()
}

/// Returns an instance of `EmailAddress` if the input satisfies all our validation
/// constraints on email addresses.
pub fn sanitize_email(email: String) -> String {
    if !is_valid_text_input(&email) || !email.validate_email() {
        panic!("{} is not a valid email.", email)
    }

    // normalize
    email.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_contains_sql_escape_chars() {
        assert!(FORBIDDEN_CHARACTERS
            .iter()
            .all(|c| contains_sql_escape_chars(&c.to_string())))
    }

    #[rstest]
    #[case::with_graphemes("🦀".repeat(256), 256)]
    #[case::with_ascii("a".repeat(256), 256)]
    fn test_is_not_too_long(#[case] input: String, #[case] max_length: usize) {
        assert!(!is_too_long(&input, max_length));
    }

    #[rstest]
    #[case::with_graphemes("🦀".repeat(257), 256)]
    #[case::with_ascii("a".repeat(257), 256)]
    fn test_has_is_too_long(#[case] input: String, #[case] max_length: usize) {
        assert!(is_too_long(&input, max_length));
    }

    #[rstest]
    #[case::empty("")]
    #[case::whitespace(" ")]
    #[case::tab("\t")]
    #[case::newline("\n")]
    fn test_is_empty_or_whitespace(#[case] input: String) {
        assert!(is_empty_or_whitespace(&input));
    }

    #[rstest]
    #[case::ascii("a")]
    #[case::grapheme("🦀")]
    fn test_is_not_empty_or_whitespace(#[case] input: String) {
        assert!(!is_empty_or_whitespace(&input));
    }
}
