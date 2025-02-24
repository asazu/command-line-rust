use clap::Parser;
use tailr::Args;

fn main() {
    let args = Args::parse();
    tailr::run(args);
}
