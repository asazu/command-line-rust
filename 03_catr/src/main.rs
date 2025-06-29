use clap::Parser as _;

fn main() {
    match catr::Args::try_parse() {
        Ok(args) => catr::run(args),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
