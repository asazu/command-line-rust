use bstr::io::BufReadExt;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
pub struct Args {
    /// Input file(s)
    #[arg(default_value = "-")]
    files: Vec<PathBuf>,

    /// Prints the first LINES
    #[arg(short = 'n', long, default_value_t = 10)]
    lines: usize,

    /// Prints the first BYTES of each files
    #[arg(short = 'c', long, conflicts_with = "lines")]
    bytes: Option<usize>,
}

fn open(path: impl AsRef<Path>) -> io::Result<Box<dyn BufRead>> {
    Ok(match path.as_ref().to_str() {
        Some("-") => Box::new(io::stdin().lock()),
        _ => Box::new(BufReader::new(File::open(path)?)),
    })
}

fn head_lines(mut input: impl BufRead, lines: usize) -> io::Result<()> {
    let mut out = io::stdout().lock();
    let mut count = lines;
    input.for_byte_line_with_terminator(|line| {
        if count == 0 {
            Ok(false)
        } else {
            out.write_all(line)?;
            count -= 1;
            Ok(true)
        }
    })?;
    Ok(())
}

fn head_bytes(mut input: impl BufRead, bytes: usize) -> io::Result<()> {
    let mut buf = vec![0; bytes];
    let bytes = input.read(&mut buf)?;
    io::stdout().lock().write_all(&buf[..bytes])?;
    Ok(())
}

pub fn run(args: Args) {
    let shows_header = args.files.len() > 1;

    for (i, path) in args.files.iter().enumerate() {
        if shows_header {
            if i == 0 {
                println!("==> {} <==", path.display());
            } else {
                println!("\n==> {} <==", path.display());
            }
        }
        match open(path) {
            Err(e) => eprintln!("{}: {}", path.display(), e),
            Ok(input) => {
                let res = if let Some(bytes) = args.bytes {
                    head_bytes(input, bytes)
                } else {
                    head_lines(input, args.lines)
                };
                if let Err(e) = res {
                    eprintln!("{}: {}", path.display(), e);
                }
            }
        }
    }
}
