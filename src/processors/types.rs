//!

use ansi_term::Color::*;

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
    pub fn new() -> OrganizedTestResults {
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
