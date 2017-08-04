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

    let help_text = r#"drow is an opinionated static site generator

    learn:
        drow howto [<thing>]    → read detailed guides to using drow
        drow idea               → submit a question or idea to the drow devs

    use:
        drow start [<location>] → create a new site
        drow build              → build your site
        drow admin              → manage your site with a nifty admin panel
        drow post <title>       → start a new post on your site
        drow page <title>       → create a new page for your site

    help:
        drow version            → show what version you're using
        drow help [<command>]   → show this help text"#;

    let app = App::new("drow")
        .about(crate_description!())
        .author(crate_authors!(", "))
        .version(crate_version!())
        .help(help_text)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("start")
                .arg(&directory_arg)
                .about("Sets up a new drow site"),
        )
        .subcommand(
            SubCommand::with_name("post")
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
                ),
        )
        .subcommand(
            SubCommand::with_name("page")
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
                ),
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
