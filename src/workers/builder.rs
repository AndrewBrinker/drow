use config::Config;
use fail::Fail;

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
    }

    fn build_assets(config: Config) -> Result<(), Fail> {
        unimplemented!();
    }

    fn build_pages(config: Config) -> Result<(), Fail> {
        unimplemented!();
    }

    fn build_posts(config: Config) -> Result<(), Fail> {
        unimplemented!();
    }
}


