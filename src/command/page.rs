use config::Config;
use std::path::PathBuf;
use std::fs::{create_dir, File};

/// Takes in a page title, creates a file called "<title>.md" in the pages
/// directory.
pub fn page(config: Config, title: &str) {
    info!("Creating new page");
    let directory = config.pages_dir();
    let disp = directory.display();
    let mut new_page = PathBuf::new();
    new_page.push(directory);
    new_page.push(title);
    new_page.set_extension("md");

    if !directory.exists() {
        warn!("'{}' doesn't exist", disp);
        info!("creating directory '{}'", disp);

        let res = create_dir(directory);
        if res.is_err() {
            error!("couldn't create directory '{}'", disp);
            return;
        }
    }

    info!("ensuring '{}' is a directory", disp);
    if !directory.is_dir() {
        error!("'{}' isn't a directory", disp);
        error!("cannot continue. Exiting...");
        return;
    }

    info!("checking if '{}' already exists", new_page.display());
    if new_page.exists() {
        error!("'{}' already exists", new_page.display());
        error!("cannot continue. Exiting...");
        return;
    }

    info!("creating '{}'", new_page.display());
    let res = File::create(&new_page);
    if res.is_err() {
        error!("could not create '{}'", new_page.display());
        error!("cannot continue. Exiting...");
        return;
    }
}
