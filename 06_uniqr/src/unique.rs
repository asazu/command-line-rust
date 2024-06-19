use super::iter::Unique;
use std::io::{BufRead, Error as IOError};

pub fn unique(input: Box<dyn BufRead>) -> impl Iterator<Item = Result<(usize, String), IOError>> {
    input
        .lines()
        .uniq_by(|x, y| x.as_ref().ok() == y.as_ref().ok())
        .map(|x| match x {
            (_, Err(e)) => Err(e),
            (count, Ok(v)) => Ok((count, v)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Error as IOError};

    type TestResult = Result<(), IOError>;

    fn line(count: usize, text: &str) -> (usize, String) {
        (count, text.to_owned())
    }

    fn test_unique(input: &'static str, expected: &[(usize, String)]) -> TestResult {
        let input = Box::new(Cursor::new(input));
        let actual: Vec<_> = unique(input).filter_map(|x| x.ok()).collect();

        assert!(
            actual.len() == expected.len() && actual.iter().zip(expected).all(|(x, y)| x == y),
            "\nexpected: {:?}\n  actual: {:?}",
            expected,
            actual
        );
        Ok(())
    }

    #[test]
    fn test_unique_empty() -> TestResult {
        test_unique("", &vec![])
    }

    #[test]
    fn test_unique_a() -> TestResult {
        test_unique("a\n", &vec![line(1, "a")])
    }

    #[test]
    fn test_unique_aa() -> TestResult {
        test_unique("a\na\n", &vec![line(2, "a")])
    }

    #[test]
    fn test_unique_aab() -> TestResult {
        test_unique("a\na\nb\n", &vec![line(2, "a"), line(1, "b")])
    }
}
