use std::fs::File;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{path}: {source}")]
pub struct IOError {
    path: String,

    #[source]
    source: std::io::Error,
}

impl IOError {
    pub fn new(path: &str, source: std::io::Error) -> IOError {
        let path = path.to_owned();
        IOError { path, source }
    }
}

pub fn do_io_task<'a, T, F>(path: &'a str, io_task: F) -> Result<T, IOError>
where
    F: FnOnce(&'a str) -> Result<T, std::io::Error>,
{
    io_task(path).map_err(|e| IOError::new(path, e))
}

pub fn open_file(path: &str) -> Result<File, IOError> {
    do_io_task(path, File::open)
}

pub fn create_file(path: &str) -> Result<File, IOError> {
    do_io_task(path, File::create)
}
