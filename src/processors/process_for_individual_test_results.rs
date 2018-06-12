//!

use utils::string_utils::StringUtils;

/**

*/
pub struct ProcessIndividualTestResults {
}

impl ProcessIndividualTestResults {

    /**
        we have a list of strings from input (like stdin).
        we are looking only for lines that fit this pattern
        test tests::failing::failing_one ... FAILED
        - or -
        test tests::passing::pass_one ... ok
    */
    pub fn find_test_lines(input : &Vec<String>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();

        for line in input {
            let left: String = line.from_left(4);

            if "test" == left {
                results.push(line.to_string());
            }
        }

        return  results;
    }
}

#[cfg(test)]
mod process_for_individual_test_results_tests {

    use super::ProcessIndividualTestResults;

    #[test]
    fn input_is_empty() {
        let empty_vec: Vec<String> = Vec::new();

        let result_vec: Vec<String> = ProcessIndividualTestResults::find_test_lines(&empty_vec);

        assert_eq!(result_vec.len(), empty_vec.len());
    }

}