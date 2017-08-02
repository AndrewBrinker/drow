use slog::Logger;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use std::path::Path;

/// A struct containing all CLI configuration.
#[derive(Getters)]
pub struct Config<'a> {
    /// The location of the initialization repository.
    #[get = "pub"]
    template_repo: &'a str,

    /// The location to put the built site.
    #[get = "pub"]
    build_dir: &'a Path,

    /// The path to the assets directory.
    #[get = "pub"]
    assets_dir: &'a Path,

    /// The path to the pagesdirectory.
    #[get = "pub"]
    pages_dir: &'a Path,

    /// The path to the posts directory.
    #[get = "pub"]
    posts_dir: &'a Path,

    /// The path to the templates directory.
    #[get = "pub"]
    templates_dir: &'a Path,

    /// The path to the configuration file.
    #[get = "pub"]
    config_file: &'a Path,

    /// The logger for the application.
    #[get = "pub"]
    logger: Logger,
}

impl<'a> Config<'a> {
    /// Create a new Config struct.
    pub fn new() -> Self {
        Config::default()
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        let logger = TerminalLoggerBuilder::new()
            .level(Severity::Info)
            .destination(Destination::Stdout)
            .build()
            .unwrap();

        Config {
            template_repo: "https://github.com/andrewbrinker/drow-template",
            // This path gets special treatment from GitHub Pages, and can be
            // selected as the source for a GitHub Pages site.
            // https://help.github.com/articles/configuring-a-publishing-source-for-github-pages/
            build_dir: Path::new("./docs/"),

            assets_dir: Path::new("./assets/"),
            pages_dir: Path::new("./pages/"),
            posts_dir: Path::new("./posts/"),
            templates_dir: Path::new("./templates/"),
            config_file: Path::new("Drow.toml"),

            logger: logger,
        }
    }
}
