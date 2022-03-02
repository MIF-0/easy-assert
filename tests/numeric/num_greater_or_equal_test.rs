use assert_that::{Actual, Expected};
use assert_that::simple_num_assertions::NumericAssert;

#[test]
pub fn positive_numbers_greater_success_case() {
    NumericAssert::assert_that(Actual::create_for(2)).is_greater_or_equal().to(Expected::create_for(1));
}

#[test]
pub fn positive_numbers_greater_than_negative() {
    NumericAssert::assert_that(Actual::create_for(2.0)).is_greater_or_equal().to(Expected::create_for(-1.0));
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(Actual::create_for(-3.000001)).is_greater_or_equal().to(Expected::create_for(-3.00002));
}

#[test]
pub fn numbers_are_equals() {
    NumericAssert::assert_that(Actual::create_for(-3.0)).is_greater_or_equal().to(Expected::create_for(-3.0));
}

#[test]
#[should_panic]
pub fn should_panic_when_expected_smaller() {
    NumericAssert::assert_that(Actual::create_for(1)).is_greater_or_equal().to(Expected::create_for(5));
}