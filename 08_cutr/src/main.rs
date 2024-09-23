use clap::Parser;
use cutr::Args;

fn main() {
    let args = Args::parse();
    if let Err(e) = cutr::run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
