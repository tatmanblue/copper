//!

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};

/**
    a single test result
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualTestResults {
    pub name : String,
    pub result : String
}

impl PartialEq for IndividualTestResults {
    fn eq(&self, other: &IndividualTestResults) -> bool {
        return self.name == other.name;
    }

    fn ne(&self, other: &IndividualTestResults) -> bool {
        return self.name != other.name;
    }
}

/**
    Collection of IndividualTestResults
*/
pub type TestSetCollection = Vec<IndividualTestResults>;

/**
    functions for TestSet
*/
pub trait TestSetFunctions {
    fn new() -> TestSetCollection;
    fn find_by_name(&mut self, test_name : &String) -> &mut IndividualTestResults;
    fn index_of(&self, test_name: &String) -> usize;
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

/**
    Grouping of the test results found into three categories
            success
            failed
            skipped
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizedTestResults {
    pub success : TestSetCollection,
    pub failed : TestSetCollection,
    pub skipped : TestSetCollection,
    pub total: i32
}

impl OrganizedTestResults {
    /**
        Constructor
    */
    pub fn new() -> OrganizedTestResults {
        let success : TestSetCollection = TestSetCollection::new();
        let failed : TestSetCollection = TestSetCollection::new();
        let skipped : TestSetCollection = TestSetCollection::new();
        return OrganizedTestResults {  success, failed, skipped, total : 0 };
    }
}


#[cfg(test)]
mod types_test {

    use super::*;
    use utils::logger::init_log;

    #[test]
    fn find_element_by_name_success() {
        let test_name: String = str2string!("test_one");
        let mut failed : TestSetCollection = TestSetCollection::new();
        let test = IndividualTestResults { name : test_name.to_string(), result : str2string!("")};
        failed.push(test);

        let index = failed.index_of(&test_name);

        assert_eq!(0, index);
    }

    #[test]
    fn update_test_results_success() {

        init_log();

        let test_name: String = str2string!("test_one");
        let changed_result: String = str2string!("this is the new stuff");


        let mut failed : TestSetCollection = TestSetCollection::new();
        let original_test : IndividualTestResults = IndividualTestResults { name : test_name.to_string(), result : str2string!("")};
        failed.push(original_test);

        failed.update_results(&test_name, &changed_result);


        let updated_test = failed.find_by_name(&test_name.to_string());

        assert_eq!(changed_result, updated_test.result);
        assert_eq!(test_name, updated_test.name);
    }
}