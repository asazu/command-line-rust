use clap::Parser;
use findr::{self, Args};

fn main() {
    let args = Args::parse();
    findr::run(args);
}
