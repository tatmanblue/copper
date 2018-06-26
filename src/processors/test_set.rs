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
        updates test details for a test
    */
    fn update_test_details(&mut self, test_name: &String, test_results: &String);
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

    fn update_test_details(&mut self, test_name: &String, test_details: &String) {
        trace!("update_results => test_name '{}', test_details '{}'", test_name, test_details);
        let test = self.find_by_name(test_name);
        test.append_test_details(&test_details.to_string());

    }
}

#[cfg(test)]
mod test_set_tests {
    use super::*;
    use processors::individual_test_results::{IndividualTestResults};
    use utils::logger::init_log;

    #[test]
    fn find_element_by_name_success() {
        let test_name: String = str2string!("test_one");
        let test_result: String = str2string!("FAILED");
        let mut failed : TestSetCollection = TestSetCollection::new();
        let test = IndividualTestResults::new(&test_name.to_string(), &test_result);

        failed.push(test);

        let index = failed.index_of(&test_name);

        assert_eq!(0, index);
    }

    #[test]
    fn update_test_results_success() {

        init_log();

        let test_name: String = str2string!("test_one");
        let test_result: String = str2string!("FAILED");
        let changed_result: String = str2string!("this is the new stuff");


        let mut failed : TestSetCollection = TestSetCollection::new();
        let original_test = IndividualTestResults::new(&test_name.to_string(), &test_result);
        failed.push(original_test);

        failed.update_test_details(&test_name, &changed_result);

        let updated_test = failed.find_by_name(&test_name.to_string());

        assert_eq!(1, updated_test.test_details.len());
        assert_eq!(changed_result, updated_test.test_details[0]);
        assert_eq!(test_result, updated_test.result);
        assert_eq!(test_name, updated_test.name);
    }
}