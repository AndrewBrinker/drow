//! Drow is an opinionated static site builder.
#![deny(missing_docs)]
#![allow(dead_code)]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate getset;
extern crate toml;
extern crate git2;
extern crate chrono;
extern crate unidecode;

mod config;
mod command {
    pub mod admin;
    pub mod build;
    pub mod page;
    pub mod post;
    pub mod start;
}
mod document;
mod fail;
mod name;
mod safe;

use clap::{Arg, App, SubCommand, AppSettings};
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

    let start_cmd = SubCommand::with_name("start")
        .arg(&directory_arg)
        .about("Sets up a new drow site")
        .help(r#""#);

    let admin_cmd = SubCommand::with_name("admin")
        .about("Starts a local admin site")
        .help(r#""#);

    let build_cmd = SubCommand::with_name("build")
        .about("Builds the site once")
        .help(r#""#);

    let page_cmd = SubCommand::with_name("page")
        .arg(&title_arg)
        .about("Creates a new page")
        .help(
            r#"drow page <title>

    description:
        start a new page on your site with <title> as the name.

    example:
        drow page "welcome"
        ⇒ pages/welcome.md         # file created
        ⇒ docs/welcome/index.html  # file built"#,
        );

    let post_cmd = SubCommand::with_name("post")
        .arg(&title_arg)
        .about("Creates a new post")
        .help(
            r#"drow post <title>

    description:
        start a new post on your site with <title> as the name.
        time of creation is included.

    example:
        drow post "hello"
        ⇒ posts/2017-08-02-hello.md
        drow build
        ⇒ docs/blog/2017/08/02/hello/index.html"#,
        );

    let app = App::new("drow")
        .about(crate_description!())
        .author(crate_authors!(", "))
        .version(crate_version!())
        .subcommand(start_cmd)
        .subcommand(post_cmd)
        .subcommand(page_cmd)
        .subcommand(build_cmd)
        .subcommand(admin_cmd)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .help(
            r#"drow is an opinionated static site generator

    use:
        drow start [<location>] → create a new site
        drow build              → build your site
        drow admin              → manage your site with a nifty admin panel
        drow post <title>       → start a new post on your site
        drow page <title>       → create a new page for your site
        drow deploy             → deploy your site with "git push"

    help:
        drow version            → show what version you're using
        drow help [<command>]   → show this help text"#,
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
