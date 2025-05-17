mod io;
mod iter;
mod unique;

use self::io::{IOError, create_file, do_io_task, open_file};
use self::unique::unique;
use clap::{App, Arg};
use std::io::{BufRead, BufReader, BufWriter, Write};

type MyResult<T> = Result<T, IOError>;

pub enum Input {
    Stdin,
    Path(String),
}

#[derive(Debug)]
pub struct Config {
    pub input: Option<String>,
    pub output: Option<String>,
    pub count: bool,
}

pub fn get_args() -> Config {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("asa@example.org")
        .about("Rust uniq")
        .arg(
            Arg::with_name("input")
                .value_name("INPUT")
                .help("Input file (or standard input)"),
        )
        .arg(
            Arg::with_name("output")
                .value_name("OUTPUT")
                .help("Output file (or standard output)"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .help("Prifixes lines by the number of occurences"),
        )
        .get_matches();

    Config {
        input: matches.value_of("input").map(Into::into),
        output: matches.value_of("output").map(Into::into),
        count: matches.is_present("count"),
    }
}

fn open_input(file_name: &Option<String>) -> MyResult<Box<dyn BufRead>> {
    let result: Box<dyn BufRead> = match file_name.as_deref() {
        None | Some("-") => Box::new(BufReader::new(std::io::stdin())),
        Some(file_name) => Box::new(BufReader::new(open_file(file_name)?)),
    };
    Ok(result)
}

fn open_output(file_name: &Option<String>) -> MyResult<BufWriter<Box<dyn Write>>> {
    let writer: Box<dyn Write> = match file_name.as_deref() {
        None | Some("-") => Box::new(std::io::stdout()),
        Some(file_name) => Box::new(create_file(file_name)?),
    };
    Ok(BufWriter::new(writer))
}

fn print_result(
    mut iter: impl Iterator<Item = Result<(usize, String), std::io::Error>>,
    config: &Config,
) -> MyResult<()> {
    let mut writer = open_output(&config.output)?;
    iter.try_for_each(|x| match x {
        Err(e) => Err(IOError::new(config.input.as_deref().unwrap_or("-"), e)),
        Ok((count, line)) => {
            let output = config.output.as_deref().unwrap_or("-");
            do_io_task(output, |_| {
                if config.count {
                    writeln!(writer, "{count:7} {line}")
                } else {
                    writeln!(writer, "{line}")
                }
            })
        }
    })
}

pub fn run(config: &Config) -> MyResult<()> {
    let input = open_input(&config.input)?;
    let uniq_iter = unique(input);
    print_result(uniq_iter, config)?;
    Ok(())
}
