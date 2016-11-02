#[macro_use]
extern crate slog;
extern crate slog_scope;
extern crate slog_term;
extern crate clap;
extern crate toml;
extern crate git2;
extern crate hyper;

mod command;
mod util;
use clap::{Arg, App, SubCommand};
use slog::DrainExt;

fn main() {
    let version = "1.0.0";
    let author = "Andrew Brinker <me@andrewbrinker.com>";

    // Setup logging.
    let term_log = slog_term::streamer().stdout().build().fuse();
    let log = slog::Logger::root(term_log, o!());
    slog_scope::set_global_logger(log);

    let setup = SubCommand::with_name("setup")
        .about("create a new drow site")
        .author(author)
        .version(version)
        .arg(Arg::with_name("DIRECTORY")
             .help("the directory to create the new site in")
             .index(1));

    let run = SubCommand::with_name("run")
        .about("serve your drow site locally")
        .author(author)
        .version(version)
        .arg(Arg::with_name("PORT")
             .index(1)
             .help("the port to serve the site on"));

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
        .arg(Arg::with_name("TITLE")
             .help("the title of the new post")
             .index(1)
             .required(true));

    let page = SubCommand::with_name("page")
        .about("create a new page with your default editor")
        .author(author)
        .version(version)
        .arg(Arg::with_name("TITLE")
             .help("the title of the new page")
             .index(1)
             .required(true));

    let app = App::new("drow")
        .about("An opinionated static site builder")
        .author(author)
        .version(version)
        .subcommand(setup)
        .subcommand(run)
        .subcommand(build)
        .subcommand(deploy)
        .subcommand(post)
        .subcommand(page);

    match app.get_matches().subcommand() {
        ("setup", Some(m)) => {
            let directory = m.value_of("DIRECTORY").unwrap_or(".");
            command::setup(directory);
        }
        ("run", Some(m)) => {
            let port = m.value_of("PORT").unwrap_or("3000");
            command::run(port);
        }
        ("post", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            command::post(title);
        }
        ("page", Some(m)) => {
            // This is guaranteed not to be empty by clap.
            let title = m.value_of("TITLE").unwrap();
            command::page(title);
        }
        ("build", Some(..)) => command::build(),
        ("deploy", Some(..)) => command::deploy(),
        _ => {}
    }
}
