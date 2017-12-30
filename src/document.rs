use std::path::PathBuf;
use config::Config;
use name::Name;

type Title = String;
type Content = String;

/// Represents a single page or post.
#[derive(Getters)]
pub struct Document {
    /// The title of the page or post, as given by the user.
    #[get = "pub"]
    title: Title,

    /// The filename-converted title of the page or post.
    #[get = "pub"]
    name: Name,

    /// The source location of the file.
    #[get = "pub"]
    src: PathBuf,

    /// The destination location of the file.
    #[get = "pub"]
    dest: PathBuf,

    /// The contents of the file.
    #[get = "pub"]
    content: Content,

    /// Indicates whether the file has been processed.
    #[get = "pub"]
    is_processed: bool,
}

impl Document {
    /// Creates a new page.
    pub fn page(config: Config, title: &str) -> Self {
        let title = title.to_string();

        let name = Name::from_title(&title);

        let mut src = PathBuf::new();
        src.push(config.pages_dir());
        src.push(name.to_string());
        src.set_extension("md");

        let mut dest = PathBuf::new();
        dest.push(config.build_dir());
        dest.push(name.to_string());
        dest.push("index");
        dest.set_extension("html");

        let mut content = String::new();
        content.push_str("# ");
        content.push_str(&title);
        content.push_str("\n");

        let is_processed = false;

        Document {
            title,
            name,
            src,
            dest,
            content,
            is_processed,
        }
    }

    /// Creates a new post.
    pub fn post(config: Config, title: String) -> Self {
        unimplemented!();
    }
}

