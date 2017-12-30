use std::path::PathBuf;
use config::Config;
use name::Name;

type Title = String;
type Content = String;

/// Represents a single page or post.
pub struct File {
    /// The title of the page or post, as given by the user.
    title: Title,

    /// The filename-converted title of the page or post.
    name: Name,

    /// The source location of the file.
    src: PathBuf,

    /// The destination location of the file.
    dest: PathBuf,

    /// The contents of the file.
    content: Content,

    /// Indicates whether the file has been processed.
    is_processed: bool,
}

impl File {
    /// Creates a new page.
    fn page(config: Config, title: String) -> Self {
        let name = Name::from_title(&title);
        let mut src = PathBuf::new();
        src.push(config.pages_dir());
        src.push(name.to_string());
        src.push(".md");

        let mut dest = PathBuf::new();
        dest.push(config.build_dir());
        dest.push(name.to_string());
        dest.push("/index.html");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);

        let is_processed = false;

        File {
            title,
            name,
            src,
            dest,
            content,
            is_processed,
        }
    }

    /// Creates a new post.
    fn post(config: Config, title: String) -> Self {
        unimplemented!();
    }
}

