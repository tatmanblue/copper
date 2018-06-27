//!

use input::from_file::FileReader;
use input::from_stdin::StdReader;
use input::input_trait::InputTrait;
use utils::environment::Environment;

pub enum InputTypes {
    StdIn(StdReader),
    File(FileReader),
}

impl InputTrait for InputTypes {
    fn read_all(&self) -> Vec<String> {
        match *self {
            InputTypes::StdIn(ref reader) => return reader.read_all(),
            InputTypes::File(ref reader) => return reader.read_all(),
        }
    }
}

/**
*/
pub struct InputFactory {}

impl InputFactory {
    pub fn get(env : &Environment) -> InputTypes {
        let mut name: String = "stdin".to_string();

        if true == env.read_from_file {
            name = "file".to_string();
        }

        match name.as_ref() {
            "stdin" => InputTypes::StdIn(StdReader {} ),
            "file" => InputTypes::File(FileReader { file_name: env.input_file_name.to_string() }),
            _ => {
                eprintln!("Error in OutputFactory processing get for name of '{}'", name);
                panic!("Error in OutputFactory processing get request")
            },
        }
    }
}