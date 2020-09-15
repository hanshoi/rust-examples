
use std::env;
use std::io::{self, Write};
extern crate dirs;

fn write(path: &[u8]){
    io::stdout().write_all(path).unwrap_or(());
}

fn main() -> io::Result<()> {
    let mut path = env::current_dir()?;
    let home_str = dirs::home_dir().map(
        |p| p.as_path().display().to_string()
    ).unwrap_or(String::from("/"));

    loop {
        if path.join(".git").is_dir() {
            write(path.to_str().unwrap().as_bytes());
            break
        };
        path = match path.parent() {
            Some(path) => path.to_path_buf(),
            None => {
                break;
            },
        };
        if path.to_str().unwrap() == home_str {
            break
        };
    };
    Ok(())
}
