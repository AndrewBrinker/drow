use config::Config;
use workers::document::{Document, DocumentType};

/// Takes in a page title, creates a file called "<title>.md" in the pages
/// directory.
pub fn page(config: Config, title: &str) {
    let document = Document::new(DocumentType::Page, config, title);

    if let Err(e) = document.create() {
        println!("error: {}", e.to_string());
    } else {
        println!("created '{}'", document.file_name().display());
    }
}

