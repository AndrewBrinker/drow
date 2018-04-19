use std::path::Display;

#[derive(Debug, Fail)]
pub enum DrowError<'a> {
    #[fail(display = "path doesn't exist: {}", path)]
    PathDoesntExist { path: Display<'a> },

    #[fail(display = "can't create directory: {}", path)]
    CantCreateDirectory { path: Display<'a> },

    #[fail(display = "path isn't a directory: {}", path)]
    PathIsntADirectory { path: Display<'a> },

    #[fail(display = "can't read directory: {}", path)]
    CantReadDirectory { path: Display<'a> },

    #[fail(display = "directory isn't empty: {}", path)]
    DirectoryIsntEmpty { path: Display<'a> },

    #[fail(display = "can't clone template repo: {}", path)]
    CantCloneTemplateRepo { path: Display<'a> },

    #[fail(display = "can't delete .git directory: {}", path)]
    CantDeleteGitDirectory { path: Display<'a> },

    #[fail(display = "can't initialize git repository: {}", path)]
    CantInitializeGitRepository { path: Display<'a> },

    #[fail(display = "document already exists: {}", path)]
    DocumentAlreadyExists { path: Display<'a> },

    #[fail(display = "can't create document: {}", path)]
    CantCreateDocument { path: Display<'a> },
}

