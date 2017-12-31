use config::Config;
use builder::Builder;

/// Builds the site, putting the results into the build directory.
pub fn build(config: Config) {
    let builder = Builder::new();

    if let Err(e) = builder.build(config) {
        println!("error: {}", e.to_string());
    } else {
        println!("build successful!");
    }
}
