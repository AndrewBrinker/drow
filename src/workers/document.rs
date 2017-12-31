use unidecode::unidecode;
use chrono::{Utc, DateTime};
use std::path::PathBuf;
use std::fs::{create_dir, File};
use config::Config;
use fail::Fail;

#[derive(Debug)]
pub enum DocumentType {
    Post,
    Page,
}

/// Represents a single page or post.
#[derive(Getters)]
pub struct Document {
    /// The file_name of the document.
    #[get = "pub"]
    file_name: PathBuf,

    /// The directory in which the document is placed.
    #[get = "pub"]
    dir: PathBuf,
}

impl Document {
    pub fn new<S: AsRef<str>>(document_type: DocumentType, config: Config, title: S) -> Self {
        Document::_new(document_type, config, title.as_ref())
    }

    fn _new(document_type: DocumentType, config: Config, title: &str) -> Self {
        let (name, dir) = match document_type {
            DocumentType::Post => {
                let utc: DateTime<Utc> = Utc::now();
                let timestamp = utc.format("%Y-%m-%d").to_string();
                let name_fragment = cleanup(title);

                let name = format!("{}-{}", timestamp, name_fragment);
                let dir = config.posts_dir().to_owned().to_owned();
                (name, dir)
            }
            DocumentType::Page => {
                let name = cleanup(title);
                let dir = config.pages_dir().to_owned().to_owned();
                (name, dir)
            }
        };

        let mut file_name = PathBuf::new();
        file_name.push(&dir);
        file_name.push(&name);
        file_name.set_extension("md");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);
        content.push_str("\n");

        Document {
            file_name,
            dir,
        }
    }

    pub fn create(&self) -> Result<(), Fail> {
        let directory = self.dir.clone();
        let new_post = self.file_name.clone();

	    if !directory.exists() {
	        let res = create_dir(&directory);

            if res.is_err() {
                return Err(Fail::CantCreateDirectory(directory));
            }
    	}

        if !directory.is_dir() {
            return Err(Fail::PathIsntADirectory(directory));
        }

    	if new_post.exists() {
            return Err(Fail::DocumentAlreadyExists(directory));
	    }

        let res = File::create(&new_post);
        if res.is_err() {
            return Err(Fail::CantCreateDocument(directory));
        }

        Ok(())
    }
}

/// Convert a post or page name into a string which will be used to create
/// the file.
fn cleanup<S: ToString>(s: S) -> String {
    _cleanup(s.to_string())
}

fn _cleanup(s: String) -> String {
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

