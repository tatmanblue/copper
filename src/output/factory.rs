//!

use processors::types::OrganizedTestResults;
use output::console::ConsoleOutput;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;
use shell::factory::{ShellFactory, ShellTypes};

pub enum OutputTypes {
    Console(ConsoleOutput),
    Html(HtmlOutput),
}

impl OutputTrait for OutputTypes {
    fn generate(&self, test_results : &OrganizedTestResults) -> ShellTypes {
        match *self {
            OutputTypes::Console(ref console) => return console.generate(test_results),
            OutputTypes::Html(ref html) => return html.generate(test_results),
        }
    }
}

/**
*/
pub struct OutputFactory {}

impl OutputFactory {
    pub fn get(name : &str) -> OutputTypes {
        match name.as_ref() {
            "console" => OutputTypes::Console(ConsoleOutput {} ),
            "html" => OutputTypes::Html(HtmlOutput {} ),
            &_ => {
                eprintln!("Error in OutputFactory processing get for name of '{}'", name);
                panic!("Error in OutputFactory processing get request")
            },
        }
    }
}