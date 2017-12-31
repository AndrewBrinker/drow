use unidecode::unidecode;
use chrono::{Utc, DateTime};
use std::path::PathBuf;
use config::Config;
use fail::Fail;

type Title = String;
type Content = String;

/// Represents a single page or post.
#[derive(Getters)]
pub struct Document {
    /// The filename of the document.
    #[get = "pub"]
    name: String,

    /// The location of the document.
    #[get = "pub"]
    location: PathBuf,

    /// The contents of the document.
    #[get = "pub"]
    content: Content,
}

impl Document {
    /// Creates a new page.
    pub fn page(config: Config, title: &str) -> Self {
        let name = cleanup(title.to_string());

        let mut location = PathBuf::new();
        location.push(config.pages_dir());
        location.push(name.clone());
        location.set_extension("md");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);
        content.push_str("\n");

        Document {
            name,
            location,
            content,
        }
    }

    /// Creates a new post.
    pub fn post(config: Config, title: &str) -> Self {
        let utc: DateTime<Utc> = Utc::now();
        let timestamp = utc.format("%Y-%m-%d").to_string();
        let name_fragment = cleanup(title.to_string());
        let name = format!("{}-{}", timestamp, name_fragment);

        let mut location = PathBuf::new();
        location.push(config.posts_dir());
        location.push(name.clone());
        location.set_extension("md");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);
        content.push_str("\n");

        Document {
            name,
            location,
            content,
        }
    }

    pub fn create(&self) -> Result<(), Fail> {
        unimplemented!();
    }
}

/// Convert a post or page name into a string which will be used to create
/// the file.
fn cleanup(s: String) -> String {
    unidecode(&s)
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
        .to_lowercase()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_cleanup_name() {
        let expected_result = "hello-world".to_string();
        let actual_result = cleanup("Hello, World!");

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn more_complex_cleanup_name() {
        let expected_result = "setting-expectations-for-rusts-difficulty".to_string();
        let actual_result = cleanup("Setting Expectations for Rust's Difficulty");

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn unicode_cleanup_name() {
        let expected_result = "the-aeneid".to_string();
        let actual_result = cleanup("The Æneid");

        assert_eq!(expected_result, actual_result);
    }
}

