#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;
extern crate toml;
extern crate git2;

use clap::{Arg, App, SubCommand};

mod command {
    pub mod admin;
    pub mod build;
    pub mod deploy;
    pub mod page;
    pub mod post;
    pub mod run;
    pub mod setup;
}

use command::admin::admin as do_admin;
use command::build::build as do_build;
use command::deploy::deploy as do_deploy;
use command::page::page as do_page;
use command::post::post as do_post;
use command::run::run as do_run;
use command::setup::setup as do_setup;

fn main() {
    env_logger::init().unwrap();

    let version = "1.0.0";
    let author = "Andrew Brinker <me@andrewbrinker.com>";

    let setup = SubCommand::with_name("setup")
        .about("create a new drow site")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("DIRECTORY")
                .help("the directory to create the new site in")
                .index(1),
        );

    let run = SubCommand::with_name("run")
        .about("serve your drow site locally")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("PORT")
                .index(1)
                .help("the port to serve the site on"),
        );

    let build = SubCommand::with_name("build")
        .about("build your drow site once")
        .author(author)
        .version(version);

    let deploy = SubCommand::with_name("deploy")
        .about("deploy your drow project")
        .author(author)
        .version(version);

    let post = SubCommand::with_name("post")
        .about("create a new post with your default editor")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("TITLE")
                .help("the title of the new post")
                .index(1)
                .required(true),
        );

    let page = SubCommand::with_name("page")
        .about("create a new page with your default editor")
        .author(author)
        .version(version)
        .arg(
            Arg::with_name("TITLE")
                .help("the title of the new page")
                .index(1)
                .required(true),
        );

    let admin = SubCommand::with_name("admin")
        .about(
            "start up a local admin server to edit config, write posts, and edit posts",
        )
        .author(author)
        .version(version);

    let app = App::new("drow")
        .about("An opinionated static site builder")
        .author(author)
        .version(version)
        .subcommand(setup)
        .subcommand(run)
        .subcommand(build)
        .subcommand(deploy)
        .subcommand(post)
        .subcommand(page)
        .subcommand(admin);

    match app.get_matches().subcommand() {
        ("setup", Some(m)) => {
            let directory = m.value_of("DIRECTORY").unwrap_or(".");
            do_setup(directory);
        }
        ("run", Some(m)) => {
            let port = m.value_of("PORT").unwrap_or("3000");
            do_run(port);
        }
        ("post", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            do_post(title);
        }
        ("page", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            do_page(title);
        }
        ("build", Some(..)) => do_build(),
        ("deploy", Some(..)) => do_deploy(),
        ("admin", Some(..)) => do_admin(),
        _ => {}
    }
}
