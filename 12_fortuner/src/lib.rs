use anyhow::{anyhow, bail};
use clap::Parser;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use regex::{Regex, RegexBuilder};
use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(name = "FILE", required = true, num_args = 1..)]
    files: Vec<PathBuf>,

    #[arg(short = 'm', long)]
    pattern: Option<String>,

    #[arg(short, long, requires = "pattern", default_value_t = false)]
    insensitive: bool,

    #[arg(short, long)]
    seed: Option<u64>,
}

impl Args {
    fn build_regex(&self) -> Result<Option<Regex>, regex::Error> {
        self.pattern
            .as_ref()
            .map(|pattern| {
                RegexBuilder::new(pattern)
                    .case_insensitive(self.insensitive)
                    .build()
            })
            .transpose()
    }
}

fn read_u32(input: &mut impl Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

fn read_u8(input: &mut impl Read) -> io::Result<u8> {
    let mut buf = [0u8; 4];
    input.read_exact(&mut buf)?;
    Ok(buf[0])
}

#[allow(dead_code)]
#[derive(Debug)]
struct RandomAccessFile {
    version: u32,
    numstr: u32,
    longlen: u32,
    shortlen: u32,
    flags: u32,
    delim: u8,
    offsets: Vec<u64>,
}

#[derive(Debug)]
struct RafEntry {
    path: Rc<PathBuf>,
    offset: u64,
    delim: u8,
}

impl RandomAccessFile {
    fn load_from(mut input: impl Read) -> io::Result<RandomAccessFile> {
        let version = read_u32(&mut input)?;
        let numstr = read_u32(&mut input)?;
        let longlen = read_u32(&mut input)?;
        let shortlen = read_u32(&mut input)?;
        let flags = read_u32(&mut input)?;
        let delim = read_u8(&mut input)?;

        let mut offsets = Vec::with_capacity(numstr as usize);
        for _ in 0..numstr {
            offsets.push(read_u32(&mut input)? as u64)
        }

        Ok(RandomAccessFile {
            version,
            numstr,
            longlen,
            shortlen,
            flags,
            delim,
            offsets,
        })
    }

    fn into_entries(self, path: impl AsRef<Path>) -> Vec<RafEntry> {
        let path = Rc::new(path.as_ref().to_path_buf());
        self.offsets
            .into_iter()
            .map(|offset| RafEntry {
                path: path.clone(),
                offset,
                delim: self.delim,
            })
            .collect()
    }
}

fn create_rng(seed: Option<u64>) -> SmallRng {
    match seed {
        Some(s) => SmallRng::seed_from_u64(s),
        None => SmallRng::from_os_rng(),
    }
}

fn warn(e: &io::Error, path: impl AsRef<Path>) {
    eprintln!("[warn] {}: {}", path.as_ref().to_string_lossy(), e);
}

fn is_dat_file(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();
    path.is_file() && path.extension().is_some_and(|ext| ext == "dat")
}

fn is_dir_or_dat_file(dir_entry: &DirEntry) -> bool {
    dir_entry
        .file_type()
        .inspect_err(|e| warn(e, dir_entry.path()))
        .is_ok_and(|file_type| file_type.is_dir() || is_dat_file(dir_entry.path()))
}

fn load_index_dir(path: &Path) -> Result<Vec<RafEntry>, io::Error> {
    assert!(path.is_dir());
    fs::read_dir(path)
        .inspect_err(|e| warn(e, path))?
        .filter_map(|x| x.inspect_err(|e| warn(e, path)).ok())
        .filter(is_dir_or_dat_file)
        .map(|child| load_index(&child.path()))
        .try_fold(Vec::new(), |mut acc, x| {
            acc.append(x?.as_mut());
            Ok(acc)
        })
}

fn load_index_file(path: &Path) -> Result<Vec<RafEntry>, io::Error> {
    assert!(path.is_file());
    let tgt_file = path.with_extension("");
    let dat_file = path.with_extension("dat");
    File::open(&dat_file)
        .and_then(RandomAccessFile::load_from)
        .map(|dat| dat.into_entries(tgt_file))
        .inspect_err(|e| warn(e, path))
}

fn load_index(path: &Path) -> io::Result<Vec<RafEntry>> {
    let metadata = fs::metadata(path).inspect_err(|e| warn(e, path))?;

    if metadata.is_dir() {
        load_index_dir(path)
    } else if metadata.is_file() {
        load_index_file(path)
    } else {
        Ok(Vec::new())
    }
}

fn load_indices(pathes: &[impl AsRef<Path>]) -> io::Result<Vec<RafEntry>> {
    pathes
        .iter()
        .map(AsRef::as_ref)
        .map(load_index)
        .try_fold(Vec::new(), |mut acc, x| {
            acc.append(x?.as_mut());
            Ok(acc)
        })
}

fn read_lines_until(reader: impl BufRead, delim: &str) -> io::Result<String> {
    reader
        .lines()
        .take_while(|x| x.as_ref().is_ok_and(|line| line != delim))
        .try_fold(String::new(), |acc, x| Ok(format!("{}{}\n", acc, x?)))
}

fn read_entry(entry: &RafEntry) -> anyhow::Result<String> {
    let delim = char::from_u32(entry.delim as u32)
        .unwrap_or('%')
        .to_string();

    File::open(entry.path.deref())
        .map(BufReader::new)
        .and_then(|mut reader| reader.seek(SeekFrom::Start(entry.offset)).map(|_| reader))
        .and_then(|reader| read_lines_until(reader, &delim))
        .map_err(|e| anyhow!("{}: {}", &entry.path.to_string_lossy(), e))
}

fn pick_fortune(index: Vec<RafEntry>, mut rng: impl Rng) -> anyhow::Result<()> {
    if index.is_empty() {
        bail!("No fortunes found");
    }
    let adage = read_entry(&index[rng.random_range(0..index.len())])?;
    print!("{}", adage);
    Ok(())
}

fn search_adages(index: Vec<RafEntry>, pattern: &Regex) -> anyhow::Result<()> {
    let mut cur_file = Rc::new(PathBuf::new());

    for entry in index {
        let adage = read_entry(&entry)?;
        if pattern.is_match(&adage) {
            if entry.path != cur_file {
                cur_file = entry.path.clone();
                eprintln!("({})\n%", cur_file.file_name().unwrap().to_string_lossy())
            }
            println!("{}%", adage)
        }
    }
    Ok(())
}

pub fn run(args: Args) -> anyhow::Result<()> {
    let index = load_indices(&args.files)?;
    match args
        .build_regex()
        .map_err(|e| anyhow!("Invalid --pattern: {e}"))?
    {
        Some(regex) => {
            search_adages(index, &regex)?;
        }
        None => {
            let rng = create_rng(args.seed);
            pick_fortune(index, rng)?;
        }
    }
    Ok(())
}
