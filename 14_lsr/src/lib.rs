use anyhow::{self, Context};
use clap::Parser;
use std::{
    fs,
    os::unix::prelude::*,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(default_value = ".")]
    files: Vec<PathBuf>,

    #[arg(short, long)]
    long: bool,

    #[arg(short, long)]
    all: bool,
}

struct Permissions {
    dir: bool,
    owner: u8,
    group: u8,
    other: u8,
}

impl Permissions {
    fn u8_to_string(num: u8) -> String {
        let r = if num & 4 != 0 { 'r' } else { '-' };
        let w = if num & 2 != 0 { 'w' } else { '-' };
        let x = if num & 1 != 0 { 'x' } else { '-' };
        format!("{}{}{}", r, w, x)
    }
}

impl From<u32> for Permissions {
    fn from(mode: u32) -> Self {
        let dir = (mode & 0o40000) != 0;
        let owner = ((mode >> 6) & 0b111) as u8;
        let group = ((mode >> 3) & 0b111) as u8;
        let other = (mode & 0b111) as u8;
        Self {
            dir,
            owner,
            group,
            other,
        }
    }
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            if self.dir { 'd' } else { '-' },
            Permissions::u8_to_string(self.owner),
            Permissions::u8_to_string(self.group),
            Permissions::u8_to_string(self.other),
        )
    }
}

fn print_path(path: &Path, long: bool) -> Result<(), anyhow::Error> {
    let metadata = fs::metadata(path).with_context(|| path.display().to_string())?;
    if long {
        println!(
            "{} {} {} {} {:-5} {}",
            Permissions::from(metadata.mode()),
            metadata.nlink(),
            metadata.uid(),
            metadata.gid(),
            metadata.len(),
            path.display()
        );
    } else {
        println!("{}", path.display());
    }
    Ok(())
}

fn is_hidden(entry: &fs::DirEntry) -> bool {
    entry
        .path()
        .file_name()
        .is_some_and(|x| x.as_bytes()[0] == b'.')
}

fn ls_dir(dir: &Path, all: bool, long: bool) -> Result<(), anyhow::Error> {
    let entries = fs::read_dir(dir).with_context(|| dir.display().to_string())?;
    for entry in entries {
        let entry = entry.with_context(|| dir.display().to_string())?;
        if !all && is_hidden(&entry) {
            continue;
        }
        print_path(&entry.path(), long)?;
    }
    Ok(())
}

pub fn run(args: Args) -> Result<(), anyhow::Error> {
    for file in args.files {
        if file.is_dir() {
            ls_dir(&file, args.all, args.long)?;
        } else {
            print_path(&file, args.long)?;
        }
    }
    Ok(())
}
