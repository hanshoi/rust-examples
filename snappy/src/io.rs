use std::env;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};

pub fn in_home_dir(file: impl AsRef<Path>) -> PathBuf {
    dirs::home_dir().unwrap().join(file)
}

pub fn in_current_dir(file: impl AsRef<Path>) -> PathBuf {
    env::current_dir().unwrap().join(file)
}

pub fn read(path: PathBuf) -> Result<Vec<u8>, Error> {
    let mut buffer = Vec::new();

    if path.exists() {
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;
    }

    Ok(buffer)
}

pub fn write(path: PathBuf, content: &[u8]) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(content)?;

    Ok(())
}
