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


}