//!

use ansi_term::Color::*;

use output::output_trait::OutputTrait;
use processors::types::{OrganizedTestResults, IndividualTestResults};
use shell::factory::{ShellFactory, ShellTypes};

/**
*/
pub struct ConsoleOutput {

}

impl ConsoleOutput {
    fn print_section(name : &str, section: &Vec<IndividualTestResults>) {
        println!();
        println!("{} tests: {}", name, section.len());
        for test in section {
            println!("\t{}", test.name);
        }
    }
}

impl OutputTrait for ConsoleOutput {

    fn generate(&self, test_results : &OrganizedTestResults) -> ShellTypes {
        println!("----------------------------");
        ConsoleOutput::print_section("ignored", &test_results.skipped);
        ConsoleOutput::print_section(&Green.paint("success").to_string(), &test_results.success);
        ConsoleOutput::print_section(&Red.paint("failed").to_string(), &test_results.failed);
        println!();

        return ShellFactory::get(&"console" , "");
    }
}

