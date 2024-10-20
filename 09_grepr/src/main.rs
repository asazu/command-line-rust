use clap::Parser;
use grepr::{self, Args};

fn main() {
    let args = Args::parse().set_default_paths_if_empty();
    if grepr::run(args).is_err() {
        std::process::exit(1);
    }
}
