use std::path::{Path, PathBuf};
use std::fs;
use git2::Repository;
use config::Config;

// There are three cases:
//
// - The directory exists and isn't empty => error
// - The directory exists and is empty => use it
// - The directory doesn't exist => create it and use it
pub fn setup(config: Config, directory: &str) {
    info!("Setting up drow site");
    let url = config.get_template_repo();
    let directory = Path::new(directory);
    let disp = directory.display();

    info!("Checking if '{}' exists", disp);
    if !directory.exists() {
        warn!("'{}' doesn't exist", disp);
        info!("creating directory '{}'", disp);

        match fs::create_dir(directory) {
            Ok(..) => {}
            Err(..) => {
                error!("couldn't create directory '{}'", disp);
                return;
            }
        }
    }

    info!("ensuring '{}' is a directory", disp);
    if !directory.is_dir() {
        error!("'{}' isn't a directory", disp);
        error!("cannot continue. Exiting...");
        return;
    }

    info!("trying to read '{}' if possible", disp);
    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            error!("couldn't read directory '{}'", disp);
            error!("cannot continue. Exiting...");
            return;
        }
    };

    info!("checking '{}' is empty", disp);
    if !contents.is_empty() {
        error!("directory '{}' isn't empty", disp);
        error!("cannot continue. Exiting...");
        return;
    }

    info!("downloading template");
    match Repository::clone(url, directory) {
        Ok(_) => {}
        Err(_) => {
            error!("couldn't clone template repo '{}'", disp);
            error!("cannot continue. Exiting...");
            return;
        }
    };

    info!("deleting .git directory from cloned template");
    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    match fs::remove_dir_all(&git_dir) {
        Ok(_) => {}
        Err(_) => {
            error!("unable to delete .git directory from cloned template");
            error!("cannot continue. Exiting...");
            return;
        }
    }

    info!("initializing fresh git repository in '{}'", disp);
    match Repository::init(directory) {
        Ok(..) => {}
        Err(..) => {
            error!("couldn't initialize git repository in '{}'", disp);
            error!("cannot continue. Exiting...");
            return;
        }
    }
}
