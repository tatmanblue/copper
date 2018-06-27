//!

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use input::input_trait::InputTrait;

pub struct FileReader {
    pub file_name : String
}

impl InputTrait for FileReader {
    /**
        reads all lines from file

        # Example
        ```ignore
        more results.txt | rust_test_parser
        ```
    */
    fn read_all(&self) -> Vec<String> {

        let mut results: Vec<String> = Vec::new();
        let file_handle = match File::open(self.file_name.to_string()) {
            Ok(file) => file,
            Err(_e) => {
                panic!("input file was not accessible");
            }
        };

        let file_reader = BufReader::new(&file_handle);
        for line in file_reader.lines() {
            results.push(line.unwrap());
        }

        return results;
    }
}