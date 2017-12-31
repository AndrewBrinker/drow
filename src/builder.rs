use config::Config;
use fail::Fail;

pub struct Builder {}

impl Builder {
    pub fn new() -> Builder {
        Builder {}
    }

    pub fn build(&self, config: Config) -> Result<(), Fail> {
        unimplemented!();
    }
}


