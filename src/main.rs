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

pub mod input;
pub mod output;
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
use output::console::ConsoleOutput;
use output::factory::OutputFactory;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;
use processors::process_for_individual_test_results::ProcessIndividualTestResults;
use shell::shell_trait::ShellTrait;
use utils::environment::Environment;
use utils::logger::init_log;


fn print_help() {
    println!();
    println!("{}", Color::Green.paint("rust-test-parser"));
    println!("\t by me");
    println!();
    println!("usage is pretty simple, assuming rust-test-parser is in your path. call it like this,");
    println!("in the same directory you ran {}:", Color::White.paint("cargo build"));
    println!("{}", Color::Blue.paint("\tcargo test | rust-test-parser"));
    println!();
}

fn main() {
    init_log();

    debug!("rust-test-parser has started ...");

    Environment::exit_if_print_help();

    trace!("reading configuration");

    trace!("collecting input");
    let results: Vec<String> = StdReader::read_all();
    let results: Vec<String> = ProcessIndividualTestResults::find_test_lines(&results);
    let organized_results = ProcessIndividualTestResults::group_test_results(&results);

    trace!("generating output");
    OutputFactory::get("debug").generate(&organized_results).open();
    OutputFactory::get("default").generate(&organized_results).open();

}


