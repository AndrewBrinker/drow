use std::path::Path;
use std::fs::{File, metadata, read_dir};
use git2::Repository;
use slog_scope;

pub fn setup(directory: &str) {
    let log = slog_scope::logger();
    info!(log, "Setting up drow site");

    let path_buf = Path::new(directory).to_owned();

    if let Ok(path) = path_buf.canonicalize() {
        let meta = match metadata(&path) {
            Ok(meta) => meta,
            Err(..) => {
                if let Ok(file) = File::create(&path) {
                    if let Ok(meta) = metadata(&path) {
                        meta
                    } else {
                        error!(log, format!("couldn't read directory '{}'", directory));
                        return;
                    }
                } else {
                    error!(log, format!("couldn't create directory '{}'", directory));
                    return;
                }
            }
        };

        if meta.is_dir() {
            if let Ok(read_dir) = read_dir(&path) {
                let contents: Vec<_> = read_dir.collect();

                if contents.len() == 0 {
                    let repo = "https://github.com/AndrewBrinker/drow-template";

                    if let Ok(..) = Repository::clone(repo, path) {
                        info!(log, format!("drow site successfully setup at '{}'", directory));
                    } else {
                        error!(log, format!("failed to clone repository into '{}'", directory));
                    }
                } else {
                    error!(log, format!("'{}' isn't empty", directory));
                }
            } else {
                error!(log, format!("'{}' can't be edited with current permissions", directory));
            }
        } else {
            error!(log, format!("'{}' isn't a directory", directory));
        }
    } else {
        error!(log, format!("'{}' can't be canonicalized", directory));
    }
}

