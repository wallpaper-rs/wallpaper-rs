use super::at_fraction;

#[test]
fn a_position_parses_as_two_numbers_split_on_a_comma() {
    assert_eq!(at_fraction("0.25,0.75").unwrap(), (0.25, 0.75));
    assert_eq!(at_fraction("1,0").unwrap(), (1.0, 0.0));
    assert_eq!(at_fraction(" 0.1 , 0.2 ").unwrap(), (0.1, 0.2));
}

#[test]
fn a_position_without_a_comma_or_with_a_number_that_does_not_parse_is_refused() {
    assert!(at_fraction("0.5").is_err());
    assert!(at_fraction("x,0.5").is_err());
    assert!(at_fraction("0.5,y").is_err());
}
