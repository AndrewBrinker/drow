extern crate clap;
mod command;
use clap::{Arg, App, SubCommand};

static VERSION: &'static str = "1.0.0";
static AUTHOR: &'static str = "Andrew Brinker <me@andrewbrinker.com>";

fn main() {
    let setup = SubCommand::with_name("setup")
        .about("create a new drow site")
        .author(AUTHOR)
        .version(VERSION)
        .arg(Arg::with_name("DIRECTORY")
             .help("the directory to create the new site in")
             .index(1));

    let run = SubCommand::with_name("run")
        .about("serve your drow site locally")
        .author(AUTHOR)
        .version(VERSION)
        .arg(Arg::with_name("PORT")
             .index(1)
             .help("the port to serve the site on"));

    let build = SubCommand::with_name("build")
        .about("build your drow site once")
        .author(AUTHOR)
        .version(VERSION);

    let deploy = SubCommand::with_name("deploy")
        .about("deploy your drow project")
        .author(AUTHOR)
        .version(VERSION);

    let post = SubCommand::with_name("post")
        .about("create a new post with your default editor")
        .author(AUTHOR)
        .version(VERSION)
        .arg(Arg::with_name("TITLE")
             .help("the title of the new post")
             .index(1)
             .required(true));

    let page = SubCommand::with_name("page")
        .about("create a new page with your default editor")
        .author(AUTHOR)
        .version(VERSION)
        .arg(Arg::with_name("TITLE")
             .help("the title of the new page")
             .index(1)
             .required(true));

    let drow = App::new("drow")
        .about("An opinionated static site builder")
        .author(AUTHOR)
        .version(VERSION)
        .subcommand(setup)
        .subcommand(run)
        .subcommand(build)
        .subcommand(deploy)
        .subcommand(post)
        .subcommand(page);

    let matches = drow.get_matches();

    match matches.subcommand() {
        ("setup", Some(sub_matches)) => {
            let directory = sub_matches.value_of("DIRECTORY").unwrap_or(".");
            command::setup(directory);
        }
        ("run", Some(sub_matches)) => {
            let port = sub_matches.value_of("PORT").unwrap_or("3000");
            command::run(port);
        }
        ("build", Some(..)) => {
            command::build();
        }
        ("deploy", Some(..)) => {
            command::deploy();
        }
        ("post", Some(sub_matches)) => {
            // This is guaranteed not to be empty by clap.
            let title = sub_matches.value_of("TITLE").unwrap();
            command::post(title);
        }
        ("page", Some(sub_matches)) => {
            // This is guaranteed not to be empty by clap.
            let title = sub_matches.value_of("TITLE").unwrap();
            command::page(title);
        }
        _ => {}
    }
}
