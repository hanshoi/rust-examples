use clap::Parser;

mod history;
mod io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    show: bool,
}

fn main() {
    let args = Args::parse();
    if args.show {
        for line in history::get_history() {
            println!("{line}");
        }
    }
}
