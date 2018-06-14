//!


use std::env;
use std::sync::{Once, ONCE_INIT};
use std::io::Write;

use ansi_term::*;
use env_logger::{Builder, fmt};
use log::{Record, Level, Metadata, Log, LevelFilter};

static LOGGER_INIT: Once = ONCE_INIT;

pub fn init_log() {
    LOGGER_INIT.call_once(|| {
        let format = |buf: &mut fmt::Formatter, record: &Record| {

            let record_level: String = record.level().to_string();

            if Level::Error == record.level() {
                return writeln!(
                    buf,
                    "{:>6}|{:>50} ({}:{}) -> {}",
                    Color::Red.paint(record_level),
                    record.target(),
                    record.file().unwrap(),
                    record.line().unwrap(),
                    record.args()
                );

            }

            writeln!(
                buf,
                "{:>6}|{:>50} ({}:{}) -> {}",
                record_level,
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
