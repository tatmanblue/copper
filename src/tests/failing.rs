

// this test is here to test fail output.  remove ignore attribute to make it fail
// using ignore attribute so the CI passes because I can figure out how to conditionally
// use it
#[test]
#[ignore]
fn failing_one() {
    assert!(false, "this is a failing test");
}

// this test is here to test fail output.  remove ignore attribute to make it fail
// using ignore attribute so the CI passes because I can figure out how to conditionally
// use it
#[test]
#[ignore]
fn failing_two()  {
    assert!(false, "this is a failing test");
}

#[test]
#[ignore]
fn ignore_test() { assert!(false, "this test should not have been run");}