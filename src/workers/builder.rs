use config::Config;
use error::DrowError;

pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build(&self, config: Config) -> Result<(), Fail> {
        build_assets(&config)?;
        build_pages(&config)?;
        build_posts(&config)?;

        // There are three directories to handle:
        //
        // 1. assets/ - These get copied over directly.
        // 2. pages/ - These get transformed (title.md -> title/index.html)
        // 3. posts/ - These get transformed (2017-01-20-title.md -> blog/2017/01/20/title/index.html)
        //
        // Pages and Posts may use Templates.

        // Probably want to start by copying over the assets.
        //
        // Then go through the posts and build up a data structure representing them.
        //
        // Make sure also to load up Drow.toml
        //
        // Load the posts and the Drow.toml into a uniform data structure which the page templates
        // can use.
        //
        // Then do the pages.
        //
        // For the templating, I don't think anything to complicated is needed.
        //
        // Any post or page can put something like the following at the top:
        //
        //     template: hello.html
        //     ---
        //
        //     <p>Hello!</p>
        //
        // And the template choice may look like this:
        //
        //     template: main.html
        //     ---
        //
        //     <div class="hello">
        //         {{! content }}
        //     </div>
        //
        // Which would then get passed along to main.html, which may look like this:
        //
        //     <!doctype html>
        //     <html lang="en-US">
        //         <head>
        //             <meta charset="utf-8">
        //             <title>{{= data.title }}</title>
        //         </head>
        //         <body>
        //             {{! content }}
        //         </body>
        //     </html>
        //
        // So the end result would look like this, assuming Drow.toml had title set to "Hello":
        //
        //     <!doctype html>
        //     <html lang="en-US">
        //         <head>
        //             <meta charset="utf-8">
        //             <title>Hello</title>
        //         </head>
        //         <body>
        //             <div class="hello">
        //                 <p>Hello!</p>
        //             </div>
        //         </body>
        //     </html>
        //
        // So the end result would look like this, assuming Drow.toml had title set to "Hello":

        Ok(())
    }
}

fn build_assets(config: &Config) -> Result<(), Fail> {
    unimplemented!();
}

fn build_pages(config: &Config) -> Result<(), Fail> {
    unimplemented!();
}

fn build_posts(config: &Config) -> Result<(), Fail> {
    unimplemented!();
}

