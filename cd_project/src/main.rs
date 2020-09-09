
use std::env;
use std::io::{self, Write};
extern crate dirs;

fn main() -> io::Result<()> {
    let mut path = env::current_dir().unwrap();
    let home = dirs::home_dir().unwrap();
    let home_str = home.to_str().unwrap();

    loop {
        if path.join(".git").is_dir() {
            io::stdout().write_all(path.to_str().unwrap().as_bytes())?;
            break
        };
        path = match path.parent() {
            Some(path) => path.to_path_buf(),
            None => {
                io::stdout().write_all(b".")?;
                return Ok(())
            },
        };
        if path.to_str().unwrap() == home_str {
            break
        };
    };
    Ok(())
}
