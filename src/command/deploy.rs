use config::Config;

/// Deploys the site to the location specified in the Drow.toml file.
pub fn deploy(config: Config) {
    info!("checking that we're in a drow repo");
    let config_file = config.config_file();
    if !config_file.exists() {
        error!("we are not in a drow repo");
        error!("cannot continue. Exiting...");
        return;
    }

    // TODO: implement the rest of it
    unimplemented!();
}
