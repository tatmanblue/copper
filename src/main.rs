//!

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate preferences;
extern crate open;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tera;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

#[macro_use] pub mod utils;

pub mod input;
pub mod output;
pub mod processors;
pub mod shell;
pub mod tests;


use std::env;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

use ansi_term::*;

use input::input_trait::InputTrait;
use input::from_stdin::StdReader;
use output::console::ConsoleOutput;
use output::factory::OutputFactory;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;
use processors::process_for_individual_test_results::ProcessIndividualTestResults;
use shell::shell_trait::ShellTrait;
use utils::environment::Environment;
use utils::logger::init_log;


fn main() {
    init_log();

    debug!("rust-test-parser has started ...");

    Environment::exit_if_print_help();

    trace!("reading configuration");
    let env:Environment = Environment::new();

    trace!("collecting input");
    let all_test_lines: Vec<String> = StdReader::read_all();
    let summary_lines: Vec<String> = ProcessIndividualTestResults::find_summary_test_lines(&all_test_lines);
    let error_details: Vec<String> = ProcessIndividualTestResults::find_error_details_lines(&all_test_lines);
    let mut organized_results = ProcessIndividualTestResults::group_test_results(&summary_lines);

    ProcessIndividualTestResults::merge_test_errors_into_results(&error_details, &mut organized_results);

    trace!("generating output");
    if true == env.include_console_format {
        OutputFactory::get("debug").generate(&organized_results).open();
    }

    OutputFactory::get(&env.output_format).generate(&organized_results).open();

}


