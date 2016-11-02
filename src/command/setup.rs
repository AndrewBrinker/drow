use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use git2::Repository;
use hyper::client::Client;
use slog_scope;
use util;

// There are three cases:
//
// - The directory exists and isn't empty => error
// - The directory exists and is empty => use it
// - The directory doesn't exist => create it and use it
pub fn setup(directory: &str) {
    let url = "https://github.com/AndrewBrinker/drow-template";
    let log = slog_scope::logger();
    let directory = Path::new(directory);
    let display = directory.display();

    // Sweet, sweet logging.
    info!(log, "Setting up drow site");

    // Make sure the directory exists and create it if it doesn't.
    if !directory.exists() {
        warn!(log, format!("'{}' doesn't exist", display));
        info!(log, format!("creating directory '{}'", display));

        match fs::create_dir(directory) {
            Ok(..) => info!(log, format!("created directory '{}'", display)),
            Err(..) => {
                error!(log, format!("couldn't create directory '{}'", display));
                return;
            }
        }
    }

    // Make sure we're looking at a directory.
    if !directory.is_dir() {
        error!(log, format!("'{}' isn't a directory", display));
        return;
    }

    // Get the contents of the directory as a vector.
    let contents: Vec<_> = match directory.read_dir() {
        Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
        Err(..) => {
            error!(log, format!("couldn't read directory '{}'", display));
            return;
        }
    };

    // If that vector isn't empty, you've got problems!
    if !contents.is_empty() {
        error!(log, format!("directory '{}' isn't empty", display));
        return;
    }

    info!(log, "downloading template");
    let client = Client::new();

    // Make HTTP request for zip file.
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(..) => {
            error!(log, "couldn't download template");
            return;
        }
    };

    // Read the contents of the response.
    let mut body = String::new();
    match response.read_to_string(&mut body) {
        Ok(..) => info!(log, "read downloaded content"),
        Err(..) => {
            error!(log, "couldn't read downloaded content");
            return;
        }
    }

    // Build the location for the zip file.
    let mut zip_location = PathBuf::new();
    zip_location.push(directory);
    zip_location.push("template.zip");

    // Attempt to create that zip file.
    let mut zip_file = match File::create(&zip_location) {
        Ok(zip_file) => zip_file,
        Err(..) => {
            error!(log, "couldn't create zip file '{}'", &zip_location.display());
            return;
        }
    };

    // Attempt write downloaded body to the zip file.
    match zip_file.write_all(body.as_bytes()) {
        Ok(..) => {},
        Err(..) => {
            error!(log, "couldn't write downloaded content to zip file '{}'", &zip_location.display());
        }
    }

    // Unzip zip file.
    match util::unzip_to_dir(zip_file, directory) {
        Ok(..) => info!(log, format!("unzipped '{}' into '{}'", &zip_location.display(), display)),
        Err(..) => {
            error!(log, format!("couldn't unzip file file '{}'", &zip_location.display()));
            return;
        }
    }

    // Delete zip file.
    match fs::remove_file(&zip_location) {
        Ok(..) => info!(log, format!("deleted zip file '{}'", &zip_location.display())),
        Err(..) => {
            error!(log, format!("couldn't deleted zip file '{}'", &zip_location.display()));
            return;
        }
    }

    // Initialize Git repository.
    match Repository::init(directory) {
        Ok(..) => info!(log, format!("initialized git repository in '{}'", display)),
        Err(..) => {
            error!(log, format!("couldn't initialize git repository in '{}'", display));
        }
    }
}

