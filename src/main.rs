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

use std::env;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

use env_logger::{Builder, fmt};
use log::{Record, Level, Metadata, Log, LevelFilter};

use input::input_trait::InputTrait;
use input::from_stdin::StdReader;
use processors::get_test_lines::ProcessForTestLines;


static LOGGER_INIT: Once = ONCE_INIT;

pub fn init_log() {
    LOGGER_INIT.call_once(|| {
        let format = |buf: &mut fmt::Formatter, record: &Record| {
            writeln!(
                buf,
                "{:>6}|{:>50} ({}:{}) -> {}",
                record.level(),
                record.target(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        };

        let mut builder = Builder::new();
        builder.format(format).filter(None, LevelFilter::Off);

        if env::var("RUST_LOG").is_ok() {
            builder.parse(&env::var("RUST_LOG").unwrap());
        }

        builder.init();

        trace!("logger initialized");
    });
}


fn main() {
    init_log();

    debug!("rust-test-parser has started ...");

    let results: Vec<String> = StdReader::read_all();
    let results = ProcessForTestLines::find_test_lines(results);

    println!("---------------------");
    println!("{:?}", results);
}
