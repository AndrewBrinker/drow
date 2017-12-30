use config::Config;
use std::path::PathBuf;
use std::fs::{create_dir, File};
use chrono::{Utc, DateTime};

/// Takes in a post title, creates a file called "<timestamp>-<title>.md" in the posts
/// directory.
///
/// Note that this is intended to duplicate the title-to-filename conversion used
/// by Jekyll.
pub fn post(config: Config, title: &str) {
    let logger = config.logger();

    let directory = config.posts_dir();
    let disp = directory.display();

    let utc: DateTime<Utc> = Utc::now();
    let timestamp = utc.format("%Y-%m-%d").to_string();
    // TODO: Insert Jekyll-style filename conversion.
    let file_name = format!("{}-{}", timestamp, title);

    let mut new_post = PathBuf::new();
    new_post.push(directory);
    new_post.push(file_name);
    new_post.set_extension("md");

    let config_file = config.config_file();
    if !config_file.exists() {
        error!(logger, "we are not in a drow repo");
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    if !directory.exists() {
        info!(logger, "'{}' doesn't exist", disp);

        let res = create_dir(directory);
        if res.is_err() {
            error!(logger, "couldn't create directory '{}'", disp);
            return;
        }

        info!(logger, "created directory '{}'", disp);
    }

    if !directory.is_dir() {
        error!(logger, "'{}' isn't a directory", disp);
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    if new_post.exists() {
        error!(logger, "'{}' already exists", new_post.display());
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    let res = File::create(&new_post);
    if res.is_err() {
        error!(logger, "could not create '{}'", new_post.display());
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    info!(logger, "created new post '{}'", new_post.display());
}
