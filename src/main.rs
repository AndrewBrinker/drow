//! Drow is an opinionated static site builder.
#![deny(missing_docs)]
#![allow(dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate getset;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate toml;
extern crate git2;
extern crate chrono;

mod config;
mod command {
    pub mod admin;
    pub mod build;
    pub mod deploy;
    pub mod page;
    pub mod post;
    pub mod setup;
}

use clap::{Arg, App, SubCommand};
use command::admin::admin;
use command::build::build;
use command::deploy::deploy;
use command::page::page;
use command::post::post;
use command::setup::setup;
use config::Config;

fn main() {
    pretty_env_logger::init().unwrap();
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

    let deploy_cmd = SubCommand::with_name("deploy")
        .about("deploy your drow project")
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
        .subcommand(deploy_cmd)
        .subcommand(post_cmd)
        .subcommand(page_cmd)
        .subcommand(admin_cmd);

    match app.get_matches().subcommand() {
        ("setup", Some(m)) => {
            let directory = m.value_of("DIRECTORY").unwrap_or(".");
            setup(config, directory);
        }
        ("post", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            post(config, title);
        }
        ("page", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            page(config, title);
        }
        ("build", Some(..)) => build(),
        ("deploy", Some(..)) => deploy(),
        ("admin", Some(..)) => admin(),
        _ => {}
    }
}
