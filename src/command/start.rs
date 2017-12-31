use config::Config;
use workers::repo::Repo;

/// Setup a fresh Drow repository.
///
/// Takes in the CLI configuration and the location of the new Drow site.
pub fn start(config: Config, directory: &str) {
    let repo = Repo::new(config, directory);

    if let Err(e) = repo.create() {
        println!("error: {}", e.to_string());
    } else {
        println!("created '{}'", repo.location().display());
    }
}

