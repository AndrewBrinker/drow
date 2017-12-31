use config::Config;
use workers::document::{Document, DocumentType};

/// Takes in a post title, creates a file called "<timestamp>-<title>.md" in the posts
/// directory.
///
/// Note that this is intended to duplicate the title-to-filename conversion used
/// by Jekyll.
pub fn post(config: Config, title: &str) {
    let document = Document::new(DocumentType::Post, config, title);

    if let Err(e) = document.create() {
        println!("error: {}", e.to_string());
    } else {
        println!("created '{}'", document.file_name().display());
    }
}

