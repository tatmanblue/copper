//! the tests in this source file should fail.  and that's the point, need to generate failing
//! test data for testing the app
#![allow(unused_attributes)]

#[test]
#[cfg_attr(not(feature = "generate"), ignore)]
fn fail_integration_test_1() {
    assert!(false)
}

#[test]
#[cfg_attr(not(feature = "generate"), ignore)]
fn fail_integration_test_2() {
    assert!(false)
}

// testing the name of tests in integration folders by duplicating this test
// in multiple unit test files
#[test]
fn blah_blah_blah() {
    assert!(true)
}


