//!


use serde::{Deserialize, Serialize};

/**
    a single test result
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualTestResults {
    pub name : String,
    pub result : String
}

/**
    Grouping of the test results found into three categories
            success
            failed
            skipped
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct OrganizedTestResults {
    pub success : Vec<IndividualTestResults>,
    pub failed : Vec<IndividualTestResults>,
    pub skipped : Vec<IndividualTestResults>,
    pub total: i32
}

impl OrganizedTestResults {
    /**
        Constructor
    */
    pub fn new() -> OrganizedTestResults {
        let success : Vec<IndividualTestResults> = Vec::new();
        let failed : Vec<IndividualTestResults> = Vec::new();
        let skipped : Vec<IndividualTestResults> = Vec::new();
        return OrganizedTestResults {  success, failed, skipped, total : 0 };
    }
}
