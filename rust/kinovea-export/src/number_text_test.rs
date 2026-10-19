use super::to_text;

#[test]
fn integers_have_no_decimal_part() {
    assert_eq!(to_text(-181.0, '.'), "-181");
    assert_eq!(to_text(0.0, '.'), "0");
    assert_eq!(to_text(1234567.0, '.'), "1234567");
}

#[test]
fn keeps_seven_significant_digits() {
    assert_eq!(to_text(1.0 / 3.0, '.'), "0.3333333");
    assert_eq!(to_text(2.0 / 3.0, '.'), "0.6666667");
    assert_eq!(to_text(0.04, '.'), "0.04");
    assert_eq!(to_text(-0.04, '.'), "-0.04");
}

#[test]
fn rounding_into_the_next_decade_changes_the_notation() {
    assert_eq!(to_text(9999999.7, '.'), "1E+07");
}

#[test]
fn small_magnitudes_switch_to_scientific_below_1e_minus_4() {
    assert_eq!(to_text(0.0001, '.'), "0.0001");
    assert_eq!(to_text(0.00001, '.'), "1E-05");
    assert_eq!(to_text(1.5e-7, '.'), "1.5E-07");
}

#[test]
fn large_magnitudes_switch_to_scientific_from_1e7() {
    assert_eq!(to_text(1234567.0, '.'), "1234567");
    assert_eq!(to_text(12345678.0, '.'), "1.234568E+07");
}

#[test]
fn decimal_separator_is_configurable() {
    assert_eq!(to_text(0.04, ','), "0,04");
    assert_eq!(to_text(1.5e-7, ','), "1,5E-07");
}

#[test]
fn non_finite_values_use_invariant_symbols() {
    assert_eq!(to_text(f32::NAN, '.'), "NaN");
    assert_eq!(to_text(f32::INFINITY, '.'), "Infinity");
    assert_eq!(to_text(f32::NEG_INFINITY, '.'), "-Infinity");
}
