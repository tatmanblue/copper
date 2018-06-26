//!

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};

/**
    a single test result
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualTestResults {
    /**
        test name from stdin (expected from cargo test output).
        generally looks something like: tests::failing::failing_one
    */
    pub name : String,
    /**
        the last part of this string
        test tests::failing::failing_one ... FAILED
        it will be FAILED, ok, ignored

        TODO: make it an enum?
    */
    pub result : String,
    /**
        details from std associated with the test.  in test FAILED this will be the
        output of of the test, such as
        thread 'tests::failing::failing_one' panicked at 'this is a failing test', src/tests/failing.rs:4:5

        it most likely will be empty for passing tests
    */
    pub test_details: Vec<String>
}

impl PartialEq for IndividualTestResults {
    fn eq(&self, other: &IndividualTestResults) -> bool {
        return self.name == other.name;
    }

    fn ne(&self, other: &IndividualTestResults) -> bool {
        return self.name != other.name;
    }
}

impl IndividualTestResults {
    /**
        Constructor
    */
    pub fn new(test_name: &String, test_result: &String) -> IndividualTestResults {
        return IndividualTestResults {
            name : test_name.to_string(),
            result : test_result.to_string(),
            test_details: Vec::new()
        };
    }
}