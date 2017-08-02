use config::Config;

/// Builds the site, putting the results into the build directory.
pub fn build(config: Config) {
    let logger = config.logger();

    info!(logger, "checking that we're in a drow repo");
    let config_file = config.config_file();
    if !config_file.exists() {
        error!(logger, "we are not in a drow repo");
        error!(logger, "cannot continue. Exiting...");
        return;
    }

    // TODO: implement the rest of it
    unimplemented!();
}
