
/// A struct containing all CLI configuration.
pub struct Config<'a> {
    template_repo: &'a str,
}

impl<'a> Config<'a> {
    /// Create a new Config struct.
    pub fn new() -> Self {
        Config { template_repo: "https://github.com/andrewbrinker/drow-template" }
    }

    /// Get the location of the repo containing the Drow site template.
    pub fn get_template_repo(&self) -> &'a str {
        self.template_repo
    }
}
