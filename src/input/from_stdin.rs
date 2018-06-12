//!  InputTrait implementation for reading input via stdin (pipe)


use std::io::{self, Read, BufRead};
use input::input_trait::InputTrait;

pub struct StdReader { }

impl InputTrait for StdReader {
    /**
        reads all lines from stdin

        # Example
        ```ignore
        more results.txt | rust_test_parser
        ```
    */
    fn read_all() -> Vec<String> {

        let mut results: Vec<String> = Vec::new();

        let stdin = io::stdin();
        let lines_iterator = stdin.lock().lines();

        for line in lines_iterator {
            results.push(line.unwrap());
        }

        return results;
    }
}