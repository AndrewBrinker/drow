use config::Config;

/// Builds the site, putting the results into the build directory.
pub fn build(config: Config) {
    println!("checking that we're in a drow repo");
    let config_file = config.config_file();
    if !config_file.exists() {
        println!("we are not in a drow repo");
        println!("cannot continue. Exiting...");
        return;
    }

    // TODO: implement the rest of it
    unimplemented!();
}
