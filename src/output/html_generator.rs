//!  http://siciarz.net/24-days-rust-tera/

use output::output_trait::OutputTrait;
use processors::types::OrganizedTestResults;

/**
*/
pub struct HtmlOutput {

}

impl OutputTrait for HtmlOutput {
    fn generate(test_results : &OrganizedTestResults) {
        unimplemented!()
    }
}