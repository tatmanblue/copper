//!

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

/**
*/
pub struct FileUtilities {

}

impl FileUtilities {

    /**
    */
    pub fn clean_up_results(results_dir : &str) -> io::Result<()> {
        let dir_path: &Path = Path::new(results_dir);

        if false == dir_path.is_dir() {
            return Ok(());
        }

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if false == path.is_dir() {
                println!("file found {:?}", entry.path());
                fs::remove_file(entry.path())?;
            }
        }

        return Ok(());
    }
}