use std::path::PathBuf;
use config::Config;
use fail::Fail;

/// Represents a single page or post.
#[derive(Getters)]
pub struct Repo {
    /// The location of the repository.
    #[get = "pub"]
    location: PathBuf,
}

impl Repo {
    pub fn new(_config: Config, location: &str) -> Repo {
        let location = PathBuf::from(location);

        Repo { location }
    }

    pub fn create(&self) -> Result<(), Fail> {
        unimplemented!();
    }
}

