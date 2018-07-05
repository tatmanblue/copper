//!

use processors::process_for_individual_test_results::ProcessIndividualTestResults;
use processors::types::OrganizedTestResults;

pub struct ProcessingController {
    pub all_test_lines : Vec<String>
}

impl ProcessingController {

    pub fn new(input_lines: Vec<String>) -> ProcessingController {

        return ProcessingController {
            all_test_lines : input_lines
        };
    }

    pub fn execute(&self) -> OrganizedTestResults {
        let summary_lines: Vec<String> = ProcessIndividualTestResults::find_summary_test_lines(&self.all_test_lines);
        let error_details: Vec<String> = ProcessIndividualTestResults::find_error_details_lines(&self.all_test_lines);
        let mut organized_results = ProcessIndividualTestResults::group_test_results(&summary_lines);

        ProcessIndividualTestResults::merge_test_errors_into_results(&error_details, &mut organized_results);

        organized_results.raw_data = self.all_test_lines.to_vec();

        return organized_results;
    }
}