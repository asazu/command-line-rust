use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, stdin};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    numbers_lines: bool,
    numbers_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("asa@example.org")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("numbers_lines")
                .short("n")
                .long("number")
                .help("Numbers all output lines")
                .takes_value(false)
                .conflicts_with("numbers_nonblank_lines"),
        )
        .arg(
            Arg::with_name("numbers_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Numbers nonempty output lines. Cannot use with -n")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        numbers_lines: matches.is_present("numbers_lines"),
        numbers_nonblank_lines: matches.is_present("numbers_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    let result: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    };
    Ok(result)
}

struct CatFunc {
    count: fn(&mut i32, &str),
    print: fn(&i32, &str),
}

fn select_cat_func(config: &Config) -> CatFunc {
    if config.numbers_lines {
        CatFunc {
            count: |n, _| *n += 1,
            print: |n, str| println!("{:6}\t{}", n, str),
        }
    } else if config.numbers_nonblank_lines {
        CatFunc {
            count: |n, str| {
                if !str.is_empty() {
                    *n += 1
                }
            },
            print: |n, str| {
                if str.is_empty() {
                    println!();
                } else {
                    println!("{:6}\t{}", n, str);
                }
            },
        }
    } else {
        CatFunc {
            count: |_, _| {},
            print: |_, str| println!("{}", str),
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let cat_func = select_cat_func(&config);
    let mut line_count = 0;

    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(input) => {
                for line in input.lines() {
                    let line = line?;
                    (cat_func.count)(&mut line_count, &line);
                    (cat_func.print)(&line_count, &line);
                }
            }
        }
    }
    Ok(())
}
