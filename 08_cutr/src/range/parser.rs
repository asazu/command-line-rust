use super::{Range, RangeList};
use std::num::IntErrorKind;
use std::num::NonZeroUsize;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("source string is empty")]
    EmptyField,

    #[error("invalid field value '{0}'")]
    InvalidFieldValue(String),

    #[error("invalid range with no endpoint: '{0}'")]
    InvalidRangeWithNoEndpoint(String),

    #[error("decreasing range: '{0}'")]
    DecreasingRange(String),
}

fn parse_field(s: &str) -> Result<NonZeroUsize, ParseError> {
    if s.starts_with('+') {
        return Err(ParseError::InvalidFieldValue(s.into()));
    }
    s.parse::<NonZeroUsize>().map_err(|e| match e.kind() {
        IntErrorKind::Empty => ParseError::EmptyField,
        _ => ParseError::InvalidFieldValue(s.into()),
    })
}

fn parse_range(s: &str) -> Result<Range, ParseError> {
    match s.split_once('-') {
        Some(("", "")) => Err(ParseError::InvalidRangeWithNoEndpoint(s.into())),
        Some((start, "")) => {
            let start = parse_field(start)?;
            Ok(Range::From(start))
        }
        Some(("", end)) => {
            let end = parse_field(end)?;
            Ok(Range::To(end))
        }
        Some((start, end)) => {
            let start = parse_field(start)?;
            let end = parse_field(end)?;
            if start <= end {
                Ok(Range::Between(start, end))
            } else {
                Err(ParseError::DecreasingRange(s.into()))
            }
        }
        None => {
            let field = parse_field(s)?;
            Ok(Range::Between(field, field))
        }
    }
}

pub fn parse_range_list(s: &str) -> Result<RangeList, ParseError> {
    let separators = [',', ' '];
    let ranges = s
        .split(separators)
        .map(parse_range)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RangeList::from(ranges))
}

#[cfg(test)]
mod tests {
    use super::super::{range_between, range_from, range_to};
    use super::*;

    #[test]
    fn test_parse_field() {
        assert_eq!(parse_field("123").unwrap().get(), 123);
    }

    #[test]
    fn test_parse_field_empty_string() {
        assert_eq!(parse_field(""), Err(ParseError::EmptyField));
    }

    fn check_parse_field_invalid_field_value(field: &str) {
        assert_eq!(
            parse_field(field),
            Err(ParseError::InvalidFieldValue(field.into()))
        )
    }

    #[test]
    fn test_parse_field_invalid_field_value() {
        check_parse_field_invalid_field_value("abc");
    }

    #[test]
    fn test_parse_field_with_plus_sign() {
        check_parse_field_invalid_field_value("+123");
    }

    #[test]
    fn test_parse_field_zero() {
        check_parse_field_invalid_field_value("0");
    }

    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("12-34"), Ok(range_between(12, 34)));
    }

    #[test]
    fn test_parse_range_open_end() {
        assert_eq!(parse_range("123-"), Ok(range_from(123)));
    }

    #[test]
    fn test_parse_range_open_start() {
        assert_eq!(parse_range("-123"), Ok(range_to(123)));
    }

    #[test]
    fn test_parse_range_single_value() {
        assert_eq!(parse_range("123"), Ok(range_between(123, 123)));
    }

    #[test]
    fn test_parse_range_not_number() {
        assert_eq!(
            parse_range("abc"),
            Err(ParseError::InvalidFieldValue("abc".into()))
        );
    }

    #[test]
    fn test_parse_range_only_minus_symbol() {
        assert_eq!(
            parse_range("-"),
            Err(ParseError::InvalidRangeWithNoEndpoint("-".into()))
        );
    }

    #[test]
    fn test_parse_range_two_minus_symbols() {
        assert_eq!(
            parse_range("12-34-56"),
            Err(ParseError::InvalidFieldValue("34-56".into()))
        );
    }

    #[test]
    fn test_parse_range_decreasing_range() {
        assert_eq!(
            parse_range("34-12"),
            Err(ParseError::DecreasingRange("34-12".into()))
        )
    }

    fn assert_eq_range_list(actual: &RangeList, expected: &[Range]) {
        let actual = actual.iter().copied().collect::<Vec<_>>();
        assert_eq!(&actual, expected)
    }

    #[test]
    fn test_parse_range_list_single_range() {
        let input = "12-34";
        let expected = [range_between(12, 34)];
        let actual = parse_range_list(input).unwrap();
        assert_eq_range_list(&actual, &expected);
    }

    #[test]
    fn test_parse_range_list_multiple_ranges() {
        let input = "12-34,56,78-90";
        let expected = [
            range_between(12, 34),
            range_between(56, 56),
            range_between(78, 90),
        ];
        let actual = parse_range_list(input).unwrap();
        assert_eq_range_list(&actual, &expected);
    }

    #[test]
    fn test_parse_range_list_multiple_ranges_space_separated() {
        let input = "12-34 56 78-90";
        let expected = [
            range_between(12, 34),
            range_between(56, 56),
            range_between(78, 90),
        ];
        let actual = parse_range_list(input).unwrap();
        assert_eq_range_list(&actual, &expected);
    }

    #[test]
    fn test_parse_range_list_empty_string() {
        assert_eq!(parse_range_list(""), Err(ParseError::EmptyField));
    }
}
