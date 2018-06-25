//! for processing and organizing all lines with expectation they are cargo test output

use ansi_term::Color::*;
use utils::string_utils::StringUtils;

use processors::types::{IndividualTestResults, OrganizedTestResults};

enum LineTypes {
    UnitTest,
    IntegrationTest,
    DocTest,
    FailedTestDetails,
    WhoKnows
}

/**
    Functions for processing an array of strings (assuming from input like stdin) that
    pattern match cargo test output, and organizing the data into result tests
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

    /**
        looking for this section and returning all strings until failures line is encountered again

        failures:

        ---- tests::failing::failing_one stdout ----
            thread 'tests::failing::failing_one' panicked at 'this is a failing test', src/tests/failing.rs:4:5
        note: Run with `RUST_BACKTRACE=1` for a backtrace.

        ---- tests::failing::failing_two stdout ----
            thread 'tests::failing::failing_two' panicked at 'this is a failing test', src/tests/failing.rs:9:5

        until we find another failures: line
    */
    pub fn find_error_details_lines(input : &Vec<String>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let mut found_section: bool = false;

        for line in input {
            let left: String = line.trimmed().from_left(9);

            if "failures:" == left {
                if false == found_section {
                    found_section = true;
                } else {
                    found_section = false;
                }
                continue;
            }

            if true == found_section {
                results.push(line.to_string());
            }

        }

        return  results;
    }

    /**
        sorts input from likes of find_test_lines output into 3 categories
            success
            failed
            skipped

        test results look like this:
        test tests::passing::pass_one ... ok

        for failed tests, the additional data of the test are picked up

        failed tests include additional data in the multi-line format of

        ---- tests::failing::failing_one stdout ----
	        thread 'tests::failing::failing_one' panicked at 'this is a failing test', src/tests/failing.rs:4:5
        note: Run with `RUST_BACKTRACE=1` for a backtrace.

    */
    pub fn group_test_results(input : &Vec<String>) -> OrganizedTestResults {
        let mut results: OrganizedTestResults = OrganizedTestResults::new();

        for line in input {
            let split: Vec<&str> = line.split(" ").collect();

            if "..." == split[2] {

                let test_result: IndividualTestResults = IndividualTestResults {
                    name : split[1].to_string(),
                    result : split[3].to_string(),
                };

                match test_result.result.as_ref() {
                    "ok" => results.success.push(test_result),
                    "FAILED" => results.failed.push(test_result),
                    "ignored" =>  results.skipped.push(test_result),
                    _ => error!("this line didn't pattern match as expected '{:?}", test_result),
                }

            } else {
                debug!("What to do with this line '{}'", line);
            }
        }

        return results;
    }

    /**
    */
    pub fn merge_test_errors_into_results(input : &Vec<String>, mut results : &OrganizedTestResults) {

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

    #[test]
    fn small_failures_section_success(){
        let mut lines: Vec<String> = Vec::new();
        lines.push(str2string!("        failures:"));
        lines.push(str2string!("---- tests::failing::failing_one stdout ----"));
        lines.push(str2string!("thread 'tests::failing::failing_one' panicked at 'this is a failing test', src/tests/failing.rs:4:5"));
        lines.push(str2string!("note: Run with `RUST_BACKTRACE=1` for a backtrace."));
        lines.push(str2string!("---- tests::failing::failing_two stdout ----"));
        lines.push(str2string!("thread 'tests::failing::failing_two' panicked at 'this is a failing test', src/tests/failing.rs:9:5"));
        lines.push(str2string!("        failures:"));

        let results: Vec<String> = ProcessIndividualTestResults::find_error_details_lines(&lines);

        println!("results are => {:?}", results);
        assert!(5 == results.len());
    }

}