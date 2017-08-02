use std::path::{Path, PathBuf};
use std::fs::{create_dir, remove_dir_all};
use git2::Repository;
use config::Config;

/// Setup a fresh Drow repository.
///
/// Takes in the CLI configuration and the location of the new Drow site.
pub fn setup(config: Config, directory: &str) {
    info!("Setting up drow site");
    let url = config.template_repo();
    let directory = Path::new(directory);
    let disp = directory.display();

    info!("Checking if '{}' exists", disp);
    if !directory.exists() {
        warn!("'{}' doesn't exist", disp);
        info!("creating directory '{}'", disp);

        match create_dir(directory) {
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
    let res = Repository::clone(url, directory);
    if res.is_err() {
        error!("couldn't clone template repo '{}'", disp);
        error!("cannot continue. Exiting...");
        return;
    };

    info!("deleting .git directory from cloned template");
    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    let res = remove_dir_all(&git_dir);
    if res.is_err() {
        error!("unable to delete .git directory from cloned template");
        error!("cannot continue. Exiting...");
        return;
    }

    info!("initializing fresh git repository in '{}'", disp);
    let res = Repository::init(directory);
    if res.is_err() {
        error!("couldn't initialize git repository in '{}'", disp);
        error!("cannot continue. Exiting...");
        return;
    }
}
