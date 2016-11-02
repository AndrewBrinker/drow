use std::path::Path;
use std::fs::{File, metadata, read_dir, create_dir};
use git2::Repository;
use slog_scope;

// There are three cases:
//
// - The directory exists and isn't empty => error
// - The directory exists and is empty => use it
// - The directory doesn't exist => create it and use it
pub fn setup(directory: &str) {
    let log = slog_scope::logger();
    let directory = Path::new(directory);

    // Sweet, sweet logging.
    info!(log, "Setting up drow site");

    // Make sure the directory exists and create it if it doesn't.
    if !directory.exists() {
        warn!(log, format!("'{}' doesn't exist", directory.display()));
        info!(log, format!("attempting to create directory '{}'", directory.display()));

        match create_dir(directory) {
            Ok(..) => info!(log, format!("created directory '{}'", directory.display())),
            Err(..) => {
                error!(log, format!("couldn't create directory '{}'", directory.display()));
                return;
            }
        }
    }

    // Get the contents of the directory as a vector.
    let contents: Vec<_> = match read_dir(directory) {
        Ok(directory_iter) => directory_iter.collect(),
        Err(..) => {
            error!(log, format!("couldn't read directory '{}'", directory.display()));
            return;
        }
    };

    // If that vector isn't empty, you've got problems!
    if contents.len() != 0 {
        error!(log, format!("directory '{}' isn't empty", directory.display()));
        return;
    }

    // Otherwise, clone the repo.
    let repo = "https://github.com/AndrewBrinker/drow-template";

    if let Ok(..) = Repository::clone(repo, directory) {
        info!(log, format!("drow site successfully setup at '{}'", directory.display()));
    } else {
        error!(log, format!("failed to clone repository into '{}'", directory.display()));
    }
}

