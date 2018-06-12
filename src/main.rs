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

use input::input_trait::InputTrait;
use input::from_stdin::StdReader;

fn main() {

    debug!("rust-test-parser has started ...");

    let results: Vec<String> = StdReader::read_all();

    println!("---------------------");
    println!("{:?}", results);
}
