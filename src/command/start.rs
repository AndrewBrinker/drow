use std::fs::{create_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use git2::Repository;
use config::Config;
use fail::{Fail, report_failure};

/// Setup a fresh Drow repository.
///
/// Takes in the CLI configuration and the location of the new Drow site.
pub fn start(config: Config, directory: &str) {
    let directory = Path::new(directory);

    if !directory.exists() {
        report_failure(Fail::PathDoesntExist(directory));

        let res = create_dir(directory);
        if res.is_err() {
            report_failure(Fail::CantCreateDirectory(directory));
            return;
        }

        println!("create '{}'", directory.display());
    }

    if !directory.is_dir() {
        report_failure(Fail::PathIsntADirectory(directory));
        return;
    }

    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            report_failure(Fail::CantReadDirectory(directory));
            return;
        }
    };

    if !contents.is_empty() {
        report_failure(Fail::DirectoryIsntEmpty(directory));
        return;
    }

    let res = Repository::clone(config.template_repo(), directory);
    if res.is_err() {
        report_failure(Fail::CantCloneTemplateRepo(directory));
        return;
    };

    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    let res = remove_dir_all(&git_dir);
    if res.is_err() {
        report_failure(Fail::CantDeleteGitDirectory(directory));
        return;
    }

    let res = Repository::init(directory);
    if res.is_err() {
        report_failure(Fail::CantInitializeGitRepository(directory));
        return;
    }

    println!("started new Drow site in '{}'", directory.display());
}
