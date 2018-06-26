//!

use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use processors::individual_test_results::{IndividualTestResults};
use processors::test_set::{TestSetFunctions, TestSetCollection};

enum LineTypes {
    UnitTest,
    IntegrationTest,
    DocTest,
    FailedTestDetails,
    WhoKnows
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

        failed.update_results(&test_name, &changed_result);


        let updated_test = failed.find_by_name(&test_name.to_string());

        assert_eq!(changed_result, updated_test.result);
        assert_eq!(test_name, updated_test.name);
    }
}