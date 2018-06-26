//!

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use processors::individual_test_results::{IndividualTestResults};

/**
    Collection of IndividualTestResults
*/
pub type TestSetCollection = Vec<IndividualTestResults>;

/**
    functions for TestSet
*/
pub trait TestSetFunctions {
    /**
        constructor
    */
    fn new() -> TestSetCollection;
    /**
        find/return IndividualTestResults based on test name
    */
    fn find_by_name(&mut self, test_name : &String) -> &mut IndividualTestResults;
    /**
        returns index of test by name, throw error if its not found
    */
    fn index_of(&self, test_name: &String) -> usize;
    /**
        updates test results for a test
    */
    fn update_results(&mut self, test_name: &String, test_results: &String);
}

impl TestSetFunctions for TestSetCollection {
    fn new() -> TestSetCollection {
        return Vec::new();
    }

    fn find_by_name(&mut self, test_name : &String) -> &mut IndividualTestResults {
        for test in self {
            if test_name.to_string() == test.name {
                return test;
            }
        }

        panic!(format!("'{}' not found for find_by_name().", test_name))
    }

    fn index_of(&self, test_name: &String) -> usize {

        for index in 0..self.len() {
            if test_name.to_string() == self[index].name {
                return index;
            }
        }

        panic!(format!("'{}' not found for index_of().", test_name))
    }

    fn update_results(&mut self, test_name: &String, test_results: &String) {
        trace!("update_results => test_name '{}', test_results '{}'", test_name, test_results);
        let test = self.find_by_name(test_name);
        test.result = format!("{}.  {}", test.result, test_results.to_string());
    }
}
