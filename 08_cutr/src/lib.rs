use clap::{self, Parser};
use map_ok::MapOk;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use thiserror::Error;

mod range;
use range::{parser, RangeFilterEx, RangeList};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}: {1}")]
    IOError(PathBuf, io::Error),
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(flatten)]
    selector: Selector,

    #[arg(
        long = "delimiter",
        short = 'd',
        value_name = "DELIM",
        default_value_t = '\t',
        conflicts_with_all = ["bytes", "chars"],
    )]
    delimiter: char,

    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<PathBuf>,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct Selector {
    /// select only these bytes
    #[arg(
        short,
        long,
        value_name = "LIST",
        value_parser = parser::parse_range_list
    )]
    bytes: Option<RangeList>,

    /// select only these characters
    #[arg(
        short,
        long = "characters",
        value_name = "LIST",
        value_parser = parser::parse_range_list
    )]
    chars: Option<RangeList>,

    /// select only these fields
    #[arg(
        short,
        long,
        value_name = "LIST",
        value_parser = parser::parse_range_list
    )]
    fields: Option<RangeList>,
}

fn select_bytes(line: &str, ranges: &RangeList) -> Vec<u8> {
    line.as_bytes()
        .iter()
        .range_filter(ranges)
        .copied()
        .collect()
}

fn select_chars(line: &str, ranges: &RangeList) -> Vec<u8> {
    line.chars()
        .range_filter(ranges)
        .collect::<String>()
        .into_bytes()
}

fn select_fields(line: &str, delim: char, ranges: &RangeList) -> Vec<u8> {
    line.split(delim)
        .range_filter(ranges)
        .collect::<Vec<_>>()
        .join(&delim.to_string())
        .into_bytes()
}

fn select(line: &str, selector: &Selector, delimiter: char) -> Vec<u8> {
    match (&selector.bytes, &selector.chars, &selector.fields) {
        (Some(ranges), _, _) => select_bytes(line, ranges),
        (_, Some(ranges), _) => select_chars(line, ranges),
        (_, _, Some(ranges)) => select_fields(line, delimiter, ranges),
        _ => unreachable!(),
    }
}

fn open_file(path: &PathBuf) -> Result<Box<dyn BufRead>, MyError> {
    let reader: Box<dyn BufRead> = if path.to_str() == Some("-") {
        Box::new(io::stdin().lock())
    } else {
        let file = File::open(path).map_err(|e| MyError::IOError(path.to_owned(), e))?;
        Box::new(BufReader::new(file))
    };
    Ok(reader)
}

fn writeln(w: &mut impl Write, buf: &[u8]) -> Result<(), io::Error> {
    w.write_all(buf)?;
    w.write_all("\n".as_bytes())?;
    Ok(())
}

pub fn run(args: Args) -> Result<(), MyError> {
    let mut stdout = io::stdout().lock();
    for path in args.files {
        open_file(&path)?
            .lines()
            .map_ok(|line| select(&line, &args.selector, args.delimiter))
            .try_for_each(|v| writeln(&mut stdout, &v?))
            .map_err(|e| MyError::IOError(path.to_owned(), e))?;
    }
    Ok(())
}
