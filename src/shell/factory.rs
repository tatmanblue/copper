//!

use shell::shell_trait::ShellTrait;
use shell::browser::Browser;
use shell::console::Console;

pub enum ShellTypes {
    Console(Console),
    Browser(Browser),
}

impl ShellTrait for ShellTypes {
    fn open(&self) {
        match *self {
            ShellTypes::Console(ref console) => console.open(),
            ShellTypes::Browser(ref browser) => browser.open(),
        }
    }
}

/**
*/
pub struct ShellFactory {}

impl ShellFactory {
    pub fn get(name : &str, data : &str) -> ShellTypes {
        match name.as_ref() {
            "console" => ShellTypes::Console(Console {} ),
            "browser" => ShellTypes::Browser(Browser { file : data.to_string() } ),
            &_ => {
                eprintln!("Error in ShellFactory processing get for name of '{}'", name);
                panic!("Error in ShellFactory processing get request")
            },
        }
    }
}