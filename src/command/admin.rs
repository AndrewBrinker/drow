use config::Config;

/// Starts two local servers, one to serve a live-reloading instance of the
/// fully-built Drow site, the other to serve an admin panel to manage site
/// configuration, edit pages, add pages, edit posts, and add posts.
pub fn admin(config: Config) {
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
