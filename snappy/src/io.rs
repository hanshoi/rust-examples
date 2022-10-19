use std::env;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::Path;

pub fn read_home_dir(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
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

pub fn read_current_dir(path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
    let target = env::current_dir()?.join(path);
    let mut buffer = Vec::new();

    if target.exists() {
        let mut file = File::open(target)?;
        file.read_to_end(&mut buffer)?;
    }

    Ok(buffer)
}

pub fn write_current_dir(path: impl AsRef<Path>, content: &[u8]) -> Result<(), Error> {
    let target = env::current_dir()?.join(path);

    let mut file = File::create(target)?;
    file.write_all(content)?;

    Ok(())
}
