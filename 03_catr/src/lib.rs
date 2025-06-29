use bstr::io::BufReadExt;
use clap::{arg, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Input file(s). With no FILE, or when FILE is -, read standard input
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Number all output lines
    #[arg(short, long)]
    number: bool,

    /// Number nonempty output lines. Cannot use with -n
    #[arg(short = 'b', long, conflicts_with("number"))]
    number_nonblank: bool,
}

struct Cat<'a> {
    shows_line_count: bool,
    shouws_line_count_nonblank: bool,
    line_count: usize,
    line_count_nonblank: usize,
    out: &'a mut dyn Write,
}

impl<'a> Cat<'a> {
    fn new(args: &Args, out: &'a mut dyn Write) -> Self {
        Cat {
            shows_line_count: args.number,
            shouws_line_count_nonblank: args.number_nonblank,
            line_count: 0,
            line_count_nonblank: 0,
            out,
        }
    }

    fn count_line(&mut self, line: &[u8]) {
        self.line_count += 1;
        if line != b"\n" {
            self.line_count_nonblank += 1;
        }
    }

    fn print_line(&mut self, line: &[u8]) -> io::Result<()> {
        if self.shows_line_count{
            write!(self.out, "{:6}\t", self.line_count)?;
        } else if self.shouws_line_count_nonblank && line != b"\n" {
            write!(self.out, "{:6}\t", self.line_count_nonblank)?;
        }
        self.out.write_all(line)
    }

    fn print(&mut self, mut input: &mut dyn BufRead) -> io::Result<()> {
        input.for_byte_line_with_terminator(|line| {
            self.count_line(line);
            self.print_line(line)?;
            Ok(true)
        })
    }
}

fn open(path: impl AsRef<Path>) -> io::Result<Box<dyn BufRead>> {
    Ok(match path.as_ref().to_str() {
        Some("-") => Box::new(io::stdin().lock()),
        _ => Box::new(BufReader::new(File::open(path)?)),
    })
}

pub fn run(args: Args) {
    let mut out = io::stdout().lock();
    let mut cat = Cat::new(&args, &mut out);

    for file in &args.files {
        let res = open(file)
            .and_then(|mut input| cat.print(&mut input));

        if let Err(e) = res {
            eprintln!("{}: {}", file.display(), e);
        }
    }
}
