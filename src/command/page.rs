use config::Config;
use std::path::PathBuf;
use std::fs::{create_dir, File};

/// Takes in a page title, creates a file called "<title>.md" in the pages
/// directory.
pub fn page(config: Config, title: &str) {
    let logger = config.logger();

    let directory = config.pages_dir();
    let disp = directory.display();
    let mut new_page = PathBuf::new();
    new_page.push(directory);
    new_page.push(title);
    new_page.set_extension("md");

    let config_file = config.config_file();
    if !config_file.exists() {
        error!(logger, "we are not in a drow repo");
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    if !directory.exists() {
        info!(logger, "'{}' doesn't exist", disp);

        let res = create_dir(directory);
        if res.is_err() {
            error!(logger, "couldn't create directory '{}'", disp);
            return;
        }

        info!(logger, "created directory '{}'", disp);
    }

    if !directory.is_dir() {
        error!(logger, "'{}' isn't a directory", disp);
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
