use config::Config;
use std::path::PathBuf;
use std::fs::{create_dir, File};
use chrono::{Utc, DateTime};

/// Takes in a post title, creates a file called "<timestamp>-<title>.md" in the posts
/// directory.
pub fn post(config: Config, title: &str) {
    let directory = config.posts_dir();
    let disp = directory.display();

    let utc: DateTime<Utc> = Utc::now();
    let year = utc.format("%Y").to_string();
    let month = utc.format("%m").to_string();
    let day = utc.format("%d").to_string();

    let mut new_post = PathBuf::new();
    new_post.push(directory);
    new_post.push(year);
    new_post.push(month);
    new_post.push(day);
    new_post.push(title);
    new_post.set_extension("md");

    info!("checking that we're in a drow repo");
    let config_file = config.config_file();
    if !config_file.exists() {
        error!("we are not in a drow repo");
        error!("cannot continue. Exiting...");
        return;
    }

    if !directory.exists() {
        warn!("'{}' doesn't exist", disp);
        info!("creating directory '{}'", disp);

        let res = create_dir(directory);
        if res.is_err() {
            error!("couldn't create directory '{}'", disp);
            return;
        }
    }

    info!("ensuring '{}' is a directory", disp);
    if !directory.is_dir() {
        error!("'{}' isn't a directory", disp);
        error!("cannot continue. Exiting...");
        return;
    }

    info!("checking if '{}' already exists", new_post.display());
    if new_post.exists() {
        error!("'{}' already exists", new_post.display());
        error!("cannot continue. Exiting...");
        return;
    }

    info!("creating '{}'", new_post.display());
    let res = File::create(&new_post);
    if res.is_err() {
        error!("could not create '{}'", new_post.display());
        error!("cannot continue. Exiting...");
        return;
    }
}
