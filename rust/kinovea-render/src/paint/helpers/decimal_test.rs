//! `String.Format("{0}", Math.Round(value, 2))` as .NET Framework prints it.

use super::decimal::format_value;

#[test]
fn whole_and_negative_values_print_without_decimals() {
    assert_eq!(format_value(2.0), "2");
    assert_eq!(format_value(100.0), "100");
    assert_eq!(format_value(-1.5), "-1.5");
}

#[test]
fn negative_zero_prints_as_zero() {
    assert_eq!(format_value(-0.0), "0");
}

#[test]
fn values_round_to_two_decimals() {
    assert_eq!(format_value(1.2345), "1.23");
}

#[test]
fn ties_go_to_the_even_digit() {
    // 12.5 -> 12.
    assert_eq!(format_value(0.125), "0.12");
}
