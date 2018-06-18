//!

use processors::types::OrganizedTestResults;
use shell::factory::ShellTypes;
/**
    Allows output to generated for different needs.  aka HTML or console or whatever etc...
*/
pub trait OutputTrait {
    fn generate(&self, test_results : &OrganizedTestResults) -> ShellTypes;
}