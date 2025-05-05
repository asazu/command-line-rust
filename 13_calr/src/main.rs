use calr::Args;
use clap::Parser;

fn main() {
    if let Err(e) = Args::try_parse().map_err(Into::into).and_then(calr::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
