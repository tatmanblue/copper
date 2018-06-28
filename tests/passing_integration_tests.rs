//!
#![allow(unused_attributes)]

#[test]
#[cfg_attr(not(feature = "generate"), ignore)]
fn success_integration_test_1 () {
    assert!(true)
}

#[test]
#[cfg_attr(not(feature = "generate"), ignore)]
fn success_integration_test_2 () {
    assert!(true)
}

#[test]
#[cfg_attr(not(feature = "generate"), ignore)]
fn success_integration_test_3 () {
    assert!(true)
}