use super::Count;
use super::File;
use super::MyResult;
use std::fs;
use std::io::{stdin, BufRead, BufReader};

pub fn open(file: &File) -> MyResult<Box<dyn BufRead>> {
    let result: Box<dyn BufRead> = match file {
        File::Default | File::StdIn => Box::new(BufReader::new(stdin())),
        File::Path(path) => Box::new(BufReader::new(fs::File::open(path)?)),
    };
    Ok(result)
}

pub fn word_count(mut input: impl BufRead) -> MyResult<Count> {
    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut chars: usize = 0;
    let mut bytes: usize = 0;

    let mut line = String::new();
    // let mut reader = open(file)?;

    while input.read_line(&mut line)? > 0 {
        if line.ends_with('\n') {
            lines += 1
        }
        words += line.split_whitespace().count();
        chars += line.chars().count();
        bytes += line.len();
        line.clear();
    }

    Ok(Count {
        lines,
        words,
        chars,
        bytes,
    })
}

#[cfg(test)]
mod test {
    use super::word_count;
    use super::Count;
    use std::io::Cursor;

    #[test]
    fn test_word_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let input = Cursor::new(text);

        let result = word_count(input);

        match result {
            Err(err) => assert!(false, "failed with error: {}", err),
            Ok(actual) => {
                let expected = Count {
                    lines: 1,
                    words: 10,
                    chars: 48,
                    bytes: 48,
                };
                assert_eq!(expected, actual);
            }
        }
    }
}
