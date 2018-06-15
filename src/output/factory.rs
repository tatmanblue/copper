//!

use processors::types::OrganizedTestResults;
use output::console::ConsoleOutput;
use output::html_generator::HtmlOutput;
use output::output_trait::OutputTrait;

pub enum OutputTypes {
    Console(ConsoleOutput),
    Html(HtmlOutput),
}

impl OutputTrait for OutputTypes {
    fn generate(&self, test_results : &OrganizedTestResults) {
        match *self {
            OutputTypes::Console(ref console) => console.generate(test_results),
            OutputTypes::Html(ref html) => html.generate(test_results),
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
                eprintln!("Error in ComputationsFactory processing get for name of '{}'", name);
                panic!("Error in ComputationsFactory processing get request")
            },
        }
    }
}