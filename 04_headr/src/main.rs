use clap::Parser;
use headr::Args;

fn main() {
    let args = Args::parse();
    headr::run(args);
}
