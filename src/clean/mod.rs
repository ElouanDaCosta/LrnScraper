use std::fs;
use std::io;
use std::path::Path;

pub fn clean_data_folder() {
    let path = "data/";
    remove_dir_contents(path).unwrap();
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}
