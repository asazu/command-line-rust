use clap::Parser;
use fortuner::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = fortuner::run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
