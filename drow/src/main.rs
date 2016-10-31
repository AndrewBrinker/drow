extern crate clap;

use clap::{Arg, App, SubCommand};

static VERSION: &'static str = "1.0.0";
static AUTHOR: &'static str = "Andrew Brinker <me@andrewbrinker.com>";

fn main() {
    // Drow should:
    //
    // 1. Verify that you are currently in a drow project (identified by the
    //    presence of a `Drow.toml` file in a parent directory).
    // 2. Parse the configuration in the `Drow.toml` file and ensure that it
    //    is a valid configuration, providing useful error messages otherwise.
    // 3. Verify that the site directory is present.
    //
    // CLI API
    //
    // drow setup  - Create a new Drow project
    // drow run    - Serve your Drow project locally
    // drow build  - Build your Drow project once
    // drow deploy - Deploy your Drow project
    // drow post   - Create a new post and open your default editor
    // drow page   - Create a new page and open your default editor
    //
    // Should probably draw inspiration from the internals of Cargo.

    let setup = SubCommand::with_name("setup")
        .about("create a new drow site")
        .author(AUTHOR)
        .version(VERSION);

    let run = SubCommand::with_name("run")
        .about("serve your drow site locally")
        .author(AUTHOR)
        .version(VERSION);

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
        .version(VERSION);

    let page = SubCommand::with_name("page")
        .about("create a new page with your default editor")
        .author(AUTHOR)
        .version(VERSION);

    let matcher = App::new("drow")
        .about("An opinionated static site builder")
        .author(AUTHOR)
        .version(VERSION)
        .subcommand(setup)
        .subcommand(run)
        .subcommand(build)
        .subcommand(deploy)
        .subcommand(post)
        .subcommand(page)
        .get_matches();
}
