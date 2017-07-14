use git2::Repository;
use std::path::{Path, PathBuf};
use std::fs;

// There are three cases:
//
// - The directory exists and isn't empty => error
// - The directory exists and is empty => use it
// - The directory doesn't exist => create it and use it
pub fn setup(directory: &str) {
    let url = "https://github.com/AndrewBrinker/drow-template";
    let directory = Path::new(directory);
    let display = directory.display();

    // Sweet, sweet logging.
    info!("Setting up drow site");

    // Make sure the directory exists and create it if it doesn't.
    if !directory.exists() {
        warn!("'{}' doesn't exist", display);
        info!("creating directory '{}'", display);

        match fs::create_dir(directory) {
            Ok(..) => info!("created directory '{}'", display),
            Err(..) => {
                error!("couldn't create directory '{}'", display);
                return;
            }
        }
    }

    // Make sure we're looking at a directory.
    if !directory.is_dir() {
        error!("'{}' isn't a directory", display);
        return;
    }

    // Get the contents of the directory as a vector.
    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            error!("couldn't read directory '{}'", display);
            return;
        }
    };

    // If that vector isn't empty, you've got problems!
    if !contents.is_empty() {
        error!("directory '{}' isn't empty", display);
        return;
    }

    info!("downloading template");

    // Clone the template repo.
    match Repository::clone(url, directory) {
        Ok(_) => {}
        Err(_) => {
            error!("couldn't clone template repo '{}'", display);
            return;
        }
    };

    // Delete the .git folder in the cloned directory.
    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    match fs::remove_dir_all(&git_dir) {
        Ok(_) => {}
        Err(_) => {
            error!("unable to delete .git directory from cloned template");
            return;
        }
    }

    // Initialize Git repository.
    match Repository::init(directory) {
        Ok(..) => info!("initialized git repository in '{}'", display),
        Err(..) => {
            error!("couldn't initialize git repository in '{}'", display);
        }
    }
}
