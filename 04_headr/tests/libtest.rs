use headr::parse_positive_int;

#[test]
fn test_parse_positive_int() {
    let result = parse_positive_int("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let result = parse_positive_int("foo");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "foo");

    let result = parse_positive_int("0");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "0");
}
