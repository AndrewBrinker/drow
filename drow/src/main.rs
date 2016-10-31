extern crate clap;

use clap::{Arg, App, SubCommand};

static VERSION: &'static str = "1.0.0";
static AUTHOR: &'static str = "Andrew Brinker <me@andrewbrinker.com>";

// Drow should:
//
// 1. Verify that you are currently in a drow project (`Drow.toml`), unless
//    you're making a new Drow project, of course.
// 2. Parse the configuration in the `Drow.toml` file and ensure that it
//    is a valid configuration, providing useful error messages otherwise.
// 3. Do whatever you've asked for.

fn do_setup(directory: &str) {
    unimplemented!()
}

fn do_run(port: &str) {
    unimplemented!()
}

fn do_build() {
    unimplemented!()
}

fn do_deploy() {
    unimplemented!()
}

fn do_post(title: &str) {
    unimplemented!()
}

fn do_page(title: &str) {
    unimplemented!()
}

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
            do_setup(directory);
        }
        ("run", Some(sub_matches)) => {
            let port = sub_matches.value_of("PORT").unwrap_or("3000");
            do_run(port);
        }
        ("build", Some(..)) => {
            do_build();
        }
        ("deploy", Some(..)) => {
            do_deploy();
        }
        ("post", Some(sub_matches)) => {
            // This is guaranteed not to be empty by clap.
            let title = sub_matches.value_of("TITLE").unwrap();
            do_post(title);
        }
        ("page", Some(sub_matches)) => {
            // This is guaranteed not to be empty by clap.
            let title = sub_matches.value_of("TITLE").unwrap();
            do_page(title);
        }
        _ => {}
    }
}
