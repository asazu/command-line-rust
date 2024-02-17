use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Read, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    line_count: usize,
    byte_count: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("asa@example.org")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .multiple(true)
                .default_value("-")
                .help("Input file(s)")
        )
        .arg(
            Arg::with_name("line_count")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .default_value("10")
                .help("Prints the first LINES")
        )
        .arg(
            Arg::with_name("byte_count")
                .short("c")
                .long("bytes")
                .takes_value(true)
                .value_name("BYTES")
                .help("Prints the first BYTES bytes of each file.")
                .conflicts_with("line_count")
        )
        .get_matches();

    let line_count = matches.value_of("line_count")
        .map(parse_positive_int)
        .unwrap()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let byte_count = matches.value_of("byte_count")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        line_count,
        byte_count,
    })
}

pub fn parse_positive_int(s: &str) -> MyResult<usize> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(s.into()),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    let result: Box<dyn BufRead> = match filename {
        "-" => Box::new(BufReader::new(stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    };
    Ok(result)
}

fn output_lines(mut input: Box<dyn BufRead>, lines: usize) -> MyResult<()> {
    let mut line = String::new();
    for _ in 0..lines {
        let bytes_read = input.read_line(&mut line)?;
        if bytes_read == 0 { break; }
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

fn output_bytes(input: Box<dyn BufRead>, bytes: usize) -> MyResult<()> {
    let mut buf = vec![0; bytes];
    let bytes_read = input.take(bytes as u64).read(&mut buf)?;
    // print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
    let mut writer = BufWriter::new(stdout());
    writer.write_all(&buf[..bytes_read])?;
    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    let multiple_files = config.files.len() > 1;
    for (i, filename) in config.files.iter().enumerate() {
        if multiple_files {
            if i != 0 { println!(); }
            println!("==> {} <==", filename);
        }
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(input) => {
                if let Some(bytes) = config.byte_count {
                    output_bytes(input, bytes)?;
                } else {
                    output_lines(input, config.line_count)?;
                }
            },
        }
    }
    Ok(())
}
