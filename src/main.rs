//! Drow is an opinionated static site builder.
#![deny(missing_docs)]
#![allow(dead_code)]

#[macro_use]
extern crate slog;
extern crate sloggers;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate getset;
extern crate toml;
extern crate git2;
extern crate chrono;

mod config;
mod command {
    pub mod admin;
    pub mod build;
    pub mod page;
    pub mod post;
    pub mod start;
}

use clap::{Arg, App, SubCommand};
use command::admin::admin;
use command::build::build;
use command::page::page;
use command::post::post;
use command::start::start;
use config::Config;

fn main() {
    let config = Config::new();

    let directory_arg = Arg::with_name("DIRECTORY")
        .index(1)
        .help("the directory to start the site in");
    let title_arg = Arg::with_name("TITLE")
        .index(1)
        .required(true)
        .help("the title of the new page");

    let app = App::new("drow")
        .about(crate_description!())
        .author(crate_authors!(", "))
        .version(crate_version!())
        .subcommand(
            SubCommand::with_name("start")
                .arg(&directory_arg)
                .about("Sets up a new drow site"),
        )
        .subcommand(
            SubCommand::with_name("post")
                .arg(&title_arg)
                .about("Creates a new post"),
        )
        .subcommand(
            SubCommand::with_name("page")
                .arg(&title_arg)
                .about("Creates a new post"),
        )
        .subcommand(SubCommand::with_name("build").about("Builds the site once"))
        .subcommand(
            SubCommand::with_name("admin").about("Starts a local admin site"),
        );

    match app.get_matches().subcommand() {
        ("start", Some(m)) => start(config, m.value_of("DIRECTORY").unwrap_or(".")),
        ("post", Some(m)) => post(config, m.value_of("TITLE").unwrap()),
        ("page", Some(m)) => page(config, m.value_of("TITLE").unwrap()),
        ("build", Some(..)) => build(config),
        ("admin", Some(..)) => admin(config),
        _ => {}
    }
}
