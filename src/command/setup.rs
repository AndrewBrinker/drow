use std::path::Path;
use std::fs;
use git2::Repository;
use slog_scope;

// There are three cases:
//
// - The directory exists and isn't empty => error
// - The directory exists and is empty => use it
// - The directory doesn't exist => create it and use it
pub fn setup(directory: &str) {
    let repo = "https://github.com/AndrewBrinker/drow-template";
    let log = slog_scope::logger();
    let directory = Path::new(directory);
    let display = directory.display();

    // Sweet, sweet logging.
    info!(log, "Setting up drow site");

    // Make sure the directory exists and create it if it doesn't.
    if !directory.exists() {
        warn!(log, format!("'{}' doesn't exist", display));
        info!(log, format!("creating directory '{}'", display));

        match fs::create_dir(directory) {
            Ok(..) => info!(log, format!("created directory '{}'", display)),
            Err(..) => {
                error!(log, format!("couldn't create directory '{}'", display));
                return;
            }
        }
    }

    // Make sure we're looking at a directory.
    if !directory.is_dir() {
        error!(log, format!("'{}' isn't a directory", display));
        return;
    }

    // Get the contents of the directory as a vector.
    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            error!(log, format!("couldn't read directory '{}'", display));
            return;
        }
    };

    // If that vector isn't empty, you've got problems!
    if !contents.is_empty() {
        error!(log, format!("directory '{}' isn't empty", display));
        return;
    }

    // Otherwise, clone the repo.
    if let Ok(..) = Repository::clone(repo, directory) {
        info!(log, format!("drow site successfully setup at '{}'", display));
    } else {
        error!(log, format!("failed to clone repository into '{}'", display));
    }
}

