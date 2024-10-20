use std::io::{self, BufRead};

pub struct Lines<B> {
    buf: B,
}

impl<B: BufRead> Iterator for Lines<B> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait LinesNL {
    fn lines_nl(self) -> Lines<Self>
    where
        Self: Sized;
}

impl<B: BufRead> LinesNL for B {
    fn lines_nl(self) -> Lines<Self> {
        Lines { buf: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_ns() {
        let input = std::io::Cursor::new(
            "\
            foo\n\
            bar\r\n\
            foo bar\
        ",
        );
        let expected = vec!["foo\n", "bar\r\n", "foo bar"];
        let actual = input.lines_nl().map(|v| v.unwrap()).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
