use clap::Parser;
use itertools::Itertools;
use regex::{self, Regex, RegexBuilder};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;
use walkdir::{self, WalkDir};

mod iter;
use iter::LinesNL;

#[derive(Error, Debug)]
#[error("{0}: {1}")]
pub struct IOError(PathBuf, io::Error);

impl From<walkdir::Error> for IOError {
    fn from(value: walkdir::Error) -> Self {
        let path = value.path().map(ToOwned::to_owned).unwrap_or_default();
        IOError(path, value.into())
    }
}

#[derive(Error, Debug)]
#[error("some errors")]
pub struct MyError();

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Pattern in regular expression.
    pattern: String,

    /// Files to be searched for PATTERNS. "-" stands for standard input.
    /// If omitted, recursive searches examin the working directory,
    /// and non-recursive searches read standard input.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Ignore case distinction in patterns and input data.
    #[arg(short, long)]
    ignore_case: bool,

    /// Read all files under each directory, recursively.
    #[arg(short, long)]
    recursive: bool,

    /// Invert the sense of matching, to select non-matchin lines.
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Suppress normal output; instead print a count of matching lines
    /// for each FILEs.
    #[arg(short, long)]
    count: bool,
}

impl Args {
    pub fn set_default_paths_if_empty(mut self) -> Self {
        if self.files.is_empty() {
            self.files = if self.recursive {
                vec![".".into()]
            } else {
                vec!["-".into()]
            };
        }
        self
    }

    fn build_regex(&self) -> Result<Regex, regex::Error> {
        RegexBuilder::new(&self.pattern)
            .case_insensitive(self.ignore_case)
            .build()
    }
}

fn open<T: AsRef<Path>>(path: T) -> Result<Box<dyn BufRead>, IOError> {
    let path = path.as_ref();
    match path.to_str() {
        Some("-") => Ok(Box::new(io::stdin().lock())),
        _ => {
            let file = File::open(path).map_err(|e| IOError(path.to_owned(), e))?;
            Ok(Box::new(BufReader::new(file)))
        }
    }
}

fn print_line<I>(mut iter: I, path: &Path, should_print_path: bool) -> io::Result<()>
where
    I: Iterator<Item = io::Result<String>>,
{
    iter.try_for_each(|line| {
        let line = line?;
        if should_print_path {
            print!("{}:", path.to_string_lossy());
        }
        // A new-line code is expected to be included in `line`.
        print!("{}", line);
        Ok(())
    })
}

fn print_count<I>(mut iter: I, path: &Path, should_print_path: bool) -> io::Result<()>
where
    I: Iterator<Item = io::Result<String>>,
{
    let count = iter.try_fold(0, |acc, _| Ok::<usize, io::Error>(acc + 1))?;
    if should_print_path {
        print!("{}:", path.to_string_lossy());
    }
    println!("{}", count);
    Ok(())
}

fn grep(re: &Regex, path: &Path, should_print_path: bool, count: bool) -> Result<(), IOError> {
    let iter = open(path)?.lines_nl().filter_ok(|line| re.is_match(line));
    if count {
        print_count(iter, path, should_print_path)
    } else {
        print_line(iter, path, should_print_path)
    }
    .map_err(|e| IOError(path.to_owned(), e))?;
    Ok(())
}

fn grep_recursive(re: &Regex, path: &Path, count: bool) -> Result<(), IOError> {
    WalkDir::new(path)
        .into_iter()
        .filter_ok(|entry| entry.file_type().is_file())
        .try_for_each(|entry| {
            grep(re, entry?.path(), true, count)?;
            Ok(())
        })
}

pub fn run(args: Args) -> Result<(), MyError> {
    let should_print_path = args.files.len() > 1;
    let re = args.build_regex().map_err(|e| {
        eprintln!("{}", e);
        MyError()
    })?;

    let mut has_error = false;
    for path in &args.files {
        let result = if args.recursive {
            grep_recursive(&re, path, args.count)
        } else {
            grep(&re, path, should_print_path, args.count)
        };
        if let Err(e) = result {
            eprintln!("{}", e);
            has_error = true;
        }
    }
    if has_error {
        Err(MyError())
    } else {
        Ok(())
    }
}
