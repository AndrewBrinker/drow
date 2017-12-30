use config::Config;
use document::Document;
use std::fs::{create_dir, File};

/// Takes in a page title, creates a file called "<title>.md" in the pages
/// directory.
pub fn page(config: Config, title: &str) {
    let document = Document::page(config, title);

    let config_file = config.config_file();
    if !config_file.exists() {
        println!("we are not in a drow repo");
        println!("cannot continue. Exiting...");
        return;
    }

    if !file.dest().exists() {
        println!("'{}' doesn't exist", file.dest().display());

        let res = create_dir(file.dest());
        if res.is_err() {
            println!("couldn't create directory '{}'", file.dest().display());
            return;
        }

        println!("created directory '{}'", file.dest().display());
    }

    if !file.dest().is_dir() {
        println!("'{}' isn't a directory", file.dest().display());
        println!("cannot continue. Exiting...");
        return;
    }

    if new_page.exists() {
        println!("'{}' already exists", new_page.display());
        println!("cannot continue. Exiting...");
        return;
    }

    let res = File::create(&new_page);
    if res.is_err() {
        println!("could not create '{}'", new_page.display());
        println!("cannot continue. Exiting...");
        return;
    }

    println!("created new page '{}'", new_page.display());
}

