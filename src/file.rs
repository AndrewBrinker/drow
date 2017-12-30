use unidecode::unidecode;

/// Convert a post or page name into a string which will be used to create
/// the file.
pub fn cleanup_name(s: String) -> String {
    let s = unicode_to_ascii_approximation(s);
    let s = remove_punctuation(s);
    let s = replace_whitespace_with_dashes(s);
    let s = convert_to_lowercase(s);
    s
}

/// This is simply a more explanatory alias for the unidecode function from the
/// unidecode library. It replaces non-ASCII characters with their ASCII
/// approximation.
fn unicode_to_ascii_approximation(s: String) -> String {
    unidecode(&s)
}

/// Removes all characters that aren't alphanumeric or whitespace.
fn remove_punctuation(s: String) -> String {
    s.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect()
}

/// Replaces all whitespaces with dashes.
fn replace_whitespace_with_dashes(s: String) -> String {
    s.split_whitespace().collect::<Vec<_>>().join("-")
}

/// Converts all uppercase letters in the string to lowercase.
fn convert_to_lowercase(s: String) -> String {
    s.to_lowercase()
}

#[cfg(test)]
mod test {
    use super::cleanup_name;

    #[test]
    fn basic_cleanup_name() {
        let s = "Hello World!".to_string();
        let expected_result = "hello-world".to_string();
        let actual_result = cleanup_name(s);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn more_complex_cleanup_name() {
        let s = "Setting Expectations for Rust's Difficulty".to_string();
        let expected_result = "setting-expectations-for-rusts-difficulty".to_string();
        let actual_result = cleanup_name(s);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn unicode_cleanup_name() {
        let s = "The Æneid".to_string();
        let expected_result = "the-aeneid".to_string();
        let actual_result = cleanup_name(s);
        assert_eq!(expected_result, actual_result);
    }
}

