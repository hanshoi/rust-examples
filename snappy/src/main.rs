use clap::Parser;
use std::process::Command;

mod history;
mod io;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    save: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match cli.save {
        None => {}
        Some(_) => {}
    }
    let last_command = history::get_last_command();

    println!("{last_command}");
    let command: Vec<&str> = last_command.split(' ').collect();

    Command::new(command[0])
        .args(&command[1..])
        .status()
        .expect("process failed to execute");
}
