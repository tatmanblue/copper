
#[test]
fn failing_one() {
    assert!(false, "this is a failing test");
}

#[test]
fn failing_two()  {
    assert!(false, "this is a failing test");
}

#[test]
#[ignore]
fn ignore_test() { assert!(false, "this test should not have been run");}