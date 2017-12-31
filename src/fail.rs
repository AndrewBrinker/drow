use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Fail {
    PathDoesntExist(PathBuf),
    CantCreateDirectory(PathBuf),
    PathIsntADirectory(PathBuf),
    CantReadDirectory(PathBuf),
    DirectoryIsntEmpty(PathBuf),
    CantCloneTemplateRepo(PathBuf),
    CantDeleteGitDirectory(PathBuf),
    CantInitializeGitRepository(PathBuf),
    DocumentAlreadyExists(PathBuf),
    CantCreateDocument(PathBuf),
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use fail::Fail::*;

        match *self {
            PathDoesntExist(ref p) => {
                write!(f, "'{}' doesn't exist", p.display())
            }
            CantCreateDirectory(ref p) => {
                write!(f, "can't create directory '{}'", p.display())
            }
            PathIsntADirectory(ref p) => {
                write!(f, "'{}' isn't a directory", p.display())
            }
            CantReadDirectory(ref p) => {
                write!(f, "can't read directory {}'", p.display())
            }
            DirectoryIsntEmpty(ref p) => {
                write!(f, "'{}' isn't empty", p.display())
            }
            CantCloneTemplateRepo(ref p) => {
                write!(f, "can't clone template repo '{}'", p.display())
            }
            CantDeleteGitDirectory(ref p) => {
                write!(f, "can't delete delete .git directory from '{}'", p.display())
            }
            CantInitializeGitRepository(ref p) => {
                write!(f, "can't initialize git repository in '{}'", p.display())
            }
            DocumentAlreadyExists(ref p) => {
                write!(f, "'{}' already exists", p.display())
            }
            CantCreateDocument(ref p) => {
                write!(f, "can't create document '{}'", p.display())
            }
        }
    }
}

