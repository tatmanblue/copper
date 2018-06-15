//!

use processors::types::OrganizedTestResults;

pub trait OutputTrait {
    fn generate(test_results : &OrganizedTestResults);
}