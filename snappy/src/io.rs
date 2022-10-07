use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

pub fn read_as_bytes(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    /* `path` is relative to the home directory */
    let home = dirs::home_dir().unwrap();
    let target = home.join(path);
    let mut buffer = Vec::new();

    if target.exists() {
        let mut file = File::open(target)?;
        file.read_to_end(&mut buffer)?;
    }

    Ok(buffer)
}
