use config::Config;
use document::Document;
use std::path::PathBuf;
use std::fs::{create_dir, File};

/// Takes in a page title, creates a file called "<title>.md" in the pages
/// directory.
pub fn page(config: Config, title: &str) {
    let logger = config.logger();

    let file = Document::page(config, title);

    let config_file = config.config_file();
    if !config_file.exists() {
        error!(logger, "we are not in a drow repo");
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    if !file.dest().exists() {
        info!(logger, "'{}' doesn't exist", file.dest().display());

        let res = create_dir(file.dest());
        if res.is_err() {
            error!(logger, "couldn't create directory '{}'", file.dest().display());
            return;
        }

        info!(logger, "created directory '{}'", file.dest().display());
    }

    if !file.dest().is_dir() {
        error!(logger, "'{}' isn't a directory", file.dest().display());
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    if new_page.exists() {
        error!(logger, "'{}' already exists", new_page.display());
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    let res = File::create(&new_page);
    if res.is_err() {
        error!(logger, "could not create '{}'", new_page.display());
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    info!(logger, "created new page '{}'", new_page.display());
}

