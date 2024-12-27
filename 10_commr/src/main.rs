use clap::Parser;
use commr::{self, Args};

fn main() {
    let args = Args::parse();
    if let Err(e) = args.check().and_then(commr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
