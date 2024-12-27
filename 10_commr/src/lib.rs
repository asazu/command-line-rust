use clap::Parser;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use thiserror::Error;

mod iter;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("{0}: {1}")]
    IOError(PathBuf, io::Error),

    #[error("Both input files cannot be STDIN (\"-\")")]
    BothInputStdin,
}

pub type MyResult<T> = Result<T, MyError>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// First input file. When it is "-", read standard input
    file1: PathBuf,

    /// Second input file. When it is "-", read standard input
    file2: PathBuf,

    /// Suppress column 1 (lines unique to FILE1)
    #[arg(short('1'))]
    supresses_first_column: bool,

    /// Suppress column 2 (lines unique to FILE2)
    #[arg(short('2'))]
    supresses_second_column: bool,

    /// Suppress column 3 (lines taht appear in both files)
    #[arg(short('3'))]
    supresses_third_column: bool,

    /// Ignore case when comparing lines
    #[arg(short, long)]
    ignore_case: bool,

    /// Use DELIM for column separator
    #[arg(short, long, name = "DELIM", default_value = "\t")]
    delimiter: String,
}

impl Args {
    pub fn check(self) -> MyResult<Args> {
        match (self.file1.to_str(), self.file2.to_str()) {
            (Some("-"), Some("-")) => Err(MyError::BothInputStdin),
            _ => Ok(self),
        }
    }

    fn column_filter(&self) -> [bool; 3] {
        [
            !self.supresses_first_column,
            !self.supresses_second_column,
            !self.supresses_third_column,
        ]
    }

    fn comparator(&self) -> impl FnMut(&String, &String) -> Ordering + '_ {
        |a: &String, b: &String| {
            if self.ignore_case {
                a.to_uppercase().cmp(&b.to_uppercase())
            } else {
                a.cmp(b)
            }
        }
    }
}

struct Columns<const N: usize>([Option<String>; N]);

impl<const N: usize> Columns<N> {
    fn filter(self, cond: &[bool]) -> Option<Vec<Option<String>>> {
        let filtered = self
            .0
            .into_iter()
            .zip(cond)
            .filter_map(|(c, &b)| if b { Some(c) } else { None })
            .collect::<Vec<_>>();

        if filtered.iter().all(Option::is_none) {
            None
        } else {
            Some(filtered)
        }
    }
}

fn open_reader(path: impl AsRef<Path>) -> MyResult<Box<dyn BufRead>> {
    let path = path.as_ref();
    if path.as_os_str() == "-" {
        Ok(Box::new(io::stdin().lock()))
    } else {
        File::open(path)
            .map_err(|e| MyError::IOError(path.to_owned(), e))
            .map(BufReader::new)
            .map(Box::new)
            .map(|v| v as Box<dyn BufRead>)
    }
}

fn lines(path: &PathBuf) -> MyResult<impl Iterator<Item = MyResult<String>> + '_> {
    open_reader(path).map(|reader| {
        reader
            .lines()
            .map(|v| v.map_err(|e| MyError::IOError(path.clone(), e)))
    })
}

fn comm(
    input1: impl Iterator<Item = MyResult<String>>,
    input2: impl Iterator<Item = MyResult<String>>,
    comparator: impl FnMut(&String, &String) -> Ordering,
) -> impl Iterator<Item = MyResult<Columns<3>>> {
    iter::try_merge_ordered_by(input1, input2, comparator).map(|x| {
        x.map(|(left, right)| match (left, right) {
            (Some(l), None) => Columns([Some(l), None, None]),
            (None, Some(r)) => Columns([None, Some(r), None]),
            (Some(l), Some(_)) => Columns([None, None, Some(l)]),
            (None, None) => unreachable!(),
        })
    })
}

fn format_columns(columns: &[Option<String>], delim: &String) -> String {
    columns
        .iter()
        .map(|c| c.as_ref().unwrap_or(delim).to_owned())
        .collect::<Vec<_>>()
        .concat()
        .trim_end_matches(delim)
        .to_owned()
}

pub fn run(args: Args) -> MyResult<()> {
    let column_filter = args.column_filter();
    let input1 = lines(&args.file1)?;
    let input2 = lines(&args.file2)?;
    for columns in comm(input1, input2, args.comparator()) {
        if let Some(columns) = columns?.filter(&column_filter) {
            println!("{}", format_columns(&columns, &args.delimiter));
        }
    }
    Ok(())
}
