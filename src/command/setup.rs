use std::path::{Path, PathBuf};
use std::fs::{create_dir, remove_dir_all};
use git2::Repository;
use config::Config;

/// Setup a fresh Drow repository.
///
/// Takes in the CLI configuration and the location of the new Drow site.
pub fn setup(config: Config, directory: &str) {
    let logger = config.logger();

    let url = config.template_repo();
    let directory = Path::new(directory);
    let disp = directory.display();

    if !directory.exists() {
        info!(logger, "'{}' doesn't exist", disp);

        let res = create_dir(directory);
        if res.is_err() {
            error!(logger, "couldn't create directory '{}'", disp);
            return;
        }

        info!(logger, "created '{}'", disp);
    }

    if !directory.is_dir() {
        error!(logger, "'{}' isn't a directory", disp);
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            error!(logger, "couldn't read directory '{}'", disp);
            error!(logger, "cannot continue. Exiting...");
            return;
        }
    };

    if !contents.is_empty() {
        error!(logger, "directory '{}' isn't empty", disp);
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    let res = Repository::clone(url, directory);
    if res.is_err() {
        error!(logger, "couldn't clone template repo '{}'", disp);
        error!(logger, "cannot continue. Exiting...");
        return;
    };

    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    let res = remove_dir_all(&git_dir);
    if res.is_err() {
        error!(
            logger,
            "unable to delete .git directory from cloned template"
        );
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    let res = Repository::init(directory);
    if res.is_err() {
        error!(logger, "couldn't initialize git repository in '{}'", disp);
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    info!(logger, "setup new Drow site in '{}'", disp);
}
