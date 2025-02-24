use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailPos {
    Last(usize),
    After(usize),
}

impl FromStr for TailPos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<TailPos, ParseIntError> {
        Ok(match &s[..1] {
            "+" => TailPos::After(s[1..].parse()?),
            "-" => TailPos::Last(s[1..].parse()?),
            _ => TailPos::Last(s.parse()?),
        })
    }
}

#[derive(Parser, Debug)]
#[command(about, version, long_about = None)]
pub struct Args {
    /// Input file to be read
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Output the last NUM lines (default: 10)
    #[arg(short = 'n', long, value_name = "NUM", default_value = "10")]
    lines: TailPos,

    /// Output the last NUM bytes
    #[arg(short = 'c', long, value_name = "NUM", conflicts_with = "lines")]
    bytes: Option<TailPos>,

    // Never output headers giving file names
    #[arg(short, long)]
    quiet: bool,
}

trait SeekableBufRead: BufRead + Seek {}
impl<T: BufRead + Seek> SeekableBufRead for T {}

fn open_reader(path: impl AsRef<Path>) -> io::Result<Box<dyn SeekableBufRead>> {
    File::open(path.as_ref())
        .map(BufReader::new)
        .map(Box::new)
        .map(|v| v as Box<dyn SeekableBufRead>)
}

fn count_lines_and_bytes(input: &mut dyn SeekableBufRead) -> io::Result<(usize, usize)> {
    let mut lines: usize = 0;
    let mut bytes: usize = 0;

    let mut buf = String::new();

    while let Ok(len) = input.read_line(&mut buf) {
        if len == 0 {
            break;
        }
        lines += 1;
        bytes += len
    }
    Ok((lines, bytes))
}

fn skip_lines(input: &mut dyn SeekableBufRead, lines: usize) -> io::Result<()> {
    let mut buf = String::new();
    for _ in 0..lines {
        input.read_line(&mut buf)?;
    }
    Ok(())
}

fn tail_lines(input: &mut dyn SeekableBufRead, lines: TailPos) -> io::Result<()> {
    match lines {
        TailPos::After(lines) => {
            let lines = if lines >= 1 { lines - 1 } else { lines };
            skip_lines(input, lines)?;
        }
        TailPos::Last(lines) => {
            let (total_lines, _) = count_lines_and_bytes(input)?;
            input.rewind()?;

            if total_lines > lines {
                skip_lines(input, total_lines - lines)?;
            }
        }
    }

    let mut buf = String::new();
    while let Ok(len) = input.read_line(&mut buf) {
        if len == 0 {
            break;
        }
        print!("{buf}");
        buf.clear();
    }
    Ok(())
}

fn skip_bytes(input: &mut dyn SeekableBufRead, bytes: usize) -> io::Result<()> {
    input.seek(SeekFrom::Current(bytes as i64))?;
    Ok(())
}

fn tail_bytes(input: &mut dyn SeekableBufRead, bytes: TailPos) -> io::Result<()> {
    match bytes {
        TailPos::After(bytes) => {
            let bytes = if bytes >= 1 { bytes - 1 } else { bytes };
            skip_bytes(input, bytes)?;
        }
        TailPos::Last(bytes) => {
            let (_, total_bytes) = count_lines_and_bytes(input)?;
            input.rewind()?;

            if total_bytes > bytes {
                skip_bytes(input, total_bytes - bytes)?;
            }
        }
    }

    let mut buf = [0u8; 4096];
    while let Ok(len) = input.read(&mut buf) {
        if len == 0 {
            break;
        }
        io::stdout().write_all(&buf[..len])?;
    }
    Ok(())
}

fn tail(input: &mut dyn SeekableBufRead, args: &Args) -> io::Result<()> {
    if let Some(bytes) = args.bytes {
        tail_bytes(input, bytes)
    } else {
        tail_lines(input, args.lines)
    }
}

fn print_header(path: &Path, pos: usize) {
    if pos > 0 {
        println!();
    }
    println!("==> {} <==", path.to_string_lossy());
}

pub fn run(args: Args) {
    let should_print_header = !args.quiet && args.files.len() > 1;
    for (i, path) in args.files.iter().enumerate() {
        if should_print_header {
            print_header(path, i);
        }
        if let Err(e) = open_reader(path).and_then(|mut input| tail(input.as_mut(), &args)) {
            eprintln!("{}: {}", path.to_string_lossy(), e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_parse_position_no_sign() {
        assert_eq!(TailPos::from_str("123"), Ok(TailPos::Last(123)))
    }

    #[test]
    fn test_parse_position_minus() {
        assert_eq!(TailPos::from_str("-123"), Ok(TailPos::Last(123)))
    }

    #[test]
    fn test_parse_position_plus() {
        assert_eq!(TailPos::from_str("+123"), Ok(TailPos::After(123)))
    }

    #[test]
    fn test_count_lines_and_bytes_empty() -> io::Result<()> {
        let mut input = Cursor::new("");
        assert_eq!(count_lines_and_bytes(&mut input)?, (0, 0));
        Ok(())
    }

    #[test]
    fn test_count_lines_and_bytes() -> io::Result<()> {
        let mut input = Cursor::new("1\n12\n123");
        assert_eq!(count_lines_and_bytes(&mut input)?, (3, 8));
        Ok(())
    }

    #[test]
    fn test_skip_lines() -> io::Result<()> {
        let mut input = Cursor::new("1\n12\n123");
        skip_lines(&mut input, 2)?;
        assert_eq!(io::read_to_string(input)?, "123");
        Ok(())
    }
}
