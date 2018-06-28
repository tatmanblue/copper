//!
#![allow(unused_attributes)]

#[test]
#[ignore]
#[cfg_attr(not(feature = "generate"), ignore)]
fn ignore_integration_test_1 () {
    assert!(false)
}