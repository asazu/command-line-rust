use clap::{builder::PossibleValue, Parser, ValueEnum};
use regex::Regex;
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{path}: {source}")]
pub struct MyError {
    path: String,
    source: io::Error,
}

impl MyError {
    fn new(path: impl AsRef<Path>, source: io::Error) -> MyError {
        let path = path.as_ref().to_string_lossy().into_owned();
        MyError { path, source }
    }
}

pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Dir,
    File,
    Link,
    Other,
}

impl From<&Path> for Type {
    fn from(value: &Path) -> Type {
        if value.is_dir() {
            Type::Dir
        } else if value.is_symlink() {
            Type::Link
        } else if value.is_file() {
            Type::File
        } else {
            Type::Other
        }
    }
}

impl ValueEnum for Type {
    fn value_variants<'a>() -> &'a [Self] {
        &[Type::Dir, Type::File, Type::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Type::Dir => PossibleValue::new("d").help("directory"),
            Type::File => PossibleValue::new("f").help("regular file"),
            Type::Link => PossibleValue::new("l").help("simbolic link"),
            _ => panic!("unsupported type"),
        })
    }
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Search path(s)
    #[arg(value_name = "PATH")]
    pathes: Vec<String>,

    /// Name(s)
    #[arg(long = "name", short = 'n', value_name = "NAME")]
    names: Vec<Regex>,

    /// Type(s)
    #[arg(long = "type", short = 't', value_name = "TYPE", num_args(0..))]
    types: Vec<Type>,
}

pub struct PathIter {
    /// Errors encountered when traverse the file tree. If it's not empty, these errors
    /// should be reported at first.
    errors: VecDeque<MyResult<PathBuf>>,

    to_be_visit: Vec<PathBuf>,
}

impl PathIter {
    fn push_err(&mut self, path: &Path, err: io::Error) {
        let err = MyError::new(path, err);
        self.errors.push_back(Err(err));
    }

    fn read_dir(&mut self, path: &Path) -> Vec<PathBuf> {
        match fs::read_dir(path) {
            Err(e) => {
                self.push_err(path, e);
                Vec::new()
            }
            Ok(iter) => iter
                .filter_map(|x| match x {
                    Err(e) => {
                        self.push_err(path, e);
                        None
                    }
                    Ok(v) => Some(v.path()),
                })
                .collect(),
        }
    }
}

impl<T> From<T> for PathIter
where
    T: AsRef<Path>,
{
    fn from(value: T) -> PathIter {
        PathIter {
            errors: VecDeque::new(),
            to_be_visit: vec![value.as_ref().into()],
        }
    }
}

impl Iterator for PathIter {
    type Item = MyResult<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.errors.pop_front() {
            return Some(next);
        }
        self.to_be_visit.pop().map(|path| {
            let metadata = path.metadata().map_err(|e| MyError::new(&path, e))?;
            if metadata.is_dir() {
                let children = self.read_dir(&path);
                self.to_be_visit.extend(children.into_iter().rev());
            }
            Ok(path)
        })
    }
}

fn is_match_names_or_err(value: &MyResult<PathBuf>, names: &[Regex]) -> bool {
    if names.is_empty() {
        return true;
    }
    match value {
        Err(_) => true,
        Ok(path) => path
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|file_name| names.iter().any(|re| re.is_match(file_name))),
    }
}

fn is_match_types_or_err(value: &MyResult<PathBuf>, types: &[Type]) -> bool {
    if types.is_empty() {
        return true;
    }
    match value {
        Err(_) => true,
        Ok(path) => types.contains(&Type::from(path.as_ref())),
    }
}

pub fn run(args: Args) {
    for path in &args.pathes {
        let iter = PathIter::from(path)
            .filter(|x| is_match_names_or_err(x, &args.names))
            .filter(|x| is_match_types_or_err(x, &args.types));

        iter.for_each(|x| match x {
            Err(e) => eprintln!("{}", e),
            Ok(path) => println!("{}", path.display()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_file() -> MyResult<()> {
        let path = "tests/inputs/a/a.txt";
        let result = PathIter::from(path)
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();
        assert_eq!(result, vec![PathBuf::from(path)]);
        Ok(())
    }

    #[test]
    fn one_depth_directory() -> MyResult<()> {
        let path = "tests/inputs/f";

        let expected = vec![
            PathBuf::from("tests/inputs/f"),
            PathBuf::from("tests/inputs/f/f.txt"),
        ];

        let actual = PathIter::from(path)
            .filter_map(|x| x.ok())
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
        Ok(())
    }
}
