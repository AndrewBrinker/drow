use std::path::PathBuf;
use std::fs::{create_dir, remove_dir_all};
use git2::Repository;
use config::Config;
use fail::Fail;

/// Represents a single page or post.
#[derive(Getters)]
pub struct Repo {
	/// The location of the repository.
	#[get = "pub"]
	location: PathBuf,

    /// The location of the template repo.
    #[get = "pub"]
    template_repo: String,
}

impl Repo {
	pub fn new(config: Config, location: &str) -> Repo {
		let location = PathBuf::from(location);
        let template_repo = config.template_repo().to_string();

		Repo { location, template_repo }
	}

	pub fn create(&self) -> Result<(), Fail> {
		let directory = self.location.clone();

		if !directory.exists() {
			let res = create_dir(&directory);
			if res.is_err() {
				return Err(Fail::CantCreateDirectory(directory));
			}
		}

		if !directory.is_dir() {
			return Err(Fail::PathIsntADirectory(directory));
		}

		let contents: Vec<_> = match directory.read_dir() {
			Ok(directory_iter) => directory_iter.filter(|r| r.is_ok()).collect(),
			Err(..) => {
				return Err(Fail::CantReadDirectory(directory));
			}
		};

		if !contents.is_empty() {
			return Err(Fail::DirectoryIsntEmpty(directory));
		}

		let res = Repository::clone(&self.template_repo, &directory);
		if res.is_err() {
			return Err(Fail::CantCloneTemplateRepo(directory));
		};

		let mut git_dir = PathBuf::new();
		git_dir.push(&directory);
		git_dir.push(".git");
		let res = remove_dir_all(&git_dir);
		if res.is_err() {
			return Err(Fail::CantDeleteGitDirectory(directory));
		}

		let res = Repository::init(&directory);
		if res.is_err() {
			return Err(Fail::CantInitializeGitRepository(directory));
		}

		Ok(())
	}
}

