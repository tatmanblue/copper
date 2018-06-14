//!

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate ansi_term;
extern crate env_logger;
extern crate libc;
extern crate preferences;

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

pub mod input;
pub mod processors;
pub mod shell;
pub mod tests;
pub mod utils;

use std::env;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

use ansi_term::*;

use input::input_trait::InputTrait;
use input::from_stdin::StdReader;
use processors::process_for_individual_test_results::ProcessIndividualTestResults;
use utils::logger::init_log;

fn main() {
    init_log();

    debug!("rust-test-parser has started ...");

    let results: Vec<String> = StdReader::read_all();
    let results: Vec<String> = ProcessIndividualTestResults::find_test_lines(&results);

    println!("---------------------");
    println!("{:?}", results);

    let organized_results = ProcessIndividualTestResults::group_test_results(&results);

    organized_results.print();

}
