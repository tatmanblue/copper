//!
#![allow(unused_assignments)]

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};

static mut LAST_TEST_ID: i32 = 0;

/**
    a single test result
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualTestResults {
    /**
        "random" id.  random in that the id is generated by incrementing a counter
    */
    pub id : String,
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

/**
    counting on cargo reporting tests uniquely by:
        fully namespaces all tests in mods decorated with #[cfg(test)] attribute in the src folder
        unique names for tests in unit test folder
*/
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
    pub fn new(test_name: &str, test_result: &str) -> IndividualTestResults {

        let mut id: String = str2string!("");

        unsafe {
            LAST_TEST_ID = LAST_TEST_ID + 1;
            id = format!("t{}", LAST_TEST_ID);
        }

        return IndividualTestResults {
            id,
            name : test_name.to_string(),
            result : test_result.to_string(),
            test_details: Vec::new()
        };
    }

    /**
    */
    pub fn append_test_details(&mut self, data : &str) {
        self.test_details.push(data.to_string());
    }
}

#[cfg(test)]
mod individual_test_results_test {
    use super::*;

    #[test]
    fn append_three_lines_to_test_details() {
        let test_one_name: String = str2string!("tests::failing::failing_one");
        let test_result: String = str2string!("");
        let mut test_one: IndividualTestResults = IndividualTestResults::new(&test_one_name, &test_result);
        let appended_line: String = str2string!("line");

        test_one.append_test_details(&appended_line);
        test_one.append_test_details(&appended_line);
        test_one.append_test_details(&appended_line);

        assert_eq!(3, test_one.test_details.len());
    }
}


#[cfg(test)]
mod individual_test_results_test_smelly {

    use super::*;

    /**
        this test is suppose to ensure IndividualTestResults id fields are getting incremented for
        each new entry.   It fails "randomly" when RUST_TEST_THREADS > 1.  That's because other tests
        running can cause a change to the global LAST_TEST_ID.

        We have a code smell here.  Either this test is bad or the production code is bad.

        TODO: need to figure out the right solution.
    */

    #[test]
    fn id_incremented_for_each_new() {

        // must reset LAST_TEST_ID for the test to succeed
        unsafe {
            LAST_TEST_ID = 0;
        }

        let test_one_name: String = str2string!("tests::failing::failing_one");
        let test_result: String = str2string!("");
        let _test_one: IndividualTestResults = IndividualTestResults::new(&test_one_name, &test_result);
        let _test_two: IndividualTestResults = IndividualTestResults::new(&test_one_name, &test_result);
        let test_three: IndividualTestResults = IndividualTestResults::new(&test_one_name, &test_result);

        assert_eq!("t3", test_three.id);

    }
}