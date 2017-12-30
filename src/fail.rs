use slog::Logger;
use std::fmt;
use std::path::Path;

#[derive(Debug)]
pub enum Fail<'a> {
    PathDoesntExist(&'a Path),
    CantCreateDirectory(&'a Path),
    PathIsntADirectory(&'a Path),
    CantReadDirectory(&'a Path),
    DirectoryIsntEmpty(&'a Path),
    CantCloneTemplateRepo(&'a Path),
    CantDeleteGitDirectory(&'a Path),
    CantInitializeGitRepository(&'a Path)
}

pub fn report_failure<'a>(logger: &Logger, f: Fail<'a>) {
    error!(logger, "{}", f.to_string());
    error!(logger, "can't continue. Exiting...");
}

impl<'a> fmt::Display for Fail<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use fail::Fail::*;

        match *self {
            PathDoesntExist(p) => {
                write!(f, "'{}' doesn't exist", p.display())
            }
            CantCreateDirectory(p) => {
                write!(f, "can't create directory '{}'", p.display())
            }
            PathIsntADirectory(p) => {
                write!(f, "'{}' isn't a directory", p.display())
            }
            CantReadDirectory(p) => {
                write!(f, "can't read directory {}'", p.display())
            }
            DirectoryIsntEmpty(p) => {
                write!(f, "'{}' isn't empty", p.display())
            }
            CantCloneTemplateRepo(p) => {
                write!(f, "can't clone template repo '{}'", p.display())
            }
            CantDeleteGitDirectory(p) => {
                write!(f, "can't delete delete .git directory from '{}'", p.display())
            }
            CantInitializeGitRepository(p) => {
                write!(f, "can't initialize git repository in '{}'", p.display())
            }
        }
    }
}

