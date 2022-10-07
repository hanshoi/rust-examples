use clap::Parser;

mod history;
mod io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    show_history: bool,
}

fn main() {
    let args = Args::parse();
    if args.show_history {
        for line in history::get_history() {
            println!("{line}");
        }
    }
    let last_command = history::get_last_command();
    println!("{last_command}")
}
