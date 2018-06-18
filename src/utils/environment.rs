//! Enviroment type is responsible for data that affects the running enviroment of rust-test-parser

use std::process::exit;
use std::env;

use ansi_term::*;

/**
    Contains, finds and builds any environmental data the user controls for running
    rust-test-parser
*/
pub struct Environment {
    /**
        expectation is this will be $HOME/.rust-test-parser/templates
    */
    pub template_dir : String,
    /**
        expectation is this will be $HOME/.rust-test-parser/results
    */
    pub results_dir : String,
    /**
        default is "default" or "html" aka web page is generated with the test results
    */
    pub output_format: String,
    /**
        default is false.  when true, output also appears in the console
    */
    pub include_console_format: bool,
}

impl Environment {
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

    fn get_working_dir() -> String {
        match env::current_dir() {
            Ok(path) => return path.to_str().unwrap().to_string(),
            Err(_e) => return ".".to_string(),
        }
    }

    /*
        should be $HOME/.rust-test-parser
        if there is an error then current working directory
    */
    fn get_home_dir() -> String {
        match env::home_dir() {
            Some(path) => return format!("{}{}", path.to_str().unwrap().to_string(), "/.rust-test-parser"),
            None => return Environment::get_working_dir(),
        }
    }

    /**
        Constructor
    */
    pub fn new() -> Environment {

        let home_dir : String = Environment::get_home_dir();
        let template_dir : String = format!("{}{}", home_dir, "/templates");
        let results_dir : String = format!("{}{}", home_dir, "/results");

        return Environment {
            template_dir,
            results_dir,
            output_format: "default".to_string(),
            include_console_format: false,
        };
    }

    /**
        static method, prints help and process exits if help is passed on the commandline
    */
    pub fn exit_if_print_help() {
        if env::args().find(|a| a == "-h" || a == "--help").is_some() {
            Environment::print_help();
            exit(0);
        }
    }
}