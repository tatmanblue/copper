//!

use processors::types::OrganizedTestResults;

/**
    Allows output to generated for different needs.  aka HTML or console or whatever etc...
*/
pub trait OutputTrait {
    fn generate(test_results : &OrganizedTestResults);
}