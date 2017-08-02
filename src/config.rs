
pub struct Config<'a> {
    template_repo: &'a str,
}

impl<'a> Config<'a> {
    pub fn new() -> Self {
        Config { template_repo: "https://github.com/andrewbrinker/drow-template" }
    }

    pub fn get_template_repo(&self) -> &'a str {
        self.template_repo
    }
}
