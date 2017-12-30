use std::path::{Path, PathBuf};
use std::fs::{create_dir, remove_dir_all};
use git2::Repository;
use config::Config;
use fail::{Fail, report_failure};

/// Setup a fresh Drow repository.
///
/// Takes in the CLI configuration and the location of the new Drow site.
pub fn start(config: Config, directory: &str) {
    let logger = config.logger();
    let url = config.template_repo();
    let directory = Path::new(directory);

    if !directory.exists() {
        report_failure(logger, Fail::PathDoesntExist(directory));

        let res = create_dir(directory);
        if res.is_err() {
            report_failure(logger, Fail::CantCreateDirectory(directory));
            return;
        }

        info!(logger, "created '{}'", directory.display());
    }

    if !directory.is_dir() {
        report_failure(logger, Fail::PathIsntADirectory(directory));
        return;
    }

    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            report_failure(logger, Fail::CantReadDirectory(directory));
            return;
        }
    };

    if !contents.is_empty() {
        report_failure(logger, Fail::DirectoryIsntEmpty(directory));
        return;
    }

    let res = Repository::clone(url, directory);
    if res.is_err() {
        report_failure(logger, Fail::CantCloneTemplateRepo(directory));
        return;
    };

    let mut git_dir = PathBuf::new();
    git_dir.push(directory);
    git_dir.push(".git");
    let res = remove_dir_all(&git_dir);
    if res.is_err() {
        report_failure(logger, Fail::CantDeleteGitDirectory(directory));
        return;
    }

    let res = Repository::init(directory);
    if res.is_err() {
        report_failure(logger, Fail::CantInitializeGitRepository(directory));
        return;
    }

    info!(logger, "started new Drow site in '{}'", directory.display());
}
