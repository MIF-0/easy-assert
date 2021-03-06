use easy_assert::string_assertions::StringAssert;
use easy_assert::{actual, expected};

#[test]
pub fn contains_in_the_beginning() {
    let value = String::from("abcde");
    let part = String::from("abc");

    StringAssert::assert_that(actual(value)).contains(expected(part));
}

#[test]
pub fn contains_in_the_middle() {
    let value = String::from("abcde");
    let part = String::from("bcd");

    StringAssert::assert_that(actual(value)).contains(expected(part));
}

#[test]
pub fn contains_in_the_end() {
    let value = String::from("abcde");
    let part = String::from("cde");

    StringAssert::assert_that(actual(value)).contains(expected(part));
}

#[test]
#[should_panic]
pub fn does_not_contain() {
    let value = String::from("abcde");
    let part = String::from("cda");

    StringAssert::assert_that(actual(value)).contains(expected(part));
}
