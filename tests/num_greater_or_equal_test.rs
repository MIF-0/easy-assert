use assert_that::numeric::simple_nums::NumericAssert;

#[test]
pub fn positive_numbers_greater_success_case() {
    NumericAssert::assert_that(2).is_greater_or_equal().to(1);
}

#[test]
pub fn positive_numbers_greater_than_negative() {
    NumericAssert::assert_that(2.0).is_greater_or_equal().to(-1.0);
}

#[test]
pub fn negative_numbers_success_case() {
    NumericAssert::assert_that(-3.000001).is_greater_or_equal().to(-3.00002);
}

#[test]
pub fn numbers_are_equals() {
    NumericAssert::assert_that(-3.0).is_greater_or_equal().to(-3.0);
}

#[test]
#[should_panic]
pub fn is_greater_should_panic_when_expected_smaller() {
    NumericAssert::assert_that(1).is_greater_or_equal().to(5);
}