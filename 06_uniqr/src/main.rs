use uniqr::{get_args, run};

fn main() {
    let config = get_args();
    if let Err(e) = run(&config) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
