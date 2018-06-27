//! ShellTypes are the handlers of IO for properly display formatted test results
//! ShellTypes implement ShellTrait and open their display

use shell::shell_trait::ShellTrait;
use shell::browser::Browser;
use shell::console::Console;

/**
    ShellTypes enum
*/
pub enum ShellTypes {
    /**
        represents the standard console
    */
    Console(Console),
    /**
        represents html display in a browser
    */
    Browser(Browser),
}

impl ShellTrait for ShellTypes {
    /**
    */
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