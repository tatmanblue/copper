//!

use ansi_term::Color::*;
use utils::string_utils::StringUtils;

/**
    a single test result
*/
#[derive(Debug)]
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
pub struct OrganizedTestResults {
    pub success : Vec<IndividualTestResults>,
    pub failed : Vec<IndividualTestResults>,
    pub skipped : Vec<IndividualTestResults>,
}

impl OrganizedTestResults {
    /**
        Constructor
    */
    fn new() -> OrganizedTestResults {
        let success : Vec<IndividualTestResults> = Vec::new();
        let failed : Vec<IndividualTestResults> = Vec::new();
        let skipped : Vec<IndividualTestResults> = Vec::new();
        return OrganizedTestResults {  success, failed, skipped };
    }

    fn print_section(name : &str, section: &Vec<IndividualTestResults>) {
        println!();
        println!("{} tests: {}", name, section.len());
        for test in section {
            println!("\t{}", test.name);
        }
    }

    /**
        For now, pretty output to console
    */
    pub fn print(&self) {
        println!("----------------------------");
        OrganizedTestResults::print_section("ignored", &self.skipped);
        OrganizedTestResults::print_section(&Green.paint("success").to_string(), &self.success);
        OrganizedTestResults::print_section(&Red.paint("failed").to_string(), &self.failed);
        println!();
    }
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
        sorts input from likes of find_test_lines output into 3 categories
            success
            failed
            skipped
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
                    "ignored" => results.skipped.push(test_result),
                    _ => error!("this line didn't pattern match as expected '{:?}", test_result),
                }

            } else {
                debug!("What to do with this line '{}'", line);
            }
        }

        return results;
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