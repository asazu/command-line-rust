use clap::Parser;

fn main() {
    if let Err(e) = lsr::Args::try_parse()
        .map_err(Into::into)
        .and_then(lsr::run)
    {
        eprintln!("{e:#}");
        std::process::exit(1);
    }
}
