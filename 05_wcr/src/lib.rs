mod count;
mod counter;
mod presentation;

use self::count::Count;
use self::counter::{open, word_count};
use self::presentation::print_counts;
use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

enum File {
    Default,
    StdIn,
    Path(String),
}

impl File {
    fn name(&self) -> Option<&str> {
        match self {
            File::Default => None,
            File::StdIn => Some("-"),
            File::Path(path) => Some(path),
        }
    }
}

pub struct Config {
    files: Vec<File>,
    opt_lines: bool,
    opt_words: bool,
    opt_chars: bool,
    opt_bytes: bool,
}

impl Config {
    fn num_opts(&self) -> u32 {
        self.opt_lines as u32
            + self.opt_words as u32
            + self.opt_chars as u32
            + self.opt_bytes as u32
    }

    fn single_opt(&self) -> bool {
        self.num_opts() == 1
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("asa@example.org")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("print the byte counts"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("print the character counts")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("print the newline counts"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("print the word counts"),
        )
        .get_matches();

    let files = match matches.values_of("files") {
        None => vec![File::Default],
        Some(files) => files
            .into_iter()
            .map(|path| {
                if path == "-" {
                    File::StdIn
                } else {
                    File::Path(String::from(path))
                }
            })
            .collect(),
    };

    let mut config = Config {
        files,
        opt_lines: matches.is_present("lines"),
        opt_words: matches.is_present("words"),
        opt_chars: matches.is_present("chars"),
        opt_bytes: matches.is_present("bytes"),
    };

    if config.num_opts() == 0 {
        config.opt_lines = true;
        config.opt_words = true;
        config.opt_bytes = true;
    }

    Ok(config)
}

fn calc_total(counts: &[MyResult<Count>]) -> Count {
    counts.iter().filter_map(|count| count.as_ref().ok()).sum()
}

pub fn run(config: Config) -> MyResult<()> {
    let counts = config
        .files
        .iter()
        .map(|file| open(file).and_then(word_count))
        .collect::<Vec<_>>();
    let total = calc_total(&counts);
    print_counts(&counts, &total, &config);
    Ok(())
}
