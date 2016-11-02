use std::path::Path;
use std::fs::File;

// Yes, this is silly nonsense.
pub fn unzip_to_dir<P: AsRef<Path>>(zip_file: File, directory: P) -> Result<(), ()> {
    unimplemented!();
}

