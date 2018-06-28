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

use ansi_term::*;

use input::input_trait::InputTrait;
use input::factory::InputFactory;
use output::console::ConsoleOutput;
use output::factory::OutputFactory;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;
use processors::processing_controller::ProcessingController;
use shell::shell_trait::ShellTrait;
use utils::environment::Environment;
use utils::logger::init_log;


fn main() {
    init_log();

    debug!("copper has started ...");

    Environment::exit_if_print_help();
    Environment::exit_on_clean_up();

    trace!("reading configuration");
    let env:Environment = Environment::new();

    trace!("collecting input");
    let all_test_lines: Vec<String> = InputFactory::get(&env).read_all();
    let controller: ProcessingController = ProcessingController::new(all_test_lines);
    let organized_results = controller.execute();

    trace!("generating output");
    if true == env.include_console_format {
        OutputFactory::get("debug").generate(&organized_results).open();
    }

    OutputFactory::get(&env.output_format).generate(&organized_results).open();

}


