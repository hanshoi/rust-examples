use clap::Parser;
use std::process::Command;

mod file;
mod history;
mod io;

static QUICK_SNAPPY_FILE: &str = ".snappy";
static SNAPPY_FILE: &str = "Snappyfile";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long, default_value_t = false)]
    save: bool,

    command: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.save {
        let last_command = history::get_last_command();
        println!("Saved: {last_command}");
        io::write(
            io::in_current_dir(QUICK_SNAPPY_FILE),
            last_command.as_bytes(),
        )
        .unwrap();
    } else if cli.command.is_none() {
        let last_command =
            String::from_utf8(io::read(io::in_current_dir(QUICK_SNAPPY_FILE)).unwrap()).unwrap();
        execute(last_command)
    } else {
        let path = io::in_current_dir(SNAPPY_FILE);
        if path.exists() {
            let commands = file::parse(io::read(path).unwrap());
            for a in commands.keys() {
                println!("{a}");
            }
            let command = commands[&cli.command.unwrap()].clone();
            execute(command.command);
        } else {
            println!("No snappyfile found in current folder")
        }
    }
}

fn execute(command: String) {
    let command_list: Vec<&str> = command.split(' ').collect();

    Command::new(command_list[0])
        .args(&command_list[1..])
        .status()
        .expect("process failed to execute");
}
