use clap::Parser;
use std::process::Command;

mod history;
mod io;

static SNAPPY_FILE: &str = "snaps";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    save: bool,
}

fn main() {
    let cli = Cli::parse();
    if cli.save {
        println!("Save!");
        let last_command = history::get_last_command();
        println!("{last_command}");
        io::write_current_dir(SNAPPY_FILE, last_command.as_bytes()).unwrap();
    } else {
        let last_command = String::from_utf8(io::read_current_dir(SNAPPY_FILE).unwrap()).unwrap();

        let command: Vec<&str> = last_command.split(' ').collect();

        Command::new(command[0])
            .args(&command[1..])
            .status()
            .expect("process failed to execute");
    }
}
