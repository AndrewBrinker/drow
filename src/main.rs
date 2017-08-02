//! Drow is an opinionated static site builder.
#![deny(missing_docs)]
#![allow(dead_code)]

// Create a fully-featured CLI
#[macro_use]
extern crate clap;

// Generate getters and setters
#[macro_use]
extern crate getset;

// TOML configuration parsing
extern crate toml;

// Git interaction
extern crate git2;

// Time
extern crate chrono;

// Logging
#[macro_use]
extern crate slog;
extern crate sloggers;

mod config;
mod command {
    pub mod admin;
    pub mod build;
    pub mod page;
    pub mod post;
    pub mod setup;
}

use clap::{Arg, App, SubCommand};
use command::admin::admin;
use command::build::build;
use command::page::page;
use command::post::post;
use command::setup::setup;
use config::Config;

fn main() {
    let config = Config::new();
    let version = crate_version!();
    let author = crate_authors!(", ");
    let setup_cmd = SubCommand::with_name("setup")
        .about("create a new drow site")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("DIRECTORY")
                .help("the directory to create the new site in")
                .index(1),
        );
    let build_cmd = SubCommand::with_name("build")
        .about("build your drow site once")
        .author(author)
        .version(version);
    let post_cmd = SubCommand::with_name("post")
        .about("create a new post with your default editor")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("TITLE")
                .help("the title of the new post")
                .index(1)
                .required(true),
        );
    let page_cmd = SubCommand::with_name("page")
        .about("create a new page with your default editor")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("TITLE")
                .help("the title of the new page")
                .index(1)
                .required(true),
        );
    let admin_cmd = SubCommand::with_name("admin")
        .about(
            "start up a local admin server to edit config, write posts, and edit posts",
        )
        .author(author)
        .version(version);
    let app = App::new("drow")
        .about(crate_description!())
        .author(author)
        .version(version)
        .subcommand(setup_cmd)
        .subcommand(build_cmd)
        .subcommand(post_cmd)
        .subcommand(page_cmd)
        .subcommand(admin_cmd);

    match app.get_matches().subcommand() {
        ("setup", Some(m)) => setup(config, m.value_of("DIRECTORY").unwrap_or(".")),
        ("post", Some(m)) => post(config, m.value_of("TITLE").unwrap()),
        ("page", Some(m)) => page(config, m.value_of("TITLE").unwrap()),
        ("build", Some(..)) => build(config),
        ("admin", Some(..)) => admin(config),
        _ => {}
    }
}
